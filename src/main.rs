use std::env;
use once_cell::sync::Lazy;
use serenity::async_trait;
use serenity::client::{Client, EventHandler};
use serenity::framework::standard::{
    StandardFramework
};

static DISCORD_TOKEN: Lazy<String> = Lazy::new(|| {
    env::var("DISCORD_TOKEN").expect("Environment DISCORD_TOKEN not set.")
});
static NITRADO_TOKEN: Lazy<String> = Lazy::new(|| {
    env::var("NITRADO_TOKEN").expect("Environment NITRADO_TOKEN not set.")
});
static NITRADO_SERVICE_ID: Lazy<String> = Lazy::new(|| {
    env::var("NITRADO_SERVICE_ID").expect("Environment NITRADO_SERVICE_ID not set.")
});

mod service;
use service::SERVICE_GROUP;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!server").with_whitespace(true))
        .group(&SERVICE_GROUP);

    // Login with a bot token from the environment
    let mut client = Client::builder(&*DISCORD_TOKEN)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
