
#![allow(unused)]

use regex::Regex;

fn main() {

    //Explain of slice
    let s = String::from("HelloWorld!");

    let len = s.len();

    //Extract words from a string using split_whitespace() method

    let sentence = "Hello World, How are you Today!";

    let words:Vec<&str> = sentence.split_whitespace().collect();

    for word in words{
        println!("{}", word);
    }   


    //Extract words from a string using regex

    let sentence = "Hello World, How are you Today 2!";

    let regex = Regex::new(r"\b\w+\b").unwrap();
    let words: Vec<&str> = regex.find_iter(sentence).map(|m| m.as_str()).collect();

    for word in words {
        println!("{}", word);
    }

    println!("Length of array: {}", len);

    let slice = &s[0..5];
    println!("{}", slice);
    let slice = &s[..5];
    println!("{}", slice);

    let slice = &s[5..len];
    println!("{}", slice);
    let slice = &s[5..];
    println!("{}", slice);

    let slice = &s[0..len];
    println!("{}", slice);
    let slice = &s[0..];
    println!("{}", slice);

    let word = first_word(&s);


    println!("1º Word value is: {}", word);

    let word2 = second_word(&s);

    println!("2º Word value is: {}", word2);
    
    
    // let s1 = String::from("Hello World!");

    // let len = calculate_length(&s1);

    // println!("{s1}");

    // println!("The length of {} is {}. ", s1, len);

    // fn calculate_length (s: &String) -> usize{
    //     s.len()
        
}

fn first_word (s: &String) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]

}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let len = s.len();

    for (mut i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            i += 1;
            return &s[i..];
        }else{
            if i == len {
                println!("No 2º word")
            }
        }

    }
    &s[..]


}



