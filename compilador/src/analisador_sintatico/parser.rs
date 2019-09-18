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

    fn erro(&self, token : &str) {
        println!(" {} esperado na linha {}", token, self.tokens[self.token_atual].linha());
        println!(" encontrado {}", self.tokens[self.token_atual].lexema());
        std::process::exit(1);
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
    fn stm(&mut self) {
        if self.match_token(Tipo_Token::IF) {
            if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.expr();
                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.if_opt();
                } else {
                    println!(" ) aesperado na linha {}", self.tokens[self.token_atual].linha());
                    println!(" encontrado {}", self.tokens[self.token_atual].lexema());
                    std::process::exit(1);
                }
            } else {
                println!(" ( esperado na linha {}", self.tokens[self.token_atual].linha());
                println!(" encontrado {}", self.tokens[self.token_atual].lexema());
                std::process::exit(1);
            }
        } else if self.match_token(Tipo_Token::WHILE) {
            if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.expr();
                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.then_stm();
                } else {
                    println!(" ) esperado na linha {}", self.tokens[self.token_atual].linha());
                    println!(" encontrado {}", self.tokens[self.token_atual].lexema());
                    std::process::exit(1);
                }
            } else {
                println!(" ( esperado na linha {}", self.tokens[self.token_atual].linha());
                println!(" encontrado {}", self.tokens[self.token_atual].lexema());
                std::process::exit(1);
            }
        } else if self.match_token(Tipo_Token::ID) {
            self.token_atual -= 1;
            self.var_decl();
        } else {
            self.normal_stm();
        }
    }
    fn if_opt(&mut self) {
        self.if_opt_opt();
    }
    fn if_opt_opt(&mut self) {
        if  self.match_token(Tipo_Token::IF) ||
            self.match_token(Tipo_Token::WHILE) ||
            self.match_token(Tipo_Token::CHAVE_ESQUERDA) ||
            self.match_token(Tipo_Token::BREAK) ||
            self.match_token(Tipo_Token::CONTINUE) ||
            self.match_token(Tipo_Token::RETURN) ||
            self.match_token(Tipo_Token::PRINTK) ||
            self.match_token(Tipo_Token::PONTO_VIRGULA) ||
            self.match_token(Tipo_Token::SIMBOLO_NOT) ||
            self.match_token(Tipo_Token::SIMBOLO_BIT_NOT) ||
            self.match_token(Tipo_Token::SIMBOLO_MENOS) ||
            self.match_token(Tipo_Token::OCTAL) ||
            self.match_token(Tipo_Token::HEX) ||
            self.match_token(Tipo_Token::INT) ||
            self.match_token(Tipo_Token::STR) ||
            self.match_token(Tipo_Token::CHAR) ||
            self.match_token(Tipo_Token::FLOAT) ||
            self.match_token(Tipo_Token::BOOL) ||
            self.match_token(Tipo_Token::ID) ||
            self.match_token(Tipo_Token::PARENTESE_ESQUERDO)

        {

            self.token_atual -= 1;
            self.then_stm();

            if self.match_token(Tipo_Token::ELSE) {
                self.stm();
            } else {
                println!(" else esperado na linha {}", self.tokens[self.token_atual].linha());
                println!(" encontrado {}", self.tokens[self.token_atual].lexema());
                std::process::exit(1);
            }

        } else {
            self.stm();
        }
    }
    fn then_stm(&mut self) {
        if self.match_token(Tipo_Token::IF) {
            if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.expr();
                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.then_stm();
                    if self.match_token(Tipo_Token::ELSE) {
                        self.then_stm();
                    } else {
                        self.erro("else");
                    }
                } else {
                    println!(" ) esperado na linha {}", self.tokens[self.token_atual].linha());
                    println!(" encontrado {}", self.tokens[self.token_atual].lexema());
                    std::process::exit(1);
                }
            } else {
                println!(" ( esperado na linha {}", self.tokens[self.token_atual].linha());
                println!(" encontrado {}", self.tokens[self.token_atual].lexema());
                std::process::exit(1);
            }

        } else if self.match_token(Tipo_Token::WHILE) {
            if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.expr();
                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.then_stm();
                } else { self.erro(")"); }
            } else { self.erro("("); }
        } else {
            self.normal_stm();
        }
    }
    fn normal_stm(&mut self) {
        if self.match_token(Tipo_Token::PONTO_VIRGULA) { return; }

        if self.match_token(Tipo_Token::PRINTK) {
            if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.op_or();
                if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    if !(self.match_token(Tipo_Token::PONTO_VIRGULA)) {
                        self.erro(";");
                    }
                } else {
                    self.erro(")");
                }
            } else {
                self.erro("(");
            }
        } else if self.match_token(Tipo_Token::RETURN) {
            self.expr();
            if !(self.match_token(Tipo_Token::PONTO_VIRGULA)) {
                self.erro(";");
            }

        } else if self.match_token(Tipo_Token::CONTINUE) {
            if !(self.match_token(Tipo_Token::PONTO_VIRGULA)) {
                self.erro(";");
            }
        } else if self.match_token(Tipo_Token::BREAK) {
            if !(self.match_token(Tipo_Token::PONTO_VIRGULA)) {
                self.erro(";");
            }
        } else if self.match_token(Tipo_Token::CHAVE_ESQUERDA) {
            self.token_atual -= 1;
            self.block();
        } else {
            self.expr();
            if !(self.match_token(Tipo_Token::PONTO_VIRGULA)) {
                self.erro(";");
            }
        }
    }
    fn block(&mut self) {
        if self.match_token(Tipo_Token::CHAVE_ESQUERDA) {
            self.stm_list();
            if !(self.match_token(Tipo_Token::CHAVE_DIREITA)) {
                self.erro("}");
            }
        } else {
            self.erro("{");
        }
    }
    fn stm_list(&mut self) {

        if self.match_token(Tipo_Token::IF) ||
           self.match_token(Tipo_Token::WHILE) ||
           self.match_token(Tipo_Token::CHAVE_ESQUERDA) ||
           self.match_token(Tipo_Token::BREAK) ||
           self.match_token(Tipo_Token::CONTINUE) ||
           self.match_token(Tipo_Token::RETURN) ||
           self.match_token(Tipo_Token::PRINTK) ||
           self.match_token(Tipo_Token::PONTO_VIRGULA) ||
           self.match_token(Tipo_Token::SIMBOLO_NOT) ||
           self.match_token(Tipo_Token::SIMBOLO_BIT_NOT) ||
           self.match_token(Tipo_Token::SIMBOLO_MENOS) ||
           self.match_token(Tipo_Token::OCTAL) ||
           self.match_token(Tipo_Token::HEX) ||
           self.match_token(Tipo_Token::INT) ||
           self.match_token(Tipo_Token::STR) ||
           self.match_token(Tipo_Token::CHAR) ||
           self.match_token(Tipo_Token::FLOAT) ||
           self.match_token(Tipo_Token::BOOL) ||
           self.match_token(Tipo_Token::ID) ||
           self.match_token(Tipo_Token::PARENTESE_ESQUERDO)

        {
            self.token_atual -= 1;
            self.stm();
            self.stm_list();

        }

    }

    ///////////////////////////////////////////////////////////////////////////
    fn expr(&mut self) {
        self.op_assign();
        self.expr_opt();

    }
    fn expr_opt(&mut self) {
        if self.match_token(Tipo_Token::VIRGULA) {
            self.token_atual -= 1;
            self.expr_rec();
        }
    }
    fn expr_rec(&mut self) {
        if self.match_token(Tipo_Token::VIRGULA) {
            self.op_assign();
            self.expr_opt();
        } else {
            self.erro(",");
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_assign(&mut self) {
        self.op_or();
        self.op_assign_opt_2();
    }
    fn op_assign_opt_2(&mut self) {
        if self.match_token(Tipo_Token::SIMBOLO_IGUAL) {
            self.token_atual -= 1;
            self.op_assign_opt();
        }
    }
    fn op_assign_opt(&mut self) {
        if self.match_token(Tipo_Token::SIMBOLO_IGUAL) {
            self.op_assign();
        } else {
            self.erro("=")
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_or(&mut self) {
        self.op_and();
        self.op_or_opt();
    }
    fn op_or_rec(&mut self) {
        self.op_or();
        if self.match_token(Tipo_Token::SIMBOLO_D_OR) {
            self.op_and();
            self.op_or_opt();
        } else {
            self.erro("||")
         }

    }
    fn op_or_opt(&mut self) {
        if
           self.match_token(Tipo_Token::SIMBOLO_NOT) ||
           self.match_token(Tipo_Token::SIMBOLO_BIT_NOT) ||
           self.match_token(Tipo_Token::SIMBOLO_MENOS) ||
           self.match_token(Tipo_Token::OCTAL) ||
           self.match_token(Tipo_Token::HEX) ||
           self.match_token(Tipo_Token::INT) ||
           self.match_token(Tipo_Token::STR) ||
           self.match_token(Tipo_Token::CHAR) ||
           self.match_token(Tipo_Token::FLOAT) ||
           self.match_token(Tipo_Token::BOOL) ||
           self.match_token(Tipo_Token::ID) ||
           self.match_token(Tipo_Token::PARENTESE_DIREITO)
        {
            self.token_atual -= 1;
            self.op_or_rec();
        }

    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_and(&mut self) {
        self.op_bin_or();
        self.op_and_opt();
    }
    fn op_and_rec(&mut self) {
        self.op_and();
        if self.match_token(Tipo_Token::SIMBOLO_D_AND) {
            self.op_bin_or();
            self.op_and_opt();
        } else {
            self.erro("&&");
        }
    }
    fn op_and_opt(&mut self) {
        if
           self.match_token(Tipo_Token::SIMBOLO_NOT) ||
           self.match_token(Tipo_Token::SIMBOLO_BIT_NOT) ||
           self.match_token(Tipo_Token::SIMBOLO_MENOS) ||
           self.match_token(Tipo_Token::OCTAL) ||
           self.match_token(Tipo_Token::HEX) ||
           self.match_token(Tipo_Token::INT) ||
           self.match_token(Tipo_Token::STR) ||
           self.match_token(Tipo_Token::CHAR) ||
           self.match_token(Tipo_Token::FLOAT) ||
           self.match_token(Tipo_Token::BOOL) ||
           self.match_token(Tipo_Token::ID) ||
           self.match_token(Tipo_Token::PARENTESE_DIREITO)
        {
            self.token_atual -= 1;
            self.op_and_rec();
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_bin_or(&mut self) {
        self.op_bin_and();
        self.op_bin_or_opt();
    }
    fn op_bin_or_opt(&mut self) {
        if
           self.match_token(Tipo_Token::SIMBOLO_NOT) ||
           self.match_token(Tipo_Token::SIMBOLO_BIT_NOT) ||
           self.match_token(Tipo_Token::SIMBOLO_MENOS) ||
           self.match_token(Tipo_Token::OCTAL) ||
           self.match_token(Tipo_Token::HEX) ||
           self.match_token(Tipo_Token::INT) ||
           self.match_token(Tipo_Token::STR) ||
           self.match_token(Tipo_Token::CHAR) ||
           self.match_token(Tipo_Token::FLOAT) ||
           self.match_token(Tipo_Token::BOOL) ||
           self.match_token(Tipo_Token::ID) ||
           self.match_token(Tipo_Token::PARENTESE_DIREITO)
        {
            self.token_atual -= 1;
            self.op_bin_or_rec();
        }
    }
    fn op_bin_or_rec(&mut self) {
        self.op_bin_or();

        if self.match_token(Tipo_Token::SIMBOLO_OR) {
            self.op_bin_and();
            self.op_bin_or_opt();
        } else {
            self.erro("|");
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_bin_and(&mut self) {
        self.op_equate();
        self.op_bin_and_opt();
    }
    fn op_bin_and_opt(&mut self) {
        if
           self.match_token(Tipo_Token::SIMBOLO_NOT) ||
           self.match_token(Tipo_Token::SIMBOLO_BIT_NOT) ||
           self.match_token(Tipo_Token::SIMBOLO_MENOS) ||
           self.match_token(Tipo_Token::OCTAL) ||
           self.match_token(Tipo_Token::HEX) ||
           self.match_token(Tipo_Token::INT) ||
           self.match_token(Tipo_Token::STR) ||
           self.match_token(Tipo_Token::CHAR) ||
           self.match_token(Tipo_Token::FLOAT) ||
           self.match_token(Tipo_Token::BOOL) ||
           self.match_token(Tipo_Token::ID) ||
           self.match_token(Tipo_Token::PARENTESE_DIREITO)
        {
            self.token_atual -= 1;
            self.op_bin_and_rec();
        }
    }
    fn op_bin_and_rec(&mut self) {
        self.op_bin_and();

        if self.match_token(Tipo_Token::SIMBOLO_AND) {
            self.op_equate();
            self.op_bin_and_opt();
        } else {
            self.erro("&");
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_equate(&mut self) {
        self.op_compare();
        self.op_equate_opt_2();
        println!("asdasdasdasdasdas{}", self.tokens[self.token_atual]);
    }
    fn op_equate_opt_2(&mut self) {
            if
           self.match_token(Tipo_Token::SIMBOLO_NOT) ||
           self.match_token(Tipo_Token::SIMBOLO_BIT_NOT) ||
           self.match_token(Tipo_Token::SIMBOLO_MENOS) ||
           self.match_token(Tipo_Token::OCTAL) ||
           self.match_token(Tipo_Token::HEX) ||
           self.match_token(Tipo_Token::INT) ||
           self.match_token(Tipo_Token::STR) ||
           self.match_token(Tipo_Token::CHAR) ||
           self.match_token(Tipo_Token::FLOAT) ||
           self.match_token(Tipo_Token::BOOL) ||
           self.match_token(Tipo_Token::ID) ||
           self.match_token(Tipo_Token::PARENTESE_DIREITO)
        {
            self.token_atual -= 1;
            self.op_equate_rec();
        } else {
            self.op_equate_opt();
            }
    }
    fn op_equate_rec(&mut self) {
        self.op_equate();
        self.op_equate_opt();
        self.op_equate_opt_2();
    }
    fn op_equate_opt(&mut self) {
        if self.match_token(Tipo_Token::SIMBOLO_D_IGUAL) ||
            self.match_token(Tipo_Token::SIMBOLO_D_DIFERENTE) {
                self.op_compare();
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_compare(&mut self) {
        self.op_add();
        self.op_compare_opt_2();
    }
    fn op_compare_opt_2(&mut self) {
        if
           self.match_token(Tipo_Token::SIMBOLO_NOT) ||
           self.match_token(Tipo_Token::SIMBOLO_BIT_NOT) ||
           self.match_token(Tipo_Token::SIMBOLO_MENOS) ||
           self.match_token(Tipo_Token::OCTAL) ||
           self.match_token(Tipo_Token::HEX) ||
           self.match_token(Tipo_Token::INT) ||
           self.match_token(Tipo_Token::STR) ||
           self.match_token(Tipo_Token::CHAR) ||
           self.match_token(Tipo_Token::FLOAT) ||
           self.match_token(Tipo_Token::BOOL) ||
           self.match_token(Tipo_Token::ID) ||
           self.match_token(Tipo_Token::PARENTESE_DIREITO)
        {
            self.token_atual -= 1;
            self.op_compare_rec();
        } else {
            self.op_compare_opt();
        }
    }
    fn op_compare_rec(&mut self) {
        self.op_compare();
        self.op_compare_opt();
        self.op_compare_opt_2();
    }
    fn op_compare_opt(&mut self) {
        if
            self.match_token(Tipo_Token::SIMBOLO_MENOR_Q) ||
            self.match_token(Tipo_Token::SIMBOLO_MAIOR_Q) ||
            self.match_token(Tipo_Token::SIMBOLO_MENOR_IGUAL_Q) ||
            self.match_token(Tipo_Token::SIMBOLO_MAIOR_IGUAL_Q) {
                self.op_add();
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_add(&mut self) {
        self.op_mult();
        self.op_add_opt_2();
    }
    fn op_add_opt_2(&mut self) {
        if
           self.match_token(Tipo_Token::SIMBOLO_NOT) ||
           self.match_token(Tipo_Token::SIMBOLO_BIT_NOT) ||
           self.match_token(Tipo_Token::SIMBOLO_MENOS) ||
           self.match_token(Tipo_Token::OCTAL) ||
           self.match_token(Tipo_Token::HEX) ||
           self.match_token(Tipo_Token::INT) ||
           self.match_token(Tipo_Token::STR) ||
           self.match_token(Tipo_Token::CHAR) ||
           self.match_token(Tipo_Token::FLOAT) ||
           self.match_token(Tipo_Token::BOOL) ||
           self.match_token(Tipo_Token::ID) ||
           self.match_token(Tipo_Token::PARENTESE_DIREITO)
        {
            self.token_atual -= 1;
            self.op_add_rec();
        } else {
            self.op_add_opt();
        }
    }
    fn op_add_rec(&mut self) {
        self.op_add();
        self.op_add_opt();
        self.op_add_opt_2();
    }
    fn op_add_opt(&mut self) {
        if self.match_token(Tipo_Token::SIMBOLO_MAIS) ||
                self.match_token(Tipo_Token::SIMBOLO_MENOS) {
                    self.op_mult();
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_mult(&mut self) {
        self.op_unary();
        self.op_mult_opt_2();
    }
    fn op_mult_rec(&mut self) {
        self.op_mult();
        self.op_mult_opt();
        self.op_mult_opt_2();
    }
    fn op_mult_opt(&mut self) {
        if self.match_token(Tipo_Token::SIMBOLO_MULTI) ||
            self.match_token(Tipo_Token::SIMBOLO_DIV) ||
                self.match_token(Tipo_Token::SIMBOLO_MOD) {
                    self.op_unary();
        }
    }
    fn op_mult_opt_2(&mut self) {
        if
           self.match_token(Tipo_Token::SIMBOLO_NOT) ||
           self.match_token(Tipo_Token::SIMBOLO_BIT_NOT) ||
           self.match_token(Tipo_Token::SIMBOLO_MENOS) ||
           self.match_token(Tipo_Token::OCTAL) ||
           self.match_token(Tipo_Token::HEX) ||
           self.match_token(Tipo_Token::INT) ||
           self.match_token(Tipo_Token::STR) ||
           self.match_token(Tipo_Token::CHAR) ||
           self.match_token(Tipo_Token::FLOAT) ||
           self.match_token(Tipo_Token::BOOL) ||
           self.match_token(Tipo_Token::ID) ||
           self.match_token(Tipo_Token::PARENTESE_DIREITO)
        {
            self.token_atual -= 1;
            self.op_mult_rec();
        } else {
            self.op_mult_opt();
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_unary(&mut self) {
        if self.match_token(Tipo_Token::SIMBOLO_NOT) ||
           self.match_token(Tipo_Token::SIMBOLO_BIT_NOT) ||
           self.match_token(Tipo_Token::SIMBOLO_MENOS) {
               self.op_unary();
           } else {
               self.value();
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn value(&mut self) {
        if
           self.match_token(Tipo_Token::OCTAL) ||
           self.match_token(Tipo_Token::HEX) ||
           self.match_token(Tipo_Token::INT) ||
           self.match_token(Tipo_Token::STR) ||
           self.match_token(Tipo_Token::CHAR) ||
           self.match_token(Tipo_Token::FLOAT) ||
           self.match_token(Tipo_Token::TRUE) ||
           self.match_token(Tipo_Token::FALSE) {}

        else if self.match_token(Tipo_Token::ID) {
            self.id_opt();
        }

        else if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
            self.expr();
            if !(self.match_token(Tipo_Token::PARENTESE_DIREITO)) {
                self.erro(")")
            }

        } else { self.erro("valor ou expressão"); }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn id_opt(&mut self) {
        if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
            self.id_opt_2();
        }
    }
    fn id_opt_2(&mut self) {
        if self.match_token(Tipo_Token::PARENTESE_DIREITO) {}

        else {
            self.expr();
            if !(self.match_token(Tipo_Token::PARENTESE_DIREITO)) {
                self.erro(")");
            }
        }
    }

}
