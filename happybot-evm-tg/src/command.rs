use std::error::Error;

use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{Me, ParseMode},
    utils::command::BotCommands,
};

use crate::{error::CommandError, language::Language, start::Start, trade::Trade, wallet::Wallet};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "start using the bot.")]
    Start,
    #[command(description = "test trade.")]
    Trade,
    #[command(description = "test trade.")]
    Wallet,
    #[command(description = "switch language.")]
    Language,
}

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Start) => {
                let start: Start = Start::new();
                bot.send_message(msg.chat.id, start.text)
                    .parse_mode(ParseMode::MarkdownV2)
                    .reply_markup(start.keyboard.unwrap())
                    .await?;
            }
            Ok(Command::Trade) => {
                let trade: Trade = Trade::new();
                bot.send_message(msg.chat.id, trade.text)
                    .parse_mode(ParseMode::MarkdownV2)
                    .reply_markup(trade.keyboard.unwrap())
                    .await?;
            }
            Ok(Command::Wallet) => {
                let wallet: Wallet = Wallet::new();
                bot.send_message(msg.chat.id, wallet.text)
                    .parse_mode(ParseMode::MarkdownV2)
                    .reply_markup(wallet.keyboard.unwrap())
                    .await?;
            }
            Ok(Command::Language) => {
                let language: Language = Language::new();
                bot.send_message(msg.chat.id, language.text)
                    .parse_mode(ParseMode::MarkdownV2)
                    .reply_markup(language.keyboard.unwrap())
                    .await?;
            }
            Err(_) => {
                let error = CommandError::new();
                bot.send_message(msg.chat.id, error.text).await?;
            }
        }
    }
    Ok(())
}
