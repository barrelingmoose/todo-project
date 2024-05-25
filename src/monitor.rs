use serde_derive::{Deserialize,Serialize};
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Progress{
    Todo, 
    InProgress,
    Completed,
    Invalid
}

impl std::fmt::Display for Progress{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        match self{
        Self::Todo => write!(f, "Todo"),
        Self::InProgress => write!(f, "In Progress"),
        Self::Completed => write!(f, "Completed"),
        Self::Invalid => write!(f, "Invalid")
        }
    }
}  

#[test]
fn test_enum(){
    let todo = Progress::Todo; 
    let in_progress = Progress::InProgress; 
    let completed = Progress::Completed; 
    assert_eq!("Todo", todo.to_string());
    assert_eq!("In Progress", in_progress.to_string());
    assert_eq!("Completed", completed.to_string());
}
