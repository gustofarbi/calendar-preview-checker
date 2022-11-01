use futures::StreamExt;

mod cmd;
mod handler;
mod overlay;
mod preview;
mod progress_bar;
mod receiver;

#[tokio::main]
async fn main() {
    let matches = cmd::get().get_matches();
    let (handler_instance, submatches) = handler::get(&matches).unwrap();

    handler_instance.handle(submatches).await;
}
