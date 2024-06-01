use std::io::{self, Write}; 

pub use crate::data::TodoItem;
pub use crate::monitor::Progress;
pub use crate::json_helper::{read_from_json, write_to_json}; 

/*
    This file contains the functions that are used in the main.rs file
    The file has the following functions:
            
        --> delete_command(item_list: &mut Vec<TodoItem>, index: usize)

            delete_command() is called when the user types "delete"
            it removes the item at the index specified by the user
        
        --> update_command(item_list: &mut Vec<TodoItem>, index: usize, update: Progress)

            update_command() is called when the user types "update"
            it updates the item at the index specified by the user with the new progress
        
        --> add_command(item_list: &mut Vec<TodoItem>)

            add_command() is called when the user types "add"
            it adds a new item to the item_list with the description provided by the user
        
        --> print_command(item_list: &mut Vec<TodoItem>)

            print_command() is called when the user types "print"
            it prints out the item_list with the following format:
                Descripton | Progress

        --> select_index() -> usize

            select_index() is called when the user types "update" or "delete"
            it prompts the user to select the item to update or delete
            it returns the index of the item selected by the user 
        
        --> progress_update() -> Progress

            progress_update() is called when the user types "update"
            it prompts the user to select the new progress
            it returns the new progress selected by the user  
        
        --> process_user_command(item_list: &mut Vec<TodoItem>, user_input: &mut String)

            process_user_command() is called when the user types a command
            it calls the appropriate function based on the command entered by the user
        
        --> run()
            run() is the main function that is called in the
            it reads the data from the json file and stores 
            it then continues to prompt the user for command
            it then writes the updated data to the json file

    Notes:
        -- Currently, a lot of things are kind of "hard coded". Might want to think about 
        implementing a Command Design Pattern in the future. 
*/

pub fn print_help(){
    println!("Commands: ");
    println!("  [add] - adds a new item to the list");
    println!("  [delete] - deletes an item from the list");
    println!("  [update] - updates an item from the list");
    println!("  [print] - prints out the list");
    println!("  [help] - prints out the command options");
    println!("  [clear] - clears command console");
    println!("  [exit] - exits the program");
}

fn delete_command(item_list: &mut Vec<TodoItem>, index: usize){
    item_list.remove(index); 
}

fn update_progress(item_list: &mut Vec<TodoItem>, index: usize, update: Progress){
    item_list[index].item_progress = update; 
}

fn update_description(item_list: &mut Vec<TodoItem>, index: usize, update: String){
    item_list[index].item_description = update; 
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
}

fn print_command(item_list: &mut Vec<TodoItem>){
    print!("   Progress    | Description\n");
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

fn description_update() -> String{
    print!("New Description: ");
    let mut new_desc = String::new(); 
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut new_desc).unwrap();
    let updated_desc = new_desc.trim().to_string();
    new_desc.clear();
    return updated_desc; 
}

fn progress_update() -> Progress{
    print!("New Progress: ");
    let mut progress_upd: String = String::new(); 
    std::io::stdout().flush().unwrap(); 
    io::stdin().read_line(&mut progress_upd).unwrap();
    if progress_upd.trim().to_lowercase() == "todo"{
        return Progress::Todo;
    }
    else if progress_upd.trim().to_lowercase() == "in progress"{
        return Progress::InProgress;
    }
    else if progress_upd.trim().to_lowercase() == "completed"{
        return Progress::Completed;
    }
    else{
        print!("Invalid update\n"); 
    }
    progress_upd.clear(); 
    return Progress::Invalid;
}

fn process_user_command(item_list: &mut Vec<TodoItem>, user_input: &mut String){
    if user_input.trim().to_lowercase() == "add"{
        add_command(item_list);
        print_command(item_list);
        user_input.clear();
    }
    else if user_input.trim().to_lowercase() == "update"{
        let num :usize = select_index(); 
        print!("Update Description or Progress: ");
        let mut update_type = String::new();
        std::io::stdout().flush().unwrap();
        io::stdin().read_line(&mut update_type).unwrap();
        if update_type.trim().to_lowercase() == "description"{
            let new_desc = description_update();
            update_description(item_list, num, new_desc);
        }
        else if update_type.trim().to_lowercase() == "progress"{
            let new_prog: Progress = progress_update(); 
            if new_prog!=Progress::Invalid{
                update_progress(item_list, num, new_prog);
            }
        }
        print_command(item_list);
        user_input.clear();
    }
    else if user_input.trim().to_lowercase() == "delete"{
        let num :usize = select_index(); 
        delete_command(item_list, num); 
        print_command(item_list);
        user_input.clear();
    }
    else if user_input.trim().to_lowercase() == "print"{
        print_command(item_list);
        user_input.clear();
    }
    else if user_input.trim().to_lowercase() == "help"{
        // TODO: Clean this up later.
        print_help(); 
        user_input.clear();
    }
    else if user_input.trim().to_lowercase() == "clear"{
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        user_input.clear();
    }
    else if user_input.trim().to_lowercase() == "purge"{
        let mut to_purge: Vec<usize> = Vec::new();
        for (i, element) in item_list.iter().enumerate(){
            if element.item_progress == Progress::Completed{
                to_purge.push(i);
            }
        }
        print!("{} items to be purged\n", to_purge.len());
        print!("{:?} indexes to be purged\n", to_purge);
        print!("Are you certain you want to delete these? (y/n): ");
        let mut purge_confirm = String::new();
        std::io::stdout().flush().unwrap();
        io::stdin().read_line(&mut purge_confirm).unwrap();
        if purge_confirm.trim().to_lowercase() == "y"{
            for i in (0..to_purge.len()).rev(){
                print!("[{}] {}\n", to_purge[i], item_list[to_purge[i]]);
                delete_command(item_list, to_purge[i]);
            }
        }
        print_command(item_list);
        user_input.clear();
    }
}

pub fn run(args: Vec<String>){
    let mut user_input = String::new(); 
    let json_path = args.get(1).unwrap();
    let mut item_list: Vec<TodoItem> = read_from_json(json_path).unwrap();
    while user_input.trim().to_lowercase() != "exit"{
        print!("Enter Command: ");
        user_input.clear();
        std::io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).unwrap();
        process_user_command(&mut item_list, &mut user_input);
    }
    // TODO: Come back here for error handling. 
    let _ = write_to_json(json_path, &mut item_list); 
}
