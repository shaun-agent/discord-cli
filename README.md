# discord-cli

A minimal Rust CLI for managing Discord servers via the REST API. Built for AI agents and automation.

## Install

```bash
cargo build --release
cp target/release/discord ~/.local/bin/
```

## Setup

```bash
export DISCORD_TOKEN="your-bot-token"
export DISCORD_GUILD_ID="your-server-id"
```

## Commands

```bash
discord auth status                    # Check bot connection
discord guild info                     # Server info
discord channels list                  # List channels
discord channels create --name "dev" --type text --category <id>
discord channels edit <id> --name "new-name" --topic "new topic"
discord channels delete <id> --yes
discord roles list                     # List roles
discord roles create --name "Mod" --color "#E74C3C" --hoist
discord roles assign <user_id> <role_id>
discord roles remove <user_id> <role_id>
discord messages send <channel_id> "Hello!"
discord messages list <channel_id> --limit 10
discord members list --limit 50
discord permissions set <ch_id> --role <role_id> --deny VIEW_CHANNEL
discord permissions set <ch_id> --role <role_id> --allow VIEW_CHANNEL,SEND_MESSAGES
```

## Channel Types

`text`, `voice`, `category`, `announcement`, `forum`

## Permission Names

`VIEW_CHANNEL`, `SEND_MESSAGES`, `MANAGE_MESSAGES`, `MANAGE_CHANNELS`, `MANAGE_ROLES`, `READ_MESSAGE_HISTORY`, `ADD_REACTIONS`, `MENTION_EVERYONE`, `EMBED_LINKS`, `ATTACH_FILES`, `MANAGE_WEBHOOKS`, `MANAGE_THREADS`, `ADMINISTRATOR`, `KICK_MEMBERS`, `BAN_MEMBERS`, `CONNECT`, `SPEAK`

## Kiro Skill

A [Kiro CLI](https://kiro.dev) skill is included at `skill/SKILL.md`. Copy it to `~/.kiro/skills/discord-cli/` to enable AI agent integration.

## License

MIT
