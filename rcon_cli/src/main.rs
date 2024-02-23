

use std::time::Duration;

use anyhow::Result;
use log::info;
use tokio::time::sleep;

mod command;
mod player;
mod observer;

use crate::observer::PlayersObserver;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    runloop().await?;
    Ok(())
}

async fn runloop() -> Result<()> {
    info!("started");
    let mut observer = PlayersObserver::new();
    loop {
        observer.check().await?;
        sleep(Duration::from_secs(5)).await;
    }
}
