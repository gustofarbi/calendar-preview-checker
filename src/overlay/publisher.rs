use tokio::sync::mpsc::Sender;

use super::Item;

pub fn start_publisher(jobs_tx: Sender<(u32, String, String)>, items: Vec<Item>) {
    tokio::spawn(async move {
        for item in items {
            jobs_tx.send((item.id, item.path, item.hash)).await.unwrap();
        }
        drop(jobs_tx);
    });
}
