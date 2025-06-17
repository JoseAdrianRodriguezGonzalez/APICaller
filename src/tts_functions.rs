use crate::terminal::{curl,ContentType,request_data,create_folder};
pub fn initialize(ip:&String){
     let url=format!("http://{}/",ip);
     let method="GET".to_string();
     curl(&url,&method ,ContentType::Plain , &None, &None);
}
pub fn synthesize(ip:&String){
     let url=format!("http://{}/synthesize",ip);
    let method="POST".to_string();
    let text=request_data("Enter the text that you want to listen");
    let json = format!(r#"{{"text":"{}"}}"#, text);
    create_folder(&"audios".to_string());
    let name_audio=request_data(&"What is the name of your audio?".to_string());
    curl(&url,&method ,ContentType::Json , &Some(json), &Some(name_audio));
}