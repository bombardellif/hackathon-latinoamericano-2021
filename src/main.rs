mod bot;

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

use crate::bot::actions::*;

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
            bot::process_messsage(
                arc_responses_clone,
                arc_rules_clone,
                arc_slots_clone,
                &line.unwrap())
                .await
        });
    }
}
