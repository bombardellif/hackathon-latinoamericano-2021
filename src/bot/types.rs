#[derive(Debug)]
pub enum Event<'responses, 'slots, T> {
    BotUtteredTemplate(&'responses str),
    BotUtteredText(String),
    SlotSet(&'slots str, T),
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Intent {
    Greet,
    Goodbye,
    AskHours,
    PlainText,
}

impl Intent {
    pub fn new(input: &str) -> Self {
        if input.contains("oi") {
            Intent::Greet
        } else if input.contains("tchau") {
            Intent::Goodbye
        } else if input.contains("horas") {
            Intent::AskHours
        } else {
            Intent::PlainText
        }
    }
}

#[derive(Debug)]
pub struct UserMessage<'a> {
    pub sender_id: String,
    pub text: &'a str,
    pub intent: Intent,
}

impl<'a> UserMessage<'a> {
    pub fn new(input: &'a str) -> Self {
        UserMessage {
            sender_id: String::from("0"),
            text: input,
            intent: Intent::new(input),
        }
    }
}
