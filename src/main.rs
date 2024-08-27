use serde::{Serialize, Deserialize};
use std::io;
use std::fs;
use std::io::Write;
use std::env;

#[derive(Debug, Serialize, Deserialize, Default)]

//struct TaskRecord {
//    dots: Vec<u8>,
//    day: ..,
//}

struct ProgramState {
    user: String,
    points: u64,
    tasks: Vec<String>,
}


//    Implementar time dots con un Vector eg. [3,0,1,3,5,..] donde cada elemento es la cantidad de tareas diaras. 

    

fn main() {

    let mut quit = false;

    let prog_state: ProgramState = get_state_file();
    let name_state: String = prog_state.user; 
    let mut points: u64 = prog_state.points;
    let mut tasks: Vec<String> = prog_state.tasks;

    let name = get_name(name_state);

    println!("Hi {}, I'm your todo manager! 💻", name);
    println!("Your points are: {} 🔥", points);
    println!("Type 'p' to print, 'a' to add, 'd' to del, 'f' to finish and 'q' to quit.");

    while quit == false {
        let mut input = String::new();

        let _ = io::stdin().read_line(&mut input);
        input = input.trim().to_string();

        if input == "a" {
            println!("Type the task...\n");
            input = "".to_string();
            let _ = io::stdin().read_line(&mut input);
            input = input.trim().to_string();
            tasks.push(String::from(input));
            println!("Task added 👌");
        } else if input == "p" {
            print_tasks(&tasks);
        } else if input == "d" {
            //tasks.pop();
            tasks = del_task(&tasks , false).to_vec();
        } else if input == "f" {
            tasks = del_task(&tasks, true).to_vec();
            println!("+10 points! 💯");
            points += 10;
        } else if input == "q" {
            println!("Bye bye.. 🖖");
            quit  = true;
        } else {
            println!("type 'p' to print, 'a' to add, 'd' to del, 'q' to quit.");
        }
    }

    save_state(name, points,  &tasks);
}

fn get_name(name: String) -> String {
    let mut input = String::new();

    if name == "" {
        println!("Seems you are new uh? Type you name buddy...");
        let _ = io::stdin().read_line(&mut input);
        input = input.trim().to_string();
        return input;
    } else {
        return name;
    }
}

fn print_tasks(xs: &Vec<String>) -> () {
    let mut counter: u8 = 1; 
    for task in xs {

        println!("{}. {}", counter, task);
        counter += 1;
    }
}

fn del_task(xs: &Vec<String>, finished: bool) -> Vec<String> {
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
            match finished {
                false => println!("task {} moved to 🗑️", counter),
                _ => println!("Great you finished the task!"),
            }
            
        }
        counter += 1;
    }

    new_tasks
}

fn get_state_file() -> ProgramState  {
    let current_dir = env::current_dir().unwrap();

    //println!("Current directory: {:?}", current_dir);
    
    let is_state_json_present = std::fs::read_dir(&current_dir).unwrap()
    .any(|entry| entry.unwrap().file_name() == "state.json");

    //println!("is_state_json_present: {}", is_state_json_present);
        
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

fn save_state(name: String, points: u64,  tasks: &Vec<String>) -> () {
    let mut new_data = ProgramState::default();
    new_data.user = name;
    new_data.tasks = tasks.to_vec();
    new_data.points = points;
    let update_file = serde_json::to_string(&new_data).unwrap(); 


    let _ = fs::File::create("state.json").unwrap().write_all(update_file.as_bytes());   
}



// 1. Add scores (Did I finish all task each day?)
//  - Tasks: completed.
//  - Day Streak: 
//  - Points: +10 daily batch
// 2. Implement State (JSON)
// 3. Add sections (life, development, university, health, etc..).
// 4. Authentication
// 5. Encryption
// 6. Add time stamps
// 7. Trash can

// -- RPG Styled -- //

// Make Paths of character development. 
// Eg: Professional (Red), Personal (Green) and Vocacionally (Blue).
// Professionally: Things you do to survive.
// Personal: Things you do to grow.
// Vocacionally: Things that u do for others.

