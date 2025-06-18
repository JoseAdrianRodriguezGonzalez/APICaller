use colored::*;

use crate::terminal::{request_data};
use crate::menu::main_menu;
use crate::tts_functions::*;
use crate::tarot_functions::*;
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
pub fn run_tarot(){
    println!("{}","Tarot api selected".green());
    let options=vec![
        "OpenDay".blue().bold(),
        "Tarot-Complete".yellow().bold(),
        "Quit".red().bold()];
        let ip=request_data("Write the ip: ");
        loop {
            let selected=main_menu(&options);
            match selected {
                0=>open_day(&ip),
                1=>complete(&ip),
                _=>{
                    println!("{}","Leaving the menu from the tarot".yellow());
                    break;
                }
            }
        }
}