#[derive(Debug)]
pub enum Priority {
    Lowest,
    Low,
    Normal,
    High,
    Emergency,
}

impl Priority {
    pub(crate) fn as_str(&self) -> String {
        match self {
            Priority::Lowest => String::from("-2"),
            Priority::Low => String::from("-1"),
            Priority::Normal => String::from("0"),
            Priority::High => String::from("1"),
            Priority::Emergency => String::from("2"),
        }
    }
}