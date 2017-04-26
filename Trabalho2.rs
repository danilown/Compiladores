use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use std::convert::From;
use std::char::*;

fn main() {
    println!("Hello World");
    let mut file = File::open("hello.pas").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    //let mut contents = Vec::new();
    //file.read_to_end(&mut contents).expect("Unable to read the file");
    println!("{}", &contents);
    /*for i in &contents {
        println!("Vect: {}",char::from(i:i8));
    }
    */

    /* Modo 1 - Separando por espaço
    let mut iter = contents.split_whitespace();
    let mut indiv_word = iter.next();
    loop {
        match indiv_word {
                Some(x) => println!("{}",x),
                None => break,
            }
            indiv_word = iter.next();
    }
    */

    /* Modo 2 - Diversos separadores
    let v: Vec<&str> = contents.split(|separator| separator == ' ' || separator == '(' || separator == ')' ,
    || separator == ';' || separator == ',' || separator == ':' || separator == '\n').collect();

    for i in &v {
        println!("Vect: {}",i);
    }
    */


    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in contents.match_indices(|c: char| !(c.is_alphanumeric())){
        if last != index {
            result.push(&contents[last..index]);
        }

        if(matched == "\n" || matched == "\r" || matched == " "){
        } else {
            result.push(matched);
        }
        last = index + matched.len();
    }
    if last < contents.len() {
        result.push(&contents[last..]);
    }
    println!("{:?}", result);

    /*for i in &result {

    }*/
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}
