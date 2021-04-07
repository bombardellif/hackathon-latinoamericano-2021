pub mod interpolation;
pub mod types;
pub mod actions;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::bot::types::*;
use crate::bot::actions::*;

pub async fn process_messsage<'slots, 'responses>(
    responses: Arc<HashMap<&'responses str, &'responses str>>,
    rules: Arc<HashMap<&'responses str, Action<'responses>>>,
    slots: Arc<Mutex<HashMap<&'slots str, Option<String>>>>,
    input: &str,
) {
    let fallback_rule = Action::UtterMessage("default_fallback");

    let user_message = UserMessage::new(input);

    let action = rules.get(&user_message.intent as &str)
        .unwrap_or(&fallback_rule);

    for event in action.run(&user_message).await {
        match event {
            Event::BotUtteredTemplate(tpl_name) => {
                let response = responses.get(tpl_name).unwrap();
                let text_respose = interpolation::inflate(
                    response,
                    &slots.lock().unwrap());
                println!("{}", text_respose)
            }
            Event::BotUtteredText(text) =>
                println!("{}", text),
            Event::SlotSet(key, value) => {
                slots.lock().unwrap().insert(key, value);
            },
        }
    }
}
