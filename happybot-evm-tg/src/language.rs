use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::global::Button;

pub struct Language {
    pub text: String,
    pub keyboard: Option<InlineKeyboardMarkup>,
}

impl Language {
    pub fn new() -> Self {
        Self {
            text: String::from("🌏 Switch Language:"),
            keyboard: Some(make_keyboard()),
        }
    }
}

fn make_keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let buttons = [
        Button {
            name: Some(String::from("English")),
            flag: Some(String::from("switch_language")),
            value: Some(String::from("en")),
        },
        Button {
            name: Some(String::from("简体")),
            flag: Some(String::from("switch_language")),
            value: Some(String::from("zh-Hans")),
        },
        Button {
            name: Some(String::from("繁体")),
            flag: Some(String::from("switch_language")),
            value: Some(String::from("zh-Hant")),
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
