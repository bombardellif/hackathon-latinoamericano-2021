#[derive(Debug)]
pub enum Event<'responses, 'slots> {
    BotUtteredTemplate(&'responses str),
    BotUtteredText(String),
    SlotSet(&'slots str, Option<String>),
}

#[derive(Debug)]
pub struct UserMessage<'a> {
    pub sender_id: String,
    pub text: &'a str,
    pub intent: String,
}

impl<'a> UserMessage<'a> {
    pub fn new(input: &'a str) -> Self {
        UserMessage {
            sender_id: String::from("0"),
            text: input,
            intent: String::from(input),
        }
    }
}
