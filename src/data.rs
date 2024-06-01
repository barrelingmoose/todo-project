use serde_derive::{Deserialize, Serialize};
pub use crate::monitor::Progress; 

#[derive(Clone, Deserialize, Serialize)]
pub struct TodoItem{
    pub item_description: String,
    pub item_progress: Progress,
}

impl std::fmt::Display for TodoItem{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        if self.item_progress == Progress::InProgress{
            write!(f, "{} | {}", self.item_progress, self.item_description)
        }
        else if self.item_progress == Progress::Completed{
            write!(f, "{}   | {}", self.item_progress, self.item_description)
        }
        else{
            write!(f, "{}\t       | {}", self.item_progress, self.item_description)
        }
    }
}

