pub fn string_to_title(s: &str) -> String {
    let mut titlecase = String::new();
    for word in s.split(" ") {
        let mut chars = word.chars();
        let next = chars.next();
        if next.is_none() {
            continue;
        }
        let first = next.unwrap().to_uppercase().to_string();
        let rest = chars.collect::<String>();
        titlecase.push_str(&first);
        titlecase.push_str(&rest);
        titlecase.push_str(" ");
    }
    titlecase
}