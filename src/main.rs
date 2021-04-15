mod bot;

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

use crate::bot::actions::*;
use crate::bot::types::*;

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
        (Intent::Greet, Action::UtterMessage("utter_hello")),
        (Intent::Goodbye, Action::UtterMessage("utter_bye")),
        (Intent::AskHours, Action::CurrentTime),
        (Intent::PlainText, Action::PlainTextSlotSet),
    ].into_iter()
     .collect();
    let arc_rules = Arc::new(rules);

    let slots: HashMap<_, _> = vec![
        ("plain_text", Default::default()),
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
