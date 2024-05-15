pub use crate::data::TodoItem;
pub use crate::monitor::Progress;
use std::io::{self, Write}; 

fn add_item(item_list: &mut Vec<TodoItem>, new_item: TodoItem){
    item_list.push(new_item); 
}

fn delete_item(item_list: &mut Vec<TodoItem>, index: usize){
    item_list.remove(index); 
}

fn update_item(item_list: &mut Vec<TodoItem>, index: usize, update: Progress){
    item_list[index].item_progress = update; 
}

pub fn run(item_list: &mut Vec<TodoItem>){
    let mut user_input = String::new(); 
    while user_input.trim() != "Exit"{
        user_input.clear();
        std::io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).unwrap();
        if user_input.trim() == "add"{
            let item_string = String::from("Test");
            let item_enum = Progress::Todo;
            let new_item = TodoItem{item_description: item_string,
                                    item_progress: item_enum};

            add_item(item_list, new_item);
            print!("{}\n",item_list[0]);
            user_input.clear();
        }
        else if user_input.trim() == "update"{
            let item_update = Progress::InProgress; 
            update_item(item_list, 0, item_update);
            print!("{}\n",item_list[0]);
            user_input.clear();
        }
        else if user_input.trim() == "delete"{
            delete_item(item_list, 0); 
            user_input.clear();
        }
        else if user_input.trim() == "print"{
            for el in & *item_list{
                print!("{}\n", el);
                user_input.clear();
            }
        }
    }
}

#[test]
fn test_add(){
    let mut todo_list: Vec<TodoItem> = Vec::new();
    let item_string = String::from("Test");
    let item_enum = Progress::Todo;
    let new_item = TodoItem{item_description: item_string,
                            item_progress: item_enum};
    add_item(&mut todo_list, new_item);
    assert_eq!(todo_list[0].item_description, "Test")
}

#[test]
fn test_delete(){
    let mut todo_list: Vec<TodoItem> = Vec::new();
    let item_string = String::from("Test");
    let item_enum = Progress::Todo;
    let new_item = TodoItem{item_description: item_string,
                            item_progress: item_enum};
    add_item(&mut todo_list, new_item);
    delete_item(&mut todo_list, 0); 
    assert_eq!(todo_list.is_empty(), true);
}

#[test]
fn test_update(){
    let mut todo_list: Vec<TodoItem> = Vec::new();
    let item_string = String::from("Test");
    let item_enum = Progress::Todo;
    let new_item = TodoItem{item_description: item_string,
                            item_progress: item_enum};
    let item_update = Progress::InProgress; 
    add_item(&mut todo_list, new_item);
    update_item(&mut todo_list, 0, item_update);
    assert_eq!(todo_list[0].item_progress, Progress::InProgress)
}
