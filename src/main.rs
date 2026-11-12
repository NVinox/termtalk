mod arguments;
mod crypto;
mod ui;
mod websocket;

use ui::UI;
use websocket::Websocket;

#[tokio::main]
async fn main() {
    let args = arguments::create();
    let websocket = Websocket::new(args.host, args.port);
    let (tx_to_network, tr_from_network) = websocket
        .connect()
        .await
        .expect("Error to connect websocket");
    let ui = UI::new(tx_to_network, tr_from_network, args.username, args.key);

    ui.create().await.expect("Create UI interface error");
}
