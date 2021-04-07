use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

use chrono::offset;

mod interpolation;


#[derive(Debug)]
struct UserMessage<'a> {
    sender_id: String,
    text: &'a str,
    intent: String,
}

impl<'a> UserMessage<'a> {
    fn new(input: &'a str) -> Self {
        UserMessage {
            sender_id: String::from("0"),
            text: input,
            intent: String::from(input),
        }
    }
}

#[derive(Debug)]
enum Action<'a> {
    UtterMessage(&'a str),
    CurrentTime,
    PlainTextSlotSet,
}

#[derive(Debug)]
enum Event<'responses, 'slots> {
    BotUtteredTemplate(&'responses str),
    BotUtteredText(String),
    SlotSet(&'slots str, Option<String>),
}

impl<'responses, 'slots> Action<'responses> {
    async fn run(
        &self,
        user_message: &UserMessage<'_>,
    ) -> Vec<Event<'responses, 'slots>> {
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

async fn process_messsage<'slots, 'responses>(
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


#[tokio::main]
async fn main() {
    let responses: HashMap<_, _> = vec![
        ("utter_hello", "Oi, {plain_text}!"),
        ("utter_bye", "Tchau!"),
        ("default_fallback", "Ops, não te entendi"),
    ].into_iter()
     .collect();
    let arc_responses = Arc::new(responses);

    let rules: HashMap<_, _> = vec![
        ("oi", Action::UtterMessage("utter_hello")),
        ("tchau", Action::UtterMessage("utter_bye")),
        ("horas", Action::CurrentTime),
        ("text:", Action::PlainTextSlotSet),
    ].into_iter()
     .collect();
    let arc_rules = Arc::new(rules);

    let slots: HashMap<_, Option<String>> = vec![
        ("plain_text", None),
    ].into_iter()
     .collect();
    let arc_slots = Arc::new(Mutex::new(slots));

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let arc_responses_clone = arc_responses.clone();
        let arc_rules_clone = arc_rules.clone();
        let arc_slots_clone = arc_slots.clone();
        tokio::spawn(async move {
            process_messsage(
                arc_responses_clone,
                arc_rules_clone,
                arc_slots_clone,
                &line.unwrap())
                .await
        });
    }
}
