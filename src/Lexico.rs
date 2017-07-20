
use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::convert::From;
use std::char::*;
use std::sync::Mutex;

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
//nunca usado const simbolo_lambda:           u32 = 41;
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
// ----------------------------------------
// Adicionar no sintático:
const simbolo_abre_parenteses:  u32 = 74;
const simbolo_true:             u32 = 78; 
const simbolo_false:            u32 = 79;
// nunca usado const simbolo_identifier:       u32 = 82;
const simbolo_div:              u32 = 83;
const simbolo_record:           u32 = 85;

const NUMB:     u32 = 64; // number
const STRING:   u32 = 65; // cadeia de caracteres
const IDEN:     u32 = 66; // identifier
const COIDEN:   u32 = 67; // constant identifier
const FIIDEN:   u32 = 68; // field identifier
const VAIDEN:   u32 = 69; // variable identifier
const FUIDEN:   u32 = 70; // function identifier
const TYIDEN:   u32 = 71; // type identifier
const PRIDEN:   u32 = 72; // procedure identifier
const FLOAT:    u32 = 84; // FLoat Numbers


static mut escopo: u32 =  0;
static mut lines:  i32 =  0;
static mut next:   u32 =  0;

lazy_static! {
    pub static ref tabelaSimbolos: Mutex<Vec<Token>> = Mutex::new(Vec::new());
}

#[derive(Debug)]
pub struct Token {
    pub tok:    String,
    pub tipe:   u32,
    pub line:   i32,
    pub row:    i32,
    pub escope: u32,
    pub used:   bool,
}

fn SimbolTable<'a>() -> Vec<String> {
    
  let mut file = File::open("src/teste.pas").expect("Unable to open the file");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Unable to read the file");

  let mut result = Vec::<String>::new();
  let mut last   = 0;

  // Armazena cada token encontado no vetor Result
  for (idx, matched) in contents.match_indices(|c: char| !(c.is_alphanumeric())){
        if last != idx {
            result.push((&contents[last..idx]).to_string());
        }

        if(matched == "\r" || matched == "\t" ) { // || matched == "\n" || matched == " "
        }else {
            result.push(matched.to_string());
        }
        last = idx + matched.len();
    }

    if last < contents.len() {
        result.push((&contents[last..]).to_string());
    }
    return result;
}

fn Organize(result: &mut Vec<String>) -> Vec<String>{
    
    println!("organizing..");
    let mut comentario1 = false;
    let mut comentario2 = false;
    let mut comentario3 = false;
    let mut auxiliar = Vec::<String>::new();
    let mut i = 0;

    while (i < result.len()) {
        // Verifica comentários

        // se é espaço, pula para o próximo
        if result[i as usize] == " "{
            i += 1;
            continue
        }

        // se comentário1 estava aberto e encontra } -> fecha comentário 
        if comentario1 == true{
            if result[i as usize] == "}"{
                comentario1 = false;
                i += 1;
                continue
            }else{
                i += 1;
                continue
            }
        }

        // se comentario2 estva aberto e achou \n -> fecha comentário
        unsafe{
            if comentario2 == true{
                if result[i as usize] == "\n"{
                    comentario2 = false;
                    lines += 1;
                    i += 1;
                    continue
                }else {
                    i += 1;
                    continue
                }
            }

            if result[i as usize] == "\n"{
                lines += 1;
                i += 1;
                continue
            }
        }

        // se comentario3 está aberto e encontrou *) -> fecha comentario
        if comentario3 == true{
            if result[i as usize] == "*"{
                if result[(i+1) as usize] == ")"{
                    comentario3 = false;
                    i += 2;
                    continue
                }else{
                    i += 1;
                    continue
                }
            }else{
                i += 1;
                continue
            }
        }
 
        if result[i as usize].to_lowercase() == "{" {
            comentario1 = true;
            i += 1;
            continue
        }else if result[i as usize].to_lowercase() == "/" && result[(i+1) as usize].to_lowercase() == "/" {
            comentario2 = true;
            i += 2;
            continue
        }else if result[i as usize].to_lowercase() == "(" && result[(i+1) as usize].to_lowercase() == "*" {
            comentario3 = true;
            i += 2;
            continue
        }

        
        if result[i as usize].to_lowercase() == "." {
                    
            if result[(i+1) as usize].to_lowercase() == "." {

                let mut tk = String::new();                          // variavel para concatenar os tokents
                tk = result[i as usize].to_string();                 // pega o token atual "."
                tk.push_str(&result[(i+1) as usize].to_string());    // concatena o próximo token ""
                
                auxiliar.push(tk.to_string());
                i += 2;
                continue
            }
        }


        if (result[i as usize].to_lowercase() == "<"){

            if(result[(i+1) as usize].to_lowercase() == "="){

                let mut tk = String::new();                          // variavel para concatenar os tokents
                tk = result[i as usize].to_string();                 // pega o token atual "<"
                tk.push_str(&result[(i+1) as usize].to_string());    // concatena o próximo token "="

                auxiliar.push(tk.to_string());
                i += 2;
                continue

            }else if(result[(i+1) as usize].to_lowercase() == ">"){

                let mut tk = String::new();                          // variavel para concatenar os tokents
                tk = result[i as usize].to_string();                 // pega o token atual "<"
                tk.push_str(&result[(i+1) as usize].to_string());    // concatena o próximo token ">"

                auxiliar.push(tk.to_string());
                i += 2;
                continue
            }
        }   
            
        if (result[i as usize].to_lowercase() == ">"){

            if(result[(i+1) as usize].to_lowercase() == "="){

                let mut tk = String::new();                             // variavel para concatenar os tokents
                tk = result[i as usize].to_string();                 // pega o token atual ">"
                tk.push_str(&result[(i+1) as usize].to_string());    // concatena o próximo token "="

                auxiliar.push(tk.to_string());
                i += 2;
                continue    
            }
        }

        if (result[i as usize].to_lowercase() == ":"){
                
            if(result[(i+1) as usize].to_lowercase() == "="){
                    
                let mut tk = String::new();                 // variavel para concatenar os tokents
                tk = result[i as usize].to_string();              // pega o token atual ":"
                tk.push_str(&result[(i+1) as usize].to_string());   // concatena o próximo token "="

                auxiliar.push(tk.to_string());
                i += 2;
                continue
            }
        }
        

        if result[i as usize].to_lowercase().parse::<u64>().is_ok() { // checa se a string é numérica
            
            if(result[(i+1) as usize].to_lowercase() == "."){           // checa se depois dela vem um ponto
                                                                // Se é ponto, concatena as strings em tk
                let mut tk = String::new();                     // variavel para concatenar os tokens (tk)
                tk = result[i as usize].to_string();                  // pega o token atual
                tk.push_str(&result[(i+1) as usize].to_string());       // concatena com o próximo token

                // checa se depois do ponto vem outra string numerica
                if(result[(i+2) as usize].to_lowercase().parse::<u64>().is_ok ()){     
                                                                
                    tk.push_str(&result[(i+2) as usize].to_string());   // concatena com o próximo token

                    auxiliar.push(tk.to_string());
                    i += 3;
                    continue
                }
            }
        }

        auxiliar.push(result[i as usize].to_string());
        i += 1;
    }
    
    return auxiliar;

}

fn Lexic(result: &mut Vec<String>, GoOn: bool) -> Token {


    let reservadas = vec!["div", "or", "and", "not", "if", "then", "else", "of",
        "while", "do", "begin", "end", "read", "write", "var", "array", "func",
        "proc", "program", "true", "false", "char", "integer", "boolean", "clrscr", 
        "packed", "in", "dif", "file", "set", "case", "nil", "label", "const",
        "type", "repeat", "for", "with", "goto", "until", "to", "downto", "writeln"];

    let simbolos = vec!["+", "-", "*", "=", "<", ">", "(", ")", // <> >= <= .. :=
         "[", "]", ".", ",", ";", ":", "@", "\"", "/", "%", "..", "<>", ">=", "<=", ":="];    

    let numeros = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "."];

    let mut classificada = 0;
    
unsafe{    
    let mut next_token = Token{tok: ("").to_string(), tipe: 0, line: lines, row: 0, escope: 0, used: false};
    // Classificação do token

    println!("result in Lexic = {:?}", result[next as usize] );
    // Verifica se é palavra reservada e classifica 
    for i in 0..reservadas.len() {

        if result[next as usize].to_lowercase() == reservadas[i] {

            if result[next as usize].to_lowercase() == "begin"{
                classificada = 1;
                next_token = Token{tok: (&result[next as usize]).to_string(), tipe: simbolo_begin, line: lines, row: 0, escope: escopo, used: false};
                escopo += 1;       // Se for begin, aumenta o escopo para os próximos tokens
            }else if result[next as usize ].to_lowercase() == "end"{
                classificada = 1;
                escopo -= 1;       // se for end, retorna o escopo
                next_token = Token{tok: (&result[next as usize]).to_string(), tipe: simbolo_end, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize ].to_lowercase() == "or"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_or, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "in"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_in, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "and"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_and, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "div"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_div, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "array"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_array, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "packed"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_packed, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "dif"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_dif, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "of"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_of, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "file"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_file, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "set"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_set, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "case"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_case, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "nil"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_nil, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "not"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_not, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "proc"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_proc, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "func"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_func, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "var"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_var, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "label"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_label, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "const"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_const, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "type"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_type, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "if"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_if, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "while"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_while, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "repeat"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_repeat, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "for"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_for, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "with"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_with, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "goto"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_goto, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "then"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_then, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "else"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_else, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "do"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_do, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "until"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_until, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "to"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_to, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "downto"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_down_to, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "clrscr"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: FUIDEN, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "integer"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: TYIDEN, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "read"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: PRIDEN, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "write"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: PRIDEN, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "program"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: program, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "true"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_true, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "false"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_false, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "char"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: TYIDEN, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "boolean"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: TYIDEN, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "writeln"{
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: PRIDEN, line: lines, row: 0, escope: escopo, used: false};
            }

        if(GoOn){
            next += 1;
        }
        return next_token;
        }
    }

    // verifica se é simbolo e classifica
    for i in 0..simbolos.len() {
        if result[next as usize].to_lowercase() == simbolos[i] {
            
            if result[next as usize].to_lowercase() == ","{
                classificada = 1;
                 next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_virgula, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == ")"{
                classificada = 1;
                 next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_fecha_parenteses, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "+"{
                classificada = 1;
                 next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_soma, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "-"{
                classificada = 1;
                 next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_subtracao, line: lines, row: 0, escope: escopo, used: false};
            }else if (result[next as usize].to_lowercase() == "."){
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_ponto, line: lines, row: 0, escope: escopo, used: false};
            }else if(result[next as usize].to_lowercase() == ".."){
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_ponto_ponto, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "="{
                classificada = 1;
                 next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_igual, line: lines, row: 0, escope: escopo, used: false};
            }else if (result[next as usize].to_lowercase() == "<"){
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_menor, line: lines, row: 0, escope: escopo, used: false};      
            }else if(result[next as usize].to_lowercase() == "<="){
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_menor_igual, line: lines, row: 0, escope: escopo, used: false};
            }else if(result[next as usize].to_lowercase() == "<>"){
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_diferent, line: lines, row: 0, escope: escopo, used: false};
            }else if (result[next as usize].to_lowercase() == ">"){
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_maior, line: lines, row: 0, escope: escopo, used: false};
            }else if(result[next as usize].to_lowercase() == ">="){
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_maior_igual, line: lines, row: 0, escope: escopo, used: false};    
            }else if result[next as usize].to_lowercase() == "["{
                classificada = 1;
                 next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_abre_colchetes, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "@"{
                classificada = 1;
                 next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_arroba, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "\""{
                classificada = 1;
                 next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_aspas, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "]"{
                classificada = 1;
                 next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_fecha_colchetes, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "*"{
                classificada = 1;
                 next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_asterisco, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "/"{
                classificada = 1;
                 next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_barra, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "%"{
                classificada = 1;
                 next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_mod, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == ";"{
                classificada = 1;
                 next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_ponto_virgula, line: lines, row: 0, escope: escopo, used: false};
            }else if (result[next as usize].to_lowercase() == ":"){
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_dois_pontos, line: lines, row: 0, escope: escopo, used: false};
            }else if(result[next as usize].to_lowercase() == ":="){
                classificada = 1;
                next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_dois_pontos_igual, line: lines, row: 0, escope: escopo, used: false};
            }else if result[next as usize].to_lowercase() == "("{
                classificada = 1;
                 next_token = Token{tok: (result[next as usize]).to_string(), tipe: simbolo_abre_parenteses, line: lines, row: 0, escope: escopo, used: false};
            }

        if (GoOn) {
            next += 1;
        }
        return next_token;
        }
    }

    if result[next as usize].to_lowercase().parse::<i64>().is_ok() { // checa se a string é numérica
        
        classificada = 1;
        next_token = Token{tok: (result[next as usize]).to_string(), tipe: NUMB, line: lines, row: 0, escope: escopo, used: false};

        if (GoOn) {
            next += 1;
        }
        return next_token;
    }

    // verifica se é numero e classifica
    if result[next as usize].to_lowercase().parse::<f64>().is_ok() { // checa se a string é numérica
        
        classificada = 1;
        next_token = Token{tok: (result[next as usize]).to_string(), tipe: FLOAT, line: lines, row: 0, escope: escopo, used: false};

        if (GoOn) {
            next += 1;
        }
        return next_token;
    }
    

    // se não for nenhum dos anteriores, classifica como identificador
    if (classificada == 0){
        next_token =  Token{tok: (result[next as usize]).to_string(), tipe: IDEN, line: lines, row: 0, escope: escopo, used: false};
    }
    
    if(GoOn) {
        next += 1;
    }
    return next_token;
}
}



pub fn setupLexico() {
    
   // let mut atual = Token{tok: ("").to_string(), tipe: 0 , line: 0, row: 0, escope: 0, used: false};
    let mut result = SimbolTable();

    println!("{:?}",result);
    println!("\n\n");
    result = Organize(&mut result);
    println!("\n\n", );
    println!("{:?}",result);
    
    for i in 0..result.len() {

        let tokenAtual = Lexic(&mut result,true);

        tabelaSimbolos.lock().unwrap().push(tokenAtual);

    }
}