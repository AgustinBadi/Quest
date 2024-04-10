use serde::{Serialize, Deserialize};
use std::io;
use std::fs;
use std::io::Write;
use std::env;

#[derive(Debug, Serialize, Deserialize, Default)]
struct ProgramState {
    user: String,
    points: u64,
    tasks: Vec<String>,
    trash_can: Vec<String>,
}

fn main() {

    let mut quit = false;


    let mut list: Vec<String> = get_state_file().tasks;


    println!("Hi, I'm your todo manager! 💻");
    println!("type 'p' to print, 'a' to add, 'd' to del, 'q' to quit.");

    while quit == false {
        let mut input = String::new();

        let _ = io::stdin().read_line(&mut input);
        input = input.trim().to_string();

        if input == "a" {
            println!("Type the task...");
            input = "".to_string();
            let _ = io::stdin().read_line(&mut input);
            input = input.trim().to_string();
            list.push(String::from(input));
        } else if input == "p" {
            print_tasks(&list);
        } else if input == "d" {
            //list.pop();
            list = del_task(&list).to_vec();
        } else if input == "q" {
            save_state(&list);
            println!("Bye bye.. 🖖");
            quit  = true;
        } else {
            println!("type 'p' to print, 'a' to add, 'd' to del, 'q' to quit.");
        }
    }
}

fn print_tasks(xs: &Vec<String>) -> () {
    let mut counter: u8 = 1; 
    for task in xs {

        println!("{}. {}", counter, task);
        counter += 1;
    }
}

fn del_task(xs: &Vec<String>) -> Vec<String> {
    println!("Which one?");
    print_tasks(xs);

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    input = input.trim().to_string();

    let num: Result<u8, _> = input.parse(); // Parse string to u8
    let number = match num {
        Ok(n) => n, // Extract the value from Ok variant
        Err(e) => {
            // Handle the error gracefully
            println!("Error parsing u8: {}", e);
            // Return a default value or handle the error in some other way
            // Here, we return 0 as a default value
            0
        }
    };

    let mut new_tasks = Vec::new();
    let mut counter: u8 = 1; 

    for task in xs {
        if number != counter {
            new_tasks.push(String::from(task));
        } else {
            println!("task {} moved to 🗑️", counter);
        }
        counter += 1;
    }

    new_tasks
}

fn get_state_file() -> ProgramState  {
    let current_dir = env::current_dir().unwrap();
    
    // Check if any file named "state.json" exists
    let is_state_json_present = current_dir.iter()
        .any(|entry| entry == "state.json");

        
    if is_state_json_present {
        let json_string = fs::read_to_string("state.json").unwrap();

        let my_data: ProgramState =  serde_json::from_str(&json_string).unwrap();

        return my_data;
    } else {
        let new_data = serde_json::to_string(&ProgramState::default()).unwrap(); 

        let _ = fs::File::create("state.json").unwrap().write_all(new_data.as_bytes());
        return ProgramState::default();
    }
        
}

fn save_state(list: &Vec<String>) -> () {
    let mut new_data = ProgramState::default();
    new_data.tasks = list.to_vec();
    let update_file = serde_json::to_string(&new_data).unwrap(); 


    let _ = fs::File::create("state.json").unwrap().write_all(update_file.as_bytes());   
}






// 1. Add scores (Did I finish all task each day?)
// 2. Implement State (JSON)
// 3. Add sections (life, development, university, health, etc..).
// 4. Authentication
// 5. Encryption
// 6. Add time stamps
// 7. Trash can
// 8. 

