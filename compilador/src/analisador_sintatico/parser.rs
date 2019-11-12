use crate::analisador_lexico::tipo_token::*;
use crate::analisador_lexico::token::*;

use crate::tabela_simbolos::simbolo::*;
use crate::tabela_simbolos::sym_tab::*;

use colored::*;

//use std::fmt;

/*
#[derive(Clone, PartialEq, Debug)]
enum RegVal {
    ValInt(i128),
    ValChar(char),
    ValBool(bool),
    ValStr(String),
    ValFloat(f64),
    Nop,
}


impl fmt::Display for RegVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match &*self {
            RegVal::Nop => "void".to_string(),
            RegVal::ValBool(v) => v.to_string(),
            RegVal::ValChar(v) => v.to_string(),
            RegVal::ValFloat(v) => v.to_string(),
            RegVal::ValInt(v) => v.to_string(),
            RegVal::ValStr(v) => v.to_string(),
        };

        write!(f, "{}", printable)
    }
}
*/

pub struct Parser {
    tokens: Vec<Token>,
    token_atual: usize,
    tabela_de_simbolos: TabelaSimbolos,
    abre_escopo : bool,
    on_hold: Option<Simbolo>,
    indice_on_hold: usize,
    nome_funcao: String,
    reg_tipo: Tipo_Token,
    reg_val: String,
    temp_num: usize,
    comandos: Vec<String>,
    label_else: Vec<String>,
    while_entrs: Vec<String>,
    while_saidas: Vec<String>,

    pilha_params: Vec<(String, Tipo_Token)>,
    num_params_passado: usize,
    funcao: bool,
    return_check: bool,
}

impl Parser {
    pub fn novo(e_tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: e_tokens,
            token_atual: 0,
            tabela_de_simbolos: TabelaSimbolos::nova(),
            abre_escopo: true,
            on_hold: None,
            indice_on_hold: 0,
            nome_funcao: "Global".into(),
            reg_val: "".to_string(),
            reg_tipo: Tipo_Token::VOID,
            temp_num: 0,
            comandos: Vec::new(),
            label_else: Vec::new(),
            while_entrs: Vec::new(),
            while_saidas: Vec::new(),

            pilha_params: Vec::new(),
            num_params_passado: 0,
            funcao: false,
            return_check: false,
        }
    }

    pub fn iniciar_analise(&mut self) {
        self.decls();
        for i in self.comandos.clone() {
            println!("{}", i);
        }

    }

    //TODO: dar uso ou tirar
    pub fn _tokens(&self) -> Vec<Token> {
        self.tokens.to_vec()
    }

    pub fn tabela_de_simbolos(&self) -> TabelaSimbolos {
        self.tabela_de_simbolos.clone()
    }

    fn match_token(&self, t: Tipo_Token) -> bool {
        self.tokens[self.token_atual].token() == t
    }

    fn tipo_atual(&self) -> Tipo_Token {
        self.tokens[self.token_atual].token()
    }

    fn symtab_lookup(&self, lexema: String) -> Tipo_Token {
        //TODO: melhorar erro
        self
            .tabela_de_simbolos
            .lookup(lexema.clone())
            .unwrap_or_else(||
                panic!("entrada não encontrada na tabela de Simbolo; linha: {}, variavel: {}", self.tokens[self.token_atual].linha(), lexema)
            )
    }

    fn buscar_funcao_params(&self, lexema: String) -> Vec<Tipo_Token> {
        //TODO: melhorar erro
        self.tabela_de_simbolos.lista_params(lexema.clone()).unwrap_or_else(||
                panic!("função não encontrada na tabela de Simbolo; linha: {}, chamada por: {}", self.tokens[self.token_atual].linha(), lexema)
            )
    }

    fn consumir_token(&mut self) {
        self.token_atual += 1;
    }

    fn erro_generico(&self, msg: &str) {
        println!("{}", msg);
        std::process::exit(1);
    }

    fn erro(&self, token: &str) {
        print!(
            " {} esperado na linha {}: ",
            token.green(),
            self.tokens[self.token_atual - 1].linha()
        );
        println!(" encontrado a seguir {}", self.tokens[self.token_atual].lexema().red().underline());
        std::process::exit(1);
    }

    ///////////////////////////////////////////////////////////////////////////

    fn abrir_escopo(&mut self) {
        self.tabela_de_simbolos.abrir_escopo();
    }

    fn fechar_escopo(&mut self) {
        self.tabela_de_simbolos.fechar_escopo();
        self.abre_escopo = true;
    }

    fn add_simbolo(&mut self) {
        let s = self.on_hold.clone().unwrap();
        self.on_hold = None;

        let s2: Simbolo;

        match s {
            Simbolo::Func(a, b, _c, d, e, f) => {
                s2 = Simbolo::Func(a, b, d.len(), d, e, f);
            }

            _ => { panic!("ooooooooo"); }
        }

        let i = self.tabela_de_simbolos.add_simbolo_escopo_global(s2);
        self.tokens[self.indice_on_hold].set_symtab(i as u64);
    }

    fn add_direto(&mut self, s: Simbolo, alvo : usize) {
        let i = self.tabela_de_simbolos.add_simbolo(s);
        self.tokens[alvo].set_symtab(i as u64);
    }

    fn set_nome_on_hold(&mut self) {
        let t = self.on_hold.clone().unwrap();
        self.on_hold = None;

        let token = self.tokens[self.token_atual].clone();

        self.nome_funcao = token.lexema();

        match t {
            Simbolo::Func(_n, a, b, c, _d, _e) => {
                self.on_hold = Some(Simbolo::Func(token.lexema(), a, b, c, token.linha(), _e))
            }
            _ => { panic!("qqqq"); }
        }
    }

    fn set_tipo_on_hold(&mut self) {
        let t = self.on_hold.clone().unwrap();
        self.on_hold = None;

        let token = self.tokens[self.token_atual].clone();
        let tipo = token.token();

        match t {
            Simbolo::Func(n, _a, b, c, d, e) => {
                self.on_hold = Some(Simbolo::Func(n, tipo, b, c, d, e));
            }
            _ => { panic!("qqqq"); }
        }
    }

    fn add_on_hold_params(&mut self) {
        let t = self.on_hold.clone().unwrap();
        self.on_hold = None;

        let token = self.tokens[self.token_atual].clone();
        let tipo = token.token();

        match t {
            Simbolo::Func(n, a, b, mut c, d, e) => {
                c.push(tipo);
                self.on_hold = Some(Simbolo::Func(n, a, b, c, d, e));
            }
            _ => { panic!("qqqq"); }
        }
    }

    ///////////////////////////////////////////////////////////////////////////

    fn id_tipo(&self) -> bool {
        self.match_token(Tipo_Token::ID_BOOL)
            || self.match_token(Tipo_Token::ID_CHAR)
            || self.match_token(Tipo_Token::ID_FLOAT)
            || self.match_token(Tipo_Token::ID_INT)
            || self.match_token(Tipo_Token::ID_STR)
            || self.match_token(Tipo_Token::ID_VOID)
    }

    fn e_literal(&self) -> bool {
        self.match_token(Tipo_Token::OCTAL)
            || self.match_token(Tipo_Token::HEX)
            || self.match_token(Tipo_Token::INT)
            || self.match_token(Tipo_Token::STR)
            || self.match_token(Tipo_Token::CHAR)
            || self.match_token(Tipo_Token::FLOAT)
            || self.match_token(Tipo_Token::TRUE)
            || self.match_token(Tipo_Token::FALSE)
    }

    fn e_unaria(&mut self) -> bool {
        self.match_token(Tipo_Token::SIMBOLO_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_MENOS)
    }

    fn preparar_expr(&mut self) {

        let mut pos_relativa = 1;

        let mut curr_token = Tipo_Token::VOID;
        let mut prev_token;

        while curr_token != Tipo_Token::PONTO_VIRGULA {
            curr_token = self.tokens[self.token_atual + pos_relativa].token();
            prev_token = self.tokens[self.token_atual + pos_relativa - 1].token();
            let n_linha = self.tokens[self.token_atual + pos_relativa].linha();
            pos_relativa += 1;

            if curr_token == Tipo_Token::SIMBOLO_MENOS &&
                (
                    prev_token == Tipo_Token::OCTAL ||
                    prev_token == Tipo_Token::HEX||
                    prev_token == Tipo_Token::INT||
                    prev_token == Tipo_Token::FLOAT
                )

            {

                let mut injetado = Token {
                    lexema : "+".to_string(),
                    linha: n_linha,
                    literal: "".to_string(),
                    symtab: 0,
                    token: Tipo_Token::SIMBOLO_MAIS,
                    valor_bool: None,
                    valor_char: None,
                    valor_float: None,
                    valor_int: None,
                    valor_str: None,

                };
                self.tokens.insert(self.token_atual + pos_relativa - 1, injetado);


                injetado = Token {
                    lexema : "(".to_string(),
                    token: Tipo_Token::PARENTESE_ESQUERDO,
                    linha: n_linha,
                    literal: "".to_string(),
                    symtab: 0,
                    valor_bool: None,
                    valor_char: None,
                    valor_float: None,
                    valor_int: None,
                    valor_str: None,
                };


                self.tokens.insert(self.token_atual + pos_relativa, injetado);



                injetado = Token {
                    lexema : ")".to_string(),
                    token: Tipo_Token::PARENTESE_DIREITO,
                    linha: n_linha,
                    literal: "".to_string(),
                    symtab: 0,
                    valor_bool: None,
                    valor_char: None,
                    valor_float: None,
                    valor_int: None,
                    valor_str: None,
                };
                self.tokens.insert(self.token_atual + pos_relativa + 3, injetado);

            }



        }

    }
    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////

    fn decls(&mut self) {
            // TODO: semantica aqui

        if self.token_atual >= self.tokens.len() {
            return;
        }
        if self.match_token(Tipo_Token::EOF) {
            return;
        }
        if self.match_token(Tipo_Token::FUNC) || self.match_token(Tipo_Token::ID) {
            self.decl();
            self.decls();
        } else {
            println!("APENAS DECLARAÇÃO DE FUNÇÃO OU VARIAVEL GLOBALMENTE");
            std::process::exit(1);
        }
    }

    fn decl(&mut self) {
            // TODO: semantica aqui
        if self.match_token(Tipo_Token::FUNC) {
            self.func_decl();
        } else if self.match_token(Tipo_Token::ID) {
            self.var_decl();
        } else {
            self.erro("tipo ou func");
        }
    }
    ///////////////////////////////////////////////////////////////////////////
    fn func_decl(&mut self) {
        if self.match_token(Tipo_Token::FUNC) {

            self.comandos.push("".into());
            self.comandos.push("".into());

            self.abrir_escopo();
            self.abre_escopo = false;
            self.on_hold = Some(Simbolo::Func("".into(), Tipo_Token::VOID, 0, vec![], 0, 0));

            self.return_check = true;

            self.consumir_token();
            if self.match_token(Tipo_Token::ID) {

                self.set_nome_on_hold();
                self.indice_on_hold = self.token_atual;

                self.comandos.push(format!("FUNC__CALL__{}:", self.tokens[self.token_atual].lexema()));
                let linha = self.tokens[self.token_atual].linha();

                self.consumir_token();
                if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.consumir_token();
                    self.func_params_opt();


                    if self.return_check {
                        let msg = format!("Funcao nao retornou, funcao declarada na linha {}", linha);
                        self.erro_generico(&msg);
                    }


                    self.comandos.push(format!("RET"));
                } else {
                    self.erro("(");
                }
            } else {
                self.erro("indentificador");
            }
            self.comandos.push("".into());
            self.comandos.push("".into());
        } else {
            self.erro("func");
        }
    }
    fn func_params_opt(&mut self) {
        if self.match_token(Tipo_Token::ID) {
            self.params();
            if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                self.consumir_token();
                if self.match_token(Tipo_Token::RETURNS) {
                    self.consumir_token();

                    self.set_tipo_on_hold();
                    self.add_simbolo();

                    if self.match_token(Tipo_Token::ID_VOID) { self.return_check = false; }
                    self.t_type();
                    self.block();
                    self.nome_funcao = "Global".into();
                } else {
                    self.erro("returns");
                }
            }
        } else if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
            self.consumir_token();
            if self.match_token(Tipo_Token::RETURNS) {
                self.consumir_token();

                self.set_tipo_on_hold();
                self.add_simbolo();

                self.t_type();
                self.block();
            } else {
                self.erro("returns");
            }
        } else {
            self.erro("identificador");
            self.erro(")");
        }
    }
    fn params(&mut self) {
        self.param();
        self.params_opt();
    }
    fn params_opt(&mut self) {
        if self.match_token(Tipo_Token::VIRGULA) {
            self.consumir_token();
            self.params();
        }
    }
    fn param(&mut self) {
        if self.match_token(Tipo_Token::ID) {

            let id = self.tokens[self.token_atual].lexema();
            let alvo = self.token_atual;

            self.comandos.push(format!("{} := POP__PARAM", id));


            self.consumir_token();
            if self.match_token(Tipo_Token::AS) {
                self.consumir_token();

                let s = Simbolo::Var(id, self.tokens[self.token_atual].token(), self.tokens[self.token_atual].linha(), self.nome_funcao.clone(), 0);
                self.add_direto(s, alvo);
                self.add_on_hold_params();

                self.t_type();
            } else {
                self.erro("as");
            }
        } else {
            self.erro("indentificador");
        }
    }
    ///////////////////////////////////////////////////////////////////////////
    fn var_decl(&mut self) {
        self.var();
        if self.match_token(Tipo_Token::PONTO_VIRGULA) {
            self.consumir_token();
        } else {
            self.erro(";");
        }

    }
    fn var(&mut self) {
            // TODO: semantica aqui
        if self.match_token(Tipo_Token::ID) {

            let id = self.tokens[self.token_atual].lexema();
            let alvo = self.token_atual;

            self.consumir_token();
            if self.match_token(Tipo_Token::AS) {
                self.consumir_token();

                let linha = self.tokens[self.token_atual].linha();

                let s = Simbolo::Var(id.clone(), self.tokens[self.token_atual].token(), linha, self.nome_funcao.clone(), 0);

                self.t_type();

                let tipo = self.reg_tipo;
                self.reg_tipo = Tipo_Token::VOID;
                self.reg_val = format!("{}__NAO_DEFINIDO", tipo).into();


                self.var_opt();

                if self.reg_tipo == Tipo_Token::VOID {
                    let comando = format!("{} := {}", id.clone(), self.reg_val);
                    self.comandos.push(comando);
                } else if tipo != self.reg_tipo {
                    let msg =  format!("tipo de variavel não casa com expressão na linha {}:
                            variavel do tipo: {}
                            expressão do tipo: {}", linha, tipo, self.reg_tipo);
                    self.erro_generico(&msg);
                } else {
                    let comando = format!("{} := {}", id.clone(), self.reg_val);
                    self.comandos.push(comando);
                }



                // Adicionado na tabale de simbolos só depois de avaliar
                self.add_direto(s, alvo);
            } else {
                self.erro("as");
            }
        } else {
            self.erro("indentificador");
        }
    }
    fn var_opt(&mut self) {
        if self.match_token(Tipo_Token::SIMBOLO_IGUAL) {
            self.consumir_token();
            self.expr();
        }
    }
    ///////////////////////////////////////////////////////////////////////////
    fn t_type(&mut self) {
        if self.id_tipo() {
            self.reg_tipo = self.tokens[self.token_atual].token();
            self.consumir_token();
        } else {
            self.erro("indentificador de tipo");
        }
    }
    ///////////////////////////////////////////////////////////////////////////
    fn stm(&mut self) {

        if self.match_token(Tipo_Token::ID) && self.tokens[self.token_atual + 1].token() == Tipo_Token::AS { // diferenciar de uma expressão
            self.var_decl();
        } else if self.match_token(Tipo_Token::ID) && self.tokens[self.token_atual + 1].token() == Tipo_Token::SIMBOLO_IGUAL {
            self.var_assign();
            if self.match_token(Tipo_Token::PONTO_VIRGULA) {
                self.consumir_token();
            } else {
                self.erro(";");
            }
        } else if self.match_token(Tipo_Token::IF) {
            self.consumir_token();
            if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.consumir_token();

                self.expr();

                if self.reg_tipo != Tipo_Token::ID_BOOL {
                    let linha = self.tokens[self.token_atual].linha();
                    let msg = format!("IF's apenas aceitam expressões booleanas, que não foi o caso na linha {}", linha);
                    self.erro_generico(&msg);

                }

                let label_ok = format!("LABEL__{}", self.temp_num);
                self.temp_num += 1;
                let label_else_temp = format!("LABEL__{}", self.temp_num);
                self.label_else.push(label_else_temp.clone());
                self.temp_num += 1;
                let label_fora = format!("LABEL__{}", self.temp_num);
                self.temp_num += 1;

                let comando_if = format!("IF {} GOTO {}", self.reg_val, label_ok.clone());
                let comando_nao_entra_no_if = format!("GOTO {}", label_else_temp.clone());

                self.comandos.push(comando_if);
                self.comandos.push(comando_nao_entra_no_if);


                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.consumir_token();
                    self.comandos.push(format!("{}:", label_ok.clone())); // add label se entra no if
                    self.then_stm();
                    self.comandos.push(format!("GOTO {}", label_fora.clone())); // pula pra fora do if
                    self.if_opt();
                    self.comandos.push(format!("{}:", label_fora)); // add label se entra no if
                } else {
                    self.erro(")");
                }
            } else {
                self.erro("(");
            }




        }


        else if self.match_token(Tipo_Token::WHILE) {
            self.consumir_token();
            if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.consumir_token();
                self.expr();

                if self.reg_tipo != Tipo_Token::ID_BOOL {
                    let linha = self.tokens[self.token_atual].linha();
                    let msg = format!("WHILE's apenas aceitam expressões booleanas, que não foi o caso na linha {}", linha);
                    self.erro_generico(&msg);

                }


                let var_test = self.reg_val.clone();

                let label_entrada_temp = format!("WHILE__ENTRADA__{}", self.temp_num);
                self.temp_num += 1;
                let label_test_temp = format!("WHILE__TEST__{}", self.temp_num);
                self.temp_num += 1;
                let label_saida_temp = format!("WHILE__SADA__{}", self.temp_num);
                self.temp_num += 1;

                self.while_entrs.push(label_entrada_temp.clone());
                self.while_saidas.push(label_saida_temp.clone());



                self.comandos.push(format!("{}:", label_entrada_temp.clone()));
                self.comandos.push(format!("IF {} GOTO {}", var_test.clone(), label_test_temp.clone()));
                self.comandos.push(format!("GOTO {}", label_saida_temp.clone()));
                self.comandos.push(format!("{}:", label_test_temp.clone()));

                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.consumir_token();
                    self.then_stm();

                    self.comandos.push(format!("GOTO {}", self.while_entrs.pop().unwrap()));
                    self.comandos.push(format!("{}:", self.while_saidas.pop().unwrap()));


                } else {
                    self.erro(")");
                }
            } else {
                self.erro("(");
            }
        }




        else if self.match_token(Tipo_Token::BREAK)
            || self.match_token(Tipo_Token::CONTINUE)
            || self.match_token(Tipo_Token::RETURN)
            || self.match_token(Tipo_Token::PRINTK)
            || self.match_token(Tipo_Token::PONTO_VIRGULA)
            || self.match_token(Tipo_Token::CHAVE_ESQUERDA)
            || self.e_unaria()
            || self.e_literal()
            || self.match_token(Tipo_Token::ID)
            || self.match_token(Tipo_Token::PARENTESE_ESQUERDO)
        {
            self.normal_stm();
        } else {
            self.erro("stm")
        }
    }
    fn then_stm(&mut self) {
        if self.match_token(Tipo_Token::IF) {
            self.consumir_token();
            if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.consumir_token();

                self.expr();

                if self.reg_tipo != Tipo_Token::ID_BOOL {
                    let linha = self.tokens[self.token_atual].linha();
                    let msg = format!("IF's apenas aceitam expressões booleanas, que não foi o caso na linha {}", linha);
                    self.erro_generico(&msg);

                }

                let label_ok = format!("LABEL__{}", self.temp_num);
                self.temp_num += 1;
                let label_else_temp = format!("LABEL__{}", self.temp_num);
                self.label_else.push(label_else_temp.clone());
                self.temp_num += 1;
                let label_fora = format!("LABEL__{}", self.temp_num);
                self.temp_num += 1;

                let comando_if = format!("IF {} GOTO {}", self.reg_val, label_ok.clone());
                let comando_nao_entra_no_if = format!("GOTO {}", label_else_temp.clone());

                self.comandos.push(comando_if);
                self.comandos.push(comando_nao_entra_no_if);


                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.consumir_token();
                    self.comandos.push(format!("{}:", label_ok.clone())); // add label se entra no if
                    self.then_stm();
                    self.comandos.push(format!("GOTO {}", label_fora.clone())); // pula pra fora do if
                    self.if_opt();
                    self.comandos.push(format!("{}:", label_fora)); // add label se entra no if
                } else {
                    self.erro(")")
                }
            } else {
                self.erro("(");
            }
        }

        else if self.match_token(Tipo_Token::WHILE) {
            self.consumir_token();
            if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.consumir_token();
                self.expr();

                if self.reg_tipo != Tipo_Token::ID_BOOL {
                    let linha = self.tokens[self.token_atual].linha();
                    let msg = format!("WHILE's apenas aceitam expressões booleanas, que não foi o caso na linha {}", linha);
                    self.erro_generico(&msg);

                }


                let var_test = self.reg_val.clone();

                let label_entrada_temp = format!("WHILE__ENTRADA__{}", self.temp_num);
                self.temp_num += 1;
                let label_test_temp = format!("WHILE__TEST__{}", self.temp_num);
                self.temp_num += 1;
                let label_saida_temp = format!("WHILE__SADA__{}", self.temp_num);
                self.temp_num += 1;

                self.while_entrs.push(label_entrada_temp.clone());
                self.while_saidas.push(label_saida_temp.clone());



                self.comandos.push(format!("{}:", label_entrada_temp.clone()));
                self.comandos.push(format!("IF {} GOTO {}", var_test.clone(), label_test_temp.clone()));
                self.comandos.push(format!("GOTO {}", label_saida_temp.clone()));
                self.comandos.push(format!("{}:", label_test_temp.clone()));

                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.consumir_token();
                    self.then_stm();

                    self.comandos.push(format!("GOTO {}", self.while_entrs.pop().unwrap()));
                    self.comandos.push(format!("{}:", self.while_saidas.pop().unwrap()));


                } else {
                    self.erro(")");
                }
            } else {
                self.erro("(");
            }
        }


        else if self.match_token(Tipo_Token::BREAK)
            || self.match_token(Tipo_Token::CONTINUE)
            || self.match_token(Tipo_Token::RETURN)
            || self.match_token(Tipo_Token::PRINTK)
            || self.match_token(Tipo_Token::PONTO_VIRGULA)
            || self.match_token(Tipo_Token::CHAVE_ESQUERDA)
            || self.e_unaria()
            || self.e_literal()
            || self.match_token(Tipo_Token::ID)
            || self.match_token(Tipo_Token::PARENTESE_ESQUERDO)
        {
            self.normal_stm();
        } else {
            self.erro("muita coisa de novo no then_stm não");
        }
    }

    fn if_opt(&mut self) {
        self.comandos.push(format!("{}:", self.label_else.pop().clone().unwrap())); // add label se NAO entra no if
        if self.match_token(Tipo_Token::ELSE) {
            self.consumir_token();
            self.then_stm();
        }
    }

    fn normal_stm(&mut self) {
        if self.match_token(Tipo_Token::CHAVE_ESQUERDA) {
            self.block();
        } else if self.match_token(Tipo_Token::BREAK) {
            self.consumir_token();
            let label_saida = self.while_saidas.pop().unwrap();
            self.comandos.push(format!("GOTO {}", label_saida.clone()));
            self.while_saidas.push(label_saida);
            if self.match_token(Tipo_Token::PONTO_VIRGULA) {
                self.consumir_token();
            } else {
                self.erro(";");
            }
        } else if self.match_token(Tipo_Token::CONTINUE) {
            self.consumir_token();
            let label_entrada = self.while_entrs.pop().unwrap();
            self.comandos.push(format!("GOTO {}", label_entrada.clone()));
            self.while_entrs.push(label_entrada);
            if self.match_token(Tipo_Token::PONTO_VIRGULA) {
                self.consumir_token();
            } else {
                self.erro(";");
            }
        } else if self.match_token(Tipo_Token::PONTO_VIRGULA) {
            self.consumir_token();
        } else if self.match_token(Tipo_Token::RETURN) {


            self.return_check = false;

            // TODO: testar mais
            self.consumir_token();

            let tipo_da_funcao = self.symtab_lookup(self.nome_funcao.clone());

            if tipo_da_funcao != Tipo_Token::ID_VOID {

                self.expr();

                let tipo_da_expr = self.reg_tipo;

                if tipo_da_expr != tipo_da_funcao {
                    let msg = format!("{} é uma função que retorna {}, mas um {} foi retornado na linha {}",
                        self.nome_funcao,
                        tipo_da_funcao,
                        tipo_da_expr,
                        self.tokens[self.token_atual].linha()
                    );
                    self.erro_generico(&msg);
                }

                self.comandos.push(format!("RET__REG := {}", self.reg_val));

            }

            self.comandos.push("RET".into());

            if self.match_token(Tipo_Token::PONTO_VIRGULA) {
                self.consumir_token();
            } else {
                self.erro(";");
            }

        } else if self.match_token(Tipo_Token::PRINTK) {
            // ok
            self.consumir_token();
            if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.consumir_token();
                self.expr();


                let comando = format!("PRINTK {}", self.reg_val);
                self.comandos.push(comando);

                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.consumir_token();
                    if self.match_token(Tipo_Token::PONTO_VIRGULA) {
                        self.consumir_token();
                    } else {
                        self.erro(";");
                    }
                } else {
                    self.erro("e)");
                }
            } else {
                self.erro("(");
            }
        } else if self.e_unaria()
            || self.e_literal()
            || self.match_token(Tipo_Token::ID)
            || self.match_token(Tipo_Token::PARENTESE_ESQUERDO)
        {
            // ok
            // aqui acho que não faz nada, o mais util aqui é chamada de função
            self.expr();
            if self.match_token(Tipo_Token::PONTO_VIRGULA) {
                self.consumir_token();
            } else {
                self.erro(";");
            }
        } else {
            self.erro("normal_stm");
        }
    }

    fn block(&mut self) {
        if self.match_token(Tipo_Token::CHAVE_ESQUERDA) {
            self.consumir_token();

            if self.abre_escopo {
                self.abrir_escopo();
            } else {
                self.abre_escopo = true;
            }

            self.stm_list();

            if self.match_token(Tipo_Token::CHAVE_DIREITA) {
                self.consumir_token();
                self.fechar_escopo();
            } else {
                self.erro("}");
            }
        } else {
            self.erro("{");
        }
    }

    fn stm_list(&mut self) {
        if self.match_token(Tipo_Token::IF)
            || self.match_token(Tipo_Token::WHILE)
            || self.id_tipo()
            || self.match_token(Tipo_Token::BREAK)
            || self.match_token(Tipo_Token::CONTINUE)
            || self.match_token(Tipo_Token::RETURN)
            || self.match_token(Tipo_Token::PRINTK)
            || self.match_token(Tipo_Token::PONTO_VIRGULA)
            || self.match_token(Tipo_Token::CHAVE_ESQUERDA)
            || self.e_unaria()
            || self.e_literal()
            || self.match_token(Tipo_Token::ID)
            || self.match_token(Tipo_Token::PARENTESE_ESQUERDO)
        {
            self.stm();
            self.stm_list();
        }
    }

    fn var_assign(&mut self) {
        // TODO: testar mais isso aqui
        if self.match_token(Tipo_Token::ID) {

            let lexema_var = self.tokens[self.token_atual].lexema();
            let tipo_var = self.symtab_lookup(lexema_var.clone());
            let linha = self.tokens[self.token_atual].linha();

            self.consumir_token();
            if self.match_token(Tipo_Token::SIMBOLO_IGUAL) {
                self.consumir_token();
                self.expr();


                let tipo_expr = self.reg_tipo.clone();
                let val_expr = self.reg_val.clone();


                if tipo_var != tipo_expr {
                    let msg =  format!("tipo de variavel não casa com expressão na linha {}:
                            variavel do tipo: {}
                            expressão do tipo: {}", linha, tipo_var, tipo_expr);
                    self.erro_generico(&msg);
                }

                let comando = format!("{} := {}", lexema_var, val_expr);
                self.comandos.push(comando)


            } else {
                self.erro("=");
            }
        } else {
            self.erro("id");
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn expr(&mut self) {

        if !self.match_token(Tipo_Token::PONTO_VIRGULA) {
            self.preparar_expr();
        }

        self.op_or();

    }
    ///////////////////////////////////////////////////////////////////////////
    fn op_or(&mut self) {
        self.op_and();
        self.op_or_opt();
    }
    fn op_or_opt(&mut self) {
        if self.match_token(Tipo_Token::SIMBOLO_D_OR) {

            let val1 = self.reg_val.clone();
            let tipo1 = self.reg_tipo.clone();
            let _op = self.tipo_atual().clone();

            self.consumir_token();
            self.expr();

            let val2 = self.reg_val.clone();
            let tipo2 = self.reg_tipo.clone();
            self.reg_tipo = Tipo_Token::BOOL;

            let string_operador = "&&".to_string();

            match tipo1 {
                Tipo_Token::ID_BOOL => {
                    match tipo2 {
                        Tipo_Token::ID_BOOL => {
                            self.reg_tipo = Tipo_Token::ID_BOOL;
                        },
                        _ => { panic!("Or de booleanos apenas com booleanos"); }
                    }
                },
                _ => { panic!("Or apenas entre booleanos."); }
            }

            // fim


            let var = format!("{}{}", "VAR__", self.temp_num);
            self.temp_num += 1;
            let comando = format!("{} := {} {} {}", var, val1, string_operador, val2);
            self.comandos.push(comando);
            self.reg_val = var;




        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_and(&mut self) {
        self.op_bin_or();
        self.op_and_opt();
    }
    fn op_and_opt(&mut self) {
        if self.match_token(Tipo_Token::SIMBOLO_D_AND) {

            let val1 = self.reg_val.clone();
            let tipo1 = self.reg_tipo.clone();
            let _op = self.tipo_atual().clone();

            self.consumir_token();
            self.expr();

            let val2 = self.reg_val.clone();
            let tipo2 = self.reg_tipo.clone();
            self.reg_tipo = Tipo_Token::ID_BOOL;

            let string_operador = "&&".to_string();

            match tipo1 {
                Tipo_Token::ID_BOOL => {
                    match tipo2 {
                        Tipo_Token::ID_BOOL => {
                            self.reg_tipo = Tipo_Token::ID_BOOL;
                        },
                        _ => { panic!("And de booleanos apenas com booleanos"); }
                    }
                },
                _ => { panic!("And apenas entre booleanos."); }
            }

            // fim


            let var = format!("{}{}", "VAR__", self.temp_num);
            self.temp_num += 1;
            let comando = format!("{} := {} {} {}", var, val1, string_operador, val2);
            self.comandos.push(comando);
            self.reg_val = var;

        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_bin_or(&mut self) {
        self.op_bin_and();
        self.op_bin_or_opt();
    }
    fn op_bin_or_opt(&mut self) {
        if self.match_token(Tipo_Token::SIMBOLO_OR) {


            let val1 = self.reg_val.clone();
            let tipo1 = self.reg_tipo.clone();
            let _op = self.tipo_atual().clone();

            self.consumir_token();
            self.expr();

            let val2 = self.reg_val.clone();
            let tipo2 = self.reg_tipo.clone();
            self.reg_tipo = Tipo_Token::ID_BOOL;

            let string_operador = "|".to_string();

            match tipo1 {
                Tipo_Token::ID_INT => {
                    match tipo2 {
                        Tipo_Token::ID_INT => {
                            self.reg_tipo = Tipo_Token::ID_INT;
                        },
                        _ => { panic!("Or binario de INT apenas com INT"); },
                    }
                },
                Tipo_Token::ID_BOOL => {
                    match tipo2 {
                        Tipo_Token::ID_BOOL => {
                            self.reg_tipo = Tipo_Token::ID_BOOL;
                        },
                        _ => { panic!("Or binario de booleanos apenas com booleanos"); }
                    }
                },
                _ => { panic!("Or binario apenas entre valores numericos inteiros e booleanos apenas."); }
            }


            // fim

            let var = format!("{}{}", "VAR__", self.temp_num);
            self.temp_num += 1;
            let comando = format!("{} := {} {} {}", var, val1, string_operador, val2);
            self.comandos.push(comando);
            self.reg_val = var;

        }
    }
    ///////////////////////////////////////////////////////////////////////////
    fn op_bin_and(&mut self) {
        self.op_equate();
        self.op_bin_and_opt();
    }
    fn op_bin_and_opt(&mut self) {
        if self.match_token(Tipo_Token::SIMBOLO_AND) {

            let val1 = self.reg_val.clone();
            let tipo1 = self.reg_tipo.clone();
            let _op = self.tipo_atual().clone();

            self.consumir_token();
            self.expr();

            let val2 = self.reg_val.clone();
            let tipo2 = self.reg_tipo.clone();
            self.reg_tipo = Tipo_Token::BOOL;

            let string_operador = "&".to_string();

            match tipo1 {
                Tipo_Token::ID_INT => {
                    match tipo2 {
                        Tipo_Token::ID_INT => {
                            self.reg_tipo = Tipo_Token::ID_INT;
                        },
                        _ => { panic!("And binario de INT apenas com INT"); },
                    }
                },
                Tipo_Token::ID_BOOL => {
                    match tipo2 {
                        Tipo_Token::ID_BOOL => {
                            self.reg_tipo = Tipo_Token::ID_BOOL;
                        },
                        _ => { panic!("And binario de booleanos apenas com booleanos"); }
                    }
                },
                _ => { panic!("And binario apenas entre valores numericos inteiros e booleanos apenas."); }
            }

            //fim

            let var = format!("{}{}", "VAR__", self.temp_num);
            self.temp_num += 1;
            let comando = format!("{} := {} {} {}", var, val1, string_operador, val2);
            self.comandos.push(comando);
            self.reg_val = var;

        }
    }
    ///////////////////////////////////////////////////////////////////////////
    fn op_equate(&mut self) {
        self.op_compare();
        self.op_equate_opt();
    }
    fn op_equate_opt(&mut self) {
        if self.match_token(Tipo_Token::SIMBOLO_D_IGUAL)
            || self.match_token(Tipo_Token::SIMBOLO_D_DIFERENTE) {

            let val1 = self.reg_val.clone();
            let tipo1 = self.reg_tipo.clone();
            let op = self.tipo_atual().clone();

            self.consumir_token();
            self.expr();

            let val2 = self.reg_val.clone();
            let tipo2 = self.reg_tipo.clone();
            self.reg_tipo = Tipo_Token::ID_BOOL;

            let mut string_operador = "".to_string();

            match tipo1 {
                Tipo_Token::ID_FLOAT => {
                    match tipo2 {
                        Tipo_Token::ID_FLOAT => {
                            match op {
                                Tipo_Token::SIMBOLO_D_IGUAL => { string_operador = "==".to_string(); },
                                Tipo_Token::SIMBOLO_D_DIFERENTE => { string_operador = "!=".to_string(); },
                                _ => {},
                            }
                        },
                        _ => { panic!("Igualdades de FLOAT apenas com FLOAT"); },
                    }
                },
                Tipo_Token::ID_INT => {
                    match tipo2 {
                        Tipo_Token::ID_INT => {
                            match op {
                                Tipo_Token::SIMBOLO_D_IGUAL => { string_operador = "==".to_string(); },
                                Tipo_Token::SIMBOLO_D_DIFERENTE => { string_operador = "!=".to_string(); },
                                _ => {},
                            }
                        },
                        _ => { panic!("Igualdades de INT apenas com INT"); },
                    }
                },
                Tipo_Token::ID_BOOL => {
                    match tipo2 {
                        Tipo_Token::ID_BOOL => {
                            match op {
                                Tipo_Token::SIMBOLO_D_IGUAL => { string_operador = "==".to_string(); },
                                Tipo_Token::SIMBOLO_D_DIFERENTE => { string_operador = "!=".to_string(); },
                                _ => {},
                            }
                        },
                        _ => { panic!("Igualdades de booleanos apenas com booleanos"); }
                    }
                },
                _ => { panic!("Igualdades apenas entre valores numericos e booleanos apenas."); }
            }

            let var = format!("{}{}", "VAR__", self.temp_num);
            self.temp_num += 1;
            let comando = format!("{} := {} {} {}", var, val1, string_operador, val2);
            self.comandos.push(comando);
            self.reg_val = var;

        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_compare(&mut self) {
        self.op_add();
        self.op_compare_opt();
    }
    fn op_compare_opt(&mut self) {
        if self.match_token(Tipo_Token::SIMBOLO_MENOR_Q)
            || self.match_token(Tipo_Token::SIMBOLO_MAIOR_Q)
            || self.match_token(Tipo_Token::SIMBOLO_MAIOR_IGUAL_Q)
            || self.match_token(Tipo_Token::SIMBOLO_MENOR_IGUAL_Q)
        {

            let val1 = self.reg_val.clone();
            let tipo1 = self.reg_tipo.clone();
            let op = self.tipo_atual().clone();

            self.consumir_token();
            self.expr();

            let val2 = self.reg_val.clone();
            let tipo2 = self.reg_tipo.clone();
            self.reg_tipo = Tipo_Token::ID_BOOL;

            let mut string_operador = "".to_string();

            match tipo1 {
                Tipo_Token::ID_FLOAT => {
                    match tipo2 {
                        Tipo_Token::ID_FLOAT => {
                            match op {
                                Tipo_Token::SIMBOLO_MENOR_Q => { string_operador = "<".to_string(); },
                                Tipo_Token::SIMBOLO_MAIOR_Q => { string_operador = ">".to_string(); },
                                Tipo_Token::SIMBOLO_MAIOR_IGUAL_Q => { string_operador = ">=".to_string(); },
                                Tipo_Token::SIMBOLO_MENOR_IGUAL_Q => { string_operador = "<=".to_string(); },
                                _ => {},
                            }
                        },
                        _ => { panic!("Comparações de FLOAT apenas com FLOAT"); },
                    }
                },
                Tipo_Token::ID_INT => {
                    match tipo2 {
                        Tipo_Token::ID_INT => {
                            match op {
                                Tipo_Token::SIMBOLO_MENOR_Q => { string_operador = "<".to_string(); },
                                Tipo_Token::SIMBOLO_MAIOR_Q => { string_operador = ">".to_string(); },
                                Tipo_Token::SIMBOLO_MAIOR_IGUAL_Q => { string_operador = ">=".to_string(); },
                                Tipo_Token::SIMBOLO_MENOR_IGUAL_Q => { string_operador = "<=".to_string(); },
                                _ => {},
                            }
                        },
                        _ => { panic!("Comparações de INT apenas com INT"); },
                    }
                },
                _ => { panic!("Comparações apenas entre valores numericos apenas."); }
            }


            let var = format!("{}{}", "VAR__", self.temp_num);
            self.temp_num += 1;
            let comando = format!("{} := {} {} {}", var, val1, string_operador, val2);
            self.comandos.push(comando);
            self.reg_val = var;



        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_add(&mut self) {
        self.op_mult();
        self.op_add_opt();
    }
    fn op_add_opt(&mut self) {
        if self.match_token(Tipo_Token::SIMBOLO_MAIS) || self.match_token(Tipo_Token::SIMBOLO_MENOS)
        {

            let tipo1 = self.reg_tipo.clone();
            let val1 = self.reg_val.clone();
            let op = self.tipo_atual().clone();

            self.consumir_token();
            self.expr();

            let tipo2 = self.reg_tipo.clone();
            let val2 = self.reg_val.clone();

            let mut string_operador = "".to_string();

            match tipo1 {
                Tipo_Token::ID_FLOAT => {
                    match tipo2 {
                        Tipo_Token::ID_INT => {
                            self.reg_tipo = Tipo_Token::ID_FLOAT;
                            match op {
                                Tipo_Token::SIMBOLO_MAIS  => { string_operador = "+".to_string(); },
                                Tipo_Token::SIMBOLO_MENOS => { string_operador = "-".to_string(); },
                                _ => {},
                            }
                        },
                        _ => { panic!("Somas de FLOAT apenas com FLOAT"); },
                    }
                },
                Tipo_Token::ID_INT => {
                    match tipo2 {
                        Tipo_Token::ID_INT => {
                            self.reg_tipo = Tipo_Token::ID_INT;
                            match op {
                                Tipo_Token::SIMBOLO_MAIS  => { string_operador = "+".to_string(); },
                                Tipo_Token::SIMBOLO_MENOS => { string_operador = "-".to_string(); },
                                _ => {},
                            }
                        },
                        _ => { panic!("Somas de INT apenas com INT"); },
                    }
                },
                _ => { panic!("Somas apenas entre numericos"); },
            }


            let var = format!("{}{}", "VAR__", self.temp_num);
            self.temp_num += 1;
            let comando = format!("{} := {} {} {}", var, val1, string_operador, val2);
            self.comandos.push(comando);
            self.reg_val = var;


        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_mult(&mut self) {
        self.op_unary();
        self.op_mult_opt();
    }
    fn op_mult_opt(&mut self) {
        if self.match_token(Tipo_Token::SIMBOLO_MULTI)
            || self.match_token(Tipo_Token::SIMBOLO_DIV)
            || self.match_token(Tipo_Token::SIMBOLO_MOD)
        {

            let tipo1 = self.reg_tipo.clone();
            let val1 = self.reg_val.clone();
            let op = self.tipo_atual().clone();

            self.consumir_token();
            self.expr();

            let tipo2 = self.reg_tipo.clone();
            let val2 = self.reg_val.clone();


            let mut string_operador = "".to_string();

            match tipo1 {
                Tipo_Token::ID_FLOAT => {
                    match tipo2 {
                        Tipo_Token::ID_FLOAT => {
                            self.reg_tipo = Tipo_Token::ID_FLOAT;

                            match op {
                                Tipo_Token::SIMBOLO_MULTI => { string_operador = "*".to_string(); },
                                Tipo_Token::SIMBOLO_DIV   => { string_operador = "/".to_string(); },
                                Tipo_Token::SIMBOLO_MOD   => { string_operador = "%".to_string(); },
                                _ => {},
                            }

                        }
                        _ => { panic!("Multiplicações de FLOAT apenas com FLOAT"); },
                    }
                },
                Tipo_Token::ID_INT => {
                    match tipo2 {
                        Tipo_Token::ID_INT => {
                            self.reg_tipo = Tipo_Token::ID_INT;

                            match op {
                                Tipo_Token::SIMBOLO_MULTI => { string_operador = "*".to_string(); },
                                Tipo_Token::SIMBOLO_DIV   => { string_operador = "/".to_string(); },
                                Tipo_Token::SIMBOLO_MOD   => { string_operador = "%".to_string(); },
                                _ => {},
                            }

                        }
                        _ => { panic!("Multiplicações de INT apenas com INT"); },
                    }
                },
                _ => { panic!("Multiplicações (*, /, %) apenas entre numericos"); },
            }


            let var = format!("{}{}", "VAR__", self.temp_num);
            self.temp_num += 1;
            let comando = format!("{} := {} {} {}", var, val1, string_operador, val2);
            self.comandos.push(comando);
            self.reg_val = var;





        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_unary(&mut self) {
        if self.e_unaria() {

            let op = self.tipo_atual().clone();

            self.consumir_token();
            self.expr();

            match op {

                Tipo_Token::SIMBOLO_MENOS => {
                    match self.reg_tipo {
                        Tipo_Token::ID_INT => {
                            let val = self.reg_val.clone();
                            let var = format!("{}{}", "VAR__", self.temp_num);
                            self.temp_num += 1;
                            let comando = format!("{} := -{}", var, val);
                            self.comandos.push(comando);
                            self.reg_val = var;
                       }

                        Tipo_Token::ID_FLOAT => {
                            let val = self.reg_val.clone();
                            let var = format!("{}{}", "VAR__", self.temp_num);
                            self.temp_num += 1;
                            let comando = format!("{} := -{}", var, val);
                            self.comandos.push(comando);
                            self.reg_val = var;
                        }


                        _ => { panic!("Aplicando operador '-' a um não numerico"); },
                    }
                },

                Tipo_Token::SIMBOLO_NOT => {
                    match self.reg_tipo {
                        Tipo_Token::ID_BOOL => {
                            let val = self.reg_val.clone();
                            let var = format!("{}{}", "VAR__", self.temp_num);
                            self.temp_num += 1;
                            let comando = format!("{} := !{}", var, val);
                            self.comandos.push(comando);
                            self.reg_val = var;
                        },
                        _ => {
                            panic!("Aplicando operador '!' a um não booleano");
                        },
                    }
                },
                Tipo_Token::SIMBOLO_BIT_NOT => {
                    match self.reg_tipo {
                        Tipo_Token::ID_INT => {
                            let val = self.reg_val.clone();
                            let var = format!("{}{}", "VAR__", self.temp_num);
                            self.temp_num += 1;
                            let comando = format!("{} := ~{}", var, val);
                            self.comandos.push(comando);
                            self.reg_val = var;
                        },
                        _ => { panic!("Aplicando operador '~' a um não numerico inteiro"); },
                    }
                },
                _ => {},
            }


        } else {
            self.value();
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn value(&mut self) {
        if self.match_token(Tipo_Token::OCTAL)
            || self.match_token(Tipo_Token::HEX)
            || self.match_token(Tipo_Token::INT)
            || self.match_token(Tipo_Token::STR)
            || self.match_token(Tipo_Token::CHAR)
            || self.match_token(Tipo_Token::FLOAT)
            || self.match_token(Tipo_Token::TRUE)
            || self.match_token(Tipo_Token::FALSE)
        {

            // --------------------------- Semantica -------------------------------------------------------
            match self.tipo_atual() {
                Tipo_Token::OCTAL => self.reg_val = self.tokens[self.token_atual].valor_int().unwrap().to_string(),
                Tipo_Token::HEX   => self.reg_val = self.tokens[self.token_atual].valor_int().unwrap().to_string(),
                Tipo_Token::INT   => self.reg_val = self.tokens[self.token_atual].valor_int().unwrap().to_string(),

                Tipo_Token::STR   => self.reg_val = format!("\"{}\"", self.tokens[self.token_atual].valor_str().unwrap().to_string()),

                Tipo_Token::CHAR  => self.reg_val = format!("\'{}\'", self.tokens[self.token_atual].valor_char().unwrap().to_string()),

                Tipo_Token::FLOAT => self.reg_val = self.tokens[self.token_atual].valor_float().unwrap().to_string(),

                Tipo_Token::TRUE  => self.reg_val = self.tokens[self.token_atual].valor_bool().unwrap().to_string(),
                Tipo_Token::FALSE => self.reg_val = self.tokens[self.token_atual].valor_bool().unwrap().to_string(),


                _ => {}
            }

            match self.tipo_atual() {
                Tipo_Token::OCTAL => self.reg_tipo = Tipo_Token::ID_INT,
                Tipo_Token::HEX   => self.reg_tipo = Tipo_Token::ID_INT,
                Tipo_Token::INT   => self.reg_tipo = Tipo_Token::ID_INT,
                Tipo_Token::TRUE  => self.reg_tipo = Tipo_Token::ID_BOOL,
                Tipo_Token::FALSE => self.reg_tipo = Tipo_Token::ID_BOOL,
                Tipo_Token::STR   => self.reg_tipo = Tipo_Token::ID_STR,
                Tipo_Token::CHAR  => self.reg_tipo = Tipo_Token::ID_CHAR,

                _ => self.reg_tipo = self.tipo_atual()
            }


            // -------------------------------------------------------------------------------------------


            self.consumir_token();
        } else if self.match_token(Tipo_Token::ID) {

            let lexema = self.tokens[self.token_atual].lexema();
            let linha = self.tokens[self.token_atual].linha();
            let tipo = self.symtab_lookup(lexema.clone());
            let val = lexema.clone();

            self.consumir_token();
            self.id_opt();


            if self.funcao {

                self.reg_tipo = tipo;

                let lista_params_definidos = self.buscar_funcao_params(lexema.clone());
                let num_params_definido = lista_params_definidos.len();

                if num_params_definido != self.num_params_passado {
                    let msg = format!("{} tem {} parametros, mas foram passados {} parametros. Linha {}",
                        lexema.clone(),
                        num_params_definido,
                        self.num_params_passado,
                        linha
                    );
                    self.erro_generico(&msg);
                }


                for i in 0..lista_params_definidos.len() {

                    if lista_params_definidos[i] != self.pilha_params[i].1 {
                        let msg = format!("parametro numero {} da função {} com tipo errado, função recebe {} mas {} foi passado. linha {}",
                            i+1,
                            lexema.clone(),
                            lista_params_definidos[i],
                            self.pilha_params[i].1,
                            linha
                        );
                        self.erro_generico(&msg);
                    }

                }


                self.pilha_params.reverse();
                for i in self.pilha_params.clone() {
                    self.comandos.push(format!("PARAM {}", i.0));
                }

                let var = format!("{}{}", "VAR__", self.temp_num);
                self.temp_num += 1;

                self.comandos.push(format!("CALL FUNC__CALL__{}, {}", val.clone(), self.num_params_passado));
                self.comandos.push(format!("{} := RET__REG", var));
                self.reg_val = var;
                self.funcao = false;

                self.pilha_params = Vec::new();
                self.num_params_passado = 0;



            } else {
                self.reg_tipo = tipo;
                self.reg_val = val;
            }


        } else if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
            self.consumir_token();
            self.expr();
            if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                self.consumir_token();
            } else {
                self.erro(")");
            }
        } else {
            self.erro("expressão ou valor");
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn id_opt(&mut self) {
        if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
            self.consumir_token();
            self.id_opt_2();
            self.funcao = true;
        }
    }
    fn id_opt_2(&mut self) {
        if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
            self.consumir_token();
        } else {
            self.expr_list();
            if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                self.consumir_token();
            }
        }
    }

    fn expr_list(&mut self) {
        self.expr();
        self.pilha_params.push((self.reg_val.clone(), self.reg_tipo.clone()));
        self.expr_list_opt();
        self.num_params_passado += 1;
    }

    fn expr_list_opt(&mut self) {
        if self.match_token(Tipo_Token::VIRGULA) {
            self.consumir_token();
            self.expr_list();
        }
    }
}
