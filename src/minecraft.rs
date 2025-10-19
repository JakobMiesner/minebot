use std::env;

use rcon::Connection;

pub async fn get_players() -> Vec<String> {
  let response = rcon_command("list").await;

  if let Some(colon_pos) = response.find(':') {
    let player_list = response[colon_pos + 1..].trim();

    if player_list.is_empty() {
      Vec::new()
    } else {
      player_list.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
    }
  } else {
    eprintln!("Unexpected RCON list response format: {}", response);
    Vec::new()
  }
}

pub async fn rcon_command(command: &str) -> String {
  match Connection::builder()
    .enable_minecraft_quirks(true)
    .connect(get_rcon_address(), &env::var("RCON_PASSWORD").expect("Expected RCON_PASSWORD in the environment"))
    .await
  {
    Ok(mut connection) => match connection.cmd(command).await {
      Ok(response) => response,
      Err(e) => {
        eprintln!("Failed to execute RCON command: {:?}. Ensure the command is correct and the server is responsive.", e);
        "Failed to execute RCON command".to_string()
      }
    },
    Err(e) => {
      eprintln!("Failed to connect to RCON: {:?}. Check the RCON password and server settings.", e);
      "Failed to connect to RCON".to_string()
    }
  }
}

fn get_ip() -> String {
  env::var("MINECRAFT_IP").expect("Expected MINECRAFT_IP in the environment")
}

fn get_rcon_address() -> String {
  format!("{}:{}", get_ip(), env::var("RCON_PORT").unwrap_or_else(|_| "25575".to_string()))
}
