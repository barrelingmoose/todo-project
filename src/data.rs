pub use crate::monitor::Progress; 

pub struct TodoItem{
    pub item_description: String,
    pub item_progress: Progress
}

impl std::fmt::Display for TodoItem{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "Description: {}\nProgress: {}", self.item_description, self.item_progress)
    }
}


