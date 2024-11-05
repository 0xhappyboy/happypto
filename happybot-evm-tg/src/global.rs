use std::error::Error;

use serde::{Deserialize, Serialize};
use teloxide::{
    dispatching::dialogue::GetChatId,
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{CallbackQuery, ParseMode},
    Bot,
};

use crate::{language::Language, trade::Trade, wallet::Wallet};

// global button struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Button {
    pub name: Option<String>,
    pub flag: Option<String>,
    pub value: Option<String>,
}

// global callback handling
pub async fn global_callback_handler(
    bot: Bot,
    q: CallbackQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let data = q.clone().data.unwrap();
    let button: Button = serde_json::from_str(&data).unwrap();
    match button.flag.clone().unwrap().as_str() {
        "trade_back_button" => {
            todo!()
        }
        "trade_refresh_button" => {
            todo!()
        }
        "buy_01_eth" => {
            todo!()
        }
        "buy_05_eth" => {
            todo!()
        }
        "buy_1_eth" => {
            todo!()
        }
        "buy_3_eth" => {
            todo!()
        }
        "buy_5_eth" => {
            todo!()
        }
        "buy_x_eth" => {
            todo!()
        }
        "sell_10_percen" => {
            todo!()
        }
        "sell_20_percen" => {
            todo!()
        }
        "sell_30_percen" => {
            todo!()
        }
        "sell_50_percen" => {
            todo!()
        }
        "sell_100_percen" => {
            todo!()
        }
        "sell_x_percen" => {
            todo!()
        }
        "wallet" => {
            todo!()
        }
        "start_trade" => {
            let trade: Trade = Trade::new();
            bot.send_message(q.chat_id().unwrap(), trade.text)
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(trade.keyboard.unwrap())
                .await?;
        }
        "start_wallet" => {
            let wallet: Wallet = Wallet::new();
            bot.send_message(q.chat_id().unwrap(), wallet.text)
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(wallet.keyboard.unwrap())
                .await?;
        }
        "start_language" => {
            let language: Language = Language::new();
            bot.send_message(q.chat_id().unwrap(), language.text)
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(language.keyboard.unwrap())
                .await?;
        }
        _ => {}
    }
    Ok(())
}
