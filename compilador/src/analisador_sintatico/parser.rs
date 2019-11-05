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

/*
 * Expressões com mais de um operador só funcionam dois a dois, ou parenteses e tudo ok
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
    _reg_num_params: usize, //TODO: implementar chamada de função
    temp_num: usize,
    comandos: Vec<String>,
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
            _reg_num_params: 0,
            temp_num: 0,
            comandos: Vec::new(),
        }
    }

    pub fn iniciar_analise(&mut self) {
        self.decls();
        //println!("aaaaaaaaaaaa {}", self.reg_val);
        // println!("t {}", self.comandos.len());
        for i in self.comandos.clone() {
            println!("{}", i);
        }

    }

    //TODO:dar uso ou tirar
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

    fn consumir_token(&mut self) {
        self.token_atual += 1;
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
            self.erro("id ou func");
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
            // TODO: semantica aqui
        if self.match_token(Tipo_Token::FUNC) {

            self.abrir_escopo();
            self.abre_escopo = false;
            self.on_hold = Some(Simbolo::Func("".into(), Tipo_Token::VOID, 0, vec![], 0, 0));

            self.consumir_token();
            if self.match_token(Tipo_Token::ID) {

                self.set_nome_on_hold();
                self.indice_on_hold = self.token_atual;

                self.consumir_token();
                if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.consumir_token();
                    self.func_params_opt();
                } else {
                    self.erro("(");
                }
            } else {
                self.erro("indentificador");
            }
        } else {
            self.erro("func");
        }
    }
    fn func_params_opt(&mut self) {
            // TODO: semantica aqui
        if self.match_token(Tipo_Token::ID) {
            self.params();
            if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                self.consumir_token();
                if self.match_token(Tipo_Token::RETURNS) {
                    self.consumir_token();

                    self.set_tipo_on_hold();
                    self.add_simbolo();

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
            // TODO: semantica aqui
        self.param();
        self.params_opt();
    }
    fn params_opt(&mut self) {
            // TODO: semantica aqui
        if self.match_token(Tipo_Token::VIRGULA) {
            self.consumir_token();
            self.params();
        }
    }
    fn param(&mut self) {
            // TODO: semantica aqui
        if self.match_token(Tipo_Token::ID) {

            let id = self.tokens[self.token_atual].lexema();
            let alvo = self.token_atual;

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
            // TODO: semantica aqui
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

                let s = Simbolo::Var(id, self.tokens[self.token_atual].token(), self.tokens[self.token_atual].linha(), self.nome_funcao.clone(), 0);

                self.t_type();
                self.var_opt();


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
            // TODO: semantica aqui
        if self.match_token(Tipo_Token::SIMBOLO_IGUAL) {
            self.consumir_token();
            self.expr();
        }
    }
    ///////////////////////////////////////////////////////////////////////////
    fn t_type(&mut self) {
            // TODO: semantica aqui
        if self.id_tipo() {
            self.consumir_token();
        } else {
            self.erro("indentificador de tipo");
        }
    }
    ///////////////////////////////////////////////////////////////////////////
    fn stm(&mut self) {
            // TODO: semantica aqui

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
                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.consumir_token();
                    self.then_stm();
                    self.if_opt();
                } else {
                    self.erro(")");
                }
            } else {
                self.erro("(");
            }
        } else if self.match_token(Tipo_Token::WHILE) {
            self.consumir_token();
            if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.consumir_token();
                self.expr();
                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.consumir_token();
                    self.then_stm();
                } else {
                    self.erro(")");
                }
            } else {
                self.erro(")");
            }
        } else if self.match_token(Tipo_Token::BREAK)
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
            // TODO: semantica aqui
        if self.match_token(Tipo_Token::IF) {
            self.consumir_token();
            if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.consumir_token();
                self.expr();
                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.consumir_token();
                    self.then_stm();
                    self.if_opt();
                } else {
                    self.erro(")")
                }
            } else {
                self.erro("(");
            }
        } else if self.match_token(Tipo_Token::WHILE) {
            self.consumir_token();
            if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.consumir_token();
                self.expr();
                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.consumir_token();
                    self.then_stm();
                } else {
                    self.erro(")");
                }
            } else {
                self.erro("(");
            }
        } else if self.match_token(Tipo_Token::BREAK)
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
            // TODO: semantica aqui
        if self.match_token(Tipo_Token::ELSE) {
            self.consumir_token();
            self.then_stm();
        }
    }

    fn normal_stm(&mut self) {
            // TODO: semantica aqui

        if self.match_token(Tipo_Token::CHAVE_ESQUERDA) {
            self.block();
        } else if self.match_token(Tipo_Token::BREAK) {
            self.consumir_token();
            if self.match_token(Tipo_Token::PONTO_VIRGULA) {
                self.consumir_token();
            } else {
                self.erro(";");
            }
        } else if self.match_token(Tipo_Token::CONTINUE) {
            self.consumir_token();
            if self.match_token(Tipo_Token::PONTO_VIRGULA) {
                self.consumir_token();
            } else {
                self.erro(";");
            }
        } else if self.match_token(Tipo_Token::PONTO_VIRGULA) {
            self.consumir_token();
        } else if self.match_token(Tipo_Token::RETURN) {
            self.consumir_token();
            self.expr();
            if self.match_token(Tipo_Token::PONTO_VIRGULA) {
                self.consumir_token();
            } else {
                self.erro(";");
            }
        } else if self.match_token(Tipo_Token::PRINTK) {
            self.consumir_token();
            if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.consumir_token();
                self.op_or();
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
            // TODO: semantica aqui
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
            // TODO: semantica aqui
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
            // TODO: semantica aqui
            // TODO: falta os erros
        if self.match_token(Tipo_Token::ID) {
            self.consumir_token();
            if self.match_token(Tipo_Token::SIMBOLO_IGUAL) {
                self.consumir_token();
                self.expr();
            }
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn expr(&mut self) {
        self.preparar_expr(); // corrigir a - b para 1 + (-b)
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


            let var = format!("{}{}", "var__", self.temp_num);
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


            let var = format!("{}{}", "var__", self.temp_num);
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

            let var = format!("{}{}", "var__", self.temp_num);
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

            let var = format!("{}{}", "var__", self.temp_num);
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

            let var = format!("{}{}", "var__", self.temp_num);
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


            let var = format!("{}{}", "var__", self.temp_num);
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


            let var = format!("{}{}", "var__", self.temp_num);
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


            let var = format!("{}{}", "var__", self.temp_num);
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
                            let var = format!("{}{}", "var__", self.temp_num);
                            self.temp_num += 1;
                            let comando = format!("{} := -{}", var, val);
                            self.comandos.push(comando);
                            self.reg_val = var;
                       }

                        Tipo_Token::ID_FLOAT => {
                            let val = self.reg_val.clone();
                            let var = format!("{}{}", "var__", self.temp_num);
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
                            let var = format!("{}{}", "var__", self.temp_num);
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
                            let var = format!("{}{}", "var__", self.temp_num);
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

                Tipo_Token::STR   => self.reg_val = self.tokens[self.token_atual].valor_str().unwrap().to_string(),

                Tipo_Token::CHAR  => self.reg_val = self.tokens[self.token_atual].valor_char().unwrap().to_string(),

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

                _ => self.reg_tipo = self.tipo_atual()
            }


            // -------------------------------------------------------------------------------------------


            self.consumir_token();
        } else if self.match_token(Tipo_Token::ID) {
            // TODO: semantica aqui

            let lexema = self.tokens[self.token_atual].lexema();
            self.reg_tipo = self.symtab_lookup(lexema.clone());
            self.reg_val = lexema;

            self.consumir_token();
            self.id_opt();
        } else if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
            // TODO: semantica aqui?
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
        self.expr_list_opt();
    }

    fn expr_list_opt(&mut self) {
        if self.match_token(Tipo_Token::VIRGULA) {
            self.consumir_token();
            self.expr_list();
        }
    }
}
