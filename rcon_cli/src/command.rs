
use std::{env, ffi::CString, time::Duration};

use anyhow::{bail, Result};
use rcon::{client::RconClient, Packet, PacketType};
use log::trace;

use crate::player::Player;

pub async fn say(message: &str) -> Result<()> {
    // convert whitespaces to NBSPs because the game does not support whitespaces in RCON command arguments at the moment.
    // the game recognizes the message as latin-1 encoding.
    // note that the game RCON server is broken at calculating packet size when the packet include latin-1 characters.
    // so we ignore next further packets and disconnects.
    let message = message.replace(" ", "\x7f");
    let command = format!("Broadcast {}", message);
    let command = CString::new(command.as_str())?;
    // replace 0x7f to 0xa0
    let command = {
        let mut command = command.into_bytes();
        for char in command.iter_mut() {
            if *char == 0x7f {
                // NBSP
                *char = 0xa0;
            }
        }
        CString::new(command)?
    };
    // ignore broken response
    // e.g. `Hello\xa0world` will be converted to `Hello\xc2\xa0world`
    // the packet size is wrongly increased by the number of latin-1 characters.
    let _ = execute_command_raw(command).await;
    Ok(())
}

pub async fn fetch_current_players() -> Result<Vec<Player>> {
    let body = execute_command("ShowPlayers".into()).await?;
    let names: Vec<Player> = body.lines().skip(1)
        .filter_map(|line| {
            let components: Vec<&str> = line.split(',').collect();
            let name = components.get(0).copied();
            let id = components.get(1).copied();
            Player::new(id, name)
        })
        .collect();
    Ok(names)
}

pub async fn execute_command(command: &str) -> Result<String> {
    let client = negotiate().await?;
    let command_request = Packet::new(0, PacketType::EXEC_COMMAND, command.into())?;
    client.write_packet(command_request).await?;
    let response = next_response_with_timeout(&client, Duration::from_secs(3)).await?;
    let body = response.body()?;
    Ok(body)
}

pub async fn execute_command_raw(command: CString) -> Result<CString> {
    let client = negotiate().await?;
    let command_request = Packet::with_raw_body(0, PacketType::EXEC_COMMAND, command);
    client.write_packet(command_request).await?;
    let response = next_response_with_timeout(&client, Duration::from_secs(3)).await?;
    let body = response.raw_body();
    Ok(body.clone())
}

async fn negotiate() -> Result<RconClient> {
    // connect
    let endpoint = env::var("RCON_ENDPOINT").expect("RCON_ENDPOINT is required");
    let client = RconClient::connect(&endpoint).await?;
    trace!("connected");
    // auth
    const AUTH_PACKET_ID: i32 = 0;
    let password = env::var("RCON_PASSWORD").expect("RCON_PASSWORD is required");
    let auth_request = Packet::new(AUTH_PACKET_ID, PacketType::AUTH, password.into())?;
    client.write_packet(auth_request).await?;
    // await next auth response
    let auth_response = loop {
        let packet = client.read_packet().await?;
        if packet.r#type() == PacketType::AUTH_RESPONSE {
            break packet
        }
    };
    // check auth result
    if auth_response.id() == AUTH_PACKET_ID {
        trace!("authentication success");
        Ok(client)
    } else {
        bail!("authentication failure");
    }
}

async fn next_response_with_timeout(client: &RconClient, duration: Duration) -> Result<Packet> {
    tokio::select! {
        response = next_response(&client) => response,
        _ = tokio::time::sleep(duration) => {
            bail!("timeout")
        }
    }
}

async fn next_response(client: &RconClient) -> Result<Packet> {
    trace!("awaiting response");
    let response_value = loop {
        // the server-side packet size calculation will be broken when a packet contains a non-ascii character
        let packet = client.read_packet_ignoring_size().await?;
        trace!("{:?}", packet);
        if packet.r#type() == PacketType::RESPONSE_VALUE {
            break packet
        }
    };
    Ok(response_value)
}
