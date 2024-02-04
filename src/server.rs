use std::{net::SocketAddr, ops::ControlFlow};

use axum::{
    async_trait, body::{Body, Bytes}, extract::{
        connect_info::ConnectInfo, ws::{Message, WebSocket, WebSocketUpgrade}, FromRequestParts, Query
    }, http::{header, StatusCode, Uri}, response::{IntoResponse, Response}, routing::{get, post}, Json, Router
};
use axum_extra::{headers::UserAgent, TypedHeader};
use axum_msgpack::MsgPackRaw;
use mime_guess;
use rust_embed::RustEmbed;
use serde::{Serialize, Deserialize};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, compression::CompressionLayer};
use futures::{sink::SinkExt, stream::StreamExt};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use axum_client_ip::{InsecureClientIp, SecureClientIp, SecureClientIpSource};

use crate::{handler::{parse_log_bytes, parse_ws_request, translate_log_bytes}, util::env_getter, Args};

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

#[async_trait]
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
pub struct CambiaServer;

impl CambiaServer {
    fn init_logging(tracing: &Option<String>) {
        let tracing_level = match tracing {
            Some(v) => {
                match v.to_ascii_lowercase().as_str() {
                    "trace" => tracing::Level::TRACE,
                    "debug" => tracing::Level::DEBUG,
                    "warn" => tracing::Level::WARN,
                    "error" => tracing::Level::ERROR,
                    _ => tracing::Level::INFO,
                }
            },
            None => tracing::Level::INFO,
        };

        tracing_subscriber::fmt()
            .with_max_level(tracing_level)
            .init();
    }

    fn init_app() -> Router {
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
            .layer(
                TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
            )
            .layer(SecureClientIpSource::ConnectInfo.into_extension())
    }

    // TODO: This breaks immutability of the config
    fn init_env(args: &Args) {
        // Port
        std::env::set_var("CAMBIA_PORT", args.port.clone());
        // Log saving
        if args.save_logs {
            tracing::info!("Log saving is enabled");
            std::env::set_var("CAMBIA_SAVE_LOGS", args.save_logs.to_string());
        }
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
        ws: WebSocketUpgrade,
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ) -> impl IntoResponse {
        ws.on_upgrade(move |socket| Self::handle_socket(socket, addr))
    }
    
    async fn handle_socket(socket: WebSocket, who: SocketAddr) {
        let (mut sender, mut receiver) = socket.split();
    
        // TODO: There should be a better way to do this
        let mut recv_task = tokio::spawn(async move {
            let mut cnt = 0;
            while let Some(Ok(msg)) = receiver.next().await {
                cnt += 1;
                let processed = Self::process_message(msg, who);
                if processed.is_break() {
                    break;
                } else if let ControlFlow::Continue(val) = processed {
                    if sender.send(Message::Binary(val)).await.is_err() {
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
    
    fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), Vec<u8>> {
        match msg {
            Message::Binary(d) => {
                let enc: Vec<u8> = match parse_ws_request(d) {
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

    pub async fn start(args: Args) {
        Self::init_logging(&args.tracing);
        Self::init_env(&args);

        let app = Self::init_app();
        let listener = TcpListener::bind(format!("0.0.0.0:{}", env_getter("CAMBIA_PORT", crate::consts::DEFAULT_PORT))).await.unwrap();

        tracing::info!("Cambia server listening on http://localhost:{}", listener.local_addr().unwrap().port());
        axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    }

    // FIXME: WebSockets API is broken
    pub fn start_shuttle() -> Router {
        Self::init_app()
    }

    async fn upload_log(fmt: Format, bytes: Bytes) -> impl IntoResponse {
        let bytes_vec = bytes.to_vec();
        match parse_log_bytes(Vec::new(), bytes_vec) {
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
            Ok(parsed) => {
                (StatusCode::OK, parsed.into_response())
            },
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string().into_response()),
        }
    }
}