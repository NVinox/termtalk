mod arguments;
mod ui;

#[tokio::main]
async fn main() {
    let _ = arguments::create();

    ui::create().expect("Create UI interface error");
}
