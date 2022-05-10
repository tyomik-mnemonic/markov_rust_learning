use std::{io};

fn main() {
    //read_input - функция считыающая stdin
    let outputed:String = read_input();
    println!("The text is: \n {} \n The len is: \n {}",
        outputed, outputed.chars().count()-1 //why count is +1?
);
    let splited:Vec<&str> = spliter(outputed.as_str());
    println!("Array is: {:?}",splited);
}

fn read_input() -> String {
    let hi_message = "Input your corpus for text generation"; //& - ссылочный тип PS жестко закодирован, храниться в стеке
    println!("{}", hi_message);

    let mut corpus = String::new(); //String храниться в куче
    io::stdin() //нельзя использовать примитивы тк размер неизвестен до компиляции (ввод разный)
        .read_line(&mut corpus)
        .expect("Your corpus is wrong. Use text not float or int.");
    
    return corpus
}

struct CharElement {
    text:String,
    count:u16
}

fn spliter(text:&str)->Vec<&str>{
    let mut result:Vec<&str> = text.split("").collect();
    return result
}

//fn spliter(text:String){
    //переменная не объявлена
//    let s1:&'static str = "asd";//переменная объявлена PS примитивы храняться в стеке
    //область видимости переменной spliter_msg

//    let x = 5;
//    let y = x; //создается копия x и устанавливается владелец копии - y

//    let s2:String = String::from("s2 variable");
//    let new_s2 = s2; //создается копия значения (указатель,длина,емкость), но не копия данных в куче!
//                            //поверхностное копирование, но не совсем, это move: s2-> new_s2 и s2 более недоступна
//                            //таким образом "ошибка двойного освобождения" невозможна в Rust
//                            //new_s2 = s2.clone(); - глубокое копирование ,создает копию данных в куче. только для String и тд
//
//}//вызов drop() тут область видимости переменной spliter_msg окончена, владелец покинул скоуп, значение удаляется из памяти

