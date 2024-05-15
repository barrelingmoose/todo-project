pub enum Progress{
    Todo, 
    InProgress,
    Completed
}

impl std::fmt::Display for Progress{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        match self{
        Self::Todo => write!(f, "Todo"),
        Self::InProgress => write!(f, "In Progress"),
        Self::Completed => write!(f, "Completed"),
        }
    }
}  

#[test]
fn test_enum(){
    let todo = Todo; 
    let in_progress = InProgress; 
    let completed = Completed; 
    assert_eq!("Todo", todo.to_string());
    assert_eq!("In Progress", in_progress.to_string());
    assert_eq!("Completed", completed.to_string());
}
