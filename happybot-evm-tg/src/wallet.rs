use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::global::Button;

pub struct Wallet {
    pub text: String,
    pub keyboard: Option<InlineKeyboardMarkup>,
}

impl Wallet {
    pub fn new() -> Self {
        let welcome_text = format!(
            "*Current Wallet* :\r\n{}\r\n\r\n💰Balance: {} ETH \\(profit and loss \\+$2000\\)",
            "0x1", // token name
            "123", // token smart contract address
        );
        Self {
            text: welcome_text,
            keyboard: Some(make_keyboard()),
        }
    }
}

fn make_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let wallet_list = [
        Button {
            name: Some(String::from("0x1")),
            flag: Some(String::from("wallet")),
            value: Some(String::from("0x1")),
        },
        Button {
            name: Some(String::from("0x2")),
            flag: Some(String::from("wallet")),
            value: Some(String::from("0x2")),
        },
        Button {
            name: Some(String::from("0x3")),
            flag: Some(String::from("wallet")),
            value: Some(String::from("0x3")),
        },
        Button {
            name: Some(String::from("0x4")),
            flag: Some(String::from("wallet")),
            value: Some(String::from("0x4")),
        },
    ];
    for wallet in wallet_list.chunks(1) {
        let row = wallet
            .iter()
            .map(|w| {
                InlineKeyboardButton::callback(
                    w.name.clone().unwrap(),
                    serde_json::to_string(&w).unwrap(),
                )
            })
            .collect();
        keyboard.push(row);
    }
    InlineKeyboardMarkup::new(keyboard)
}
