#![allow(unused)]
// USE THIS COMMAND sudo /home/maxwell/led-messenger/target/release/led-messenger --gpio-mapping adafruit-hat --rows 32 --slowdown-gpio 3 --no-hardware-pulse
use std::env;

use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
    },
    prelude::*,
};
use std::{thread, time};
use clap::{crate_version, App, Arg};
use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::Rgb565,
    // text::Text,
    // pixelcolor::Rgb888,
    // mono_font::{ascii::FONT_6X10, MonoTextStyle},
    prelude::*,
    primitives::{Circle, Rectangle, Triangle},
    style::{PrimitiveStyle, TextStyleBuilder},
};
use rpi_led_matrix::{args, LedMatrix, LedColor};

const DELAY: std::time::Duration = std::time::Duration::from_millis(100);
const DISPLAY_COMMAND: &str = "!display";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let triggered = msg.content.starts_with(DISPLAY_COMMAND) && msg.author.bot == false;
        if triggered {
            let phrase = format!("{} ", DISPLAY_COMMAND);
            let message = msg.content.replace(&phrase, "");
            let response = format!("NOW DISPLAYING --- \n> {}", &message);
            if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                println!("Error sending message: {:?}", why);
            }
            write_message(&message);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn write_message(message: &str) {
    let app = args::add_matrix_args(
        App::new("C++ Library Example")
            .about("shows basic usage of matrix arguments")
            .version(crate_version!())
            .arg(
                Arg::from_usage("--loops=[LOOPS] 'number of cycles to spin the line'")
                    .default_value("5"),
            ),
    );

    let matches = app.get_matches();
    let (options, rt_options) = args::matrix_options_from_args(&matches);

    let matrix = LedMatrix::new(Some(options), Some(rt_options)).unwrap();
    let mut canvas = matrix.canvas();
    let (width, height) = (32, 32);
    let mut color = LedColor {
        red: 127,
        green: 0,
        blue: 0,
    };

    let text_style = TextStyleBuilder::new(Font6x8)
        .text_color(Rgb565::RED)
        .build();
    
    for i in (0..(message.len() * 3)).rev() {
        println!("{}", i);
        canvas.clear();
        if i > 32 {
            let text = Text::new(&message[i - 32..i], Point::new(0, 0))
                .into_styled(text_style)
                .into_iter()
                .draw(&mut canvas)
                .unwrap();
        }
        else {
            let text = Text::new(message, Point::new(i as i32, 16))
                .into_styled(text_style)
                .into_iter()
                .draw(&mut canvas)
                .unwrap();    
        }
        std::thread::sleep(DELAY);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::builder(&token).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }    
}