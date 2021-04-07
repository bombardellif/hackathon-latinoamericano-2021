use std::collections::HashMap;

use regex::Regex;

pub fn inflate(
    template: &str,
    kwargs: &HashMap<&str, Option<String>>,
) -> String {
    let re = Regex::new(r"\{(?P<name>.+?)\}").unwrap();

    let mut result = String::with_capacity(template.len());
    let mut current_end = 0;
    for capture in re.captures_iter(template) {
        let re_match = capture.get(0).unwrap();
        result.push_str(&template[current_end..re_match.start()]);
        current_end = re_match.end();

        let slot_val = kwargs
            .get(&capture["name"])
            .unwrap()
            .as_ref()
            .map_or("", String::as_str);
        result.push_str(slot_val);
    }
    result.push_str(&template[current_end..]);
    result
}
