pub fn to_list_item(s: &String) -> String {
    String::from("+ ") + s
}

pub fn to_heading(heading: &String) -> String {
    String::from("= ") + heading
}

pub fn empty_line() -> String {
    String::from("\n\\\n")
}

pub fn page_break() -> String {
    String::from("\n#pagebreak()\n")
}

pub fn reset_enum() -> String {
    String::from("#item-counter.update(0)\n")
}
