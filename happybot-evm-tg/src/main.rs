use command::message_handler;
use global::global_callback_handler;
use teloxide::prelude::*;
mod command;
mod error;
mod global;
mod start;
mod trade;
mod wallet;
mod language;
#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot = Bot::from_env();
    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(global_callback_handler));
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
