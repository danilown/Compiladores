use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use std::convert::From;
use std::char::*;


// Codigos para tokens nao terminais
const constant:                 u32 = 0;
const simple_type:              u32 = 1;
const field_list:               u32 = 2;
const index:                    u32 = 3;
const simple_expression:        u32 = 4;
const expression:               u32 = 5;
const parameter_list:           u32 = 6;
const statement:                u32 = 7;

const program:                  u32 = 8;
const simbolo_virgula:          u32 = 9;
const simbolo_fecha_parenteses: u32 = 10;
const simbolo_ponto_ponto:      u32 = 11;
const simbolo_soma:             u32 = 12;
const simbolo_subtracao:        u32 = 13;
const simbolo_or:               u32 = 14;
const simbolo_igual:            u32 = 15;
const simbolo_menor:            u32 = 16;
const simbolo_maior:            u32 = 17;
const simbolo_diferent:         u32 = 18;
const simbolo_maior_igual:      u32 = 19;
const simbolo_menor_igual:      u32 = 20;
const simbolo_in:               u32 = 21;
const simbolo_abre_colchetes:   u32 = 22;
const simbolo_ponto:            u32 = 23;
const simbolo_arroba:           u32 = 24;
const simbolo_aspas:            u32 = 25;
const simbolo_fecha_colchetes:  u32 = 26;
const simbolo_asterisco:        u32 = 27;
const simbolo_barra:            u32 = 28;
const simbolo_dif:              u32 = 29;
const simbolo_mod:              u32 = 30;
const simbolo_and:              u32 = 31;
const simbolo_packed:           u32 = 32;
const simbolo_array:            u32 = 33;
const simbolo_of:               u32 = 34;
const simbolo_file:             u32 = 35;
const simbolo_set:              u32 = 36;
const simbolo_end:              u32 = 37;
const simbolo_ponto_virgula:    u32 = 38;
const simbolo_dois_pontos:      u32 = 39;
const simbolo_case:             u32 = 40;
//const simbolo_lambda:           u32 = 41;
const simbolo_nil:              u32 = 42;
const simbolo_not:              u32 = 43;
const simbolo_proc:             u32 = 44;
const simbolo_func:             u32 = 45;
const simbolo_var:              u32 = 46;
const simbolo_label:            u32 = 47;
const simbolo_const:            u32 = 48;
const simbolo_type:             u32 = 49;
const simbolo_begin:            u32 = 50;
const simbolo_if:               u32 = 51;
const simbolo_while:            u32 = 52;
const simbolo_repeat:           u32 = 53;
const simbolo_for:              u32 = 54;
const simbolo_with:             u32 = 55;
const simbolo_goto:             u32 = 56;
const simbolo_dois_pontos_igual:u32 = 57;
const simbolo_then:             u32 = 58;
const simbolo_else:             u32 = 59;
const simbolo_do:               u32 = 60;
const simbolo_until:            u32 = 61;
const simbolo_to:               u32 = 62;
const simbolo_down_to:          u32 = 63;
// Adicionar no sintático:
const simbolo_clrscr:           u32 = 64;
const simbolo_abre_parenteses   u32 = 65;
const simbolo_integer           u32 = 66;
const simbolo_read              u32 = 67;
const simbolo_write             u32 = 68;
const simbolo_true              u32 = 69;
const simbolo_false             u32 = 70;
const simbolo_char              u32 = 71;
const simbolo_boolean           u32 = 72;

static mut escopo;
static enum {reservada, simbolo, constante, identificador}
struct Token {
    tok: &'static str,
    tipe:   i32,
    line:   i32,
    row:    i32,
    escope: i32,
    used:   bool,
}

fn SimbolTable -> Vec<&str> {
    
  let mut file = File::open("hello.pas").expect("Unable to open the file");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Unable to read the file");

  let mut result = Vec::new();
  let mut last   = 0;

  // Armazena cada token encontado no vetor Result
  for (index, matched) in contents.match_indices(|c: char| !(c.is_alphanumeric())){
        if last != index {
            result.push(&contents[last..index]);
        }

        if(matched == "\r" || matched == "\n" || matched == " "){
        } else {
            result.push(matched);
        }
        last = index + matched.len();
    }

    if last < contents.len() {
        result.push(&contents[last..]);
    }
    return(result);
}

fn Lexic(next: usize) {

    
    let reservadas = vec!["div", "or", "and", "not", "if", "then", "else", "of",
        "while", "do", "begin", "end", "read", "write", "var", "array", "func",
        "proc", "program", "true", "false", "char", "integer", "boolean", "clrscr", 
        "packed", "in", "dif", "file", "set", "case", "nil", "label", "const",
        "type", "repeat", "for", "with", "goto", "until", "to", "downto"]; // 42 palavras reservadas

    let simbolos = vec!["+", "-", "*", "=", "<", ">", "(", ")", // <> >= <= .. :=
         "[", "]", ".", ",", ";", ":", "@", "\"", "/", "%"];    // 18 simbolos

    let numeros = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let mut classificada = 0;
    let mut next_token = Token{tok: "", tipe: 0, line: 0, row: 0, escope: 0, used: false;

    // Classificação do token
    
    // Verifica se é palavra reservada e classifica 
    for i in 0..reservadas.len() {

        if result[next].to_lowercase() == reservadas[i] {

            if result[next].to_lowercase() == "begin"{
                classificada = 1;
                let mut next_token = Token{tok: result[next], tipo: simbolo_begin, line: 0, row: 0, escope: escopo, used: false};
                escopo++;       // Se for begin, aumenta o escopo para os próximos tokens
                return next_token;
            }
            if result[next].to_lowercase() == "end"{
                classificada = 1;
                escopo--;       // se for end, retorna o escopo
                let mut next_token = Token{tok: result[next], tipo: simbolo_end, line: 0, row: 0, escope: escopo, used: false};
                return next_token;
            }

            if result[next].to_lowercase() == "or"{
                classificada = 1;
                let mut next_token = Token{tok: result[next], tipo: simbolo_or, line: 0, row: 0, escope: escopo, used: false};
                return next_token;
            }

            if result[next].to_lowercase() == "in"{
                classificada = 1;
                let mut next_token = Token{tok: result[next], tipo: simbolo_in, line: 0, row: 0, escope: escopo, used: false};
                return next_token;
            }
            if result[next].to_lowercase() == "and"{
                classificada = 1;
                let mut next_token = Token{tok: result[next], tipo: simbolo_and, line: 0, row: 0, escope: escopo, used: false};
                return next_token;
            }
            if result[next].to_lowercase() == "div"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_div, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "array"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_array, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "packed"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_packed, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "dif"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_dif, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "of"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_of, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "file"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_file, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "set"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_set, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "case"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_case, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "nil"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_nil, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "not"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_not, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "proc"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_proc, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "func"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_func, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "var"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_var, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "label"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_label, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "const"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_const, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "type"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_type, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "if"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_if, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "while"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_while, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "repeat"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_repeat, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "for"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_for, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "with"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_with, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "goto"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_goto, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "then"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_then, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "else"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_else, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "do"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_do, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "until"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_until, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "to"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_to, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "downto"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_down_to, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "clrscr"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_clrscr, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "integer"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_integer, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "read"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_read, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "write"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_write, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }if result[next].to_lowercase() == "program"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: program, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "true"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_true, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "false"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_false, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "char"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_char, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
            if result[next].to_lowercase() == "boolean"{
                    classificada = 1;
                    let mut next_token = Token{tok: result[next], tipo: simbolo_boolean, line: 0, row: 0, escope: escopo, used: false};
                    return next_token;
            }
        }
    }
    
    // verifica se é simbolo e classifica
    for i in 0..simbolos.len() {
        if result[next].to_lowercase() == simbolos[i] {

            if result[next].to_lowercase() == ","{
                classificada = 1;
                let mut next_token = Token{tok: result[next], tipo: simbolo_virgula, line: 0, row: 0, escope: escopo, used: false};
                return next_token;
            }
            if result[next].to_lowercase() == ")"{
                classificada = 1;
                let mut next_token = Token{tok: result[next], tipo: simbolo_fecha_parenteses, line: 0, row: 0, escope: escopo, used: false};
                return next_token;
            }
            if result[next].to_lowercase() == "+"{
                classificada = 1;
                let mut next_token = Token{tok: result[next], tipo: simbolo_soma, line: 0, row: 0, escope: escopo, used: false};
                return next_token;
            }
            if result[next].to_lowercase() == "-"{
                classificada = 1;
                let mut next_token = Token{tok: result[next], tipo: simbolo_subtracao, line: 0, row: 0, escope: escopo, used: false};
                return next_token;
            }
            if (result[next].to_lowercase() == "."){
                    
                if(result[next+1].to_lowercase() == "."){

                    let mut tk = String::new();                 // variavel para concatenar os tokents
                    tk = result[next].to_string();              // pega o token atual "."
                    tk.push_str(&result[next+1].to_string());   // concatena o próximo token ""

                    classificada = 1;
                    let mut next_token = Token{tok: &tk, tipo: simbolo_ponto_ponto, line: 0, row: 0, escope: escopo, used: false};
                    return(next_token);

                }

                classificada = 1;
                let mut next_token = Token{tok: result[next], tipo: simbolo_ponto, line: 0, row: 0, escope: escopo, used: false};
                return next_token;
            } 
            if result[next].to_lowercase() == "="{
                classificada = 1;
                let mut next_token = Token{tok: result[next], tipo: simbolo_igual, line: 0, row: 0, escope: escopo, used: false};
                return next_token;
            }
            if (result[next].to_lowercase() == "<"){

                if(result[next+1].to_lowercase() == "="){

                    let mut tk = String::new();                 // variavel para concatenar os tokents
                    tk = result[next].to_string();              // pega o token atual "<"
                    tk.push_str(&result[next+1].to_string());   // concatena o próximo token "="

                    classificada = 1;
                    let mut next_token = Token{tok: &tk, tipo: simbolo_menor_igual, line: 0, row: 0, escope: escopo, used: false};
                    return(next_token);
                }
                    
                if(result[next+1].to_lowercase() == ">"){

                    let mut tk = String::new();                 // variavel para concatenar os tokents
                    tk = result[next].to_string();              // pega o token atual "<"
                    tk.push_str(&result[next+1].to_string());   // concatena o próximo token ">"

                    classificada = 1;
                    let mut next_token = Token{tok: &tk, tipo: simbolo_diferent, line: 0, row: 0, escope: escopo, used: false};
                    return(next_token);
                }

                classificada = 1;
                let mut next_token = Token{tok: &tk, tipo: simbolo_menor, line: 0, row: 0, escope: escopo, used: false};
                return(next_token);
                    
            }
            if (result[next].to_lowercase() == ">"){
                if(result[next+1].to_lowercase() == "="){

                    let mut tk = String::new();                 // variavel para concatenar os tokents
                    tk = result[next].to_string();              // pega o token atual ">"
                    tk.push_str(&result[next+1].to_string());   // concatena o próximo token "="

                    let mut next_token = Token{tok: &tk, tipo: simbolo_maior_igual, line: 0, row: 0, escope: escopo, used: false};
                    return(next_token);
                }
                classificada = 1;
                let mut next_token = Token{tok: &tk, tipo: simbolo_maior, line: 0, row: 0, escope: escopo, used: false};
                return(next_token);
            }
            if result[next].to_lowercase() == "["{
                classificada = 1;
                let mut next_token = Token{tok: result[next], tipo: simbolo_abre_colchetes, line: 0, row: 0, escope: escopo, used: false};
                return next_token;
            }
            if result[next].to_lowercase() == "@"{
                classificada = 1;
                let mut next_token = Token{tok: result[next], tipo: simbolo_arroba, line: 0, row: 0, escope: escopo, used: false};
                return next_token;
            }
            if result[next].to_lowercase() == "\""{
                classificada = 1;
                let mut next_token = Token{tok: result[next], tipo: simbolo_aspas, line: 0, row: 0, escope: escopo, used: false};
                return next_token;
            }
            if result[next].to_lowercase() == "]"{
                classificada = 1;
                let mut next_token = Token{tok: result[next], tipo: simbolo_fecha_colchetes, line: 0, row: 0, escope: escopo, used: false};
                return next_token;
            }
            if result[next].to_lowercase() == "*"{
                classificada = 1;
                let mut next_token = Token{tok: result[next], tipo: simbolo_asterisco, line: 0, row: 0, escope: escopo, used: false};
                return next_token;
            }
            if result[next].to_lowercase() == "/"{
                classificada = 1;
                let mut next_token = Token{tok: result[next], tipo: simbolo_barra, line: 0, row: 0, escope: escopo, used: false};
                return next_token;
            }
            if result[next].to_lowercase() == "%"{
                classificada = 1;
                let mut next_token = Token{tok: result[next], tipo: simbolo_mod, line: 0, row: 0, escope: escopo, used: false};
                return next_token;
            }
            if result[next].to_lowercase() == ";"{
                classificada = 1;
                let mut next_token = Token{tok: result[next], tipo: simbolo_ponto_virgula, line: 0, row: 0, escope: escopo, used: false};
                return next_token;
            }
            if (result[next].to_lowercase() == ":"){
                if(result[next+1].to_lowercase() == "="){
                    
                    let mut tk = String::new();                 // variavel para concatenar os tokents
                    tk = result[next].to_string();              // pega o token atual ":"
                    tk.push_str(&result[next+1].to_string());   // concatena o próximo token "="

                    classificada = 1;
                    let mut next_token = Token{tok: &tk, tipo: simbolo_dois_pontos_igual, line: 0, row: 0, escope: escopo, used: false};
                    return(next_token);
                }
                classificada = 1;
                let mut next_token = Token{tok: result[next], tipo: simbolo_dois_pontos, line: 0, row: 0, escope: escopo, used: false};
                return next_token;
            }
            if result[next].to_lowercase() == "("{
                classificada = 1;
                let mut next_token = Token{tok: result[next], tipo: simbolo_abre_parenteses, line: 0, row: 0, escope: escopo, used: false};
                return next_token;
            }
        }
    }




    // verifica se numero. se sim classifica como tipo 2
    /* TALVEZ TENHA QUE VERIFICAR NUMEROS SEPARADOS POR . OU ,*/
    for i in 0..numeros.len() {
        if result[next].to_lowercase() == numeros[i] {
            classificada = 1;
            let mut next_token = Token{tok: result[next], tipo: 2, line: 0, row: 0, tok: result[next]};
            return next_token;
        }
    }

    // se não for nada, classifica como identificador, isto é, tipo 3
    if (classificada == 0 ){
        let mut next_token = Token{tipo: 3, line: 0, row: 0, tok: result[next]};
        return next_token;
    }

    return next_token;
}



fn main() {
    escopo = 0;
    let mut atual = Token{tipo: 0, line: 0, row: 0, tok: ""};
    let mut x = 0;

    let result = SimbolTable();
    atual = lexic(x);
    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}







