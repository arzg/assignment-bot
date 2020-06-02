use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{env, sync::Mutex};

fn main() -> anyhow::Result<()> {
    let token = env::var("DISCORD_TOKEN")?;

    let mut client = Client::new(&token, Handler::new())?;
    client.start()?;

    Ok(())
}

struct Handler {
    bot: Mutex<assignment_bot::Bot>,
}

impl Handler {
    fn new() -> Self {
        Self {
            bot: Mutex::new(assignment_bot::Bot::default()),
        }
    }
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let mut bot = self.bot.lock().unwrap();

        // Don’t handle messages from bots.
        if !msg.author.bot {
            if let Some(reply) = bot.handle_msg(&msg) {
                if let Err(e) = msg.channel_id.say(&ctx.http, reply) {
                    eprintln!("Error sending message: {:?}", e);
                    if let Err(e) = msg.delete(ctx.http) {
                        eprintln!("Error deleting message: {:?}", e);
                    }
                }
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
