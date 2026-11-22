mod arguments;
mod crypto;
mod key_reader;
mod terminal_guard;
mod ui;
mod websocket;

use ui::UI;
use websocket::Websocket;

#[tokio::main]
async fn main() {
    let args = arguments::create();
    let key = match key_reader::create() {
        Ok(key) => key,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    let websocket = Websocket::new(args.host, args.port);
    let (tx_to_network, tr_from_network) = websocket
        .connect()
        .await
        .expect("Error to connect websocket");
    let ui = UI::new(tx_to_network, tr_from_network, args.username, key);

    ui.create().await.expect("Create UI interface error");
}
