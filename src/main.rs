use std::{io};

fn main() {
    let otputed:String = read_input();
    println!("The text is: \n {} \n The len is: \n {}",
    otputed, otputed.chars().count()
    );
}

fn read_input() -> String {
    println!("Input your corpus for text generation");

    let mut corpus = String::new();
    io::stdin()
        .read_line(&mut corpus)
        .expect("Your corpus is wrong. Use text not float or int.");
    
    return corpus
}

struct CharElement {
    text:String,
    count:u16
}

fn spliter(text:String){

}
