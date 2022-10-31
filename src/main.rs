use futures::StreamExt;
use preview::Preview;

mod cmd;
mod preview;
mod handler;

#[tokio::main]
async fn main() {
    let matches = cmd::get().get_matches();
    let (handler_instance, submatches) = handler::get(&matches).unwrap();

    handler_instance.handle(submatches).await;
}
