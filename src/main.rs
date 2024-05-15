mod monitor;
mod data; 
mod operations; 

pub use crate::monitor::Progress;
pub use crate::data::TodoItem;


fn main() {
    let mut todo_list: Vec<TodoItem> = Vec::new(); 
    operations::run(&mut todo_list); 
    println!("{}",todo_list[0]);
}
