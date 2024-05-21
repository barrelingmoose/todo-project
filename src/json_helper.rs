pub use crate::data::TodoItem;

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
    let output_path = "./test_json";
    std::fs::write(output_path, i.clone());
    Ok(())
}