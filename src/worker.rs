use std::time::Duration;

use futures::stream;
use reqwest::Client;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::StreamExt;
use crate::url::build_urls;

pub struct Worker {
    results_tx: Sender<u32>,
    mounting: String,
    concurrency: usize,
}

impl Worker {
    pub fn new(results_tx: Sender<u32>, mounting: String, concurrency: usize) -> Self {
        Worker {
            results_tx,
            mounting,
            concurrency,
        }
    }

    pub async fn start(&self, jobs_rx: Receiver<u32>) {
        tokio_stream::wrappers::ReceiverStream::new(jobs_rx)
            .for_each_concurrent(self.concurrency, |id| async move {
                let client = Client::builder()
                    .timeout(Duration::from_secs(3))
                    .build()
                    .unwrap();

                stream::iter(build_urls(id, &self.mounting))
                    .for_each_concurrent(self.concurrency, |url| async {
                        let response = client.head(url).send().await;
                        if response.is_err() {
                            self.results_tx.send(id).await.unwrap();
                            return;
                        }
                        let response = response.unwrap();
                        if !response.status().is_success() {
                            self.results_tx.send(id).await.unwrap();
                            return;
                        }
                    })
                    .await;
            })
            .await;
    }
}
