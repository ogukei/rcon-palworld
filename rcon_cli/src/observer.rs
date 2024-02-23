
use std::collections::HashSet;

use anyhow::Result;
use log::{trace, info};

use crate::{command::{fetch_current_players, say}, player::Player};

pub struct PlayersObserver {
    known_players: Option<HashSet<Player>>,
}

impl PlayersObserver {
    pub fn new() -> Self {
        Self { known_players: None }
    }

    pub async fn check(&mut self) -> Result<()> {
        let players = fetch_current_players().await?;
        let players: HashSet<Player> = HashSet::from_iter(players);
        if let Some(known_players) = self.known_players.as_ref() {
            let joined: Vec<Player> = players.difference(&known_players).cloned().collect();
            let left: Vec<Player> = known_players.difference(&players).cloned().collect();
            if !joined.is_empty() {
                for player in joined.iter() {
                    info!("{} ({}) joined", player.name(), player.id());
                    say(&format!("{} joined!", player.name())).await?;
                }
            }
            if !left.is_empty() {
                for player in left.iter() {
                    info!("{} ({}) left", player.name(), player.id());
                    say(&format!("{} left!", player.name())).await?;
                }
            }
            if joined.is_empty() && left.is_empty() {
                trace!("nothing is changed");
            }
        }
        self.known_players = Some(players);
        Ok(())
    }
}
