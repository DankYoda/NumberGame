use iced::widget::{button, column, text, text_input, Column, Theme, Renderer};
use rand::Rng;
use std::cmp::Ordering;
use iced::{Fill, Center};

fn main() -> iced::Result {
    iced::application(GuessNumber::default, GuessNumber::update, GuessNumber::view)
        .title("Iced Iced Baby")
        .centered()
        .run()
}

#[derive(Clone)]
pub enum Message {
    CheckValue,
    ContentChanged(String)
}

pub struct GuessNumber {
    message: String,
    guess: String,
    secret_number: u32
}

impl GuessNumber {
    pub fn view(&self) -> Column<'_, Message> {
        // We use a column: a simple vertical layout
        column![
            text(&self.message),
            text_input::<Message, Theme, Renderer>("", &self.guess)
                .on_input(Message::ContentChanged).width(50),
            button("Check Value").on_press(Message::CheckValue),
        ]
        .padding(100)
        .width(Fill)
        .align_x(Center)
        .spacing(50)
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::CheckValue => {
                let guess: u32 = self.guess.trim().parse().unwrap_or_else(|_| 0);
                match guess.cmp(&self.secret_number) {
                    Ordering::Less => self.message = String::from("Too small!"),
                    Ordering::Greater => self.message = String::from("Too big!"),
                    Ordering::Equal => self.message = String::from("You win!"),
                }
            }
            Message::ContentChanged(content) => {
                self.guess = content;
            }
        }
    }
}
impl Default for GuessNumber {
    fn default() -> Self {
        Self {
            message: String::from("Guess the secret number!"),
            guess: String::from("0"),
            secret_number: rand::rng().random_range(1..=100),
        }
    }
}