# minebot

A simple Discord bot to interact with your Minecraft server.

## Installation

There are multiple ways to run the bot, or you can [build from source](#build-from-source).

### Docker Compose (recommended)

1. Create a `compose.yaml` file with the following content:

```yaml
version: "3.8"
services:
  minebot:
    image: ghcr.io/the-noah/minebot:latest
    container_name: minebot
    restart: unless-stopped
    environment:
      - DISCORD_TOKEN=your_discord_bot_token
      - GUILD_ID=your_discord_guild_id
      - MINECRAFT_IP=your_minecraft_server_ip
```

2. Run the bot

```bash
docker compose up -d
```

### Docker

1. Run the bot

```bash
docker run -d --restart unless-stopped --name minebot \
  -e DISCORD_TOKEN=your_discord_bot_token \
  -e GUILD_ID=your_discord_guild_id \
  -e MINECRAFT_IP=your_minecraft_server_ip \
  ghcr.io/the-noah/minebot:latest
```

### Binary (via Cargo)

1. Install the bot

```bash
cargo install minebot
```

2. Create a `.env` file in the root of the project and add the following:

```env
DISCORD_TOKEN=your_discord_bot_token
GUILD_ID=your_discord_guild_id
MINECRAFT_IP=your_minecraft_server_ip
```

3. Run the bot

```bash
minebot
```

## Usage

The bot will automatically register slash commands when it starts. You can use the following commands:

- `/ping` - Check if the bot is online
- `/status` - Get the status of the Minecraft server with player info
- `/say` - Send a message in Minecraft chat as the server (requires admin permissions on Discord)
- `/whitelist`
  - `/whitelist add <username>` - Add a player to the whitelist (requires admin permissions on Discord)
  - `/whitelist remove <username>` - Remove a player from the whitelist (requires admin permissions on Discord)
  - `/whitelist list` - List all players on the whitelist (requires admin permissions on Discord)

The bot will also automatically update its status to show the number of players online.

## Build from source


 Create a `.env` file in the root of the project and add the following:

```env
DISCORD_TOKEN=your_discord_bot_token
GUILD_ID=your_discord_guild_id
MINECRAFT_IP=your_minecraft_server_ip
RCON_PASSWORD=your_rcon_password
```
#### optional
```env
RCON_PORT=your_rcon_port (default: 25575)
```

### Use docker compose

run ```docker compose up``` 

### Build manually

1. Build bot

```bash
cd minebot
cargo build --release
```

2. Run the bot

```bash
./target/release/minebot
```

## License

[MIT](LICENSE)
