use once_cell::sync::Lazy;
use regex::Regex;
use serenity::all::{CreateCommandOption, Permissions};
use serenity::builder::CreateCommand;
use serenity::model::application::{ResolvedOption, ResolvedValue};

use crate::minecraft;

fn is_valid_mc_username(s: &str) -> bool {
  static RE: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| Regex::new(r"^[A-Za-z0-9_]{3,16}$").unwrap());
  RE.is_match(s)
}

pub async fn run<'a>(options: &'a [ResolvedOption<'a>]) -> String {
  let subcommand = options.first();

  if let Some(ResolvedOption {
    name,
    value: ResolvedValue::SubCommand(subcommand_options),
    ..
  }) = subcommand
  {
    let command = name.to_owned();

    match command {
      "add" | "remove" => {
        if let Some(ResolvedOption {
          value: ResolvedValue::String(username),
          ..
        }) = subcommand_options.first()
        {
          if (is_valid_mc_username(username)) {
            minecraft::rcon_command(&format!("whitelist {} {}", command, username)).await.replace("_", "\\_")
          } else {
            "Not a valid username".to_string()
          }
        } else {
          "No username provided".to_string()
        }
      }
      "list" => {
        minecraft::rcon_command("whitelist reload").await;
        let result = minecraft::rcon_command("whitelist list").await;
        let mut players = result.find(':').map(|i| &result[i + 2..]).unwrap_or(result.as_str()).split(", ").collect::<Vec<&str>>();

        players.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        return players.iter().map(|player| format!("- {}", player.replace("_", "\\_"))).collect::<Vec<String>>().join("\n");
      }
      _ => "Unknown subcommand".to_string(),
    }
  } else {
    "No subcommand provided".to_string()
  }
}

pub fn register() -> CreateCommand {
  CreateCommand::new("whitelist")
    .description("Manage the whitelist")
    .default_member_permissions(Permissions::ADMINISTRATOR)
    .add_option(
      CreateCommandOption::new(serenity::all::CommandOptionType::SubCommand, "add", "Add a player to the whitelist")
        .add_sub_option(CreateCommandOption::new(serenity::all::CommandOptionType::String, "username", "Minecraft username").required(true)),
    )
    .add_option(
      CreateCommandOption::new(serenity::all::CommandOptionType::SubCommand, "remove", "Remove a player from the whitelist")
        .add_sub_option(CreateCommandOption::new(serenity::all::CommandOptionType::String, "username", "Minecraft username").required(true)),
    )
    .add_option(CreateCommandOption::new(
      serenity::all::CommandOptionType::SubCommand,
      "list",
      "List all players on the whitelist",
    ))
}
