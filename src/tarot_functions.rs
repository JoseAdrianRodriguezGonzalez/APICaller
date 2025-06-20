use colored::*;
use crate::terminal::{curl,ContentType,request_data};
use crate::menu::main_menu;
pub fn open_day(ip:&str){
    println!("{}","Open day mode selected".green());
    let options=vec![
        "startReading".blue().bold(),
        "reading3Cards".yellow().bold(),
        "Quit".red().bold()];
        loop{
            let selected=main_menu(&options);
            match selected{
                0=>start_reading_openday(ip),
                1=>reading_openday(ip),
                _=>{
                    println!("{}","Quitting".yellow());
                    break;
                }
            }
        }
}
fn start_reading_openday(_ip:&str){
    let url=format!("http://{}/llm-api/start_openday_es",_ip);
    curl(&url, &"POST".to_string(), ContentType::Plain, &None, &None);
}
fn reading_openday(_ip:&str){
    let url=format!("http://{}/llm-api/reading_3_cards_es",_ip);
    let card1=request_data("Enter the first card");
    let card2=request_data("Enter the second card");
    let card3=request_data("Enter the third card");

    let json=format!(
        r#"{{"cards":["{}","{}","{}"]}}"#,
        card1,card2,card3
    );
    curl(&url, &"POST".to_string(), ContentType::Json, &Some(json), &None);
}
pub fn complete(ip:&str){
    println!("{}","You've selected the complete tarot's version: ".yellow().bold());
    let options=vec![
        "start".blue().bold(),
        "reading element".yellow().bold(),
        "reading union".green().bold(),
        "reading mix elements".cyan().bold(),
        "reading mix unions".magenta().bold(),
        "reading advice".white().bold(),
        "reading advice final".purple().bold(),
        "Quit".red().bold()
    ];
    let mut language=String::from("en");
    loop {
        let selected=main_menu(&options);
        match selected {
            0=>start(&mut language, ip),
            1=>reading_element(&language,ip),
            2=>reading_union(&language, ip),
            3=>reading_mix_elements(&language, ip),
            4=>reading_mix_unions(&language, ip),
            5=>reading_advice(&language, ip),
            6=>reading_advice_final(&language, ip),
            _=>{
                println!("{}","Leaving");
                break;
            }
        }
    }
}
fn start(lang:&mut String,_ip:&str){
    let mut lan=request_data("Choose a language(EN/ES): ");
    lan=lan.to_lowercase();
    lang.clear();
    lang.push_str(&lan);
    let url=format!("http://{}/llm-api/start{}",_ip,if lang=="en"{""}else{"_es"});
    curl(&url, &"POST".to_string(), ContentType::Plain,&None,&None);
}
fn reading_element(lang:&str,_ip:&str){
    let url=format!("http://{}/llm-api/reading_element{}",_ip,if lang=="en"{""}else{"_es"});
    println!("The lecture is in {}",lang);
    let card=request_data("Give me a card: ");
    let element=request_data("Give me an element: ");
    let json=format!(
        r#"{{"card":"{}",
             "element":"{}"}}"#,
        card,element
    );
    curl(&url, &"POST".to_string(), ContentType::Json, &Some(json), &None);
}
fn reading_union(lang:&str,_ip:&str){
    let url=format!("http://{}/llm-api/reading_union{}",_ip,if lang=="en"{""}else{"_es"});
    println!("The lecture is in {}",lang);
    let card=request_data("Give me a card: ");
    let uni=request_data("Give me an union: ");
    let json=format!(
        r#"{{"card":"{}",
             "union":"{}"}}"#,
        card,uni
    );
    curl(&url, &"POST".to_string(), ContentType::Json, &Some(json), &None);
}
fn reading_mix_elements(lang:&str,_ip:&str){
    let url=format!("http://{}/llm-api/reading_mix_elements{}",_ip,if lang=="en"{""}else{"_es"});
    println!("The lecture is in {}",lang);
    let reading_1=request_data("Give me the first lecture");
    let reading_2=request_data("Give me the second lecture");
    let json=format!(
        r#"{{"reading_1":"{}",
             "reading_2":"{}"}}"#,
        reading_1,reading_2
    );
    curl(&url, &"POST".to_string(), ContentType::Json, &Some(json), &None);
}
fn reading_mix_unions(lang:&str,_ip:&str){
    let url=format!("http://{}/llm-api/reading_mix_union{}",_ip,if lang=="en"{""}else{"_es"});
    println!("The lecture is in {}",lang);
    let reading_mix_elements=request_data("Give me the lecture with elements mix");
    let reading_union=request_data("Give me the union lecture");
    let json=format!(
        r#"{{"elements_mix_text":"{}",
             "reading_union":"{}"}}"#,
        reading_mix_elements,reading_union
    );
    curl(&url, &"POST".to_string(), ContentType::Json, &Some(json), &None);
}

fn reading_advice(lang:&str,_ip:&str){
    let url=format!("http://{}/llm-api/reading_advice{}",_ip,if lang=="en"{""}else{"_es"});
    println!("The lecture is in {}",lang);
    let card_1=request_data("Give me the first card");
    let card_2=request_data("Give me the second card");
    let json=format!(
        r#"{{"card_1":"{}",
             "card_2":"{}"}}"#,
        card_1,card_2
    );
    curl(&url, &"POST".to_string(), ContentType::Json, &Some(json), &None);
}
//
fn reading_advice_final(lang:&str,_ip:&str){
    let url=format!("http://{}/llm-api/reading_advice_final{}",_ip,if lang=="en"{""}else{"_es"});
    println!("The lecture is in {}",lang);
    let card_1=request_data("Give me the first lecture");
    let card_2=request_data("Give me the second lecture");
    let card_3=request_data("Give me the third lecture");
    let card_4=request_data("Give me the fourth lecture");
    let advice=request_data("Give me an advice");
    let json=format!(
        r#"{{"cardReading_1":"{}",
             "cardReading_2":"{}",
             "cardReading_3":"{}",
             "cardReading_4":"{}",
             "inputAdvice":"{}"}}"#,
        card_1,card_2,card_3,card_4,advice
    );
    curl(&url, &"POST".to_string(), ContentType::Json, &Some(json), &None);
}