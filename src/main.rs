
use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
    },
    prelude::*,
};
/// Extremely simple use of arguments to create matrix options
use std::env;
use rpi_led_matrix::{LedMatrix, LedMatrixOptions, LedColor};
use std::{thread, time};

// fn main() {
//     loop {
//         draw_line();
//         thread::sleep(time::Duration::from_millis(5000));
//         draw_circle();
//         println!("Hello, world!");
//         thread::sleep(time::Duration::from_millis(5000));
//     }
// }

fn led_matrix() -> LedMatrix {
    let mut options = LedMatrixOptions::new();
    // options.set_hardware_mapping("adafruit-hat-pwm");
    // options.set_chain_length(2);
    options.set_hardware_pulsing(false);
    options.set_brightness(100);
    //options.set_inverse_colors(true);
    //options.set_refresh_rate(true);
    LedMatrix::new(Some(options)).unwrap()
}

fn draw_line() {
    println!("draw_line");
    let matrix = led_matrix();
    let mut canvas = matrix.canvas();
    let (width, height) = (32, 32);
    let mut color = LedColor {
        red: 127,
        green: 0,
        blue: 0,
    };

    canvas.clear();
    for x in 0..width {
        color.blue = 255 - 3 * x as u8;
        canvas.draw_line(x, 0, width - 1 - x, height - 1, &color);
        thread::sleep(time::Duration::new(0, 10000000));
    }
}

fn draw_circle() {
    println!("draw_circle");
    let matrix = led_matrix();
    let mut canvas = matrix.canvas();
    let (width, height) = (32, 32);
    let mut color = LedColor {
        red: 127,
        green: 0,
        blue: 0,
    };
    let (x, y) = (width / 2, height / 2);

    canvas.clear();
    for r in 0..(width / 2) {
        color.green = color.red;
        color.red = color.blue;
        color.blue = (r * r) as u8;
        canvas.draw_circle(x, y, r as u32, &color);
        thread::sleep(time::Duration::new(0, 100000000));
    }
}

const DISPLAY_COMMAND: &str = "!display";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let triggered = msg.content.starts_with(DISPLAY_COMMAND);
        if triggered {
            let response = format!("NOW DISPLAYING --- \n> {}", msg.content);
            draw_circle();
            if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                println!("Error sending message: {:?}", why);
            }
            println!("{} is displaying a message!", msg.author.name);
            draw_line();
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
// DISCORD_TOKEN=ODEyMTA1NzcwMzYzMzg3OTc3.YC76bw.UL_A3W99AXVDDANz04FfvOlc9_A


#[tokio::main]
async fn main() {
    // let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let token = "ODEyMTA1NzcwMzYzMzg3OTc3.YC76bw.UL_A3W99AXVDDANz04FfvOlc9_A";
    let mut client = Client::builder(&token).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
