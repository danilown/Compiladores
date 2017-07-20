/**
 * @brief      Verifica se a variável foi declarada no escopo.
 *
 * @param      identificador      string que representa a variável procurada.
 * @param      indiceAtual        indice atual da tabela de simbolos.
 * @param      tamTabelaSimbolos  tamanho da tabela de simbolos.
 *
 * @return     retorna 0 no sucesso (declarada apenas uma vez no escopo e antes do indice dado), um valor maior que 0 em erro.
 */
pub fn checaDeclarações (identificador:String, indiceAtual:i32, tamTabelaSimbolos:i32) -> i8 {

	let mut declaracoesIntervalo=0;
	let mut declaracoesEscopo=0;

	for i in 0..tamTabelaSimbolos {

		if tabelaSimbolos[i].token == identificador && tabelaSimbolos[i-1].tipo == TYIDEN {

			if tabelaSimbolos[indiceAtual].escopo <= tabelaSimbolos[i].escopo {

				declaracoesEscopo++;

				if i < indiceAtual {

					tabelaSimbolos[i].usado=true;
					declaracoesIntervalo++;
				}
			}
		}
	}

	//Variável declarada no intervalo apropriado e unicamente no escopo
	if declaracoesIntervalo == 1 && declaracoesEscopo == 1

		return 0

	//Não declarada no intevalo
	if declaracoesIntervalo == 0

		return 1

	//Declarada múltiplas vezes no escopo
	if declaracoesEscopo > 1

		return 2
}

/**
 * @brief      Checa se a variável está recebendo o tipo apropriado
 *
 * @param      indiceRecebedor  Indice na tabela de símbolos da variável que receberá o valor
 * @param      indiceAlocado    Indice na tabela de símbolos do valor alocado na variável
 *
 * @return     Verdadeiro se o tipo for apropriado, falso caso contrário
 */
pub fn isTipoApropriado (indiceRecebedor:i32, indiceAlocado:i32) -> bool {

	return (tabelaSimbolos[indiceRecebedor].tipo == tabelaSimbolos[indiceAlocado].tipo)
}

/**
 * @brief      Checa se uma expressão é válida
 *
 * @param      tamTabelaSimbolos  tamanho da tabela de simbolos.
 */
pub fn isExpressaoValida (tamTabelaSimbolos:i32) {

	for i in 0..tamTabelaSimbolos {

		match tabelaSimbolos[i].token {

			"+"||"-"||"*"||"/"||"%" => {

				let mut cont=1;

				while tabelaSimbolos[i-cont].tipo != NUMB || tabelaSimbolos[i-cont].tipo != VAIDEN {
					
					if tabelaSimbolos[i-cont].tipo == simbolo_igual {

						println!("ERRO: Expressão inválida na linha: {}",tamTabelaSimbolos[i].line);
						return;
					}

					cont++;
				}

				cont = 1;

				while tabelaSimbolos[i+cont].tipo != NUMB || tabelaSimbolos[i-cont].tipo != VAIDEN {
					
					if tabelaSimbolos[i+cont].line != tabelaSimbolos[i].line {

						println!("ERRO: Expressão inválida na linha: {}",tamTabelaSimbolos[i].line);
						return;
					}

					cont++;
				}
			},
			
			_ => {},
		}
	}
}