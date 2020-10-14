use std::{env, path::Path};
use rand::Rng;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    http::AttachmentType,
};

struct Handler;

const HELP_MESSAGE :&str = "Hey Human you have summoned me. Lets see how i can help.
                            Need Technical help -> Post in the .....

                            Looking for code of conduct -> 

                            Something wrong -> Reach ....


                            I hope your issue has been resolved.
                            --Have a good day 
                            --ISA BOT";

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
	let number = rand::thread_rng().gen_range(0,4);
	let photos  = ["attachment://gala.png","attachment://gala2.png","attachment://gala3.png","attachment://gala4.png"];
	let photos2  = ["./gala.png","./gala2.png","./gala3.png","./gala4.png",];

        if msg.content == "!gala" {
            
            let msg = msg.channel_id.send_message(&ctx.http, |m| {
                m.content("Here is a random gala memory!!!");
                m.embed(|e| {
                    //e.title("This is a title");
                    //e.description("This is a description");
                    e.image(photos[number]);
                    //e.fields(vec![
                        //("This is the first field", "This is a field body", true),
                        //("This is the second field", "Both of these fields are inline", true),
                    //]);
                    //e.field("This is the third field", "This is not an inline field", false);
                    //e.footer(|f| {
                        //f.text("This is a footer");

                        //f
                    //});

                    e
                });
                m.add_file(AttachmentType::Path(Path::new(photos2[number])));
                m
            }).await;

            if let Err(why) = msg {
                println!("Error sending message: {:?}", why);
            }
        }
        if msg.content =="!help"{
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await{
                    println!("Error sending message : {:?}",why);

            }
        }


        
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
