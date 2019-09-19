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

    fn match_token (&self, t : Tipo_Token) -> bool {
        self.tokens[self.token_atual].token() == t
    }

    fn consumir_token(&mut self) {
        self.token_atual += 1;
    }

    fn erro(&self, token : &str) {
        println!(" {} esperado na linha {}", token, self.tokens[self.token_atual].linha());
        println!(" encontrado {}", self.tokens[self.token_atual].lexema());
        std::process::exit(1);
    }
    ///////////////////////////////////////////////////////////////////////////

    fn decls(&mut self) {
        if self.token_atual >= self.tokens.len() { return; }
        println!("decls\n");
        if self.match_token(Tipo_Token::FUNC) || self.match_token(Tipo_Token::ID) {
            self.decl();
            self.decls();
        }

    }

    fn decl(&mut self) {

        println!("decl\n");
        if self.match_token(Tipo_Token::FUNC) {
            self.func_decl();
        } else if self.match_token(Tipo_Token::ID) {
            self.var_decl();
        } else {
            self.erro("id ou func");
        }


    }
    ///////////////////////////////////////////////////////////////////////////
    fn func_decl(&mut self) {
        println!("func_decl");
        if self.match_token(Tipo_Token::FUNC) {
            self.consumir_token();
            if self.match_token(Tipo_Token::ID) {
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
        println!("func_params_opt");
        if self.match_token(Tipo_Token::ID) {
            self.params();
            if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                self.consumir_token();
                if self.match_token(Tipo_Token::RETURNS) {
                    self.consumir_token();
                    self.t_type();
                    self.block();
                } else {
                    self.erro("returns");
                }
            }
        } else if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
            self.consumir_token();
            if self.match_token(Tipo_Token::RETURNS) {
                self.consumir_token();
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
        println!("params");
        if self.match_token(Tipo_Token::ID) {
            self.param();
            self.params_opt();
        } else {
            self.erro("indentificador");
        }
    }
    fn params_opt(&mut self) {
        println!("params_opt");
        if self.match_token(Tipo_Token::VIRGULA) {
            self.consumir_token();
            self.params();
        }
    }
    fn param(&mut self) {
        println!("param");
        if self.match_token(Tipo_Token::ID) {
            self.consumir_token();
            if self.match_token(Tipo_Token::AS) {
                self.consumir_token();
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
        println!("var_decl");
        if self.match_token(Tipo_Token::ID) {
            self.var();
            if self.match_token(Tipo_Token::PONTO_VIRGULA) {
                self.consumir_token();
            } else {
                self.erro(";");
            }
        } else {
            self.erro("indentificador");
        }
    }
    fn var(&mut self) {
        println!("var");
        if self.match_token(Tipo_Token::ID) {
            self.consumir_token();
            if self.match_token(Tipo_Token::AS) {
                self.consumir_token();
                self.t_type();
                self.var_opt();
            } else {
                self.erro("as");
            }
        } else {
            self.erro("indentificador");
        }
    }
    fn var_opt(&mut self) {
        println!("var_opt");
        if self.match_token(Tipo_Token::SIMBOLO_IGUAL) {
            self.consumir_token();
            self.op_or();
        }
    }
    ///////////////////////////////////////////////////////////////////////////
    fn t_type(&mut self) {
        println!("t_type");
        if self.match_token(Tipo_Token::ID_CHAR)
            || self.match_token(Tipo_Token::ID_STR)
                || self.match_token(Tipo_Token::ID_INT)
                || self.match_token(Tipo_Token::ID_FLOAT)
                || self.match_token(Tipo_Token::ID_VOID)
                || self.match_token(Tipo_Token::ID_BOOL) {
                    self.consumir_token();
                } else {
                    self.erro("indentificador de tipo");
        }
    }
    ///////////////////////////////////////////////////////////////////////////
    fn stm(&mut self) {
        println!("stm");

        if self.match_token(Tipo_Token::IF) {
            self.consumir_token();
            if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.consumir_token();
                self.expr();
                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.consumir_token();
                    self.if_opt();
                } else { self.erro(")"); }
            } else { self.erro("("); }
        }


        else if self.match_token(Tipo_Token::WHILE) {
            self.consumir_token();
            if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.consumir_token();
                self.expr();
                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.consumir_token();
                    self.then_stm();
                } else { self.erro(")"); }
            } else { self.erro("("); }
        }


        else if self.match_token(Tipo_Token::ID) {
            self.var_decl();
        }


        else if self.match_token(Tipo_Token::BREAK)
            || self.match_token(Tipo_Token::CONTINUE)
            || self.match_token(Tipo_Token::RETURN)
            || self.match_token(Tipo_Token::PRINTK)
            || self.match_token(Tipo_Token::PONTO_VIRGULA)
            || self.match_token(Tipo_Token::CHAVE_ESQUERDA)
            || self.match_token(Tipo_Token::SIMBOLO_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_MENOS)
            || self.match_token(Tipo_Token::OCTAL)
            || self.match_token(Tipo_Token::HEX)
            || self.match_token(Tipo_Token::INT)
            || self.match_token(Tipo_Token::STR)
            || self.match_token(Tipo_Token::CHAR)
            || self.match_token(Tipo_Token::FLOAT)
            || self.match_token(Tipo_Token::TRUE)
            || self.match_token(Tipo_Token::FALSE)
            || self.match_token(Tipo_Token::ID)
            || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.normal_stm();
        }
    }
    fn if_opt(&mut self) {
        println!("if_opt");

        // first de stm igual ao de then_stm
        // vamos direto ao then_stm que facilita
        // para declarar variaveis tem estar entre {}


        if self.match_token(Tipo_Token::IF)
            || self.match_token(Tipo_Token::WHILE)
            || self.match_token(Tipo_Token::BREAK)
            || self.match_token(Tipo_Token::CONTINUE)
            || self.match_token(Tipo_Token::RETURN)
            || self.match_token(Tipo_Token::PRINTK)
            || self.match_token(Tipo_Token::PONTO_VIRGULA)
            || self.match_token(Tipo_Token::CHAVE_ESQUERDA)
            || self.match_token(Tipo_Token::SIMBOLO_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_MENOS)
            || self.match_token(Tipo_Token::OCTAL)
            || self.match_token(Tipo_Token::HEX)
            || self.match_token(Tipo_Token::INT)
            || self.match_token(Tipo_Token::STR)
            || self.match_token(Tipo_Token::CHAR)
            || self.match_token(Tipo_Token::FLOAT)
            || self.match_token(Tipo_Token::TRUE)
            || self.match_token(Tipo_Token::FALSE)
            || self.match_token(Tipo_Token::ID)
            || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.then_stm();
                if self.match_token(Tipo_Token::ELSE) {
                    self.consumir_token();
                    self.then_stm();
                }
        } else { self.erro("após if ( expr ), algo não"); }
    }
    fn then_stm(&mut self) {
        println!("then_stm");
        if self.match_token(Tipo_Token::IF) {
            self.consumir_token();
            if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.consumir_token();
                self.expr();
                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.consumir_token();
                    self.then_stm();
                    if self.match_token(Tipo_Token::ELSE) {
                        self.consumir_token();
                        self.then_stm();
                    } else {
                        self.erro("else");
                    }
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
                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.consumir_token();
                    self.then_stm();
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
                || self.match_token(Tipo_Token::SIMBOLO_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_MENOS)
                || self.match_token(Tipo_Token::OCTAL)
                || self.match_token(Tipo_Token::HEX)
                || self.match_token(Tipo_Token::INT)
                || self.match_token(Tipo_Token::STR)
                || self.match_token(Tipo_Token::CHAR)
                || self.match_token(Tipo_Token::FLOAT)
                || self.match_token(Tipo_Token::TRUE)
                || self.match_token(Tipo_Token::FALSE)
                || self.match_token(Tipo_Token::ID)
                || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
            self.normal_stm();
        }

        else {
            self.erro("muita coisa de novo no then_stm não");
        }
    }
    fn normal_stm(&mut self) {
        println!("normal_stm");


        if self.match_token(Tipo_Token::CHAVE_ESQUERDA) {
            self.block();
        }

        else if self.match_token(Tipo_Token::BREAK) {
            self.consumir_token();
            if self.match_token(Tipo_Token::PONTO_VIRGULA) {
                self.consumir_token();
            } else { self.erro(";"); }
        }

        else if self.match_token(Tipo_Token::CONTINUE) {
            self.consumir_token();
            if self.match_token(Tipo_Token::PONTO_VIRGULA) {
                self.consumir_token();
            } else { self.erro(";"); }
        }

        else if self.match_token(Tipo_Token::PONTO_VIRGULA) {
            self.consumir_token();
        }


        else if self.match_token(Tipo_Token::RETURN) {
            self.consumir_token();
            self.expr();
            if self.match_token(Tipo_Token::PONTO_VIRGULA) {
                self.consumir_token();
            } else { self.erro(";"); }
        }

        else if self.match_token(Tipo_Token::PRINTK) {
            self.consumir_token();
            if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.consumir_token();
                self.op_or();
                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.consumir_token();
                    if self.match_token(Tipo_Token::PONTO_VIRGULA) {
                        self.consumir_token();
                    } else { self.erro(";"); }
                } else { self.erro(")"); }
            } else { self.erro("("); }

        }




        else if self.match_token(Tipo_Token::SIMBOLO_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_MENOS)
            || self.match_token(Tipo_Token::OCTAL)
            || self.match_token(Tipo_Token::HEX)
            || self.match_token(Tipo_Token::INT)
            || self.match_token(Tipo_Token::STR)
            || self.match_token(Tipo_Token::CHAR)
            || self.match_token(Tipo_Token::FLOAT)
            || self.match_token(Tipo_Token::TRUE)
            || self.match_token(Tipo_Token::FALSE)
            || self.match_token(Tipo_Token::ID)
            || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.expr();
                if self.match_token(Tipo_Token::PONTO_VIRGULA) {
                    self.consumir_token();
                } else { self.erro(";"); }
        }




        else { self.erro("normal_stm"); }

    }
    fn block(&mut self) {
        println!("block");
        if self.match_token(Tipo_Token::CHAVE_ESQUERDA) {
            self.consumir_token();
            self.stm_list();
            if self.match_token(Tipo_Token::CHAVE_DIREITA) {
                self.consumir_token();
            } else {
                self.erro("}");
            }
        } else {
            self.erro("{");
        }
    }
    fn stm_list(&mut self) {
        println!("stm_list");
        if self.match_token(Tipo_Token::IF)
            || self.match_token(Tipo_Token::WHILE)
            || self.match_token(Tipo_Token::ID)
            || self.match_token(Tipo_Token::BREAK)
            || self.match_token(Tipo_Token::CONTINUE)
            || self.match_token(Tipo_Token::RETURN)
            || self.match_token(Tipo_Token::PRINTK)
            || self.match_token(Tipo_Token::PONTO_VIRGULA)
            || self.match_token(Tipo_Token::CHAVE_ESQUERDA)
            || self.match_token(Tipo_Token::SIMBOLO_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_MENOS)
            || self.match_token(Tipo_Token::OCTAL)
            || self.match_token(Tipo_Token::HEX)
            || self.match_token(Tipo_Token::INT)
            || self.match_token(Tipo_Token::STR)
            || self.match_token(Tipo_Token::CHAR)
            || self.match_token(Tipo_Token::FLOAT)
            || self.match_token(Tipo_Token::TRUE)
            || self.match_token(Tipo_Token::FALSE)
            || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
            self.stm();
            self.stm_list();
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn expr(&mut self) {
        println!("expr");
        if self.match_token(Tipo_Token::ID)
            || self.match_token(Tipo_Token::SIMBOLO_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_MENOS)
            || self.match_token(Tipo_Token::OCTAL)
            || self.match_token(Tipo_Token::HEX)
            || self.match_token(Tipo_Token::INT)
            || self.match_token(Tipo_Token::STR)
            || self.match_token(Tipo_Token::CHAR)
            || self.match_token(Tipo_Token::FLOAT)
            || self.match_token(Tipo_Token::TRUE)
            || self.match_token(Tipo_Token::FALSE)
            || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.op_assign();
                self.expr_opt();
        } else {
            self.erro("expressão");
        }
    }
    fn expr_opt(&mut self) {
        println!("expr_opt");
        if self.match_token(Tipo_Token::VIRGULA) {
            self.expr_rec();
        }
    }
    fn expr_rec(&mut self) {
        println!("expr_rec");
        if self.match_token(Tipo_Token::VIRGULA) {
            self.consumir_token();
            self.op_assign();
            self.expr_opt();
        } else {
            self.erro(",");
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_assign(&mut self) {
        println!("op_assign");
        if self.match_token(Tipo_Token::ID)
            || self.match_token(Tipo_Token::SIMBOLO_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_MENOS)
            || self.match_token(Tipo_Token::OCTAL)
            || self.match_token(Tipo_Token::HEX)
            || self.match_token(Tipo_Token::INT)
            || self.match_token(Tipo_Token::STR)
            || self.match_token(Tipo_Token::CHAR)
            || self.match_token(Tipo_Token::FLOAT)
            || self.match_token(Tipo_Token::TRUE)
            || self.match_token(Tipo_Token::FALSE)
            || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.op_or();
                self.op_assign_opt_2();
            } else {
                self.erro("expressão de atribuição");
            }
    }
    fn op_assign_opt_2(&mut self) {
        println!("op_assign_opt_2");

        if self.match_token(Tipo_Token::SIMBOLO_IGUAL) {
            self.op_assign_opt();
        }

    }
    fn op_assign_opt(&mut self) {
        println!("op_assign_opt");
        if self.match_token(Tipo_Token::SIMBOLO_IGUAL) {
            self.consumir_token();
            self.op_assign();
        } else {
            self.erro("=");
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_or(&mut self) {
        println!("op_or");
        if self.match_token(Tipo_Token::ID)
            || self.match_token(Tipo_Token::SIMBOLO_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_MENOS)
            || self.match_token(Tipo_Token::OCTAL)
            || self.match_token(Tipo_Token::HEX)
            || self.match_token(Tipo_Token::INT)
            || self.match_token(Tipo_Token::STR)
            || self.match_token(Tipo_Token::CHAR)
            || self.match_token(Tipo_Token::FLOAT)
            || self.match_token(Tipo_Token::TRUE)
            || self.match_token(Tipo_Token::FALSE)
            || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.op_and();
                self.op_or_opt();
            } else {
                self.erro("expressão de or");
            }
    }
    fn op_or_rec(&mut self) {
        println!("op_or_rec");
        if self.match_token(Tipo_Token::ID)
            || self.match_token(Tipo_Token::SIMBOLO_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_MENOS)
            || self.match_token(Tipo_Token::OCTAL)
            || self.match_token(Tipo_Token::HEX)
            || self.match_token(Tipo_Token::INT)
            || self.match_token(Tipo_Token::STR)
            || self.match_token(Tipo_Token::CHAR)
            || self.match_token(Tipo_Token::FLOAT)
            || self.match_token(Tipo_Token::TRUE)
            || self.match_token(Tipo_Token::FALSE)
            || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.op_or();
                if self.match_token(Tipo_Token::SIMBOLO_D_OR) {
                    self.consumir_token();
                    self.op_and();
                    self.op_or_opt();
                } else {
                    self.erro("||");
                }
            } else {
                self.erro("expressão de or rec");
            }
    }
    fn op_or_opt(&mut self) {
        println!("op_or_opt");
        if self.match_token(Tipo_Token::ID)
            || self.match_token(Tipo_Token::SIMBOLO_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_MENOS)
            || self.match_token(Tipo_Token::OCTAL)
            || self.match_token(Tipo_Token::HEX)
            || self.match_token(Tipo_Token::INT)
            || self.match_token(Tipo_Token::STR)
            || self.match_token(Tipo_Token::CHAR)
            || self.match_token(Tipo_Token::FLOAT)
            || self.match_token(Tipo_Token::TRUE)
            || self.match_token(Tipo_Token::FALSE)
            || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.op_or_rec();
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_and(&mut self) {
        println!("op_and");
        if self.match_token(Tipo_Token::ID)
            || self.match_token(Tipo_Token::SIMBOLO_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_MENOS)
            || self.match_token(Tipo_Token::OCTAL)
            || self.match_token(Tipo_Token::HEX)
            || self.match_token(Tipo_Token::INT)
            || self.match_token(Tipo_Token::STR)
            || self.match_token(Tipo_Token::CHAR)
            || self.match_token(Tipo_Token::FLOAT)
            || self.match_token(Tipo_Token::TRUE)
            || self.match_token(Tipo_Token::FALSE)
            || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.op_bin_or();
                self.op_and_opt();
            } else {
                self.erro("expressão de and");
            }
    }
    fn op_and_rec(&mut self) {
        println!("op_and_rec");
        if self.match_token(Tipo_Token::ID)
            || self.match_token(Tipo_Token::SIMBOLO_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_MENOS)
            || self.match_token(Tipo_Token::OCTAL)
            || self.match_token(Tipo_Token::HEX)
            || self.match_token(Tipo_Token::INT)
            || self.match_token(Tipo_Token::STR)
            || self.match_token(Tipo_Token::CHAR)
            || self.match_token(Tipo_Token::FLOAT)
            || self.match_token(Tipo_Token::TRUE)
            || self.match_token(Tipo_Token::FALSE)
            || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.op_and();
                if self.match_token(Tipo_Token::SIMBOLO_D_AND) {
                    self.consumir_token();
                    self.op_bin_or();
                    self.op_and_opt();
                } else {
                    self.erro("==");
                }
        }
    }
    fn op_and_opt(&mut self) {
        println!("op_and_opt");
        if self.match_token(Tipo_Token::ID)
            || self.match_token(Tipo_Token::SIMBOLO_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_MENOS)
            || self.match_token(Tipo_Token::OCTAL)
            || self.match_token(Tipo_Token::HEX)
            || self.match_token(Tipo_Token::INT)
            || self.match_token(Tipo_Token::STR)
            || self.match_token(Tipo_Token::CHAR)
            || self.match_token(Tipo_Token::FLOAT)
            || self.match_token(Tipo_Token::TRUE)
            || self.match_token(Tipo_Token::FALSE)
            || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.op_and_rec();
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_bin_or(&mut self) {
        println!("op_bin_or");
        if self.match_token(Tipo_Token::SIMBOLO_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_MENOS)
                || self.match_token(Tipo_Token::OCTAL)
                || self.match_token(Tipo_Token::HEX)
                || self.match_token(Tipo_Token::INT)
                || self.match_token(Tipo_Token::STR)
                || self.match_token(Tipo_Token::CHAR)
                || self.match_token(Tipo_Token::FLOAT)
                || self.match_token(Tipo_Token::TRUE)
                || self.match_token(Tipo_Token::FALSE)
                || self.match_token(Tipo_Token::ID)
                || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.op_bin_and();
                    self.op_bin_or_opt();
                } else {
                    self.erro("expressão or");
            }
    }
    fn op_bin_or_opt(&mut self) {
        println!("op_bin_or_opt");
        if self.match_token(Tipo_Token::SIMBOLO_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_MENOS)
                || self.match_token(Tipo_Token::OCTAL)
                || self.match_token(Tipo_Token::HEX)
                || self.match_token(Tipo_Token::INT)
                || self.match_token(Tipo_Token::STR)
                || self.match_token(Tipo_Token::CHAR)
                || self.match_token(Tipo_Token::FLOAT)
                || self.match_token(Tipo_Token::TRUE)
                || self.match_token(Tipo_Token::FALSE)
                || self.match_token(Tipo_Token::ID)
                || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.op_bin_or_rec();
        }
    }
    fn op_bin_or_rec(&mut self) {
        println!("op_bin_or_rec");
        if self.match_token(Tipo_Token::SIMBOLO_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_MENOS)
                || self.match_token(Tipo_Token::OCTAL)
                || self.match_token(Tipo_Token::HEX)
                || self.match_token(Tipo_Token::INT)
                || self.match_token(Tipo_Token::STR)
                || self.match_token(Tipo_Token::CHAR)
                || self.match_token(Tipo_Token::FLOAT)
                || self.match_token(Tipo_Token::TRUE)
                || self.match_token(Tipo_Token::FALSE)
                || self.match_token(Tipo_Token::ID)
                || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.op_bin_or();
                    if self.match_token(Tipo_Token::SIMBOLO_OR) {
                        self.consumir_token();
                        self.op_bin_and();
                        self.op_bin_or_opt();
                    } else {
                        self.erro("|");
                    }
        } else { self.erro("op bin or rec"); }

    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_bin_and(&mut self) {

        println!("op_bin_and");
        if self.match_token(Tipo_Token::SIMBOLO_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_MENOS)
                || self.match_token(Tipo_Token::OCTAL)
                || self.match_token(Tipo_Token::HEX)
                || self.match_token(Tipo_Token::INT)
                || self.match_token(Tipo_Token::STR)
                || self.match_token(Tipo_Token::CHAR)
                || self.match_token(Tipo_Token::FLOAT)
                || self.match_token(Tipo_Token::TRUE)
                || self.match_token(Tipo_Token::FALSE)
                || self.match_token(Tipo_Token::ID)
                || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.op_equate();
                    self.op_bin_and_opt();
                } else {
                    self.erro("op bin and");
        }

    }
    fn op_bin_and_opt(&mut self) {
        println!("op_bin_and_opt");
        if self.match_token(Tipo_Token::SIMBOLO_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_MENOS)
                || self.match_token(Tipo_Token::OCTAL)
                || self.match_token(Tipo_Token::HEX)
                || self.match_token(Tipo_Token::INT)
                || self.match_token(Tipo_Token::STR)
                || self.match_token(Tipo_Token::CHAR)
                || self.match_token(Tipo_Token::FLOAT)
                || self.match_token(Tipo_Token::TRUE)
                || self.match_token(Tipo_Token::FALSE)
                || self.match_token(Tipo_Token::ID)
                || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.op_bin_and_rec();
        }
    }
    fn op_bin_and_rec(&mut self) {
        println!("op_bin_and_rec");
        if self.match_token(Tipo_Token::SIMBOLO_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_MENOS)
                || self.match_token(Tipo_Token::OCTAL)
                || self.match_token(Tipo_Token::HEX)
                || self.match_token(Tipo_Token::INT)
                || self.match_token(Tipo_Token::STR)
                || self.match_token(Tipo_Token::CHAR)
                || self.match_token(Tipo_Token::FLOAT)
                || self.match_token(Tipo_Token::TRUE)
                || self.match_token(Tipo_Token::FALSE)
                || self.match_token(Tipo_Token::ID)
                || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.op_bin_and();
                    if self.match_token(Tipo_Token::SIMBOLO_AND) {
                        self.consumir_token();
                        self.op_equate();
                        self.op_bin_and_opt();
                    }
        } else {
                    self.erro("op bin and rec");
        }

    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_equate(&mut self) {
        println!("op_equate");

        if self.match_token(Tipo_Token::SIMBOLO_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_MENOS)
                || self.match_token(Tipo_Token::OCTAL)
                || self.match_token(Tipo_Token::HEX)
                || self.match_token(Tipo_Token::INT)
                || self.match_token(Tipo_Token::STR)
                || self.match_token(Tipo_Token::CHAR)
                || self.match_token(Tipo_Token::FLOAT)
                || self.match_token(Tipo_Token::TRUE)
                || self.match_token(Tipo_Token::FALSE)
                || self.match_token(Tipo_Token::ID)
                || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.op_compare();
                    self.op_equate_opt_2();
        } else {
                    self.erro("op equate");
        }
    }
    fn op_equate_opt_2(&mut self) {
        println!("op_equate_opt");
        if self.match_token(Tipo_Token::SIMBOLO_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_MENOS)
                || self.match_token(Tipo_Token::OCTAL)
                || self.match_token(Tipo_Token::HEX)
                || self.match_token(Tipo_Token::INT)
                || self.match_token(Tipo_Token::STR)
                || self.match_token(Tipo_Token::CHAR)
                || self.match_token(Tipo_Token::FLOAT)
                || self.match_token(Tipo_Token::TRUE)
                || self.match_token(Tipo_Token::FALSE)
                || self.match_token(Tipo_Token::ID)
                || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.op_equate_rec();
        }
    }
    fn op_equate_rec(&mut self) {
        println!("op_equate_rec");
        if self.match_token(Tipo_Token::SIMBOLO_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_MENOS)
                || self.match_token(Tipo_Token::OCTAL)
                || self.match_token(Tipo_Token::HEX)
                || self.match_token(Tipo_Token::INT)
                || self.match_token(Tipo_Token::STR)
                || self.match_token(Tipo_Token::CHAR)
                || self.match_token(Tipo_Token::FLOAT)
                || self.match_token(Tipo_Token::TRUE)
                || self.match_token(Tipo_Token::FALSE)
                || self.match_token(Tipo_Token::ID)
                || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.op_equate();
                    self.op_equate_opt();
                    self.op_equate_opt_2();
        } else {
             self.erro("op equate rec");
        }
    }
    fn op_equate_opt(&mut self) {
        println!("op_equate_opt");
        if self.match_token(Tipo_Token::SIMBOLO_D_IGUAL) {
            self.consumir_token();
            self.op_compare();
        } else if self.match_token(Tipo_Token::SIMBOLO_D_DIFERENTE) {
            self.consumir_token();
            self.op_compare();
        } else {
            self.erro("==");
            self.erro("!=");
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_compare(&mut self) {
        println!("op_compare");
        if self.match_token(Tipo_Token::SIMBOLO_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_MENOS)
                || self.match_token(Tipo_Token::OCTAL)
                || self.match_token(Tipo_Token::HEX)
                || self.match_token(Tipo_Token::INT)
                || self.match_token(Tipo_Token::STR)
                || self.match_token(Tipo_Token::CHAR)
                || self.match_token(Tipo_Token::FLOAT)
                || self.match_token(Tipo_Token::TRUE)
                || self.match_token(Tipo_Token::FALSE)
                || self.match_token(Tipo_Token::ID)
                || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.op_add();
                    self.op_compare_opt_2();
        } else {
                    self.erro("op compare");
        }
    }
    fn op_compare_opt_2(&mut self) {
        println!("op_compare_opt_2");
        if self.match_token(Tipo_Token::SIMBOLO_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_MENOS)
                || self.match_token(Tipo_Token::OCTAL)
                || self.match_token(Tipo_Token::HEX)
                || self.match_token(Tipo_Token::INT)
                || self.match_token(Tipo_Token::STR)
                || self.match_token(Tipo_Token::CHAR)
                || self.match_token(Tipo_Token::FLOAT)
                || self.match_token(Tipo_Token::TRUE)
                || self.match_token(Tipo_Token::FALSE)
                || self.match_token(Tipo_Token::ID)
                || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.op_compare_rec();
        }
    }
    fn op_compare_rec(&mut self) {
        println!("op_compare_rec");
        if self.match_token(Tipo_Token::SIMBOLO_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_MENOS)
                || self.match_token(Tipo_Token::OCTAL)
                || self.match_token(Tipo_Token::HEX)
                || self.match_token(Tipo_Token::INT)
                || self.match_token(Tipo_Token::STR)
                || self.match_token(Tipo_Token::CHAR)
                || self.match_token(Tipo_Token::FLOAT)
                || self.match_token(Tipo_Token::TRUE)
                || self.match_token(Tipo_Token::FALSE)
                || self.match_token(Tipo_Token::ID)
                || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.op_compare();
                    self.op_compare_opt();
                    self.op_compare_opt_2();
        } else {
                    self.erro("op compare rec");
        }

    }
    fn op_compare_opt(&mut self) {
        println!("op_compare_opt");
        if self.match_token(Tipo_Token::SIMBOLO_MENOR_Q)
        || self.match_token(Tipo_Token::SIMBOLO_MAIOR_Q)
        || self.match_token(Tipo_Token::SIMBOLO_MAIOR_IGUAL_Q)
        || self.match_token(Tipo_Token::SIMBOLO_MENOR_IGUAL_Q) {
            self.consumir_token();
            self.op_add();
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_add(&mut self) {
        println!("op_add");
        if self.match_token(Tipo_Token::SIMBOLO_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_MENOS)
                || self.match_token(Tipo_Token::OCTAL)
                || self.match_token(Tipo_Token::HEX)
                || self.match_token(Tipo_Token::INT)
                || self.match_token(Tipo_Token::STR)
                || self.match_token(Tipo_Token::CHAR)
                || self.match_token(Tipo_Token::FLOAT)
                || self.match_token(Tipo_Token::TRUE)
                || self.match_token(Tipo_Token::FALSE)
                || self.match_token(Tipo_Token::ID)
                || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.op_mult();
                    self.op_add_opt_2();
        } else {
                    self.erro("op add");
        }

    }
    fn op_add_opt_2(&mut self) {
        println!("op_add_opt_2");
        if self.match_token(Tipo_Token::SIMBOLO_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_MENOS)
                || self.match_token(Tipo_Token::OCTAL)
                || self.match_token(Tipo_Token::HEX)
                || self.match_token(Tipo_Token::INT)
                || self.match_token(Tipo_Token::STR)
                || self.match_token(Tipo_Token::CHAR)
                || self.match_token(Tipo_Token::FLOAT)
                || self.match_token(Tipo_Token::TRUE)
                || self.match_token(Tipo_Token::FALSE)
                || self.match_token(Tipo_Token::ID)
                || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.op_add_rec();
        }
    }
    fn op_add_rec(&mut self) {
        println!("op_add_rec");
        if self.match_token(Tipo_Token::SIMBOLO_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_MENOS)
                || self.match_token(Tipo_Token::OCTAL)
                || self.match_token(Tipo_Token::HEX)
                || self.match_token(Tipo_Token::INT)
                || self.match_token(Tipo_Token::STR)
                || self.match_token(Tipo_Token::CHAR)
                || self.match_token(Tipo_Token::FLOAT)
                || self.match_token(Tipo_Token::TRUE)
                || self.match_token(Tipo_Token::FALSE)
                || self.match_token(Tipo_Token::ID)
                || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.op_add();
                    self.op_add_opt();
                    self.op_add_opt_2();
        } else {
            self.erro("op add rec");
        }
    }
    fn op_add_opt(&mut self) {
        println!("op_add_opt");
        if self.match_token(Tipo_Token::SIMBOLO_MAIS)
            || self.match_token(Tipo_Token::SIMBOLO_MENOS) {
                self.consumir_token();
                self.op_mult();
            } else {
                self.erro("+");
                self.erro("-");
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_mult(&mut self) {
        println!("op_mult");
        if self.match_token(Tipo_Token::SIMBOLO_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_MENOS)
                || self.match_token(Tipo_Token::OCTAL)
                || self.match_token(Tipo_Token::HEX)
                || self.match_token(Tipo_Token::INT)
                || self.match_token(Tipo_Token::STR)
                || self.match_token(Tipo_Token::CHAR)
                || self.match_token(Tipo_Token::FLOAT)
                || self.match_token(Tipo_Token::TRUE)
                || self.match_token(Tipo_Token::FALSE)
                || self.match_token(Tipo_Token::ID)
                || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.op_unary();
                    self.op_mult_opt_2();
        } else {
                    self.erro("op mult");
        }
    }
    fn op_mult_rec(&mut self) {
        println!("op_mult_rec");
        if self.match_token(Tipo_Token::SIMBOLO_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_MENOS)
                || self.match_token(Tipo_Token::OCTAL)
                || self.match_token(Tipo_Token::HEX)
                || self.match_token(Tipo_Token::INT)
                || self.match_token(Tipo_Token::STR)
                || self.match_token(Tipo_Token::CHAR)
                || self.match_token(Tipo_Token::FLOAT)
                || self.match_token(Tipo_Token::TRUE)
                || self.match_token(Tipo_Token::FALSE)
                || self.match_token(Tipo_Token::ID)
                || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.op_mult();
                    self.op_mult_opt();
                    self.op_mult_opt_2();
        } else {
                    self.erro("op mult rec");
        }
    }
    fn op_mult_opt(&mut self) {
        println!("op_mult_opt");
        if self.match_token(Tipo_Token::SIMBOLO_MULTI)
            || self.match_token(Tipo_Token::SIMBOLO_DIV)
            || self.match_token(Tipo_Token::SIMBOLO_MOD) {
                self.consumir_token();
                self.op_unary();
        }
    }
    fn op_mult_opt_2(&mut self) {
        println!("op_mult_opt_2");
        if self.match_token(Tipo_Token::SIMBOLO_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
                || self.match_token(Tipo_Token::SIMBOLO_MENOS)
                || self.match_token(Tipo_Token::OCTAL)
                || self.match_token(Tipo_Token::HEX)
                || self.match_token(Tipo_Token::INT)
                || self.match_token(Tipo_Token::STR)
                || self.match_token(Tipo_Token::CHAR)
                || self.match_token(Tipo_Token::FLOAT)
                || self.match_token(Tipo_Token::TRUE)
                || self.match_token(Tipo_Token::FALSE)
                || self.match_token(Tipo_Token::ID)
                || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                    self.op_mult_rec();
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn op_unary(&mut self) {
        println!("op_unary");
        if self.match_token(Tipo_Token::SIMBOLO_NOT)
        || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
        || self.match_token(Tipo_Token::SIMBOLO_MENOS) {
                self.consumir_token();
                self.op_unary();
        } else if self.match_token(Tipo_Token::OCTAL)
            || self.match_token(Tipo_Token::HEX)
            || self.match_token(Tipo_Token::INT)
            || self.match_token(Tipo_Token::STR)
            || self.match_token(Tipo_Token::CHAR)
            || self.match_token(Tipo_Token::FLOAT)
            || self.match_token(Tipo_Token::TRUE)
            || self.match_token(Tipo_Token::FALSE)
            || self.match_token(Tipo_Token::ID)
            || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.value();
            } else {
                self.erro("op unary");
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn value(&mut self) {
        println!("value");
        if self.match_token(Tipo_Token::OCTAL)
        || self.match_token(Tipo_Token::HEX)
        || self.match_token(Tipo_Token::INT)
        || self.match_token(Tipo_Token::STR)
        || self.match_token(Tipo_Token::CHAR)
        || self.match_token(Tipo_Token::FLOAT)
        || self.match_token(Tipo_Token::TRUE)
        || self.match_token(Tipo_Token::FALSE) {
            self.consumir_token();
        } else if self.match_token(Tipo_Token::ID) {
            self.consumir_token();
            self.id_opt();
        } else if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
            self.consumir_token();
            self.expr();
            if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                self.consumir_token();
            } else { self.erro(")"); }
        } else { self.erro("("); }
    }

    ///////////////////////////////////////////////////////////////////////////
    fn id_opt(&mut self) {
        println!("id_opt");
        if self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
            self.consumir_token();
            self.id_opt_2();
        }
    }
    fn id_opt_2(&mut self) {
        println!("id_opt_2");
        if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
            self.consumir_token();
        } else if self.match_token(Tipo_Token::SIMBOLO_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_BIT_NOT)
            || self.match_token(Tipo_Token::SIMBOLO_MENOS)
            || self.match_token(Tipo_Token::OCTAL)
            || self.match_token(Tipo_Token::HEX)
            || self.match_token(Tipo_Token::INT)
            || self.match_token(Tipo_Token::CHAR)
            || self.match_token(Tipo_Token::FLOAT)
            || self.match_token(Tipo_Token::TRUE)
            || self.match_token(Tipo_Token::FALSE)
            || self.match_token(Tipo_Token::ID)
            || self.match_token(Tipo_Token::PARENTESE_ESQUERDO) {
                self.expr();
                if self.match_token(Tipo_Token::PARENTESE_DIREITO) {
                    self.consumir_token();
                } else {
                    self.erro(")");
                }
            } else {
                self.erro("if opt 2");
            }
    }

}
