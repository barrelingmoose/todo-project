pub use crate::data::TodoItem;

/*
    This file contains helper functions for reading and writing to the json file
    The file is stored in the same directory as the executable
    The file name is test.json

    The following functions are used in the operations.rs file:
        --> read_from_json()
        --> write_to_json()

    read_from_json() and write_to_json() are used in the operations.rs file

    Notes: 
        -- Currently, the file is overwritten every time the program is run. This 
        should be changed in the future to optimize the program.

        -- The file is stored in the same directory as the executable. This should
        be changed in the future.

        -- The file name is test.json. This should be changed in the future. 
        The user should be able to change the file name in the future.

        -- The write_to_json function us using two pops to remove the last comma and
        last bracket. This should be changed in the future.
*/
pub fn write_to_json(item_list: &mut Vec<TodoItem>)->Result<(), std::io::Error>{
    let mut i: String = "[\n".to_owned();
    for el in item_list{
        let j = serde_json::to_string_pretty(el)?;
        i.push_str(&j); 
        i.push_str(",\n");
    }
    // This is very ugly and I should find a more elegant solution
    i.pop();
    i.pop(); 
    i.push_str("\n]");
    let output_path = "./test.json";
    std::fs::write(output_path, i.clone());
    Ok(())
}

pub fn read_from_json()->Result<Vec<TodoItem>, std::io::Error>{
    let input_path = "./test.json";
    let contents = std::fs::read_to_string(input_path)?;
    let item_list: Vec<TodoItem> = serde_json::from_str(&contents)?;
    Ok(item_list)
}