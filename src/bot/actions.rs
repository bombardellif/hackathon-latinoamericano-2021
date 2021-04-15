use chrono::offset;

use crate::bot::types::*;

#[derive(Debug)]
pub enum Action<'a> {
    UtterMessage(&'a str),
    CurrentTime,
    PlainTextSlotSet,
}

pub type SlotVal = Option<String>;

impl<'responses, 'slots> Action<'responses> {
    pub async fn run(
        &self,
        user_message: &UserMessage<'_>,
    ) -> Vec<Event<'responses, 'slots, SlotVal>> {
        match self {
            Action::UtterMessage(tpl_name) =>
                vec![
                    Event::BotUtteredTemplate(tpl_name),
                ],
            Action::CurrentTime =>
                vec![
                    Event::BotUtteredText(
                        offset::Local::now().time()
                            .format("%H:%M:%S")
                            .to_string()),
                ],
            Action::PlainTextSlotSet =>
                vec![
                    Event::SlotSet(
                        "plain_text",
                        Some(user_message.text.to_string())),
                ],
        }
    }
}
