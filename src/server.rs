use std::{net::SocketAddr, ops::ControlFlow};

use axum::{
    Router, routing::{get, post}, response::{IntoResponse, Response},
    http::{StatusCode, Uri, header}, Json, async_trait,
    extract::{
        FromRequestParts, Query, TypedHeader, connect_info::ConnectInfo,
        ws::{Message, WebSocket, WebSocketUpgrade}
    },
    body::{Bytes, boxed, Full},
    headers::UserAgent
};
use axum_msgpack::MsgPackRaw;
use mime_guess;
use rust_embed::RustEmbed;
use serde::{Serialize, Deserialize};
use tower_http::{cors::CorsLayer, compression::CompressionLayer};
use futures::{sink::SinkExt, stream::StreamExt};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

use crate::{parse_log_bytes, parse_ws_request};

static INDEX_HTML: &str = "index.html";

#[cfg(debug_assertions)]
static PORT: u16 = 3031;
#[cfg(not(debug_assertions))]
static PORT: u16 = 3030;

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
    async fn static_handler(uri: Uri) -> impl IntoResponse {
        let path = uri.path().trim_start_matches('/').to_string();
    
        if path.is_empty() || path == INDEX_HTML {
            return Self::index_html().await.into_response();
        }
    
        match Static::get(path.as_str()) {
            Some(content) => {
                let body = boxed(Full::from(content.data));
                let mime = mime_guess::from_path(path).first_or_octet_stream();
    
                Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .body(body)
                    .unwrap()
            }
            None => {
                if path.contains('.') {
                    return Self::not_found().await.into_response();
                }
    
                Self::index_html().await.into_response()
            }
        }
    }
    
    async fn index_html() -> impl IntoResponse {
        match Static::get(INDEX_HTML) {
            Some(content) => {
                let body = boxed(Full::from(content.data));
    
                Response::builder()
                    .header(header::CONTENT_TYPE, "text/html")
                    .body(body)
                    .unwrap()
            }
            None => Self::not_found().await.into_response(),
        }
    }
    
    async fn not_found() -> impl IntoResponse {
        (StatusCode::NOT_FOUND, "404")
    }
    
    async fn ws_handler(
        ws: WebSocketUpgrade,
        user_agent: Option<TypedHeader<UserAgent>>,
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ) -> impl IntoResponse {
        let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
            user_agent.to_string()
        } else {
            String::from("Unknown browser")
        };
        println!("`{user_agent}` at {addr} connected.");
        ws.on_upgrade(move |socket| Self::handle_socket(socket, addr))
    }
    
    async fn handle_socket(socket: WebSocket, who: SocketAddr) {
        let (mut sender, mut receiver) = socket.split();
    
        // FIXME: There should be a better way to do this
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
                    Ok(b) => println!("Received {} messages", b),
                    Err(b) => println!("Error receiving messages {:?}", b)
                }
            }
        }
    
        // returning from the handler closes the websocket connection
        println!("Websocket context {} destroyed", who);
    }
    
    /// helper to print contents of messages to stdout. Has special treatment for Close.
    fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), Vec<u8>> {
        match msg {
            Message::Binary(d) => {
                let enc: Vec<u8>;
                if let Ok(res) = parse_ws_request(d) {
                    enc = rmp_serde::encode::to_vec_named(&res).unwrap();
                } else {
                    enc = Vec::new();
                }
                return ControlFlow::Continue(enc);
            }
            Message::Close(c) => {
                if let Some(cf) = c {
                    println!(
                        ">>> {} sent close with code {} and reason `{}`",
                        who, cf.code, cf.reason
                    );
                } else {
                    println!(">>> {} somehow sent close message without CloseFrame", who);
                }
                return ControlFlow::Break(());
            }
            _ => (),
        }
        ControlFlow::Continue(Vec::new())
    }

    pub async fn start() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();

        let single_upload = Router::new()
            .route("/v1/upload", post(Self::upload_log))
            .layer(CorsLayer::permissive())
            .layer(CompressionLayer::new().gzip(true).no_br().no_zstd());

        let multi_upload_ws = Router::new()
            .route("/v1/upload_multi", get(Self::ws_handler));

        let app = Router::new()
            .fallback(Self::static_handler)
            .nest("/api", single_upload)
            .nest("/ws", multi_upload_ws)
            .layer(
                TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
            );

        let addr = SocketAddr::from(([127, 0, 0, 1], PORT));

        tracing::info!("Cambia server listening on http://{}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>())
            .await
            .unwrap();
    }

    async fn upload_log(fmt: Format, bytes: Bytes) -> impl IntoResponse {
        let bytes_vec = bytes.to_vec();
        match parse_log_bytes(bytes_vec) {
            Ok(parsed) => {
                println!("{}", serde_json::to_string(&parsed).unwrap());
                (StatusCode::OK, fmt.render(parsed))
            },
            Err(e) => (StatusCode::BAD_REQUEST, e.to_string().into_response()),
        }
    }
}