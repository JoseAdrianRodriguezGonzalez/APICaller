mod terminal;
mod menu;
mod actions;
mod tts_functions;
mod tarot_functions;
use colored::*;
fn main(){
    terminal::clear_screen();
    let optiones=vec!["Run API tarot".blue().bold(),
                                        "Run text to tts".green().bold(),
                                        "Quit".red().italic()];
    loop{
        let selection=menu::main_menu(&optiones);
        match selection{
            0=>actions::run_tarot(),
            1=>actions::run_text_to_tts(),
            _=>{
                println!("{}","Goodbye!".red());
                break;
            }
        }
    }
}