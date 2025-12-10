use crate::guess_module::GuessNumber;
mod guess_module;


fn main() -> iced::Result {
    iced::run(GuessNumber::update, GuessNumber::view)
}