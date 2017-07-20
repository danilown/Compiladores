use std::process::Command;

mod Lexico;
mod Sintatico;
mod Semantico;

fn main() {
    
    Lexico::setupLexico();

    let val = Lexico::tabelaSimbolos.lock().unwrap().len();

    for i in 0..val {
 
        let ref temp = Lexico::tabelaSimbolos.lock().unwrap()[i];

        println!("{:?}",*temp);

    }

    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}