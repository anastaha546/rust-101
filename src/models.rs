use std::str::FromStr;


#[derive(Debug)]pub struct Task {
    pub id: i32,
    pub title: String,
    pub status: TaskStatus,
}

#[derive(Debug)]
pub enum TaskStatus {
    Active,
    Completed,
    Error
}

impl FromStr for TaskStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(TaskStatus::Active),
            "completed" => Ok(TaskStatus::Completed),
            other => Err(format!("Invalid status: {}", other)),
        }
    }
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Active 
    }
}