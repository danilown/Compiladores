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
const simbolo_packed: u32 = 32;

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

fn sitype() {
	let mut simbolo;

	simbolo = recebe_token();
	match simbolo {
		TYIDEN => {
			consome_token();
			return;
		},

		simbolo_abre_parenteses => {
			consome_token();
			/* MEPA 101 */
			loop {
				simbolo = consome_token();
				if simbolo == IDEN {
					/* MEPA 011 */
					simbolo = consome_token();
					match simbolo {
						simbolo_virgula => continue,

						simbolo_fecha_parenteses => {
							/* MEPA 002 */
							return;
						},

						_ => {
							println!("ERRO, VIRGULA OU FECHA PARENTESES ESPERADO.");
							return;
						},
					}
				}
				else {
					println!("ERRO, IDENFIER ESPERADO.");
					return
				}
			}
		},

		_ => {
			consta();
			simbolo = recebe_token();
			if simbolo == simbolo_ponto_ponto {
				consome_token();
				consta();
				/* MEPA 103 */
				return;
			}
			else {
				println!("ERRO, PONTO PONTO ESPERADO.");
				return;
			}
		},
	}
}

fn consta() {
	let mut simbolo;

	simbolo = consome_token();
	if simbolo == STRING {
		return;
	}
	match simbolo {
		simbolo_soma => ,

		simbolo_subtracao => ,

		_ => ,
	}
	simbolo = consome_token();
	match simbolo {
		COIDEN => {
			/* MEPA 071 */
			return;
		},

		NUMB => {
			/* MEPA 072 */
			return;
		},

		_ => {
			println!("ERRO, COIDEN OU NUMB ESPERADO.");
			return;
		},
	}
}

fn type() {
	let mut simbolo;

	simbolo = recebe_token();
	match simbolo {
		simbolo_arroba => {
			consome_token();
			simbolo = recebe_token();
			if simbolo == TYIDEN {
				/* MEPA 111  */
				return;
			}
			else {
				println!("ERRO, TYIDEN ESPERADO.");
				return;
			}
		},

		simbolo_packed => {

		},
	}
}

fn filist() {

}

fn infipo() {
	let mut simbolo;

	loop{
		simbolo = recebe_token();
		match simbolo {
			simbolo_abre_colchetes => {
				consome_token();
				loop{
					expr();
					simbolo = consome_token();
					match simbolo {
						simbolo_aspas => continue,

						simbolo_fecha_colchetes => break,

						_ => {
							println!("ERRO, ASPAS OU FECHA COLCHETES ESPERADO.");
							return;
						},
					}
				}
			},

			simbolo_ponto => {
				consome_token();
				simbolo = consome_token();
				if simbolo == FIIDEN {
					continue;
				}
				else {
					println!("ERRO, FIIDEN ESPERADO.");
					return;
				}
			},

			simbolo_arroba => {
				consome_token();
				continue;
			},
			_ => break,
		}
	}
}

fn factor() {

}

fn term() {
	let mut simbolo;

	loop {
		factor();
		simbolo = recebe_token();
		match simbolo {
			simbolo_asterisco => {
				consome_token();
				continue;
			},

			simbolo_barra => {
				consome_token();
				continue;
			},

			simbolo_div => {
				consome_token();
				continue;
			},

			simbolo_mod => {
				consome_token();
				continue;
			},

			simbolo_and => {
				consome_token();
				continue;
			},

			_ => break,
		}
	}
}

fn siexpr() {
	let mut simbolo;

	simbolo = recebe_token();
	match simbolo {
		simbolo_soma => consome_token(),

		simbolo_subtracao => consome_token(),

		_ => ,
	}
	loop {
		term();
		simbolo = recebe_token();
		match simbolo {
			simbolo_soma => {
				consome_token();
				continue;
			},

			simbolo_subtracao => {
				consome_token();
				continue;
			},

			simbolo_or => {
				consome_token();
				continue;
			},

			_ => break;
		}
	}
}

fn expr() {
	let mut simbolo;

	siexpr();
	simbolo = recebe_token();
	match simbolo {
		simbolo_igual => consome_token(),

		simbolo_menor => consome_token(),

		simbolo_maior => consome_token(),

		simbolo_diferente => consome_token(),

		simbolo_maior_igual => consome_token(),

		simbolo_menor_igual => consome_token(),

		simbolo_in => consome_token(),

		_ => return,
	}
	siexpr();
}

fn palist() {

}

fn block() {

}

fn statm() {

}

fn progrm() {

}
