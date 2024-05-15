mod monitor;
mod data; 

pub use crate::monitor::Progress;
pub use crate::data::TodoItem; 


fn main() {
    let item_string = String::from("Test");
    let item_enum = monitor::Progress::Todo;
    let new_item = TodoItem{item_description: item_string,
                            item_progress: item_enum};
    println!("{}",new_item);
}
