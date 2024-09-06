use serenity::{all::{Context, EventHandler, Message, Ready}, async_trait};
use utils::{config_manager::get_config, key_manager::get_key};

mod utils;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        println!("{}: {}", msg.author.name, msg.content);
        }
    
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}


fn main() {
    let key = get_key();
    let token = key.token;
    let config = get_config();
    let garden_id = config.garden_id;
    
    println!("Hello, world!");
}
