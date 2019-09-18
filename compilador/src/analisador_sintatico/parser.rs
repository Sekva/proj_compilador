use crate::analisador_lexico::token::*;
use crate::analisador_lexico::tipo_token::*;

pub struct Parser {
    tokens : Vec<Token>,
    token_atual : usize,
}


impl Parser {

    pub fn novo(e_tokens : Vec<Token>) -> Parser {
        Parser {
            tokens : e_tokens,
            token_atual : 0,
        }
    }

    pub fn iniciar_analise(&mut self) {
        self.decls();
    }

    fn match_token (&mut self, t : Tipo_Token) -> bool {
        let teste = self.tokens[self.token_atual].token() == t;
        if teste { self.token_atual += 1; }
        return teste;
    }
    ///////////////////////////////////////////////////////////////////////////

    fn decls(&mut self) {
        if self.token_atual >= self.tokens.len() { return; }

        if(self.tokens[self.token_atual].token() == Tipo_Token::FUNC) ||
          (self.tokens[self.token_atual].token() == Tipo_Token::ID) {
            self.decl();
            self.decls();
        }

    }

    fn decl(&mut self) {

        match self.tokens[self.token_atual].token() {
            Tipo_Token::FUNC => { self.func_decl() },
            Tipo_Token::ID => { self.var_decl() },
            _ => {
                println!("Esperado declaração de função ou de variavel na
                    linha {}", self.tokens[self.token_atual].linha());
                std::process::exit(1);
            }
        }

    }
    ///////////////////////////////////////////////////////////////////////////
    fn func_decl(&mut self) {
        if self.tokens[self.token_atual].token() == Tipo_Token::FUNC {
            self.token_atual += 1;
            if self.tokens[self.token_atual].token() == Tipo_Token::ID {
                self.token_atual += 1;
                if self.tokens[self.token_atual].token() == Tipo_Token::PARENTESE_ESQUERDO {
                    self.token_atual += 1;
                    self.func_params_opt();
                }
            }
        } else {
            println!("Esparado declaração de função na linha {}", self.tokens[self.token_atual].linha());
            std::process::exit(1);
        }
    }
    fn func_params_opt(&mut self) {
        if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
            if self.match_token(Tipo_Token::RETURNS) {
                self.t_type();
                self.block();
            } else {
                println!(" returns esperado na linha {}", self.tokens[self.token_atual].linha());
                std::process::exit(1);
            }
        } else if self.match_token(Tipo_Token::ID) {
            self.token_atual -= 1;
            self.params();
            if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                if self.match_token(Tipo_Token::RETURNS) {
                    self.t_type();
                    self.block();
                } else {
                    println!(" returns esperado na linha {}", self.tokens[self.token_atual].linha());
                    std::process::exit(1);
                }
            } else {
                println!(" ) esperado na linha {}", self.tokens[self.token_atual].linha());
                std::process::exit(1);
            }
        } else {
            println!("Esparado declaração de parametros da função na linha {}", self.tokens[self.token_atual].linha());
            std::process::exit(1);
        }
    }
    fn params(&mut self) {
        self.param();
        self.params_opt();
    }
    fn params_opt(&mut self) {
        if self.match_token(Tipo_Token::VIRGULA) {
            self.params();
        }
    }
    fn param(&mut self) {
        if self.match_token(Tipo_Token::ID) {
            if self.match_token(Tipo_Token::AS) {
                self.t_type();
            } else {
                println!(" as esperado na linha {}", self.tokens[self.token_atual].linha());
                std::process::exit(1);
            }
        }
    }
    ///////////////////////////////////////////////////////////////////////////
    fn var_decl(&mut self) {
        self.var();

        if !self.match_token(Tipo_Token::PONTO_VIRGULA) {
            println!(" ; esperado na linha {}", self.tokens[self.token_atual].linha());
            std::process::exit(1);
        }
    }
    fn var(&mut self) {
        if self.match_token(Tipo_Token::ID) {
            if self.match_token(Tipo_Token::AS) {
                self.t_type();
                self.var_opt();
            } else {
                println!(" as esperado na linha {}", self.tokens[self.token_atual].linha());
                std::process::exit(1);
            }
        } else {
            println!(" identificador esperado na linha {}", self.tokens[self.token_atual].linha());
            std::process::exit(1);
        }
    }
    fn var_opt(&mut self) {
        if self.match_token(Tipo_Token::SIMBOLO_IGUAL) {
            self.op_or();
        }
    }
    ///////////////////////////////////////////////////////////////////////////
    fn t_type(&mut self) {
        if !(self.match_token(Tipo_Token::ID_CHAR) ||
             self.match_token(Tipo_Token::ID_STR)  ||
             self.match_token(Tipo_Token::ID_INT)  ||
             self.match_token(Tipo_Token::ID_FLOAT)||
             self.match_token(Tipo_Token::ID_VOID) ||
             self.match_token(Tipo_Token::ID_BOOL)) {
                println!(" identificador de tipo esperado na linha {}", self.tokens[self.token_atual].linha());
                println!(" encontrado {}", self.tokens[self.token_atual].lexema());
                std::process::exit(1);
        }
    }
    ///////////////////////////////////////////////////////////////////////////
    fn stm(&mut self) {panic!();}
    fn if_opt(&mut self) {panic!();}
    fn if_opt_opt(&mut self) {panic!();}
    fn then_stm(&mut self) {panic!();}
    fn normal_stm(&mut self) {panic!();}
    fn block(&mut self) {panic!();}
    fn stm_list(&mut self) {panic!();}

    ///////////////////////////////////////////////////////////////////////////
    fn expr(&mut self) {panic!();}
    fn expr_rec(&mut self) {panic!();}
    fn expr_opt(&mut self) {panic!();}

    ///////////////////////////////////////////////////////////////////////////
    fn op_assign(&mut self) {panic!();}
    fn op_assign_opt_2(&mut self) {panic!();}
    fn op_assign_opt(&mut self) {panic!();}

    ///////////////////////////////////////////////////////////////////////////
    fn op_or(&mut self) {panic!();}
    fn op_or_rec(&mut self) {panic!();}
    fn op_or_opt(&mut self) {panic!();}

    ///////////////////////////////////////////////////////////////////////////
    fn op_and(&mut self) {panic!();}
    fn op_and_rec(&mut self) {panic!();}
    fn op_and_opt(&mut self) {panic!();}

    ///////////////////////////////////////////////////////////////////////////
    fn op_bit_or(&mut self) {panic!();}
    fn op_bit_or_opt(&mut self) {panic!();}
    fn op_bin_or_rec(&mut self) {panic!();}

    ///////////////////////////////////////////////////////////////////////////
    fn op_bin_and(&mut self) {panic!();}
    fn op_bin_and_opt(&mut self) {panic!();}
    fn op_bin_and_rec(&mut self) {panic!();}

    ///////////////////////////////////////////////////////////////////////////
    fn op_equate(&mut self) {panic!();}
    fn op_equate_opt_2(&mut self) {panic!();}
    fn op_equate_rec(&mut self) {panic!();}
    fn op_equate_opt(&mut self) {panic!();}

    ///////////////////////////////////////////////////////////////////////////
    fn op_compare(&mut self) {panic!();}
    fn op_compare_opt_2(&mut self) {panic!();}
    fn op_compare_rec(&mut self) {panic!();}
    fn op_compare_opt(&mut self) {panic!();}

    ///////////////////////////////////////////////////////////////////////////
    fn op_add(&mut self) {panic!();}
    fn op_add_opt_2(&mut self) {panic!();}
    fn op_add_rec(&mut self) {panic!();}
    fn op_add_opt(&mut self) {panic!();}

    ///////////////////////////////////////////////////////////////////////////
    fn op_mult(&mut self) {panic!();}
    fn op_mult_rec(&mut self) {panic!();}
    fn op_mult_opt(&mut self) {panic!();}
    fn op_mult_opt_2(&mut self) {panic!();}

    ///////////////////////////////////////////////////////////////////////////
    fn op_unary(&mut self) {panic!();}

    ///////////////////////////////////////////////////////////////////////////
    fn value(&mut self) {panic!();}

    ///////////////////////////////////////////////////////////////////////////
    fn id_opt(&mut self) {panic!();}
    fn id_opt_2(&mut self) {panic!();}

}
