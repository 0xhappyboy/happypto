use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::global::Button;

pub struct Trade {
    pub text: String,
    pub keyboard: Option<InlineKeyboardMarkup>,
}

impl Trade {
    pub fn new() -> Self {
        let welcome_text = format!(
            "{}\r\n{}\r\n\r\n*Price* {}\r\nMC {}\r\nPool:{}\r\n\r\n💎*Your Hold* : {} \\({}\\)\r\n💰Balance: {}",
            "`token name`", // token name
            "`0x00`", // token smart contract address 
            1, // token price
            1, // token market cap
            1, // token liquidity pool
            "2 ETH", // number of tokens held by users
            "$999999", // the USD equivalent of the tokens held by the user
            "999 ETH" // users wallet balance
        );
        Self {
            text: welcome_text,
            keyboard: Some(make_keyboard()),
        }
    }
}

fn make_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    // line 1
    let other_function = [
        Button {
            name: Some(String::from("Back")),
            flag: Some(String::from("trade_back_button")),
            value: None,
        },
        Button {
            name: Some(String::from("Refresh")),
            flag: Some(String::from("trade_refresh_button")),
            value: None,
        },
    ];
    let buy_line = [Button {
        name: Some(String::from("-- Buy --")),
        flag: None,
        value: None,
    }];
    let buys = [
        Button {
            name: Some(String::from("🟢 Buy 0.1 ETH")),
            flag: Some(String::from("buy_01_eth")),
            value: None,
        },
        Button {
            name: Some(String::from("🟢 Buy 0.5 ETH")),
            flag: Some(String::from("buy_05_eth")),
            value: None,
        },
        Button {
            name: Some(String::from("🟢 Buy 1 ETH")),
            flag: Some(String::from("buy_1_eth")),
            value: None,
        },
        Button {
            name: Some(String::from("🟢 Buy 3 ETH")),
            flag: Some(String::from("buy_3_eth")),
            value: None,
        },
        Button {
            name: Some(String::from("🟢 Buy 5 ETH")),
            flag: Some(String::from("buy_5_eth")),
            value: None,
        },
        Button {
            name: Some(String::from("🟢 Buy x ETH")),
            flag: Some(String::from("buy_x_eth")),
            value: None,
        },
    ];
    let sell_line = [Button {
        name: Some(String::from("-- Sell --")),
        flag: None,
        value: None,
    }];
    let sells = [
        Button {
            name: Some(String::from("🔴 Sell 10%")),
            flag: Some(String::from("sell_10_percen")),
            value: None,
        },
        Button {
            name: Some(String::from("🔴 Sell 20%")),
            flag: Some(String::from("sell_20_percen")),
            value: None,
        },
        Button {
            name: Some(String::from("🔴 Sell 30%")),
            flag: Some(String::from("sell_30_percen")),
            value: None,
        },
        Button {
            name: Some(String::from("🔴 Sell 50%")),
            flag: Some(String::from("sell_50_percen")),
            value: None,
        },
        Button {
            name: Some(String::from("🔴 Sell 100%")),
            flag: Some(String::from("sell_100_percen")),
            value: None,
        },
        Button {
            name: Some(String::from("🔴 Sell x%")),
            flag: Some(String::from("sell_x_percen")),
            value: None,
        },
    ];
    for button in other_function.chunks(2) {
        let row = button
            .iter()
            .map(|version| {
                InlineKeyboardButton::callback(
                    version.name.to_owned().unwrap(),
                    serde_json::to_string(version).unwrap(),
                )
            })
            .collect();

        keyboard.push(row);
    }
    for button in buy_line.chunks(1) {
        let row = button
            .iter()
            .map(|version| {
                InlineKeyboardButton::callback(
                    version.name.to_owned().unwrap(),
                    serde_json::to_string(version).unwrap(),
                )
            })
            .collect();
        keyboard.push(row);
    }
    for button in buys.chunks(3) {
        let row = button
            .iter()
            .map(|version| {
                InlineKeyboardButton::callback(
                    version.name.to_owned().unwrap(),
                    serde_json::to_string(version).unwrap(),
                )
            })
            .collect();

        keyboard.push(row);
    }
    for button in sell_line.chunks(1) {
        let row = button
            .iter()
            .map(|version| {
                InlineKeyboardButton::callback(
                    version.name.to_owned().unwrap(),
                    serde_json::to_string(version).unwrap(),
                )
            })
            .collect();

        keyboard.push(row);
    }
    for button in sells.chunks(3) {
        let row = button
            .iter()
            .map(|version| {
                InlineKeyboardButton::callback(
                    version.name.to_owned().unwrap(),
                    serde_json::to_string(version).unwrap(),
                )
            })
            .collect();

        keyboard.push(row);
    }
    InlineKeyboardMarkup::new(keyboard)
}
