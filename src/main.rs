use clap::{Parser, Subcommand};
use reqwest::blocking::Client;
use serde_json::Value;
use std::env;
use std::process;

const BASE_URL: &str = "https://discord.com/api/v10";

#[derive(Parser)]
#[command(name = "discord", about = "Discord server management CLI")]
struct Cli {
    /// Override guild ID (defaults to DISCORD_GUILD_ID env var)
    #[arg(long, global = true)]
    guild: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check bot auth status
    Auth {
        #[command(subcommand)]
        action: AuthAction,
    },
    /// Manage channels
    Channels {
        #[command(subcommand)]
        action: ChannelAction,
    },
    /// Manage roles
    Roles {
        #[command(subcommand)]
        action: RoleAction,
    },
    /// Send and manage messages
    Messages {
        #[command(subcommand)]
        action: MessageAction,
    },
    /// List server members
    Members {
        #[command(subcommand)]
        action: MemberAction,
    },
    /// Manage channel permissions
    Permissions {
        #[command(subcommand)]
        action: PermissionAction,
    },
    /// Manage guild info
    Guild {
        #[command(subcommand)]
        action: GuildAction,
    },
}

#[derive(Subcommand)]
enum AuthAction {
    /// Show bot user info and verify token
    Status,
}

#[derive(Subcommand)]
enum ChannelAction {
    /// List all channels
    List,
    /// Create a channel
    Create {
        #[arg(long)]
        name: String,
        #[arg(long, default_value = "text")]
        r#type: String,
        #[arg(long)]
        category: Option<String>,
        #[arg(long)]
        topic: Option<String>,
    },
    /// Edit a channel
    Edit {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        topic: Option<String>,
    },
    /// Delete a channel
    Delete {
        id: String,
        #[arg(long)]
        yes: bool,
    },
}

#[derive(Subcommand)]
enum RoleAction {
    /// List all roles
    List,
    /// Create a role
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        color: Option<String>,
        #[arg(long)]
        hoist: bool,
        #[arg(long)]
        mentionable: bool,
    },
    /// Edit a role
    Edit {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        color: Option<String>,
    },
    /// Delete a role
    Delete { id: String },
    /// Assign a role to a user
    Assign { user_id: String, role_id: String },
    /// Remove a role from a user
    Remove { user_id: String, role_id: String },
}

#[derive(Subcommand)]
enum MessageAction {
    /// Send a message
    Send {
        channel_id: String,
        content: String,
    },
    /// List recent messages
    List {
        channel_id: String,
        #[arg(long, default_value = "10")]
        limit: u32,
    },
    /// Delete a message
    Delete {
        channel_id: String,
        message_id: String,
    },
}

#[derive(Subcommand)]
enum MemberAction {
    /// List server members
    List {
        #[arg(long, default_value = "10")]
        limit: u32,
    },
}

#[derive(Subcommand)]
enum PermissionAction {
    /// Set permission overwrite on a channel
    Set {
        channel_id: String,
        #[arg(long)]
        role: String,
        #[arg(long)]
        allow: Option<String>,
        #[arg(long)]
        deny: Option<String>,
    },
    /// Remove permission overwrite
    Remove {
        channel_id: String,
        #[arg(long)]
        role: String,
    },
}

#[derive(Subcommand)]
enum GuildAction {
    /// Show guild info
    Info,
}

struct Discord {
    client: Client,
    token: String,
    guild_id: String,
}

impl Discord {
    fn new(guild_override: Option<String>) -> Self {
        let token = env::var("DISCORD_TOKEN").unwrap_or_else(|_| {
            eprintln!("Error: DISCORD_TOKEN not set");
            process::exit(1);
        });
        let guild_id = guild_override.unwrap_or_else(|| {
            env::var("DISCORD_GUILD_ID").unwrap_or_else(|_| {
                eprintln!("Error: DISCORD_GUILD_ID not set (use --guild or env var)");
                process::exit(1);
            })
        });
        Self {
            client: Client::new(),
            token,
            guild_id,
        }
    }

    fn get(&self, path: &str) -> Value {
        let url = format!("{}{}", BASE_URL, path);
        let resp = self.client.get(&url)
            .header("Authorization", format!("Bot {}", self.token))
            .header("User-Agent", "DiscordBot (discord-cli, 0.1.0)")
            .send()
            .unwrap_or_else(|e| { eprintln!("Request failed: {e}"); process::exit(1); });
        let status = resp.status();
        let body: Value = resp.json().unwrap_or_else(|e| { eprintln!("Parse failed: {e}"); process::exit(1); });
        if !status.is_success() {
            eprintln!("Error {}: {}", status, serde_json::to_string_pretty(&body).unwrap());
            process::exit(1);
        }
        body
    }

    fn post(&self, path: &str, json: &Value) -> Value {
        let url = format!("{}{}", BASE_URL, path);
        let resp = self.client.post(&url)
            .header("Authorization", format!("Bot {}", self.token))
            .header("User-Agent", "DiscordBot (discord-cli, 0.1.0)")
            .json(json)
            .send()
            .unwrap_or_else(|e| { eprintln!("Request failed: {e}"); process::exit(1); });
        let status = resp.status();
        let body: Value = resp.json().unwrap_or_else(|e| { eprintln!("Parse failed: {e}"); process::exit(1); });
        if !status.is_success() {
            eprintln!("Error {}: {}", status, serde_json::to_string_pretty(&body).unwrap());
            process::exit(1);
        }
        body
    }

    fn patch(&self, path: &str, json: &Value) -> Value {
        let url = format!("{}{}", BASE_URL, path);
        let resp = self.client.patch(&url)
            .header("Authorization", format!("Bot {}", self.token))
            .header("User-Agent", "DiscordBot (discord-cli, 0.1.0)")
            .json(json)
            .send()
            .unwrap_or_else(|e| { eprintln!("Request failed: {e}"); process::exit(1); });
        let status = resp.status();
        let body: Value = resp.json().unwrap_or_else(|e| { eprintln!("Parse failed: {e}"); process::exit(1); });
        if !status.is_success() {
            eprintln!("Error {}: {}", status, serde_json::to_string_pretty(&body).unwrap());
            process::exit(1);
        }
        body
    }

    fn delete(&self, path: &str) {
        let url = format!("{}{}", BASE_URL, path);
        let resp = self.client.delete(&url)
            .header("Authorization", format!("Bot {}", self.token))
            .header("User-Agent", "DiscordBot (discord-cli, 0.1.0)")
            .send()
            .unwrap_or_else(|e| { eprintln!("Request failed: {e}"); process::exit(1); });
        if !resp.status().is_success() {
            let body: Value = resp.json().unwrap_or_default();
            eprintln!("Error: {}", serde_json::to_string_pretty(&body).unwrap());
            process::exit(1);
        }
    }

    fn put(&self, path: &str, json: &Value) {
        let url = format!("{}{}", BASE_URL, path);
        let resp = self.client.put(&url)
            .header("Authorization", format!("Bot {}", self.token))
            .header("User-Agent", "DiscordBot (discord-cli, 0.1.0)")
            .json(json)
            .send()
            .unwrap_or_else(|e| { eprintln!("Request failed: {e}"); process::exit(1); });
        if !resp.status().is_success() {
            let body: Value = resp.json().unwrap_or_default();
            eprintln!("Error: {}", serde_json::to_string_pretty(&body).unwrap());
            process::exit(1);
        }
    }
}

fn channel_type_id(t: &str) -> u32 {
    match t {
        "text" => 0,
        "voice" => 2,
        "category" => 4,
        "announcement" => 5,
        "forum" => 15,
        _ => { eprintln!("Unknown channel type: {t}. Use: text, voice, category, announcement, forum"); process::exit(1); }
    }
}

fn parse_color(s: &str) -> u32 {
    let hex = s.trim_start_matches('#');
    u32::from_str_radix(hex, 16).unwrap_or_else(|_| { eprintln!("Invalid color: {s}"); process::exit(1); })
}

fn parse_permissions(s: &str) -> u64 {
    let mut bits: u64 = 0;
    for name in s.split(',') {
        bits |= match name.trim().to_uppercase().as_str() {
            "VIEW_CHANNEL" => 1 << 10,
            "SEND_MESSAGES" => 1 << 11,
            "MANAGE_MESSAGES" => 1 << 13,
            "EMBED_LINKS" => 1 << 14,
            "ATTACH_FILES" => 1 << 15,
            "READ_MESSAGE_HISTORY" => 1 << 16,
            "MENTION_EVERYONE" => 1 << 17,
            "MANAGE_CHANNELS" => 1 << 4,
            "MANAGE_ROLES" => 1 << 28,
            "MANAGE_WEBHOOKS" => 1 << 29,
            "MANAGE_THREADS" => 1 << 34,
            "ADMINISTRATOR" => 1 << 3,
            "KICK_MEMBERS" => 1 << 1,
            "BAN_MEMBERS" => 1 << 2,
            "ADD_REACTIONS" => 1 << 6,
            "CONNECT" => 1 << 20,
            "SPEAK" => 1 << 21,
            other => { eprintln!("Unknown permission: {other}"); process::exit(1); }
        };
    }
    bits
}

fn main() {
    let cli = Cli::parse();
    let dc = Discord::new(cli.guild);

    match cli.command {
        Commands::Auth { action } => match action {
            AuthAction::Status => {
                let user = dc.get("/users/@me");
                println!("Bot: {} ({})", user["username"], user["id"]);
                let guilds = dc.get("/users/@me/guilds");
                if let Some(arr) = guilds.as_array() {
                    println!("Guilds: {}", arr.len());
                    for g in arr {
                        println!("  - {} ({})", g["name"], g["id"]);
                    }
                }
            }
        },
        Commands::Guild { action } => match action {
            GuildAction::Info => {
                let g = dc.get(&format!("/guilds/{}?with_counts=true", dc.guild_id));
                println!("Name: {}", g["name"]);
                println!("ID: {}", g["id"]);
                println!("Owner: {}", g["owner_id"]);
                println!("Members: ~{}", g["approximate_member_count"]);
            }
        },
        Commands::Channels { action } => match action {
            ChannelAction::List => {
                let channels = dc.get(&format!("/guilds/{}/channels", dc.guild_id));
                if let Some(arr) = channels.as_array() {
                    let mut sorted = arr.clone();
                    sorted.sort_by_key(|c| c["position"].as_i64().unwrap_or(0));
                    for ch in &sorted {
                        let t = match ch["type"].as_u64().unwrap_or(0) {
                            0 => "text", 2 => "voice", 4 => "category", 5 => "announcement", 13 => "stage", 15 => "forum", 16 => "media", _ => "other"
                        };
                        println!("{:20} {:12} {}", ch["id"].as_str().unwrap_or(""), t, ch["name"].as_str().unwrap_or(""));
                    }
                }
            },
            ChannelAction::Create { name, r#type, category, topic } => {
                let mut body = serde_json::json!({ "name": name, "type": channel_type_id(&r#type) });
                if let Some(cat) = category { body["parent_id"] = Value::String(cat); }
                if let Some(t) = topic { body["topic"] = Value::String(t); }
                let ch = dc.post(&format!("/guilds/{}/channels", dc.guild_id), &body);
                println!("Created: {} ({})", ch["name"], ch["id"]);
            },
            ChannelAction::Edit { id, name, topic } => {
                let mut body = serde_json::json!({});
                if let Some(n) = name { body["name"] = Value::String(n); }
                if let Some(t) = topic { body["topic"] = Value::String(t); }
                let ch = dc.patch(&format!("/channels/{}", id), &body);
                println!("Updated: {} ({})", ch["name"], ch["id"]);
            },
            ChannelAction::Delete { id, yes } => {
                if !yes {
                    eprint!("Delete channel {}? [y/N] ", id);
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    if !input.trim().eq_ignore_ascii_case("y") {
                        println!("Cancelled");
                        return;
                    }
                }
                dc.delete(&format!("/channels/{}", id));
                println!("Deleted channel {}", id);
            },
        },
        Commands::Roles { action } => match action {
            RoleAction::List => {
                let roles = dc.get(&format!("/guilds/{}/roles", dc.guild_id));
                if let Some(arr) = roles.as_array() {
                    let mut sorted = arr.clone();
                    sorted.sort_by_key(|r| std::cmp::Reverse(r["position"].as_i64().unwrap_or(0)));
                    for r in &sorted {
                        println!("{:20} {:20} pos:{}", r["id"].as_str().unwrap_or(""), r["name"].as_str().unwrap_or(""), r["position"]);
                    }
                }
            },
            RoleAction::Create { name, color, hoist, mentionable } => {
                let mut body = serde_json::json!({ "name": name, "hoist": hoist, "mentionable": mentionable });
                if let Some(c) = color { body["color"] = Value::Number(parse_color(&c).into()); }
                let r = dc.post(&format!("/guilds/{}/roles", dc.guild_id), &body);
                println!("Created: {} ({})", r["name"], r["id"]);
            },
            RoleAction::Edit { id, name, color } => {
                let mut body = serde_json::json!({});
                if let Some(n) = name { body["name"] = Value::String(n); }
                if let Some(c) = color { body["color"] = Value::Number(parse_color(&c).into()); }
                let r = dc.patch(&format!("/guilds/{}/roles/{}", dc.guild_id, id), &body);
                println!("Updated: {} ({})", r["name"], r["id"]);
            },
            RoleAction::Delete { id } => {
                dc.delete(&format!("/guilds/{}/roles/{}", dc.guild_id, id));
                println!("Deleted role {}", id);
            },
            RoleAction::Assign { user_id, role_id } => {
                dc.put(&format!("/guilds/{}/members/{}/roles/{}", dc.guild_id, user_id, role_id), &serde_json::json!({}));
                println!("Assigned role {} to user {}", role_id, user_id);
            },
            RoleAction::Remove { user_id, role_id } => {
                dc.delete(&format!("/guilds/{}/members/{}/roles/{}", dc.guild_id, user_id, role_id));
                println!("Removed role {} from user {}", role_id, user_id);
            },
        },
        Commands::Messages { action } => match action {
            MessageAction::Send { channel_id, content } => {
                let msg = dc.post(&format!("/channels/{}/messages", channel_id), &serde_json::json!({ "content": content }));
                println!("Sent: {} ({})", msg["id"], msg["channel_id"]);
            },
            MessageAction::List { channel_id, limit } => {
                let msgs = dc.get(&format!("/channels/{}/messages?limit={}", channel_id, limit));
                if let Some(arr) = msgs.as_array() {
                    for m in arr.iter().rev() {
                        println!("[{}] {}: {}", m["id"], m["author"]["username"], m["content"].as_str().unwrap_or(""));
                    }
                }
            },
            MessageAction::Delete { channel_id, message_id } => {
                dc.delete(&format!("/channels/{}/messages/{}", channel_id, message_id));
                println!("Deleted message {}", message_id);
            },
        },
        Commands::Members { action } => match action {
            MemberAction::List { limit } => {
                let members = dc.get(&format!("/guilds/{}/members?limit={}", dc.guild_id, limit));
                if let Some(arr) = members.as_array() {
                    for m in arr {
                        let user = &m["user"];
                        let nick = m["nick"].as_str().unwrap_or("-");
                        println!("{:20} {:20} nick:{}", user["id"], user["username"], nick);
                    }
                }
            },
        },
        Commands::Permissions { action } => match action {
            PermissionAction::Set { channel_id, role, allow, deny } => {
                let allow_bits = allow.map(|a| parse_permissions(&a)).unwrap_or(0);
                let deny_bits = deny.map(|d| parse_permissions(&d)).unwrap_or(0);
                let body = serde_json::json!({
                    "allow": allow_bits.to_string(),
                    "deny": deny_bits.to_string(),
                    "type": 0
                });
                dc.put(&format!("/channels/{}/permissions/{}", channel_id, role), &body);
                println!("Set permissions on channel {} for role {}", channel_id, role);
            },
            PermissionAction::Remove { channel_id, role } => {
                dc.delete(&format!("/channels/{}/permissions/{}", channel_id, role));
                println!("Removed permission overwrite for role {} on channel {}", role, channel_id);
            },
        },
    }
}
