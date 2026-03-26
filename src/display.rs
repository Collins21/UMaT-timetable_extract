use std::path::PathBuf;

use crate::{exam_engine, input_mod::InputMod, timetable_engine};

pub fn display() {
    let opening = r#"
░██     ░██ ░███     ░███            ░██████████
░██     ░██ ░████   ░████                ░██    
░██     ░██ ░██░██ ░██░██  ░██████       ░██    
░██     ░██ ░██ ░████ ░██       ░██      ░██    
░██     ░██ ░██  ░██  ░██  ░███████      ░██    
 ░██   ░██  ░██       ░██ ░██   ░██      ░██    
  ░██████   ░██       ░██  ░█████░██     ░██    
                                                
   _____ _                _        _     _                  _                  _             
  |_   _(_)_ __ ___   ___| |_ __ _| |__ | | ___    _____  _| |_ _ __ __ _  ___| |_ ___  _ __ 
    | | | | '_ ` _ \ / _ \ __/ _` | '_ \| |/ _ \  / _ \ \/ / __| '__/ _` |/ __| __/ _ \| '__|
    | | | | | | | | |  __/ || (_| | |_) | |  __/ |  __/>  <| |_| | | (_| | (__| || (_) | |   
    |_| |_|_| |_| |_|\___|\__\__,_|_.__/|_|\___|  \___/_/\_\\__|_|  \__,_|\___|\__\___/|_|   
                                                                                                                                          
    "#;
    println!("{}", opening);

    println!(
        "What do you want to work on today? \n [1] extract class timetable \n [2] extract exam schedule"
    );
    let mut update = String::new();
    let mut file_path = String::new();

    std::io::stdin()
        .read_line(&mut update)
        .expect("Failed to read input");
    println!("Enter the file directory: (e.g., C:/Users/USER/Documents/file.xlsx): ");
    std::io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read input");
    println!("File path set to: {}", file_path);

    if update.trim() == "1" {
        println!("Enter your course abbreviation and year (e.g., CE 4): ");
    } else {
        println!("Enter your course abbreviation and year (e.g., CE IV): ");
    }
    let mut level_input = String::new();
    std::io::stdin().read_line(&mut level_input).unwrap();
    let file = file_path.trim().to_string().replace("\"", "");
    let args = InputMod {
        level: level_input.trim().to_string().to_lowercase(),
        file_path: PathBuf::from(file),
    };
    if update.trim() == "1" {
        println!("Timetable extraction selected...");
        timetable_engine::extract_timetable(args);
    } else if update.trim() == "2" {
        println!("Exam schedule extraction selected.");
        exam_engine::extract_exam_schedule(args);
    } else {
        println!("Invalid selection. Defaulting to timetable extraction.");
        timetable_engine::extract_timetable(args);
    }
}
