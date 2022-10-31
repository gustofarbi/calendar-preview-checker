use clap::ArgMatches;

use async_trait::async_trait;

use crate::{overlay::Overlay, preview::Preview};

#[async_trait]
pub trait Handler {
    async fn handle(&self, matches: &ArgMatches);
}

pub fn get(matches: &ArgMatches) -> Option<(Box<dyn Handler>, &ArgMatches)> {
    match matches.subcommand() {
        Some(("preview", submatches)) => Some((Box::new(Preview::new()), submatches)),
        Some(("overlay", submatches)) => Some((Box::new(Overlay::new()), submatches)),
        Some((&_, _)) => None,
        None => None,
    }
}
