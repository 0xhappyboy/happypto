use teloxide::types::InlineKeyboardMarkup;

pub struct CommandError {
    pub text: String,
    pub keyboard: Option<InlineKeyboardMarkup>,
}

impl CommandError {
    pub fn new() -> Self {
        let welcome_text = format!("Please enter the correct command! ",);
        Self {
            text: welcome_text,
            keyboard: None,
        }
    }
}
