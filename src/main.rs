mod monitor;
mod data; 
mod operations; 
mod json_helper;

pub use crate::monitor::Progress;
pub use crate::data::TodoItem;


fn main() {
    let mut todo_list: Vec<TodoItem> = Vec::new(); 
    operations::run(&mut todo_list); 
}
