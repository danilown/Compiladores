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
const simbolo_virgula: u32 = 9;
const simbolo_fecha_parenteses: u32 = 10;
const simbolo_ponto_ponto: u32 = 11;
const simbolo_soma: u32 = 12;
const simbolo_subtracao: u32 = 13;
const simbolo_or: u32 = 14;
const simbolo_igual: u32 = 15;
const simbolo_menor: u32 = 16;
const simbolo_maior: u32 = 17;
const simbolo_diferent: u32 = 18;
const simbolo_maior_igual: u32 = 19;
const simbolo_menor_igual: u32 = 20;
const simbolo_in: u32 = 21;
const simbolo_abre_colchetes: u32 = 22;
const simbolo_ponto: u32 = 23;
const simbolo_arroba: u32 = 24;
const simbolo_aspas: u32 = 25;
const simbolo_fecha_colchetes: u32 = 26;
const simbolo_asterisco: u32 = 27;
const simbolo_barra: u32 = 28; 
const simbolo_dif: u32 = 29;
const simbolo_mod: u32 = 30;
const simbolo_and: u32 = 31;

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

fn asd(){

} 

fn sitype(){
	let mut simbolo;

	simbolo = recebe_token();
	match simbolo {
		TYIDEN => 
			return,

		simbolo_abre_parenteses => {
			loop {
				simbolo = recebe_token();
				if simbolo == IDEN {
					simbolo = recebe_token();
					match simbolo {
						simbolo_virgula => continue,
						simbolo_fecha_parenteses => break,
						_ => println!("ERRO, VIRGULA OU FECHA PARENTESES ESPERADO."),

					}
				}
				else {
					println!("ERRO, IDENFIER ESPERADO.");
				}
			}
		},

		constant => {
			consta();
			simbolo = recebe_token();
			if simbolo == simbolo_ponto_ponto {
				simbolo = recebe_token();
				if simbolo == constant {
					consta();
				}
				else {
					println!("ERRO, CONSTANT ESPERADO.");
				}
			}
			else {
				println!("ERRO, PONTO PONTO ESPERADO.");
			}
		},
		_ => 
			println!("ERRO");
	}
}

fn consta() {
	let mut simbolo;

	simbolo = recebe_token();
	if simbolo == STRING {
		return;
	}
	match simbolo {
		simbolo_soma => ,
		simbolo_subtracao => ,
		_ => ,
	}
	simbolo = recebe_token();
	match simbolo {
		COIDEN => return,
		NUMB => return,
		_ => println!("ERRO, COIDEN OU NUMB ESPERADO."),
	}
}

fn type(){

}

fn filist(){

}

fn infipo(){
	let mut simbolo;

	loop{
		simbolo = recebe_token();
		match simbolo {
			simbolo_abre_colchetes => {
				loop{
					simbolo = recebe_token();
					if simbolo == expr {
						expr();
						simbolo = recebe_token();
						match simbolo {
							simbolo_aspas => continue,
							simbolo_fecha_colchetes => break,
							_ => println!("ERRO, ASPAS OU FECHA COLCHETES ESPERADO.");
						}
					}
					else {
						println!("ERRO, EXPR ESPERADO.");
						return;
					}
				}
			},
			simbolo_ponto => {
				simbolo = recebe_token();
				if simbolo == FIIDEN {
					continue;
				}
				else {
					println!("ERRO, FIIDEN ESPERADO.");
				}
			},
			simbolo_arroba => continue,
			_ => break,
		}
	}
}

fn factor(){

}

fn term(){
	let mut simbolo;

	loop {
		simbolo = recebe_token();
		if simbolo == factor {
			factor();
			match simbolo {
				simbolo_asterisco => continue,
				simbolo_barra => continue,
				simbolo_div => continue,
				simbolo_mod => continue,
				simbolo_and => continue,
				_ => break,
			}
		}
		else {
			println!("ERRO, FACTOR ESPERADO.");
			break;
		}
	}			
}

fn siexpr(){
	let mut simbolo;

	simbolo = recebe_token();
	match simbolo {
		simbolo_soma => ,
		simbolo_subtracao => ,
		_ => ,
	}
	loop {
		simbolo = recebe_token();
		if simbolo == term {
			term();
			match simbolo {
				simbolo_soma => continue,
				simbolo_subtracao => continue,
				simbolo_or => continue,
				_ => break;
			}
		}
		else {
			println!("ERRO, TERM ESPERADO.");
		}
	}
}

fn expr(){
	let mut simbolo;

	simbolo = recebe_token();
	if simbolo == siexpr {
		siexpr();
		simbolo = recebe_token();
		match simbolo {
			simbolo_igual => ,
			simbolo_menor => ,
			simbolo_maior => ,
			simbolo_diferente => ,
			simbolo_maior_igual => ,
			simbolo_menor_igual => ,
			simbolo_in => ,
			_ => return,
		}
	}
	else{
		println!("ERRO, SIEXPR ESPERADO.");
		return;
	}
	simbolo = recebe_token();
	if simbolo = siexpr {
		siexpr();
	}
	else{
		println!("ERRO, SIEXPR ESPERADO.");
	}
}

fn palist(){

}

fn block(){

}

fn statm(){

}

fn progrm(){

}