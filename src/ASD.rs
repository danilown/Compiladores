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
const simbolo_array: u32 = 33;
const simbolo_of: u32 = 34;
const simbolo_file: u32 = 35;
const simbolo_set: u32 = 36;
const simbolo_end: u32 = 37;
const simbolo_ponto_virgula: u32 = 38;
const simbolo_dois_pontos: u32 = 39;
const simbolo_case: u32 = 40;
const simbolo_lambda: u32 = 41;
const simbolo_nil: u32 = 42;
const simbolo_not: u32 = 43;
const simbolo_proc: u32 = 44;
const simbolo_func: u32 = 45;
const simbolo_var: u32 = 46;
const simbolo_label: u32 = 47;
const simbolo_const: u32 = 48;
const simbolo_type: u32 = 49;
const simbolo_begin: u32 = 50;
const simbolo_if: u32 = 51;
const simbolo_while: u32 = 52;
const simbolo_repeat: u32 = 53;
const simbolo_for: u32 = 54;
const simbolo_with: u32 = 55;
const simbolo_goto: u32 = 56;
const simbolo_dois_pontos_igual: u32 = 57;
const simbolo_then: u32 = 58;
const simbolo_else: u32 = 59;
const simbolo_do: u32 = 60;
const simbolo_until: u32 = 61;
const simbolo_to: u32 = 62;
const simbolo_down_to: u32 = 63;

// Codigos para tokens terminais
const NUMB: u32 = 64; 	// number
const STRING: u32 = 65;	// cadeia de caracteres
const IDEN: u32 = 66;	// identifier
const COIDEN: u32 = 67;	// constant identifier
const FIIDEN: u32 = 68;	// filed identifier
const VAIDEN: u32 = 69; // variable identifier
const FUIDEN: u32 = 70;	// function identifier
const TYIDEN: u32 = 71;	// type identifier
const PRIDEN: u32 = 72;	// procedure identifier

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

		simbolo_packed => consome_token(),

		_ => ,
	}

	simbolo = recebe_token();
	match simbolo {
		simbolo_array => {
			consome_token();
			/* MEPA 114 */
			simbolo = consome_token();
			if simbolo == simbolo_abre_colchetes {
				loop {
					sitype();
					/* MEPA 115 */
					simbolo = consome_token();
					match simbolo {
						simbolo_virgula =>  continue,
						simbolo_fecha_colchetes => break,
					}
				}
				simbolo = consome_token();
				if simbolo == simbolo_of {
					/* MEPA 116 */
					type();
					/* MEPA 117 */
					return;
				}
				else {
					println!("ERRO, 'OF' ESPERADO!");
					return;
				}
			}
			else {
				println!("ERRO, ABRE COLCHETES ESPERADO!");
				return;
			}
		},

		simbolo_file => {
			consome_token();
			simbolo = consome_token();
			if simbolo == simbolo_of {
				type();
				/* MEPA 113 */
				return;
			}
			else {
				println!("ERRO, 'OF' ESPERADO!");
			}
		},

		simbolo_set => {
			consome_token();
			simbolo = consome_token();
			if simbolo == simbolo_of {
				sitype();
				/* MEPA 112 */
				return;
			}
			else {
				println!("ERRO, 'OF' ESPERADO!");
			}
		},

		simbolo_record => {
			consome_token();
			/* MEPA 121 */
			filist();
			/* MEPA 122 */
			simbolo = consome_token();
			if simbolo == simbolo_end {
				return;
			}
			else {
				println!("ERRO, 'END' ESPERADO!");
			}

		},

		_ => {
			sitype();
			return;
		},
	}
}

fn filist() {
	let mut simbolo;

	loop {
		simbolo = consome_token();

		if simbolo == IDEN {
			/* MEPA 030 */
			simbolo = consome_token();
			if simbolo == simbolo_virgula {
				continue;
			}
			else {
				if simbolo = simbolo_dois_pontos {
					/* MEPA 102 */
					type();
				}
				else {
					println!("ERRO, DOIS PONTOS OU VIRGULA ESPERADO.");
					return;
				}
			}
		}
		simbolo = consome_token();
		if simbolo == simbolo_ponto_virgula {
			continue;
		}
		else {
			break;
		}
	}

	if simbolo == simbolo_case {
		/* MEPA 001 */
		simbolo = consome_token();
		if simbolo == IDEN {
			simbolo = consome_token();
			if simbolo != simbolo_virgula {
				println!("ERRO, VIRGULA ESPERADA.");
				return;
			}
		}
		simbolo = consome_token();
		if simbolo != simbolo_lambda {
			println!("ERRO, LAMBDA ESPERADA.");
			return;
		}
		/* MEPA 002 */
		simbolo = consome_token();
		if simbolo != TYIDEN {
			println!("ERRO, TYIDEN ESPERADO.");
			return;
		}
		/* MEPA 132 */
		simbolo = consome_token();
		if simbolo != simbolo_of {
			println!("ERRO, 'OF' ESPERADO.");
			return;
		}
		loop {
			simbolo = consome_token();
			match simbolo {
				STRING => ,

				simbolo_soma => {
					simbolo = consome_token();
					match simbolo {
						COIDEN => ,
						NUMB => ,
						_ => {
							println!("ERRO, COIDEN ou NUMB ESPERADO.");
							return;
						},
					}
				},

				simbolo_subtracao => {
					simbolo = consome_token();
					match simbolo {
						COIDEN => ,
						NUMB => ,
						_ => {
							println!("ERRO, COIDEN ou NUMB ESPERADO.");
							return;
						},
					}
				},

				COIDEN => ,
				NUMB => ,
				_ => {
					simbolo = consome_token();
					match simbolo {
						simbolo_ponto_virgula => continue,
						simbolo_lambda => {
							/* MEPA 135 */
							return;
						},
						_ => {
							println!("ERRO, PONTO E VIRGULA OU LAMBDA ESPERADO.");
							return;
						},
					}
				},
			}
			simbolo = consome_token();
			match simbolo {
				simbolo_virgula => continue,
				simbolo_dois_pontos => {
					/* MEPA 133 */
					simbolo = consome_token();
					if simbolo != simbolo_abre_parenteses {
						println!("ERRO, ABRE PARENTESES ESPERADO.");
						return;
					}
					/* MEPA 001 */
					filist()
					/* MEPA 134 */
					simbolo = consome_token();
					if simbolo != simbolo_fecha_parenteses {
						println!("ERRO, FECHA PARENTESES ESPERADO.");
						return;
					}
				},
				_ => {
					println!("ERRO, VIRGULA OU DOIS PONTOS ESPERADO.");
					return;
				},
			}

			simbolo = consome_token();
			match simbolo {
				simbolo_ponto_virgula => continue,
				simbolo_lambda => {
					/* MEPA 135 */
					return;
				},
				_ => {
					println!("ERRO, PONTO E VIRGULA OU LAMBDA ESPERADO.");
					return;
				},
			}

		}
	}
	else {
		if simbolo == simbolo_lambda {
			/* MEPA 131 */
			return;
		}
		else {
			println!("ERRO, CASE OU LAMBDA ESPERADO.");
			return;
		}
	}
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
	let mut simbolo;

	simbolo = consome_token();
	match simbolo {
		COIDEN => return,
		NUMB => {
			/* MEPA 073 */
			return;
		},

		simbolo_nil => {
			/* MEPA 110 */
			return;
		},

		STRING => return,
		VAIDEN => {
			/* MEPA 170 */
			infipo();
			/* MEPA 167 */
			return;
		},

		FUIDEN => {
			/* MEPA 077 */
			simbolo = consome_token();
			match simbolo {
				simbolo_abre_parenteses => {
					loop {
						expr();
						/* MEPA 078 */
						simbolo = consome_token();
						match simbolo {
							simbolo_virgula => continue,
							simbolo_abre_parenteses => {
								/* MEPA 079 */
								return;
							},
						}
					}
				},

				simbolo_lambda => {
					/* MEPA 079 */
					return;
				},

				_ => {
					println!("ERRO, ABRE PARENTESES OU LAMBDA ESPERADO.");
					return;
				},
			}
		},

		simbolo_abre_parenteses => {
			expr();
			/* MEPA 154 */
			simbolo = consome_token();
			if simbolo != simbolo_abre_parenteses {
				println!("ERRO, ABRE PARENTESES ESPERADO.");
				return;
			}
			return;
		},

		simbolo_not => {
			factor();
			/* MEPA 155 */
			return;
		},

		simbolo_abre_colchetes => {
			simbolo = consome_token();
			if simbolo == simbolo_fecha_colchetes {
				/* MEPA 161 */
				return;
			}
			loop {
				expr();
				/* MEPA 162 */
				simbolo = consome_token();
				if simbolo == simbolo_ponto_ponto {
					expr();
					/* MEPA 163 */
				}
				simbolo = consome_token();
				match simbolo {
					simbolo_virgula => continue,
					simbolo_fecha_colchetes => return,
					_ => {
						println!("ERRO, VIRGULA OU FECHA COLCHETES ESPERADO.");
						return;
					},
				}
			}
		},

		_ => {
			println!("ERRO, COIDEN OU NUMB SIMBOLO_NIL OU STRING OU VAIDEN OU FUIDEN OU ABRE PARENTESES OU ABRE COLCHETES OU NOT ESPERADO.");
			return;
		},
	}
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
	let mut simbolo = consome_token();

	if simbolo == simbolo_abre_parenteses {
		loop {
			/* MEPA 008 */
			simbolo = consome_token();
			if simbolo == simbolo_proc {
				loop {
					simbolo = consome_token();
					if simbolo != IDEN {
						println!("ERRO, IDEN ESPERADO!");
						return;
					}
					/* MEPA 045 */
					if simbolo == simbolo_virgula {
						continue;
					}
					else {
						break;
					}
				}
			}
			else {
				if simbolo == simbolo_func {
					/* MEPA 006 */
				}
				else {
					if simbolo == simbolo_var {
						/* MEPA 007 */
					}
				}
				loop {
					simbolo = consome_token();
					if simbolo != IDEN {
						println!("ERRO, IDEN ESPERADO!");
						return;
					}
					/* MEPA 005 */
					simbolo = consome_token();
					match simbolo {
						simbolo_virgula => continue,
						simbolo_dois_pontos => {
							/* MEPA 002 */
							simbolo = consome_token();
							if simbolo != TYIDEN {
								println!("ERRO, VIRGULA OU FECHA PARENTESES ESPERADO!");
								return;
							}
							/* MEPA 009 */
							break;
						},
					}
				}
			}
			simbolo = consome_token();
			match simbolo {
				simbolo_virgula => {
					/* MEPA 004 */
					continue;
				},
				simbolo_fecha_parenteses => return,
			}
		}
	}
	else {
		return;
	}
}

fn block() {
	let mut simbolo;

	loop {
		simbolo = consome_token();
		match simbolo {
			simbolo_label => {
				loop {
					simbolo = consome_token();
					if simbolo != NUMB {
						println!("ERRO, NUMB ESPERADO!");
						return;
					}
					/* MEPA 070 */
					simbolo = consome_token();
					match simbolo {
						simbolo_virgula => continue,
						simbolo_ponto_virgula => break,
						_ => {
							println!("ERRO, VIRGULA OU PONTO E VIRGULA ESPERADO!");
							return;
						},
					}
				}
			},
			simbolo_const => {
				simbolo = consome_token();
				if simbolo != IDEN {
					println!("ERRO, IDEN ESPERADO!");
					return;
				}
				/* MEPA 010 */
				loop {
					simbolo = consome_token();
					if simbolo != simbolo_igual {
						println!("ERRO, IGUAL ESPERADO!");
						return;
					}
					/* MEPA 002 */
					consta();
					/* MEPA 012 */
					simbolo = consome_token();
					if simbolo != simbolo_ponto_virgula {
						println!("ERRO, PONTO E VIRGULA ESPERADO!");
						return;
					}
					/* MEPA 001 */
					simbolo = consome_token();
					if simbolo = IDEN {
						/* MEPA 010 */
						continue;
					}
					else {
						break;
					}
				}
			},
			simbolo_type => {
				simbolo = consome_token();
				if simbolo != IDEN {
					println!("ERRO, IDEN ESPERADO!");
					return;
				}
				/* MEPA 050 */
				loop {
					simbolo = consome_token();
					if simbolo != simbolo_igual {
						println!("ERRO, IGUAL ESPERADO!");
						return;
					}
					/* MEPA 002 */
					type();
					/* MEPA 051 */
					simbolo = consome_token();
					if simbolo == simbolo_ponto_virgula {
						println!("ERRO, PONTO E VIRGULA ESPERADO!");
						return;
					}
					/* MEPA 001 */
					simbolo = consome_token();
					if simbolo == IDEN {
						/* MEPA 050 */
						continue;
					}
					else {
						break;
					}
				}
			},
			simbolo_var => {
				simbolo = consome_token();
				if simbolo != IDEN {
					println!("ERRO, IDEN ESPERADO!");
					return;
				}
				/* MEPA 060 */
				loop {
					simbolo = consome_token();
					match simbolo {
						simbolo_virgula => {
							simbolo = consome_token();
							if simbolo != simbolo_virgula {
								println!("ERRO, VIRGULA ESPERADA!");
								return;
							}
							simbolo = consome_token();
							if simbolo != IDEN {
								println!("ERRO, IDEN ESPERADO!");
								return;
							}
							/* MEPA 065 */
							continue;
						},
						simbolo_dois_pontos => {
							/* MEPA 102 */
							type();
							/* MEPA 069 */
							simbolo = consome_token();
							if simbolo != simbolo_ponto_virgula {
								println!("ERRO, PONTO E VIRGULA ESPERADO!");
								return;
							}
							/* MEPA 001 */
							simbolo = consome_token();
							if simbolo == IDEN {
								/* MEPA 060 */
								continue;
							}
							else {
								break;
							}
						},
						_ => {
							println!("ERRO, VIRGULA OU DOIS PONTOS ESPERADO!");
							return;
						},
					}
				}
			},
			simbolo_proc => {
				simbolo = consome_token();
				if simbolo != IDEN {
					println!("ERRO, IDEN ESPERADO!");
					return;
				}
				/* MEPA 040 */
				palist();
				simbolo = consome_token();
				if simbolo != simbolo_ponto_virgula {
					println!("ERRO, PONTO E VIRGULA ESPERADO!");
					return;
				}
				/* MEPA 001 */
				block();
				/* MEPA 141 */
				simbolo = consome_token();
				if simbolo != simbolo_ponto_virgula {
					println!("ERRO, PONTO E VIRGULA ESPERADO!");
					return;
				}
				continue;
			},
			simbolo_func => {
				simbolo = consome_token();
				if simbolo != IDEN {
					println!("ERRO, IDEN ESPERADO!");
					return;
				}
				/* MEPA 020 */
				palist();
				simbolo = consome_token();
				if simbolo != simbolo_dois_pontos {
					println!("ERRO, DOIS PONTOS ESPERADOS!");
					return;
				}
				/* MEPA 002 */
				simbolo = consome_token();
				if simbolo != TYIDEN {
					println!("ERRO, TYIDEN ESPERADO!");
					return;
				}
				/* MEPA 074 */
				simbolo = consome_token();
				if simbolo != simbolo_ponto_virgula {
					println!("ERRO, PONTO E VIRGULA ESPERADO!");
					return;
				}
				/* MEPA 001 */
				block();
				/* MEPA 141 */
				simbolo = consome_token();
				if simbolo != simbolo_ponto_virgula {
					println!("ERRO, PONTO E VIRGULA ESPERADO!");
					return;
				}
				continue;
			},
			simbolo_begin => {
				/* MEPA 142 */
				loop {
					statm();
					simbolo = consome_token();
					match simbolo {
						simbolo_ponto_virgula => continue,
						simbolo_end => return,
						_ => {
							println!("ERRO, PONTO E VIRGULA OU END ESPERADO!");
							return;
						},
					}
				}
			},
			_ => {
				println!("ERRO, LABEL OU CONST OU TYPE OU VAR OU PROC OU FUNC OU BEGIN ESPERADO!");
				return;
			},
		}
	}
}

fn statm() {
	let mut simbolo = consome_token();

	match simbolo {
		NUMB => {
			/* MEPA 075 */
			simbolo = consome_token();
			if simbolo != simbolo_dois_pontos {
				println!("ERRO, DOIS PONTOS ESPERADO!");
				return;
			}
			statm();
			return;
		},
		VAIDEN => {
			infipo();
			simbolo = consome_token();
			if simbolo != simbolo_dois_pontos_igual {
				println!("ERRO, DOIS PONTOS IGUAL ESPERADO!");
				return;
			}
			expr();
			/* MEPA 080 */
			return;
		},
		FUIDEN => {
			simbolo = consome_token();
			if simbolo != simbolo_dois_pontos_igual {
				println!("ERRO, DOIS PONTOS IGUAL ESPERADO!");
				return;
			}
			expr();
			/* MEPA 080 */
			return;
		},
		PRIDEN => {
			/* MEPA 077 */
			simbolo = consome_token();
			match simbolo {
				simbolo_abre_parenteses => {
					loop {
						simbolo = consome_token();
						if simbolo == PRIDEN {
							/* MEPA 078 */
						}
						else {
							expr();
							/* MEPA 078 */
						}
						simbolo = consome_token();
						match simbolo {
							simbolo_virgula => continue,
							simbolo_fecha_parenteses => {
								/* MEPA 079 */
								return;
							},
							_ => {
								println!("ERRO, FECHA PARENTESES OU VIRGULA ESPERADO!");
								return;
							} 
						}
					}
				},
				simbolo_lambda => {
					/* MEPA 079 */
						return;
				},
				_ => {
					println!("ERRO, ABRE PARENTESES OU LAMBDA ESPERADO!");
					return;
				},
			}
		},
		simbolo_begin => {
			loop {
				statm();
				simbolo = consome_token();
				match simbolo {
					simbolo_ponto_virgula => continue,
					simbolo_end => return,
					_ => {
						println!("ERRO, PONTO E VIRGULA OU END ESPERADO!");
						return;
					},
				}
			}
		},
		simbolo_if => {
			expr();
			simbolo = consome_token();
			if simbolo != simbolo_then {
				println!("ERRO, THEN ESPERADO!");
				return;
			}
			/* MEPA 086 */
			statm();
			simbolo = consome_token();
			if simbolo == simbolo_else {
				/* MEPA 087 */
				statm();
				simbolo = consome_token();
			}
			/* MEPA 095 */
			if simbolo != simbolo_lambda {
				println!("ERRO, LAMBDA ESPERADO!");
				return;
			}
			/* MEPA 088 */
			return;
		},
		simbolo_case => {
			/* MEPA 094 */
			expr();
			simbolo = consome_token();
			if simbolo != simbolo_of {
				println!("ERRO, 'OF' ESPERADO!");
				return;
			}
			loop {
				simbolo = consome_token();
				match simbolo {
					STRING => ,
					simbolo_soma => {
						simbolo = consome_token();
						match simbolo {
							COIDEN => {
								/* MEPA 071 */
							},
							NUMB => {
								/* MEPA 072 */
							},
							_ => {
								println!("ERRO, COIDEN OU NUMB ESPERADO!");
								return;
							},
						}
					},
					simbolo_subtracao => {
						simbolo = consome_token();
						match simbolo {
							COIDEN => {
								/* MEPA 071 */
							},
							NUMB => {
								/* MEPA 072 */
							},
							_ => {
								println!("ERRO, COIDEN OU NUMB ESPERADO!");
								return;
							},
						}
					},
					COIDEN => ,
					NUMB => ,
					simbolo_ponto_virgula => {
						/* MEPA 097 */
						continue;
					},
					simbolo_end => {
						/* MEPA 098 */
						return;
					},
					_ => {
						println!("ERRO, STRING OU SOMA OU SUBTRACAO OU COIDEN OU NUMB OU PONTO E VIRGULA OU END ESPERADO!");
						return;
					},
				}
				simbolo = consome_token();
				match simbolo {
					simbolo_virgula => {
						/* MEPA 095 */
						continue;
					},
					simbolo_dois_pontos => {
						/* MEPA 096 */
						statm();
						simbolo = consome_token();
					},
					_ => {
						println!("ERRO, VIRGULA ou DOIS PONTOS ESPERADO!");
						return;
					},
				}
				match simbolo {
					simbolo_ponto_virgula => {
						/* MEPA 097 */
						continue;
					},
					simbolo_end => {
						/* MEPA 098 */
						return;
					},
					_ => {
						println!("ERRO, PONTO E VIRGULA OU END ESPERADO!");
						return;
					},
				}
			}
		},
		simbolo_while => {
			/* MEPA 081 */
			expr();
			simbolo = consome_token();
			if simbolo != simbolo_do {
				println!("ERRO, 'DO' ESPERADO!");
				return;
			}
			/* MEPA 082 */
			statm();
			/* MEPA 083 */
			return;
		},
		simbolo_repeat => {
			/* MEPA 084 */
			loop {
				statm();
				simbolo = consome_token();
				match simbolo {
					simbolo_ponto_virgula => continue,
					simbolo_until => break,
					_ => {
						println!("ERRO, PONTO E VIRGULA OU UNTIL ESPERADO!");
						return;
					},
				}
			}
			expr();
			/* MEPA 085 */
			return;
		},
		simbolo_for => {
			simbolo = consome_token();
			if simbolo != VAIDEN {
				println!("ERRO, VAIDEN ESPERADO!");
				return;
			}
			infipo();
			simbolo = consome_token();
			if simbolo != simbolo_dois_pontos_igual {
				println!("ERRO, DOIS PONTOS IGUAL ESPERADO!");
				return;
			}
			expr();
			/* MEPA 089 */
			simbolo = consome_token();
			match simbolo {
				simbolo_to => ,
				simbolo_down_to => ,
				_ => {
					println!("ERRO, TO OU DOWN_TO ESPERADO!");
					return;
				},
			}
			expr();
			simbolo = consome_token();
			if simbolo != simbolo_do {
				println!("ERRO, DO ESPERADO!");
				return;
			}
			/* MEPA 090 */
			statm();
			/* MEPA 091 */
			return;
		},
		simbolo_with => {
			loop {
				simbolo = consome_token();
				if simbolo != VAIDEN {
					println!("ERRO, VAIDEN ESPERADO!");
					return;
				}
				infipo();
				/* MEPA 092 */
				simbolo = consome_token();
				match simbolo {
					simbolo_virgula => continue,
					simbolo_do => break,
					_ => {
						println!("ERRO, VIRGULA OU DO ESPERADO!");
						return;
					},
				}
			}
			statm();
			/* MEPA 093 */
			return;
		},
		simbolo_goto => {
			simbolo = consome_token();
			if simbolo != NUMB {
				println!("ERRO, NUMB ESPERADO!");
				return;
			}
			/* MEPA 076 */
			return;
		},
		_ => return,
	}
}

fn progrm() {
	let mut simbolo = consome_token();

	if simbolo == IDEN {
		simbolo = consome_token();
		if simbolo == simbolo_abre_parenteses {
			loop {
				simbolo = consome_token();
				if simbolo == IDEN {
					simbolo = consome_token();
					match simbolo {
						simbolo_virgula => continue,

						simbolo_fecha_parenteses => break,

						_ => {
							println!("ERRO, VIRGULA OU FECHA PARENTESES ESPERADO!");
							return;
						},
					}
				}
				else {
					println!("ERRO, IDEN ESPERADO!");
					return;
				}
			}
			simbolo = consome_token();
			if simbolo == simbolo_ponto_virgula {
				block();
				simbolo = consome_token();
				if simbolo == simbolo_ponto {
					return;
				}
				else {
					println!("ERRO, PONTO ESPERADO!");
					return;
				}
			}
			else {
				println!("ERRO, PONTO E VIRGULA ESPERADO!");
				return;
			}
		}
	}
	else{
		println!("ERRO, IDEN ESPERADO!");
		return;
	}
}
