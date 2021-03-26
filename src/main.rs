use futures::{FutureExt, StreamExt};
use packline_core::app::App;
use packline_core::connector::{Connector, TCPConnector};
use packline_flow::FlowConnector;
use tokio::net::{TcpStream};
use tokio_util::codec::{BytesCodec, Framed};
use std::borrow::Borrow;

mod core;

#[tokio::main]
async fn main() {
    let _ = tokio::spawn(async {
        let app = &mut packline_core::app::App {};

        //TODO: detect program shutdown step and send oneshot signal.
        let (_tx, rx) = tokio::sync::oneshot::channel();

        let mut connector = TCPConnector::new(Box::new(FlowConnector { app: &App {} }));
        connector
            .run(app, tokio::runtime::Handle::current(), &mut rx.fuse())
            .await;

        println!("After run!")
    }).await;
}

// async fn handle_client(stream: TcpStream) {
//     let mut framed = Framed::new(stream, BytesCodec::new());
//     println!("New client thread spawned");
//
//     let packet = framed.next().await;
//     println!("{:#?}", packet);
// }
