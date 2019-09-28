use crate::analisador_lexico::tipo_token::*;
use crate::analisador_lexico::token::*;

use crate::tabela_simbolos::simbolo::*;
use crate::tabela_simbolos::sym_tab::*;

pub struct Parser {
    tokens: Vec<Token>,
    token_atual: usize,
    tabela_de_simbolos: TabelaSimbolos,
    abre_escopo : bool,
    on_hold: Option<Simbolo>,
    indice_on_hold: usize,
    nome_funcao: String,
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
            nome_funcao: "Global".into()
        }
    }

    pub fn iniciar_analise(&mut self) {
        self.decls();
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.to_vec()
    }

    pub fn tabela_de_simbolos(&self) -> TabelaSimbolos {
        self.tabela_de_simbolos.clone()
    }

    fn match_token(&self, t: Tipo_Token) -> bool {
        self.tokens[self.token_atual].token() == t
    }

    fn consumir_token(&mut self) {
        self.token_atual += 1;
    }

    fn erro(&self, token: &str) {
        println!(
            " {} esperado na linha {}",
            token,
            self.tokens[self.token_atual - 1].linha()
        );
        println!(" encontrado {}", self.tokens[self.token_atual].lexema());
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
            Simbolo::Func(a, b, _c, d, e) => {
                s2 = Simbolo::Func(a, b, d.len(), d, e);
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
            Simbolo::Func(_n, a, b, c, _d) => {
                self.on_hold = Some(Simbolo::Func(token.lexema(), a, b, c, token.linha()))
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
            Simbolo::Func(n, _a, b, c, d) => {
                self.on_hold = Some(Simbolo::Func(n, tipo, b, c, d));
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
            Simbolo::Func(n, a, b, mut c, d) => {
                c.push(tipo);
                self.on_hold = Some(Simbolo::Func(n, a, b, c, d));
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
    ///////////////////////////////////////////////////////////////////////////

    fn decls(&mut self) {
        if self.token_atual >= self.tokens.len() {
            return;
        }
        if self.match_token(Tipo_Token::EOF) {
            return;
        }
        //println!("decls\n");
        if self.match_token(Tipo_Token::FUNC) || self.match_token(Tipo_Token::ID) {
            //println!("decls -> decl");
            self.decl();
            //println!("decls -> decls");
            self.decls();
        } else {
            self.erro("id ou func");
        }
    }

    fn decl(&mut self) {
        //println!("decl\n");
        if self.match_token(Tipo_Token::FUNC) {
            //println!("decl -> func_decl");
            self.func_decl();
        } else if self.match_token(Tipo_Token::ID) {
            //println!("decl -> var_decl");
            self.var_decl();
        } else {
            self.erro("tipo ou func");
        }
    }
    ///////////////////////////////////////////////////////////////////////////
    fn func_decl(&mut self) {
        //println!("func_decl");
        if self.match_token(Tipo_Token::FUNC) {

            self.abrir_escopo();
            self.abre_escopo = false;
            self.on_hold = Some(Simbolo::Func("".into(), Tipo_Token::VOID, 0, vec![], 0));

            self.consumir_token();
            if self.match_token(Tipo_Token::ID) {

                self.set_nome_on_hold();
                self.indice_on_hold = self.token_atual;

                self.consumir_token();
                if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.consumir_token();
                    //println!("func_decl -> func_params_opt");
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
        //println!("func_params_opt");
        if self.match_token(Tipo_Token::ID) {
            self.params();
            if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                self.consumir_token();
                if self.match_token(Tipo_Token::RETURNS) {
                    self.consumir_token();

                    self.set_tipo_on_hold();
                    self.add_simbolo();

                    //println!("func_params_opt -> t_type");
                    self.t_type();
                    //println!("func_params_opt -> block");
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

                //println!("func_params_opt -> t_type");
                self.t_type();
                //println!("func_params_opt -> block");
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
        //println!("params");
        //println!("params -> param");
        self.param();
        //println!("params -> param_opt");
        self.params_opt();
    }
    fn params_opt(&mut self) {
        //println!("params_opt");
        if self.match_token(Tipo_Token::VIRGULA) {
            self.consumir_token();
            //println!("params_opt -> params");
            self.params();
        }
    }
    fn param(&mut self) {
        //println!("param");
        if self.match_token(Tipo_Token::ID) {

            let id = self.tokens[self.token_atual].lexema();
            let alvo = self.token_atual;

            self.consumir_token();
            if self.match_token(Tipo_Token::AS) {
                self.consumir_token();
                //println!("param -> t_type");

                let s = Simbolo::Var(id, self.tokens[self.token_atual].token(), self.tokens[self.token_atual].linha(), self.nome_funcao.clone());
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
        //println!("var_decl");
        //println!("var_decl -> var");
        self.var();
        if self.match_token(Tipo_Token::PONTO_VIRGULA) {
            self.consumir_token();
        } else {
            self.erro(";");
        }
    }
    fn var(&mut self) {
        //println!("var");
        if self.match_token(Tipo_Token::ID) {

            let id = self.tokens[self.token_atual].lexema();
            let alvo = self.token_atual;

            self.consumir_token();
            if self.match_token(Tipo_Token::AS) {
                self.consumir_token();
                //println!("var -> t_type");

                let s = Simbolo::Var(id, self.tokens[self.token_atual].token(), self.tokens[self.token_atual].linha(), self.nome_funcao.clone());
                self.add_direto(s, alvo);

                self.t_type();
                //println!("var -> var_opt");
                self.var_opt();
            } else {
                self.erro("as");
            }
        } else {
            self.erro("indentificador");
        }
    }
    fn var_opt(&mut self) {
        //println!("var_opt");
        if self.match_token(Tipo_Token::SIMBOLO_IGUAL) {
            self.consumir_token();
            //println!("var_opt -> op_or");
            self.op_or();
        }
    }
    ///////////////////////////////////////////////////////////////////////////
    fn t_type(&mut self) {
        //println!("t_type");
        if self.id_tipo() {
            self.consumir_token();
        } else {
            self.erro("indentificador de tipo");
        }
    }
    ///////////////////////////////////////////////////////////////////////////
    fn stm(&mut self) {
        //println!("\n\nstm\n\n");

        if self.match_token(Tipo_Token::ID) && self.tokens[self.token_atual + 1].token() == Tipo_Token::AS { // diferenciar de uma expressão
            //println!("stm -> var_decl");
            self.var_decl();
        } else if self.match_token(Tipo_Token::IF) {
            self.consumir_token();
            if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.consumir_token();
                //println!("stm -> expr");
                self.expr();
                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.consumir_token();
                    //println!("stm -> then_stm");
                    self.then_stm();
                    //println!("stm -> if_opt");
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
                //println!("stm -> expr");
                self.expr();
                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.consumir_token();
                    //println!("stm -> then_stm");
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
            //println!("stm -> normal_stm");
            self.normal_stm();
        } else {
            self.erro("stm")
        }
    }
    fn then_stm(&mut self) {
        //println!("then_stm");
        if self.match_token(Tipo_Token::IF) {
            self.consumir_token();
            if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.consumir_token();
                //println!("then_stm -> expr");
                self.expr();
                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.consumir_token();
                    //println!("then_stm -> then_stm");
                    self.then_stm();
                    //println!("then_stm -> if_opt");
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
                //println!("then_stm -> expr");
                self.expr();
                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.consumir_token();
                    //println!("then_stm -> then_stm");
                    self.then_stm();
                } else {
                    self.erro("h)");
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
            //println!("then_stm -> normal_stm");
            self.normal_stm();
        } else {
            self.erro("muita coisa de novo no then_stm não");
        }
    }

    fn if_opt(&mut self) {
        if self.match_token(Tipo_Token::ELSE) {
            self.consumir_token();
            //println!("if_opt -> then_stm");
            self.then_stm();
        }
    }

    fn normal_stm(&mut self) {
        //println!("normal_stm");

        if self.match_token(Tipo_Token::CHAVE_ESQUERDA) {
            //println!("normal_stm -> block");
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
            //println!("normal_stm -> expr");
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
                //println!("normal_stm -> op_or");
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
            //println!("normal_stm -> expr");
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
        //println!("block");
        if self.match_token(Tipo_Token::CHAVE_ESQUERDA) {
            self.consumir_token();

            if self.abre_escopo {
                self.abrir_escopo();
            } else {
                self.abre_escopo = true;
            }


            //println!("block -> stm_list");
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
        //println!("stm_list");
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
            //println!("stm -> stm_list");
            self.stm();
            //println!("stm_list -> stm_list");
            self.stm_list();
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn expr(&mut self) {
        //println!("expr");
        //println!("expr -> or_or");
        self.op_or();
    }
    ///////////////////////////////////////////////////////////////////////////
    fn op_or(&mut self) {
        //println!("op_or");
        //println!("or_or -> op_and");
        self.op_and();
        //println!("or_or -> op_or_opt");
        self.op_or_opt();
    }
    fn op_or_opt(&mut self) {
        //println!("op_or_opt");
        if self.match_token(Tipo_Token::SIMBOLO_D_OR) {
            self.consumir_token();
            //println!("op_or_opt -> expr");
            self.expr();
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_and(&mut self) {
        //println!("op_and");
        //println!("or_and -> op_bin_or");
        self.op_bin_or();
        //println!("or_and -> op_and_opt");
        self.op_and_opt();
    }
    fn op_and_opt(&mut self) {
        //println!("op_and_opt");
        if self.match_token(Tipo_Token::SIMBOLO_D_AND) {
            self.consumir_token();
            //println!("op_and_opt -> expr");
            self.expr();
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_bin_or(&mut self) {
        //println!("op_bin_or");
        //println!("op_bin_or -> op_bin_and");
        self.op_bin_and();
        //println!("op_bin_or -> op_bin_or_opt");
        self.op_bin_or_opt();
    }
    fn op_bin_or_opt(&mut self) {
        //println!("op_bin_or_opt");
        if self.match_token(Tipo_Token::SIMBOLO_OR) {
            self.consumir_token();
            //println!("op_bin_or_opt -> expr");
            self.expr();
        }
    }
    ///////////////////////////////////////////////////////////////////////////
    fn op_bin_and(&mut self) {
        //println!("op_bin_and");
        //println!("op_bin_and -> op_equate");
        self.op_equate();
        //println!("op_bin_and -> op_bin_and_opt");
        self.op_bin_and_opt();
    }
    fn op_bin_and_opt(&mut self) {
        //println!("op_bin_and_opt");
        if self.match_token(Tipo_Token::SIMBOLO_AND) {
            self.consumir_token();
            //println!("op_bin_and_opt -> expr");
            self.expr();
        }
    }
    ///////////////////////////////////////////////////////////////////////////
    fn op_equate(&mut self) {
        //println!("op_equate");
        //println!("op_equate -> op_compare");
        self.op_compare();
        //println!("op_equate -> op_compare_opt");
        self.op_equate_opt();
    }
    fn op_equate_opt(&mut self) {
        //println!("op_equate_opt");
        if self.match_token(Tipo_Token::SIMBOLO_D_IGUAL) {
            self.consumir_token();
            //println!("op_equate_opt -> expr");
            self.expr();
        } else if self.match_token(Tipo_Token::SIMBOLO_D_DIFERENTE) {
            self.consumir_token();
            self.expr();
            //println!("op_equate_opt -> expr");
        } else {
            return;
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_compare(&mut self) {
        //println!("op_compare");
        //println!("op_compare -> op_add");
        self.op_add();
        //println!("op_compare -> op_compare_opt");
        self.op_compare_opt();
    }
    fn op_compare_opt(&mut self) {
        //println!("op_compare_opt");
        if self.match_token(Tipo_Token::SIMBOLO_MENOR_Q)
            || self.match_token(Tipo_Token::SIMBOLO_MAIOR_Q)
            || self.match_token(Tipo_Token::SIMBOLO_MAIOR_IGUAL_Q)
            || self.match_token(Tipo_Token::SIMBOLO_MENOR_IGUAL_Q)
        {
            self.consumir_token();
            //println!("op_compare_opt -> expr");
            self.expr();
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_add(&mut self) {
        //println!("op_add");
        //println!("op_add -> op_mult");
        self.op_mult();
        //println!("op_add -> op_add_opt");
        self.op_add_opt();
    }
    fn op_add_opt(&mut self) {
        //println!("op_add_opt");
        if self.match_token(Tipo_Token::SIMBOLO_MAIS) || self.match_token(Tipo_Token::SIMBOLO_MENOS)
        {
            self.consumir_token();
            //println!("op_add_opt -> expr");
            self.expr();
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_mult(&mut self) {
        //println!("op_mult");
        //println!("op_mult -> op_unary");
        self.op_unary();
        //println!("op_mult -> op_mult_opt");
        self.op_mult_opt();
    }
    fn op_mult_opt(&mut self) {
        //println!("op_mult_opt");
        if self.match_token(Tipo_Token::SIMBOLO_MULTI)
            || self.match_token(Tipo_Token::SIMBOLO_DIV)
            || self.match_token(Tipo_Token::SIMBOLO_MOD)
        {
            self.consumir_token();
            //println!("op_mult_opt -> expr");
            self.expr();
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_unary(&mut self) {
        //println!("op_unary");
        if self.e_unaria() {
            self.consumir_token();
            //println!("op_unary -> expr");
            self.expr();
        } else {
            //println!("op_unary -> value");
            self.value();
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn value(&mut self) {
        //println!("value");
        if self.match_token(Tipo_Token::OCTAL)
            || self.match_token(Tipo_Token::HEX)
            || self.match_token(Tipo_Token::INT)
            || self.match_token(Tipo_Token::STR)
            || self.match_token(Tipo_Token::CHAR)
            || self.match_token(Tipo_Token::FLOAT)
            || self.match_token(Tipo_Token::TRUE)
            || self.match_token(Tipo_Token::FALSE)
        {
            self.consumir_token();
        } else if self.match_token(Tipo_Token::ID) {
            self.consumir_token();
            //println!("value -> id_opt");
            self.id_opt();
        } else if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
            self.consumir_token();
            //println!("value -> expr");
            self.expr();
            if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                self.consumir_token();
            } else {
                self.erro("f)");
            }
        } else {
            self.erro("(");
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn id_opt(&mut self) {
        //println!("id_opt");
        if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
            self.consumir_token();
            //println!("id_opt -> id_opt");
            self.id_opt_2();
        }
    }
    fn id_opt_2(&mut self) {
        //println!("id_opt_2");
        if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
            self.consumir_token();
        } else {
            //println!("id_opt_2 -> expr_list");
            self.expr_list();
            if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                self.consumir_token();
            }
        }
    }

    fn expr_list(&mut self) {
        //println!("expr_lit -> expr");
        self.expr();
        //println!("expr_lit -> expr_list_opt");
        self.expr_list_opt();
    }

    fn expr_list_opt(&mut self) {
        if self.match_token(Tipo_Token::VIRGULA) {
            self.consumir_token();
            //println!("expr_list_opt -> expr_list");
            self.expr_list();
        }
    }
}
