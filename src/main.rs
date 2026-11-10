mod ui;

#[tokio::main]
async fn main() {
    ui::ui().expect("Create UI interface error");
}
