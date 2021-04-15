pub mod interpolation;
pub mod types;
pub mod actions;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::bot::types::*;
use crate::bot::actions::*;

fn process_event<'slots, 'responses>(
    responses: &HashMap<&'responses str, &'responses str>,
    slots: &Mutex<HashMap<&'slots str, SlotVal>>,
    event: Event<'responses, 'slots, SlotVal>,
) {
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

pub async fn process_messsage<'slots, 'responses>(
    responses: Arc<HashMap<&'responses str, &'responses str>>,
    rules: Arc<HashMap<Intent, Action<'responses>>>,
    slots: Arc<Mutex<HashMap<&'slots str, SlotVal>>>,
    input: &str,
) {
    let fallback_rule = Action::UtterMessage("default_fallback");

    let user_message = UserMessage::new(input);

    let action = rules.get(&user_message.intent)
        .unwrap_or(&fallback_rule);

    for event in action.run(&user_message).await {
        process_event(&responses, &slots, event)
    }
}
