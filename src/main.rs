use std::io;


fn main() {
    let mut quit = false;

    let mut list: Vec<String> = Vec::new();


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

