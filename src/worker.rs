use std::sync::Arc;
use std::time::Duration;

use futures::stream;
use reqwest::Client;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::url::build_urls;
use crate::StreamExt;

pub struct Worker {
    mounting: String,
    refinement: bool,
    concurrency: usize,
}

impl Worker {
    pub fn new(mounting: String, refinement: bool, concurrency: usize) -> Self {
        Worker {
            mounting,
            refinement,
            concurrency,
        }
    }

    pub async fn start(&self, jobs_rx: Receiver<u32>, results_tx: &Arc<Sender<u32>>) {
        tokio_stream::wrappers::ReceiverStream::new(jobs_rx)
            .for_each_concurrent(self.concurrency, |id| async move {
                let client = Client::builder()
                    .timeout(Duration::from_secs(3))
                    .build()
                    .unwrap();

                stream::iter(build_urls(id, &self.mounting, self.refinement))
                    .for_each_concurrent(self.concurrency, |url| async {
                        let response = client.head(url).send().await;
                        if response.is_err() {
                            results_tx.send(id).await.unwrap();
                            return;
                        }
                        let response = response.unwrap();
                        if !response.status().is_success() {
                            results_tx.send(id).await.unwrap();
                            return;
                        }
                    })
                    .await;
            })
            .await;

        println!("workers finished");
    }
}
