pub use crate::data::TodoItem;
pub use crate::monitor::Progress;
use std::io::{self, Write}; 

fn delete_command(item_list: &mut Vec<TodoItem>, index: usize){
    item_list.remove(index); 
}

fn update_command(item_list: &mut Vec<TodoItem>, index: usize, update: Progress){
    item_list[index].item_progress = update; 
}

fn add_command(item_list: &mut Vec<TodoItem>){
    print!("Provide Item Descripton: ");
    let mut item_string = String::new(); 
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut item_string).unwrap();
    let item_enum = Progress::Todo;
    let new_item = TodoItem{item_description: item_string.trim().to_string(),
                            item_progress: item_enum};
    item_list.push(new_item.clone()); 
    print!("{}\n",new_item);
}

fn print_command(item_list: &mut Vec<TodoItem>){
    print!("   Descripton | Progress\n");
    for (i, el) in &mut item_list.iter().enumerate(){
        print!("[{}]{}\n", i, el);
    }
}

fn select_index() -> usize{
    print!("Select Item to update: ");
    let mut index_req = String::new(); 
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut index_req).unwrap();
    let num: usize = index_req.trim().parse().expect("Please type a number!");
    index_req.clear();
    return num; 
}

fn progress_update() -> Progress{
    print!("New Progress: ");
    let mut progress_upd: String = String::new(); 
    std::io::stdout().flush().unwrap(); 
    io::stdin().read_line(&mut progress_upd).unwrap();
    if progress_upd.trim() == "Todo"{
        return Progress::Todo;
    }
    else if progress_upd.trim() == "In Progress"{
        return Progress::InProgress;
    }
    else if progress_upd.trim() == "Completed"{
        return Progress::Completed;
    }
    else{
        print!("Invalid update\n"); 
    }
    progress_upd.clear(); 
    return Progress::Invalid;
}

// Might be a good place to test out a new design pattern 
fn process_user_command(item_list: &mut Vec<TodoItem>, user_input: &mut String){
    if user_input.trim() == "add"{
        add_command(item_list);
        user_input.clear();
    }
    else if user_input.trim() == "update"{
        let num :usize = select_index(); 
        let new_prog: Progress = progress_update(); 
        if new_prog!=Progress::Invalid{
            update_command(item_list, num, new_prog);
        }
        user_input.clear();
    }
    else if user_input.trim() == "delete"{
        let num :usize = select_index(); 
        delete_command(item_list, num); 
        user_input.clear();
    }
    else if user_input.trim() == "print"{
        print_command(item_list);
        user_input.clear();
    }

}

pub fn run(item_list: &mut Vec<TodoItem>){
    let mut user_input = String::new(); 
    while user_input.trim() != "Exit"{
        user_input.clear();
        std::io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).unwrap();
        process_user_command(item_list, &mut user_input);
    }
}
