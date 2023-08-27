use core::fmt;

enum TaskStatus {
    DONE,
    PENDING,
}

impl TaskStatus {
    fn stringify(&self) -> String {
        match &self {
            &Self::DONE => "DONE".to_string(),
            &Self::PENDING => "PENDING".to_string(),
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            &Self::DONE => {
                write!(f, "DONE")
            }
            &Self::PENDING => {
                write!(f, "PENDING")
            }
        }
    }
}

fn main() {
    println!("{}", TaskStatus::DONE);
    println!("{}", TaskStatus::PENDING);
    let outcome = TaskStatus::DONE.to_string();
    println!("{}", outcome);
}
