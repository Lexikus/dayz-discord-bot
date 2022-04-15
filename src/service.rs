use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use serenity::model::channel::Message;
use serenity::client::Context;
use serenity::framework::standard::{
    Args,
    CommandResult,
    macros::{
        command,
        group
    }
};
use super::NITRADO_TOKEN;
use super::NITRADO_SERVICE_ID;

#[derive(Debug, Serialize, Deserialize)]
pub struct RestartParameter {
    pub message: String,
    pub restart_message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BanParameter {
    pub category: String,
    pub key: String,
    pub value: String
}

#[group]
#[commands(service)]
pub struct Service;

#[command]
#[sub_commands(restart, player_count, ban)]
pub async fn service(_ctx: &Context, _msg: &Message) -> CommandResult {
    Ok(())
}

#[command]
#[allowed_roles("Admin")]
pub async fn restart(ctx: &Context, msg: &Message) -> CommandResult {
    let parameter = RestartParameter {
        message: "Restart".to_string(),
        restart_message: "Discord bot requested a restart".to_string()
    };
    let client = reqwest::Client::new()
        .post(format!("https://api.nitrado.net/services/{}/gameservers/restart", *NITRADO_SERVICE_ID))
        .header("Authorization", format!("Bearer {}", *NITRADO_TOKEN))
        .json(&parameter);

    let response = client.send().await?;

    if !response.status().is_success() {
        msg.reply(ctx, "Something went wrong. Server will not restart").await?;
        eprintln!("{}", response.text().await?);
        return Ok(());
    }

    msg.reply(ctx, "Server restarts").await?;

    Ok(())
}

#[command("player-count")]
pub async fn player_count(ctx: &Context, msg: &Message) -> CommandResult {
    let client = reqwest::Client::new()
        .get(format!("https://api.nitrado.net/services/{}/gameservers", *NITRADO_SERVICE_ID))
        .header("Authorization", format!("Bearer {}", *NITRADO_TOKEN));

    let response = client.send().await?;

    if !response.status().is_success() {
        msg.reply(ctx, "Something went wrong.").await?;
        eprintln!("{}", response.text().await?);
        return Ok(());
    }

    let text = response.text().await?;
    let body: Value = from_str(&text)?;
    let current = body["data"]["gameserver"]["query"]["player_current"].as_i64().unwrap_or(0i64);
    let max = body["data"]["gameserver"]["query"]["player_max"].as_i64().unwrap_or(0i64);
    msg.reply(ctx, format!("{current}/{max}")).await?;

    Ok(())
}

// FIXME: this needs to request the gameservers endpoint first,
// fetch the banlist, and create a new list with windows newlines
#[command]
#[allowed_roles("Admin")]
pub async fn ban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let identifier = args.rest().to_string();
    if identifier.is_empty() {
        msg.reply(ctx, "No identifier provided").await?;
        return Ok(());
    }

    let parameter = BanParameter {
        category: "general".to_string(),
        key: "bans".to_string(),
        value: identifier
    };
    // let client = reqwest::Client::new()
    //     .post(format!("https://api.nitrado.net/services/{}/gameservers/settings", *NITRADO_SERVICE_ID))
    //     .header("Authorization", format!("Bearer {}", *NITRADO_TOKEN))
    //     .json(&parameter);

    // let response = client.send().await?;

    // if !response.status().is_success() {
    //     msg.reply(ctx, "Something went wrong.").await?;
    //     eprintln!("{}", response.text().await?);
    //     return Ok(());
    // }

    msg.reply(ctx, format!("Player {} is \"banned\"(not really) now!", parameter.value)).await?;

    Ok(())
}