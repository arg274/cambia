use std::{net::SocketAddr, ops::ControlFlow};
use std::ops::RangeInclusive;
use axum::{body::{Body, Bytes}, extract::{
    connect_info::ConnectInfo, ws::{Message, WebSocket, WebSocketUpgrade}, FromRequestParts, Query
}, http::{header, StatusCode, Uri}, response::{IntoResponse, Response}, routing::{get, post}, Extension, Json, Router};
use axum_extra::{headers::UserAgent, TypedHeader};
use axum_msgpack::MsgPackRaw;
use rust_embed::RustEmbed;
use serde::{Serialize, Deserialize};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, compression::CompressionLayer};
use futures::{sink::SinkExt, stream::StreamExt};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use axum_client_ip::{InsecureClientIp, SecureClientIp, SecureClientIpSource};
use cambia_core::error::CambiaError;
use cambia_core::handler::{parse_log_bytes, translate_log_bytes};
use cambia_core::response::CambiaResponse;
use crate::Args;
use crate::util::save_rip_log;

static INDEX_HTML: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "web/build/"]
struct Static;

enum Format {
    Json,
    MsgPack,
}

impl Format {
    fn render<T>(self, data: T) -> Response
    where
        T: Serialize,
    {
        match self {
            Format::Json => Json(data).into_response(),
            Format::MsgPack => MsgPackRaw(data).into_response()
        }
    }
}

impl<S> FromRequestParts<S> for Format
where
    S: Send + Sync, {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut axum::http::request::Parts, state: &S) -> Result<Self, Self::Rejection> {
        #[derive(Deserialize)]
        struct FormatQuery {
            fmt: String,
        }

        let Query(query) = match Query::<FormatQuery>::from_request_parts(parts, state).await {
            Ok(query) => query,
            Err(_) => return Ok(Self::Json),
        };

        if query.fmt == "msgpack" {
            Ok(Self::MsgPack)
        } else {
            Ok(Self::Json)
        }
    }
}

// TODO: Check for security implications
pub struct CambiaServer {
    args: Args
}

impl CambiaServer {
    pub fn new(args: Args) -> Self {
        Self{
            args
        }
    }

    fn init_app(self) -> Router {
        let single_upload = Router::new()
            .route("/v1/upload", post(Self::upload_log))
            .route("/v1/translate", post(Self::translate_log))
            .layer(CorsLayer::permissive())
            .layer(CompressionLayer::new().gzip(true).no_br().no_zstd());

        let multi_upload_ws = Router::new()
            .route("/v1/upload_multi", get(Self::ws_handler));

        Router::new()
            .fallback(Self::static_handler)
            .nest("/api", single_upload)
            .nest("/ws", multi_upload_ws)
            .layer(Extension(self.args))
            .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true)))
            .layer(SecureClientIpSource::ConnectInfo.into_extension())
    }

    async fn static_handler(uri: Uri, user_agent: Option<TypedHeader<UserAgent>>, insecure_ip: InsecureClientIp, secure_ip: SecureClientIp) -> impl IntoResponse {
        let path = uri.path().trim_start_matches('/').to_string();

        // Avoid log spamming on asset requests
        let extension = std::path::Path::new(&path).extension().and_then(std::ffi::OsStr::to_str);
        if extension.is_none() {
            let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
                user_agent.to_string()
            } else {
                String::from("Unknown browser")
            };
            tracing::info!("/{path} from UserAgent({user_agent}) | {insecure_ip:?} | {secure_ip:?}");
        }

        if path.is_empty() || path == INDEX_HTML {
            return Self::index_html().await.into_response();
        }

        match Static::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();

                Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .body(Body::from(content.data))
                    .unwrap()
            }
            None => {
                Self::index_html().await.into_response()
            }
        }
    }

    async fn index_html() -> impl IntoResponse {
        match Static::get(INDEX_HTML) {
            Some(content) => {
                let body = Body::from(content.data);

                Response::builder()
                    .header(header::CONTENT_TYPE, "text/html")
                    .body(body)
                    .unwrap()
            }
            None => {
                Self::not_found().await.into_response()
            },
        }
    }

    async fn not_found() -> impl IntoResponse {
        (StatusCode::NOT_FOUND, "404")
    }

    async fn ws_handler(
        Extension(args): Extension<Args>,
        ws: WebSocketUpgrade,
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ) -> impl IntoResponse {
        ws.on_upgrade(move |socket| Self::handle_socket(args, socket, addr))
    }

    async fn handle_socket(args: Args, socket: WebSocket, who: SocketAddr) {
        let (mut sender, mut receiver) = socket.split();

        // TODO: There should be a better way to do this
        let mut recv_task = tokio::spawn(async move {
            let mut cnt = 0;
            while let Some(Ok(msg)) = receiver.next().await {
                cnt += 1;
                let processed = Self::process_message(&args, msg, who);
                if processed.is_break() {
                    break;
                } else if let ControlFlow::Continue(val) = processed {
                    if sender.send(Message::Binary(Bytes::from(val))).await.is_err() {
                        return cnt;
                    }
                }
            }
            cnt
        });

        tokio::select! {
            rv_b = (&mut recv_task) => {
                match rv_b {
                    Ok(b) => tracing::trace!("Received {} messages", b),
                    Err(b) => tracing::trace!("Error receiving messages {:?}", b)
                }
            }
        }

        tracing::trace!("Websocket context {} destroyed", who);
    }

    fn process_message(args: &Args, msg: Message, who: SocketAddr) -> ControlFlow<(), Vec<u8>> {
        match msg {
            Message::Binary(d) => {
                let enc: Vec<u8> = match Self::parse_ws_request(args, d.to_vec()) {
                    Ok(res) => rmp_serde::encode::to_vec_named(&res).unwrap(),
                    Err(e) => rmp_serde::encode::to_vec_named(&e).unwrap(),
                };
                return ControlFlow::Continue(enc);
            }
            Message::Close(c) => {
                if let Some(cf) = c {
                    tracing::trace!(
                        ">>> {} sent close with code {} and reason `{}`",
                        who, cf.code, cf.reason
                    );
                } else {
                    tracing::warn!(">>> {} somehow sent close message without CloseFrame", who);
                }
                return ControlFlow::Break(());
            }
            _ => (),
        }
        ControlFlow::Continue(Vec::new())
    }

    fn parse_ws_request(args: &Args, mut ws_body: Vec<u8>) -> Result<CambiaResponse, CambiaError> {
        // xxH64 is 8 bytes
        if ws_body.len() < 8 {
            return Err(CambiaError::new_anon("WS message length too small"));
        }

        let log_bytes = ws_body.split_off(8);
        let res = parse_log_bytes(ws_body, &log_bytes);

        if let Some(save_logs) = args.save_logs.clone() {
            if let Ok(ref res) = res {
                save_rip_log(save_logs, &res.id, &log_bytes);
            }
        }

        res
    }

    pub async fn start(self) {
        let port = self.args.port.clone();

        let app = self.init_app();
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();

        tracing::info!("Cambia server listening on http://localhost:{}", listener.local_addr().unwrap().port());
        axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    }

    async fn upload_log(fmt: Format, bytes: Bytes) -> impl IntoResponse {
        let bytes_vec = bytes.to_vec();
        match parse_log_bytes(Vec::new(), &bytes_vec) {
            Ok(parsed) => {
                tracing::debug!("{}", serde_json::to_string(&parsed).unwrap());
                (StatusCode::OK, fmt.render(parsed))
            },
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string().into_response()),
        }
    }

    async fn translate_log(bytes: Bytes) -> impl IntoResponse {
        let bytes_vec = bytes.to_vec();

        match translate_log_bytes(bytes_vec) {
            Ok(parsed) => (StatusCode::OK, parsed.into_response()),
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string().into_response()),
        }
    }
}

pub fn port_in_range(s: &str) -> Result<String, String> {
    const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

    let port: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a port number"))?;
    if PORT_RANGE.contains(&port) {
        Ok(port.to_string())
    } else {
        Err(format!("port not in range {}-{}", PORT_RANGE.start(), PORT_RANGE.end()))
    }
}
