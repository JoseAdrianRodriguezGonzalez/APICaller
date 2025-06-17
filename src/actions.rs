use colored::*;

use crate::terminal::{ContentType,curl,request_data,create_folder};
use crate::menu::main_menu;
use crate::tts_functions::*;

pub fn run_text_to_tts(){
    println!("{}","Tarot selected".blue());
    let options=vec![
        "Initialize".green().bold(),
        "synthesize text to tts".cyan().bold(),
        "Quit".red().bold()
    ];
    let ip=request_data("Write the ip: ");
    loop{
        let selected=main_menu(&options);
        match selected{
            0=>initialize(&ip),
            1=>synthesize(&ip),
            _=>{
                println!("{}","Leaving the menu from the API".yellow());
                break;
            }
        }
    }
}
