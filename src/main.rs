mod arguments;
mod ui;
mod websocket;

use websocket::Websocket;

#[tokio::main]
async fn main() {
    let args = arguments::create();
    let websocket = Websocket::new(args.host, args.port);
    let (tx_to_network, tr_from_network) = websocket.connect().await;

    ui::create(tx_to_network, tr_from_network, &args.username)
        .await
        .expect("Create UI interface error");
}
