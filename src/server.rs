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
    
    /// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
    /// of websocket negotiation). After this completes, the actual switching from HTTP to
    /// websocket protocol will occur.
    /// This is the last point where we can extract TCP/IP metadata such as IP address of the client
    /// as well as things from HTTP headers such as user-agent of the browser etc.
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
        // finalize the upgrade process by returning upgrade callback.
        // we can customize the callback by sending additional info such as address.
        ws.on_upgrade(move |socket| Self::handle_socket(socket, addr))
    }
    
    /// Actual websocket statemachine (one will be spawned per connection)
    async fn handle_socket(socket: WebSocket, who: SocketAddr) {
        // By splitting socket we can send and receive at the same time. In this example we will send
        // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
        let (mut sender, mut receiver) = socket.split();
    
        // Spawn a task that will push several messages to the client (does not matter what client does)
        // let mut send_task = tokio::spawn(async move {
        //     let n_msg = 20;
        //     for i in 0..n_msg {
        //         // In case of any websocket error, we exit.
        //         if sender
        //             .send(Message::Text(format!("Server message {i} ...")))
        //             .await
        //             .is_err()
        //         {
        //             return i;
        //         }
    
        //         tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        //     }
        //     println!("Sending close to {who}...");
        //     if let Err(e) = sender
        //         .send(Message::Close(Some(CloseFrame {
        //             code: axum::extract::ws::close_code::NORMAL,
        //             reason: Cow::from("Goodbye"),
        //         })))
        //         .await
        //     {
        //         println!("Could not send Close due to {}, probably it is ok?", e);
        //     }
        //     n_msg
        // });
    
        // FIXME: There should be a better way to do this
        // This second task will receive messages from client and print them on server console
        let mut recv_task = tokio::spawn(async move {
            let mut cnt = 0;
            while let Some(Ok(msg)) = receiver.next().await {
                cnt += 1;
                // print message and break if instructed to do so
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
    
        // If any one of the tasks exit, abort the other.
        tokio::select! {
            // rv_a = (&mut send_task) => {
            //     match rv_a {
            //         Ok(_) => println!("Messages sent to {}", who),
            //         Err(a) => println!("Error sending messages {:?}", a)
            //     }
            //     recv_task.abort();
            // },
            rv_b = (&mut recv_task) => {
                match rv_b {
                    Ok(b) => println!("Received {} messages", b),
                    Err(b) => println!("Error receiving messages {:?}", b)
                }
                // send_task.abort();
            }
        }
    
        // returning from the handler closes the websocket connection
        println!("Websocket context {} destroyed", who);
    }
    
    /// helper to print contents of messages to stdout. Has special treatment for Close.
    fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), Vec<u8>> {
        match msg {
            Message::Text(t) => {
                println!(">>> {} sent str: {:?}", who, t);
            }
            Message::Binary(d) => {
                let enc: Vec<u8>;
                // println!(">>> {} sent {} bytes", who, d.len());
                if let Ok(res) = parse_ws_request(d) {
                    enc = rmp_serde::encode::to_vec_named(&res).unwrap();
                    // println!(">>> Parsed and MessagePack encoded");
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
    
            Message::Pong(v) => {
                println!(">>> {} sent pong with {:?}", who, v);
            }
            // You should never need to manually handle Message::Ping, as axum's websocket library
            // will do so for you automagically by replying with Pong and copying the v according to
            // spec. But if you need the contents of the pings you can see them here.
            Message::Ping(v) => {
                println!(">>> {} sent ping with {:?}", who, v);
            }
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