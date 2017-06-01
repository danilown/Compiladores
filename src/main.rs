use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

//Checar expressao
//Identificador
//Erro: Abre comentario e nao fecha >> fim de arquivo inesperado

//funçao assign, decide o papel do token checas os booleans necessarios

// Codigos para tokens nao terminais
const constant: u32 = 0;
const simple_type: u32 = 1;
const field_list: u32 = 2;
const index: u32 = 3;
const simple_expression: u32 = 4;
const expression: u32 = 5;
const parameter_list: u32 = 6;
const statement: u32 = 7;
const program: u32 = 8;

// Codigos para tokens terminais
const NUMB: u32 = 9; 	// number
const STRING: u32 = 10;	// cadeia de caracteres
const IDEN: u32 = 11;	// identifier
const COIDEN: u32 = 12;	// constant identifier
const FIIDEN: u32 = 13;	// filed identifier
const VAIDEN: u32 = 14; // variable identifier
const FUIDEN: u32 = 15;	// function identifier
const TYIDEN: u32 = 16;	// type identifier
const PRIDEN: u32 = 17;	// procedure identifier

fn main() {

    let debug = true;
    let mut reserved = Vec::new();
    let intervals;
    let mut characters:Vec<char> = Vec::new();

    characters = arq_comp(debug);
    reserved = arq_res(debug);
    intervals = set_inter(&reserved,debug);
    compare(characters,reserved,intervals,debug);

    let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
}

// Trata o arquivo a ser compilado
fn arq_comp (debug:bool) -> Vec<char> {

    let mut file = File::open("src/hello.pas").expect("Unable to open the file");
    let mut contents = Vec::new();
    let mut characters = Vec::new();
    file.read_to_end(&mut contents).expect("Unable to read the file");

    for i in 0..contents.len() {

        characters.push(char::from(contents[i]));
    }

    if debug {

        println!("{:?}", characters);
    }

    characters
}

// Trata o arquivo de palavras reservadas
fn arq_res (debug:bool) -> Vec<String> {

    let mut file = File::open("src/tabela.txt").expect("Unable to open the file");
    let mut contents = String::new();
    let mut reserved_ref = Vec::new();
    let mut reserved_str = Vec::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    reserved_ref = contents.split('\n').collect();
    reserved_ref.pop();

    for i in 0..reserved_ref.len() {

        reserved_str.push(reserved_ref[i].to_string());
    }

    if debug {

        println!("{:?}", reserved_str);
    }

    reserved_str
}

/*Pega o valor numérico de um caracter na primeira posição de uma string, que está em um certo indice de um
vector de strings e retorna esse valor que pode ir de 0 a 36 , 0-9 retorna 0-9, a-z retorna 10-35*/
fn get_charval_fp (vec:&Vec<String>,indice: usize) -> u32 {

    vec[indice].chars().next().unwrap().to_digit(36).unwrap()
}

/*Pega o valor numérico de um caracter na primeira posição de uma string, que está em um certo indice de um
vector de strings e retorna esse valor que pode ir de 0 a 36 , 0-9 retorna 0-9, a-z retorna 10-35*/
fn pegar_char (string:&String) -> u32 {

    string.chars().next().unwrap().to_digit(36).unwrap()
}

// Cria a estrutura auxiliar que determina o intervalo inicial de palavra reservada
fn set_inter (reserved:&Vec<String>,debug:bool) -> [[u32;26];2] {

    let mut intervals = [[0;26];2];
    let mut aux = 200;

    for i in 0..reserved.len() {

        if aux != get_charval_fp(&reserved,i)-10 {

            intervals[0][(get_charval_fp(&reserved,i)-10) as usize] = i as u32;
            aux = get_charval_fp(&reserved,i)-10;
        }
        intervals[1][(get_charval_fp(&reserved,i)-10) as usize] = i as u32;
    }

    if debug {

        println!("{:?}", intervals[0]);
        println!("{:?}", intervals[1]);
    }

    intervals
}

fn compare(characters:Vec<char>,reserved:Vec<String>,intervals:[[u32;26];2],debug:bool) {

    let mut auxiliar = String::new();
    let mut string_completa=false;
    let mut controle:[bool;6] = [false;6];

    enum checker {

        IS_STRING=0,
        IS_COMM1=1,
        IS_COMM2=2,
        IS_COMM3=3,
        ALMOST_COMM2=4,
        ALMOST_COMM3=5,
    };

    //Checa os caracteres do arquivo de entrada
    for i in 0..characters.len() {

        println!("{}", characters[i]);

        match test_case(characters[i],controle) {

            -1 => {},

            0 => string_completa=true,

            1 => auxiliar.push(characters[i]),

            2 => controle[checker::IS_STRING as usize] = !controle[checker::IS_STRING as usize],

            3 => controle[checker::IS_COMM1 as usize] = !controle[checker::IS_COMM1 as usize],

            4 => controle[checker::IS_COMM2 as usize] = !controle[checker::IS_COMM2 as usize],

            5 => controle[checker::IS_COMM3 as usize] = !controle[checker::IS_COMM3 as usize],

            6 => {

                controle[checker::ALMOST_COMM2 as usize] = !controle[checker::ALMOST_COMM2 as usize];
                continue;
            },

            7 => {

                controle[checker::ALMOST_COMM3 as usize] = !controle[checker::ALMOST_COMM3 as usize];
                continue;
            },

            _ => println!("Algo de errado não está certo"),
        }

        if (controle[checker::ALMOST_COMM2 as usize]) { controle[checker::ALMOST_COMM2 as usize] =false; }

        else if (controle[checker::ALMOST_COMM3 as usize]) { controle[checker::ALMOST_COMM3 as usize] =false; }

        if string_completa && auxiliar.len()>0 {

            if debug {

                println!("{:?}",auxiliar);
            }

            reserv(&auxiliar,&intervals,&reserved);
            auxiliar=String::new();
            string_completa=false;
        }

        else if auxiliar.len()==0{

            string_completa=false;
        }
    }
}

fn test_case (character:char,controle:[bool;6]) -> i8 {

    enum helper {

        STRING_PRONTA = 0,
        CONCATENA_STRING = 1,
        EH_STRING = 2,
        COMM1 = 3,
        COMM2 = 4,
        COMM3 = 5,
        ALMOST_COMM2=6,
        ALMOST_COMM3=7,
    };

    enum checker {

        IS_STRING=0,
        IS_COMM1=1,
        IS_COMM2=2,
        IS_COMM3=3,
        ALMOST_COMM2=4,
        ALMOST_COMM3=5,
    };

    match character {

        '\'' => {

            return helper::EH_STRING as i8;
        },

        ';' | ':' | ' ' | '.' | ',' => {

            if controle[checker::IS_STRING as usize] { return helper::CONCATENA_STRING as i8; }

            else if controle[checker::IS_COMM1 as usize] { return helper::CONCATENA_STRING as i8; }

            else if controle[checker::IS_COMM2 as usize] { return helper::CONCATENA_STRING as i8; }

            else if controle[checker::IS_COMM3 as usize] { return helper::CONCATENA_STRING as i8; }

            else { return helper::STRING_PRONTA as i8; }
        },

        '/' => {

            if controle[checker::ALMOST_COMM2 as usize] { return helper::COMM2 as i8; }

            else { return helper::ALMOST_COMM2 as i8; }
        }

        '(' | ')' => return helper::ALMOST_COMM3 as i8,

        '*' => {

            if controle[checker::ALMOST_COMM3 as usize] { return helper::COMM3 as i8; }

            else { return -1; }
        },

        '{' | '}' => return helper::COMM1 as i8,

        '\n' => return helper::STRING_PRONTA as i8,

        _ => return helper::CONCATENA_STRING as i8,
    }
}

fn reserv(auxiliar:&String,intervals:&[[u32;26];2],reserved:&Vec<String>) {

    let mut inter_init=0;
    let mut inter_final=0;
    let mut indice;

    indice = pegar_char(&auxiliar) -10;
    inter_init=intervals[0 as usize][indice as usize];
    inter_final=intervals[1 as usize][indice as usize];

    //Se o intervalo começar e terminar com 0, não há palavra reservada que começe com essa letra, logo não é reservada
    if inter_init==inter_final && inter_init==0 {

        return;
    }

    //Checa dentro do intervalo de possibilidade
    for i in inter_init .. inter_final+1 {

        if reserved[i as usize] == auxiliar.to_lowercase().to_string() {
            println!("sucesso");
            return;
        }
    }

}
