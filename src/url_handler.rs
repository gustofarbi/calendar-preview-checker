use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use reqwest::Client;
use tokio::sync::mpsc::Sender;

pub struct UrlHandler {
    id: u32,
    client: Client,
    break_loop: AtomicBool,
}

impl UrlHandler {
    pub fn new(
        id: u32,
        client: Client,
        break_loop: AtomicBool,
    ) -> Self {
        UrlHandler { id, client, break_loop }
    }

    pub async fn try_one(&self, url: String, results_tx: &Arc<Sender<u32>>) {
        if self.break_loop.load(Ordering::SeqCst) {
            return;
        }
        let response = self.client.head(url).send().await;
        if response.is_err() {
            self.break_loop.store(true, Ordering::SeqCst);
            results_tx.send(self.id).await.unwrap();
            return;
        }
        let response = response.unwrap();
        if !response.status().is_success() {
            self.break_loop.store(true, Ordering::SeqCst);
            results_tx.send(self.id).await.unwrap();
            return;
        }
    }
}
