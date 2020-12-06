use lazy_static::lazy_static;
use serenity::async_trait;
use serenity::client::bridge::gateway::GatewayIntents;
use serenity::model::guild::Guild;
use serenity::model::id::GuildId;
use serenity::model::prelude::{Message, Ready};
use serenity::prelude::*;
use serenity::Client;
use std::sync::Mutex;

lazy_static! {
    pub static ref GUILDS: Mutex<Vec<Guild>> = Mutex::new(vec![]);
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // This is because clion is stupid
    //noinspection ALL
    async fn guild_create(&self, _ctx: Context, guild: Guild, is_new: bool) {
        GUILDS.lock().unwrap().push(guild);
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub async fn start_bot() {
    let token = "Nzg0ODE3Mzc3NTA1MzEyNzk5.X8u0Iw.19LZfbDGuKJj1m_xWmIukSjdxwA";

    let mut client = Client::builder(&token)
        .add_intent(GatewayIntents::GUILD_MEMBERS)
        .add_intent(GatewayIntents::GUILDS)
        .event_handler(Handler)
        .await
        .expect("Unable to create client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e)
    }
}
