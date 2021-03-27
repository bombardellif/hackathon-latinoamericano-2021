use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct UserMessage {
    sender_id: String,
    text: String,
    intent: String,
}

trait Action<'a> {
    fn run(&self, responses: &HashMap<&'a str, &'a str>) -> &'a str;
}

#[derive(Debug)]
struct UtterMessage<'a> {
    utterance_name: &'a str
}
impl<'a> Action<'a> for UtterMessage<'_> {
    fn run(&self, responses: &HashMap<&'a str, &'a str>) -> &'a str {
        responses.get(self.utterance_name).unwrap_or(&"")
    }
}

struct ActionCurrentTime {}
impl<'a> Action<'a> for ActionCurrentTime {
    fn run(&self, _: &HashMap<&'a str, &'a str>) -> &'a str {
        "18:00"
    }
}


fn parse_input(input: &str) -> UserMessage {
    UserMessage {
        sender_id: String::from("0"),
        text: String::from(input),
        intent: String::from(input),
    }
}

fn main() {
    let responses: HashMap<_, _> = vec![
        ("utter_hello", "Oi!"),
        ("default_fallback", "Ops, não te entendi"),
    ].into_iter()
     .collect();

    let rules: HashMap<_, &dyn Action> = vec![
        ("oi", &UtterMessage{utterance_name: "utter_hello"} as &dyn Action),
        ("horas", &ActionCurrentTime{} as &dyn Action),
    ].into_iter()
     .collect();

    let fallback_rule: &dyn Action =
        &UtterMessage{utterance_name: "default_fallback"};

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let user_message = parse_input(&line.unwrap());

        let action = rules.get(&user_message.intent as &str)
            .unwrap_or(&fallback_rule);

        let utterance = action.run(&responses);

        println!("{}", utterance);
    }
}
