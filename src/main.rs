#[macro_use]
extern crate lazy_static;
use std::process::Command;

pub mod Lexico;
mod Sintatico;
// mod Semantico;


fn main() {
    
    Sintatico::asd();

    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}