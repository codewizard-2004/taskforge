#[derive(Debug, PartialEq, Eq)]
pub enum Priority{
    Low,
    Medium,
    High
}

#[derive(Debug, PartialEq, Eq)]
pub enum Status{
    Todo,
    Ongoing,
    Done,
}

#[derive(Debug)]
pub struct Task{
    pub id: u64,
    pub title: String,
    pub priority: Priority,
    pub deadline: Option<String>,
    pub status: Status
}