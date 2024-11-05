use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::global::Button;

pub struct Start {
    pub text: String,
    pub keyboard: Option<InlineKeyboardMarkup>,
}

impl Start {
    pub fn new() -> Self {
        let welcome_text = format!(
            "💎Happy Trade Bot\r\n\r\n👛Your wallet list\n{}\r\n\r\n💰Wallet Balance: {}\r\n\r\n",
            "`0x00`", // list of all wallet addresses created by the user in this bot
            1,        // total user wallet balance
        );
        Self {
            text: welcome_text,
            keyboard: Some(make_keyboard()),
        }
    }
}

fn make_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let buttons = [
        Button {
            name: Some(String::from("💰 Trade")),
            flag: Some(String::from("start_trade")),
            value: None,
        },
        Button {
            name: Some(String::from("👛 Wallet")),
            flag: Some(String::from("start_wallet")),
            value: None,
        },
        Button {
            name: Some(String::from("🌏 Language")),
            flag: Some(String::from("start_language")),
            value: None,
        },
    ];
    for button in buttons.chunks(3) {
        let row = button
            .iter()
            .map(|b| {
                InlineKeyboardButton::callback(
                    b.name.clone().unwrap(),
                    serde_json::to_string(b).unwrap(),
                )
            })
            .collect();
        keyboard.push(row);
    }
    InlineKeyboardMarkup::new(keyboard)
}
