use serenity::{all::{Context, EventHandler, Message, Ready}, async_trait};



struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("{}: {}", msg.author.name, msg.content);
        }
    
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}


fn main() {
    println!("Hello, world!");
}
