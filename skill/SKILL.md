---
name: discord-cli
version: 1.0.0
description: "Discord: Manage servers, channels, roles, members, and messages via CLI."
metadata:
  openclaw:
    category: "productivity"
    requires:
      bins: ["discord"]
      env: ["DISCORD_TOKEN", "DISCORD_GUILD_ID"]
---

# discord

Manage Discord servers via the REST API.

```bash
discord <resource> <action> [flags]
```

## Prerequisites

- `DISCORD_TOKEN` env var — Bot token from Discord Developer Portal
- `DISCORD_GUILD_ID` env var — Target server ID

## Resources & Commands

### auth

| Command | Description |
|---------|-------------|
| `discord auth status` | Show bot info and connected guilds |

### guild

| Command | Description |
|---------|-------------|
| `discord guild info` | Show server name, owner, member count |

### channels

| Command | Description |
|---------|-------------|
| `discord channels list` | List all channels |
| `discord channels create --name <name> --type <text\|voice\|category\|announcement\|forum> [--category <id>] [--topic <text>]` | Create a channel |
| `discord channels edit <id> [--name <name>] [--topic <text>]` | Edit a channel |
| `discord channels delete <id> [--yes]` | Delete a channel (--yes skips confirm) |

### roles

| Command | Description |
|---------|-------------|
| `discord roles list` | List all roles |
| `discord roles create --name <name> [--color "#hex"] [--hoist] [--mentionable]` | Create a role |
| `discord roles edit <id> [--name <name>] [--color "#hex"]` | Edit a role |
| `discord roles delete <id>` | Delete a role |
| `discord roles assign <user_id> <role_id>` | Assign role to user |
| `discord roles remove <user_id> <role_id>` | Remove role from user |

### messages

| Command | Description |
|---------|-------------|
| `discord messages send <channel_id> "<content>"` | Send a message |
| `discord messages list <channel_id> [--limit N]` | List recent messages |
| `discord messages delete <channel_id> <message_id>` | Delete a message |

### members

| Command | Description |
|---------|-------------|
| `discord members list [--limit N]` | List server members |

### permissions

| Command | Description |
|---------|-------------|
| `discord permissions set <channel_id> --role <role_id> [--allow PERM1,PERM2] [--deny PERM1,PERM2]` | Set permission overwrite |
| `discord permissions remove <channel_id> --role <role_id>` | Remove permission overwrite |

**Permission names:** VIEW_CHANNEL, SEND_MESSAGES, MANAGE_MESSAGES, MANAGE_CHANNELS, MANAGE_ROLES, READ_MESSAGE_HISTORY, ADD_REACTIONS, MENTION_EVERYONE, EMBED_LINKS, ATTACH_FILES, MANAGE_WEBHOOKS, MANAGE_THREADS, ADMINISTRATOR, KICK_MEMBERS, BAN_MEMBERS, CONNECT, SPEAK

## Channel Types

| Type | Description |
|------|-------------|
| text | Standard text channel |
| voice | Voice channel |
| category | Channel category (folder) |
| announcement | Announcement channel |
| forum | Forum channel (threaded posts) |

## Examples

```bash
# Check bot status
discord auth status

# Create a channel under a category
discord channels create --name "dev-chat" --type text --category 123456789

# Create a role and assign it
discord roles create --name "Moderator" --color "#E74C3C" --hoist
discord roles assign 111222333 444555666

# Lock a channel to a specific role
discord permissions set 123456789 --role 000000000 --deny VIEW_CHANNEL
discord permissions set 123456789 --role 444555666 --allow VIEW_CHANNEL,SEND_MESSAGES

# Send an announcement
discord messages send 123456789 "🚀 New release v2.0!"
```
