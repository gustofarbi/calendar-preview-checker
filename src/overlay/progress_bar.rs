use futures::StreamExt;
use tokio::sync::mpsc::Receiver;
use tokio_stream::wrappers::ReceiverStream;

use indicatif::ProgressBar;

pub fn start(progress_bar_rx: Receiver<()>, progress_bar: ProgressBar) {
    tokio::spawn(async move {
        ReceiverStream::new(progress_bar_rx)
            .for_each(|_| async {
                progress_bar.inc(1);
            })
            .await;
    });
}
