use std::{sync::Arc, time::Duration};

use futures::{stream, StreamExt};
use reqwest::Client;

use tokio::sync::{
    mpsc::{self, Sender},
    watch, Barrier,
};

struct Worker {
    results_tx: Sender<u32>,
}

impl Worker {
    fn new(results_tx: Sender<u32>) -> Self {
        Worker { results_tx }
    }

    pub async fn start<T: Send + 'static>(
        &self,
        jobs_rx: watch::Receiver<u32>,
        barrier: Arc<Barrier>,
    ) {
        tokio_stream::wrappers::WatchStream::new(jobs_rx)
            .for_each_concurrent(5, |id| async move {
                let client = Client::builder()
                    .timeout(Duration::from_secs(3))
                    .build()
                    .unwrap();

                stream::iter(build_urls(id))
                    .for_each_concurrent(5, |url| async {
                        println!("checking url: {}", url);
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

        barrier.wait().await;
    }
}

static SIZES: [u32; 6] = [500, 1080, 1242, 1440, 2048, 2560];
static LOCALES: [&str; 6] = ["de-DE", "fr-FR", "nl-NL", "de-AT", "es-ES", "it-IT"];

#[tokio::main]
async fn main() {
    let (jobs_tx, jobs_rx) = watch::channel(5);
    let (results_tx, mut results_rx) = mpsc::channel(20);
    let barrier = Arc::new(Barrier::new(1));

    tokio::spawn(async move {
        while let Some(id) = results_rx.recv().await {
            println!("{}", id);
        }
    });

    let worker = Worker::new(results_tx);
    worker.start::<u32>(jobs_rx, barrier).await;

    for id in 1..10 {
        jobs_tx.send(id).unwrap();
    }
}

fn build_urls(id: u32) -> Vec<String> {
    let mut urls = Vec::<String>::new();

    for size in SIZES {
        for locale in LOCALES {
            let url = format!(
                "https://mp-prod-de-preview-service.s3.eu-central-1.amazonaws.com/resources/calendar-designs/{}/{}/mt-spirale_cv-cover-foreground_ref_{}.webp",
                id,
                locale,
                size,
            );
            urls.push(url);
        }
    }

    urls
}
