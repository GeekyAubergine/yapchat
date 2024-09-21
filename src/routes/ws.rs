use std::{
    borrow::Cow, collections::HashMap, net::SocketAddr, ops::ControlFlow, str::FromStr, sync::Arc,
    thread::sleep, time::Duration,
};

use axum::{
    extract::{
        ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade},
        ConnectInfo, Path, State,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::{headers::UserAgent, TypedHeader};
use futures::{stream::SplitSink, SinkExt, StreamExt};
use tokio::sync::{Mutex, RwLock};
use tracing::{debug, info};
use uuid::Uuid;

use crate::infrastructure::appstate::AppState;

#[derive(Debug, Clone)]
pub struct WsClients {
    clients: Arc<RwLock<HashMap<Uuid, SplitSink<WebSocket, Message>>>>,
}

impl WsClients {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_client(&self, id: Uuid, ws: SplitSink<WebSocket, Message>) {
        info!("Adding client: {:?}", id);
        self.clients.write().await.insert(id, ws);
    }

    pub async fn remove_client(&self, id: Uuid) {
        self.clients.write().await.remove(&id);
    }

    pub async fn send(&self, uuid: &Uuid, msg: String) -> ControlFlow<()> {
        let mut clients = self.clients.write().await;

        if let Some(mut client) = clients.get_mut(&uuid) {
        info!("Sending message to client: {:?}", uuid);
            if client.send(Message::Text(msg)).await.is_err() {
                self.clients.write().await.remove(&uuid);
                return ControlFlow::Break(());
            }
        }

        ControlFlow::Continue(())
    }

    pub async fn broadcast(&self, msg: String) {
        let clients = self.clients.read().await;

        for client in clients.keys() {
            self.send(client, msg.clone()).await;
        }
    }
}

// async fn websocket_handler(
//     ws: WebSocketUpgrade,
//     Extension(state): Extension<AppState>,
// ) -> impl IntoResponse {
//     ws.on_upgrade(|socket| websocket(socket, state))
// }

// async fn websocket(mut stream: WebSocket, state: AppState) {
//     // By splitting we can send and receive at the same time.
//     let (mut sender, mut receiver) = stream.split();

//     // let mut rx = {
//     //     let mut state = state.lock().await;
//     //     state.clients_count += 1;
//     //     state.rx.clone()
//     // };

//     // This task will receive watch messages and forward it to this connected client.
//     let mut send_task = tokio::spawn(async move {
//         loop {
//             // let msg = rx.borrow().clone();

//             let msg = Message::Text("Hello, World!".into());

//             if stream.send(msg).await.is_err() {
//                 break;
//             }

//             sleep(Duration::from_secs(2));
//         }
//     });

//     // This task will receive messages from this client.
//     // let mut recv_task = tokio::spawn(async move {
//     //     while let Some(Ok(Message::Text(text))) = receiver.next().await {
//     //         println!("this example does not read any messages, but got: {text}");
//     //     }
//     // });

//     // // If any one of the tasks exit, abort the other.
//     // tokio::select! {
//     //     _ = (&mut send_task) => recv_task.abort(),
//     //     _ = (&mut recv_task) => send_task.abort(),
//     // };

//     // This client disconnected
//     // state.lock().await.clients_count -= 1;
// }

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(user_uuid): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, user_uuid, state))
}

async fn handle_socket(mut socket: WebSocket, user_uuid: String, state: AppState) {
    let (mut sender, mut reciever) = socket.split();

    let uuid = Uuid::from_str(&user_uuid).unwrap();

    state.ws_clients().add_client(uuid, sender).await;

    while let Some(msg) = reciever.next().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            state.ws_clients().remove_client(uuid).await;
            return;
        };
    }
}
// async fn ws_handler(
//     ws: WebSocketUpgrade,
//     user_agent: Option<TypedHeader<UserAgent>>,
//     ConnectInfo(addr): ConnectInfo<SocketAddr>,
//     State(state): State<AppState>,
// ) -> impl IntoResponse {
//     println!("New websocket connection from {addr}...");
//     let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
//         user_agent.to_string()
//     } else {
//         String::from("Unknown browser")
//     };
//     println!("`{user_agent}` at {addr} connected.");
//     // finalize the upgrade process by returning upgrade callback.
//     // we can customize the callback by sending additional info such as address.
//     ws.on_upgrade(move |socket| handle_socket(socket, addr, state))
// }

// /// Actual websocket statemachine (one will be spawned per connection)
// async fn handle_socket(mut socket: WebSocket, who: SocketAddr, state: AppState) {
//     // send a ping (unsupported by some browsers) just to kick things off and get a response
//     if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
//         println!("Pinged {who}...");
//     } else {
//         println!("Could not send ping {who}!");
//         // no Error here since the only thing we can do is to close the connection.
//         // If we can not send messages, there is no way to salvage the statemachine anyway.
//         return;
//     }

//     // receive single message from a client (we can either receive or send with socket).
//     // this will likely be the Pong for our Ping or a hello message from client.
//     // waiting for message from a client will block this task, but will not block other client's
//     // connections.
//     if let Some(msg) = socket.recv().await {
//         if let Ok(msg) = msg {
//             if process_message(msg, who).is_break() {
//                 return;
//             }
//         } else {
//             println!("client {who} abruptly disconnected");
//             return;
//         }
//     }

//     // Since each client gets individual statemachine, we can pause handling
//     // when necessary to wait for some external event (in this case illustrated by sleeping).
//     // Waiting for this client to finish getting its greetings does not prevent other clients from
//     // connecting to server and receiving their greetings.
//     for i in 1..5 {
//         if socket
//             .send(Message::Text(format!("Hi {i} times!")))
//             .await
//             .is_err()
//         {
//             println!("client {who} abruptly disconnected");
//             return;
//         }
//         tokio::time::sleep(std::time::Duration::from_millis(100)).await;
//     }

//     // By splitting socket we can send and receive at the same time. In this example we will send
//     // unsolicited messages to client based on some sort of server's internal event (i.e .timer).

//     // Spawn a task that will push several messages to the client (does not matter what client does)
//     let mut send_task = tokio::spawn(async move {
//         let n_msg = 20;
//         for i in 0..n_msg {
//             // In case of any websocket error, we exit.
//             if socket
//                 .send(Message::Text(format!("Server message {i} ...")))
//                 .await
//                 .is_err()
//             {
//                 return i;
//             }

//             tokio::time::sleep(std::time::Duration::from_millis(300)).await;
//         }

//         println!("Sending close to {who}...");
//         if let Err(e) = socket
//             .send(Message::Close(Some(CloseFrame {
//                 code: axum::extract::ws::close_code::NORMAL,
//                 reason: Cow::from("Goodbye"),
//             })))
//             .await
//         {
//             println!("Could not send Close due to {e}, probably it is ok?");
//         }
//         n_msg
//     });

//     /// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
//     /// of websocket negotiation). After this completes, the actual switching from HTTP to
//     /// websocket protocol will occur.
//     /// This is the last point where we can extract TCP/IP metadata such as IP address of the client
//     /// as well as things from HTTP headers such as user-agent of the browser etc.
//     async fn ws_handler(
//         ws: WebSocketUpgrade,
//         user_agent: Option<TypedHeader<UserAgent>>,
//         ConnectInfo(addr): ConnectInfo<SocketAddr>,
//     ) -> impl IntoResponse {
//         let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
//             user_agent.to_string()
//         } else {
//             String::from("Unknown browser")
//         };
//         println!("`{user_agent}` at {addr} connected.");
//         // finalize the upgrade process by returning upgrade callback.
//         // we can customize the callback by sending additional info such as address.
//         ws.on_upgrade(move |socket| handle_socket(socket, addr))
//     }

//     /// Actual websocket statemachine (one will be spawned per connection)
//     async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
//         // send a ping (unsupported by some browsers) just to kick things off and get a response
//         if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
//             println!("Pinged {who}...");
//         } else {
//             println!("Could not send ping {who}!");
//             // no Error here since the only thing we can do is to close the connection.
//             // If we can not send messages, there is no way to salvage the statemachine anyway.
//             return;
//         }

//         // receive single message from a client (we can either receive or send with socket).
//         // this will likely be the Pong for our Ping or a hello message from client.
//         // waiting for message from a client will block this task, but will not block other client's
//         // connections.
//         if let Some(msg) = socket.recv().await {
//             if let Ok(msg) = msg {
//                 if process_message(msg, who).is_break() {
//                     return;
//                 }
//             } else {
//                 println!("client {who} abruptly disconnected");
//                 return;
//             }
//         }

//         // Since each client gets individual statemachine, we can pause handling
//         // when necessary to wait for some external event (in this case illustrated by sleeping).
//         // Waiting for this client to finish getting its greetings does not prevent other clients from
//         // connecting to server and receiving their greetings.
//         for i in 1..5 {
//             if socket
//                 .send(Message::Text(format!("Hi {i} times!")))
//                 .await
//                 .is_err()
//             {
//                 println!("client {who} abruptly disconnected");
//                 return;
//             }
//             tokio::time::sleep(std::time::Duration::from_millis(100)).await;
//         }

//         // By splitting socket we can send and receive at the same time. In this example we will send
//         // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
//         let (mut sender, mut receiver) = socket.split();

//         // Spawn a task that will push several messages to the client (does not matter what client does)
//         let mut send_task = tokio::spawn(async move {
//             let n_msg = 20;
//             for i in 0..n_msg {
//                 // In case of any websocket error, we exit.
//                 if sender
//                     .send(Message::Text(format!("Server message {i} ...")))
//                     .await
//                     .is_err()
//                 {
//                     return i;
//                 }

//                 tokio::time::sleep(std::time::Duration::from_millis(300)).await;
//             }

//             println!("Sending close to {who}...");
//             if let Err(e) = sender
//                 .send(Message::Close(Some(CloseFrame {
//                     code: axum::extract::ws::close_code::NORMAL,
//                     reason: Cow::from("Goodbye"),
//                 })))
//                 .await
//             {
//                 println!("Could not send Close due to {e}, probably it is ok?");
//             }
//             n_msg
//         });

//         // This second task will receive messages from client and print them on server console
//         let mut recv_task = tokio::spawn(async move {
//             let mut cnt = 0;
//             while let Some(Ok(msg)) = receiver.next().await {
//                 cnt += 1;
//                 // print message and break if instructed to do so
//                 if process_message(msg, who).is_break() {
//                     break;
//                 }
//             }
//             cnt
//         });

//         // If any one of the tasks exit, abort the other.
//         tokio::select! {
//             rv_a = (&mut send_task) => {
//                 match rv_a {
//                     Ok(a) => println!("{a} messages sent to {who}"),
//                     Err(a) => println!("Error sending messages {a:?}")
//                 }
//                 recv_task.abort();
//             },
//             rv_b = (&mut recv_task) => {
//                 match rv_b {
//                     Ok(b) => println!("Received {b} messages"),
//                     Err(b) => println!("Error receiving messages {b:?}")
//                 }
//                 send_task.abort();
//             }
//         }

//         // returning from the handler closes the websocket connection
//         println!("Websocket context {who} destroyed");
//     }

//     /// helper to print contents of messages to stdout. Has special treatment for Close.
//     fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
//         match msg {
//             Message::Text(t) => {
//                 println!(">>> {who} sent str: {t:?}");
//             }
//             Message::Binary(d) => {
//                 println!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
//             }
//             Message::Close(c) => {
//                 if let Some(cf) = c {
//                     println!(
//                         ">>> {} sent close with code {} and reason `{}`",
//                         who, cf.code, cf.reason
//                     );
//                 } else {
//                     println!(">>> {who} somehow sent close message without CloseFrame");
//                 }
//                 return ControlFlow::Break(());
//             }

//             Message::Pong(v) => {
//                 println!(">>> {who} sent pong with {v:?}");
//             }
//             // You should never need to manually handle Message::Ping, as axum's websocket library
//             // will do so for you automagically by replying with Pong and copying the v according to
//             // spec. But if you need the contents of the pings you can see them here.
//             Message::Ping(v) => {
//                 println!(">>> {who} sent ping with {v:?}");
//             }
//         }
//         ControlFlow::Continue(())
//     }
// }
