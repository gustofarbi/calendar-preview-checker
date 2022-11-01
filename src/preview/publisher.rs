use tokio::sync::mpsc::Sender;

pub fn start(jobs_tx: Sender<u32>, ids: Vec<u32>) {
    tokio::spawn(async move {
        for id in ids {
            jobs_tx.send(id).await.unwrap();
        }
        drop(jobs_tx);
    });
}
