use std::env;

use dotenv::dotenv;

use serenity::{
  all::{ActivityData, CreateInteractionResponse, CreateInteractionResponseMessage},
  async_trait,
  model::{application::Interaction, gateway::Ready, id::GuildId},
  prelude::*,
};

mod commands;
mod minecraft;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, ctx: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);

    let guild_id = GuildId::new(
      env::var("GUILD_ID")
        .expect("Expected GUILD_ID in the environment")
        .parse()
        .expect("GUILD_ID must be an integer"),
    );

    let commands = guild_id
      .set_commands(
        &ctx.http,
        vec![
          commands::ping::register(),
          commands::say::register(),
          commands::status::register(),
          commands::whitelist::register(),
        ],
      )
      .await;

    if let Err(why) = commands {
      println!("Error setting commands: {:?}", why);
    }

    let ctx = ctx.clone();

    tokio::spawn(async move {
      let mut last_player_count = usize::max_value();

      let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));

      loop {
        let players = minecraft::get_players().await;

        if players.len() != last_player_count {
          last_player_count = players.len();

          let status_text = format!("{} player{} online", players.len(), if players.len() == 1 { "" } else { "s" });

          ctx.set_activity(Some(ActivityData::custom(status_text)));
        }

        interval.tick().await;
      }
    });
  }

  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
      println!("Command: {command:#?}");

      let content = match command.data.name.as_str() {
        "ping" => Some(commands::ping::run(&command.data.options())),
        "say" => Some(commands::say::run(&command.data.options()).await),
        "status" => Some(commands::status::run().await),
        "whitelist" => Some(commands::whitelist::run(&command.data.options()).await),
        _ => Some("Unknown command".to_string()),
      };

      if let Some(content) = content {
        let data = CreateInteractionResponseMessage::new().content(content);
        let builder = CreateInteractionResponse::Message(data);

        if let Err(why) = command.create_response(&ctx.http, builder).await {
          println!("Error sending response: {:?}", why);
        }
      }
    }
  }
}

#[tokio::main]
async fn main() {
  dotenv().ok();

  let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");

  let intents = GatewayIntents::GUILD_MESSAGES;

  let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

  let shard_manager = client.shard_manager.clone();

  tokio::spawn(async move {
    tokio::signal::ctrl_c().await.expect("Error setting up signal handler");

    println!("Shutting down...");

    shard_manager.shutdown_all().await;

    println!("Goodbye!");
  });

  if let Err(why) = client.start().await {
    println!("Client error: {:?}", why);
  }
}
