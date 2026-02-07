use std::path::PathBuf;

use crate::input_mod::InputMod;

pub fn display() -> InputMod {
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

    println!("Do you want to upload a different timetable? (y/n):");
    let mut update = String::new();
    let mut file_path = String::new();

    std::io::stdin()
        .read_line(&mut update)
        .expect("Failed to read input");
    if update.trim() == "y" {
        println!("Enter the file directory: (e.g., C:/Users/USER/Documents/timetable.xlsx)");
        std::io::stdin()
            .read_line(&mut file_path)
            .expect("Failed to read input");
        println!("File path set to: {}", file_path);
    } else {
        file_path = String::from("assets/tables.xlsx");
    }
    println!("Enter your course abbreviation and year (e.g., CE 4):");
    let mut level_input = String::new();
    std::io::stdin().read_line(&mut level_input).unwrap();
    let file = file_path.trim();
    InputMod {
        level: level_input.trim().to_string(),
        file_path: PathBuf::from(file),
    }
}
