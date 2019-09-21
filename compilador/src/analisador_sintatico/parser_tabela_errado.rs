use crate::analisador_lexico::token::*;
use crate::analisador_lexico::tipo_token::*;

use std::collections::HashMap;


#[derive(Clone,PartialEq, Eq, Hash, Debug)]
enum AlphaPilha {
    Token(Tipo_Token),
    Var(String),
}


pub struct Parser {
    tokens : Vec<Token>,
    token_atual : usize,
    tabela : HashMap<String, HashMap<Tipo_Token, Vec<AlphaPilha>>>,
    pilha : Vec<AlphaPilha>,
}


impl Parser {

    pub fn novo(e_tokens : Vec<Token>) -> Parser {

        let t = HashMap::new();

        let mut p = Vec::new();
        p.push(AlphaPilha::Token(Tipo_Token::EOF));

        let mut parser = Parser {
            tokens : e_tokens,
            token_atual : 0,
            tabela : t,
            pilha : p
        };

        parser.iniciar_tabela();
        return parser;
    }

    fn iniciar_tabela(&mut self) {

        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::FUNC, vec![AlphaPilha::Var("<Decl>".to_string()),
                                               AlphaPilha::Var("<Decls>".to_string())
            ]);
            temp.insert(Tipo_Token::ID_CHAR, vec![AlphaPilha::Var("<Decl>".to_string()),
                                             AlphaPilha::Var("<Decls>".to_string())
            ]);
            temp.insert(Tipo_Token::EOF, vec![]);

            temp.insert(Tipo_Token::ID_STR, vec![AlphaPilha::Var("<Decl>".to_string()),
                                             AlphaPilha::Var("<Decls>".to_string())
            ]);
             temp.insert(Tipo_Token::ID_INT, vec![AlphaPilha::Var("<Decl>".to_string()),
                                             AlphaPilha::Var("<Decls>".to_string())
            ]);
            temp.insert(Tipo_Token::ID_FLOAT, vec![AlphaPilha::Var("<Decl>".to_string()),
                                             AlphaPilha::Var("<Decls>".to_string())
            ]);
            temp.insert(Tipo_Token::ID_VOID, vec![AlphaPilha::Var("<Decl>".to_string()),
                                             AlphaPilha::Var("<Decls>".to_string())
            ]);
            temp.insert(Tipo_Token::ID_BOOL, vec![AlphaPilha::Var("<Decl>".to_string()),
                                             AlphaPilha::Var("<Decls>".to_string())
            ]);
            assert_eq!(self.tabela.insert("<Decls>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::FUNC, vec![AlphaPilha::Var("<Func_Decl>".to_string())]);
            temp.insert(Tipo_Token::ID_BOOL, vec![AlphaPilha::Var("<Var_Decl>".to_string())]);
            temp.insert(Tipo_Token::ID_CHAR, vec![AlphaPilha::Var("<Var_Decl>".to_string())]);
            temp.insert(Tipo_Token::ID_STR, vec![AlphaPilha::Var("<Var_Decl>".to_string())]);
            temp.insert(Tipo_Token::ID_INT, vec![AlphaPilha::Var("<Var_Decl>".to_string())]);
            temp.insert(Tipo_Token::ID_FLOAT, vec![AlphaPilha::Var("<Var_Decl>".to_string())]);
            temp.insert(Tipo_Token::ID_VOID, vec![AlphaPilha::Var("<Var_Decl>".to_string())]);
            assert_eq!(self.tabela.insert("<Decl>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::FUNC, vec![AlphaPilha::Token(Tipo_Token::FUNC),
                                               AlphaPilha::Token(Tipo_Token::ID),
                                               AlphaPilha::Token(Tipo_Token::PARENTESE_ESQUERDO),
                                               AlphaPilha::Var("<Func_ParamsOpt>".to_string())
            ]);

            assert_eq!(self.tabela.insert("<Func_Decl>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::ID_CHAR, vec![
                AlphaPilha::Var("<Params>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO),
                AlphaPilha::Token(Tipo_Token::RETURNS),
                AlphaPilha::Var("<Type>".to_string()),
                AlphaPilha::Var("<Block>".to_string())
            ]);
            temp.insert(Tipo_Token::ID_STR, vec![
                AlphaPilha::Var("<Params>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO),
                AlphaPilha::Token(Tipo_Token::RETURNS),
                AlphaPilha::Var("<Type>".to_string()),
                AlphaPilha::Var("<Block>".to_string())
            ]);
            temp.insert(Tipo_Token::ID_INT, vec![
                AlphaPilha::Var("<Params>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO),
                AlphaPilha::Token(Tipo_Token::RETURNS),
                AlphaPilha::Var("<Type>".to_string()),
                AlphaPilha::Var("<Block>".to_string())
            ]);
            temp.insert(Tipo_Token::ID_FLOAT, vec![
                AlphaPilha::Var("<Params>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO),
                AlphaPilha::Token(Tipo_Token::RETURNS),
                AlphaPilha::Var("<Type>".to_string()),
                AlphaPilha::Var("<Block>".to_string())
            ]);
            temp.insert(Tipo_Token::ID_VOID, vec![
                AlphaPilha::Var("<Params>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO),
                AlphaPilha::Token(Tipo_Token::RETURNS),
                AlphaPilha::Var("<Type>".to_string()),
                AlphaPilha::Var("<Block>".to_string())
            ]);
            temp.insert(Tipo_Token::ID_BOOL, vec![
                AlphaPilha::Var("<Params>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO),
                AlphaPilha::Token(Tipo_Token::RETURNS),
                AlphaPilha::Var("<Type>".to_string()),
                AlphaPilha::Var("<Block>".to_string())
            ]);
            temp.insert(Tipo_Token::PARENTESE_DIREITO, vec![
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO),
                AlphaPilha::Token(Tipo_Token::RETURNS),
                AlphaPilha::Var("<Type>".to_string()),
                AlphaPilha::Var("<Block>".to_string())
            ]);
            assert_eq!(self.tabela.insert("<Func_ParamsOpt>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::ID_CHAR, vec![
                AlphaPilha::Var("<Param>".to_string()),
                AlphaPilha::Var("<ParamsOpt>".to_string())
            ]);
            temp.insert(Tipo_Token::ID_STR, vec![
                AlphaPilha::Var("<Param>".to_string()),
                AlphaPilha::Var("<ParamsOpt>".to_string())
            ]);
            temp.insert(Tipo_Token::ID_INT, vec![
                AlphaPilha::Var("<Param>".to_string()),
                AlphaPilha::Var("<ParamsOpt>".to_string())
            ]);
            temp.insert(Tipo_Token::ID_FLOAT, vec![
                AlphaPilha::Var("<Param>".to_string()),
                AlphaPilha::Var("<ParamsOpt>".to_string())
            ]);
            temp.insert(Tipo_Token::ID_VOID, vec![
                AlphaPilha::Var("<Param>".to_string()),
                AlphaPilha::Var("<ParamsOpt>".to_string())
            ]);
            temp.insert(Tipo_Token::ID_BOOL, vec![
                AlphaPilha::Var("<Param>".to_string()),
                AlphaPilha::Var("<ParamsOpt>".to_string())
            ]);
            assert_eq!(self.tabela.insert("<Params>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::VIRGULA, vec![
                AlphaPilha::Token(Tipo_Token::VIRGULA),
                AlphaPilha::Var("<Params>".to_string())
            ]);
            temp.insert(Tipo_Token::PARENTESE_DIREITO, vec![]);
            assert_eq!(self.tabela.insert("<ParamsOpt>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::ID_BOOL, vec![
                AlphaPilha::Var("<Type>".to_string()),
                AlphaPilha::Token(Tipo_Token::ID),
            ]);
            temp.insert(Tipo_Token::ID_CHAR, vec![
                AlphaPilha::Var("<Type>".to_string()),
                AlphaPilha::Token(Tipo_Token::ID),
            ]);
            temp.insert(Tipo_Token::ID_STR, vec![
                AlphaPilha::Var("<Type>".to_string()),
                AlphaPilha::Token(Tipo_Token::ID),
            ]);
            temp.insert(Tipo_Token::ID_INT, vec![
                AlphaPilha::Var("<Type>".to_string()),
                AlphaPilha::Token(Tipo_Token::ID),
            ]);
            temp.insert(Tipo_Token::ID_FLOAT, vec![
                AlphaPilha::Var("<Type>".to_string()),
                AlphaPilha::Token(Tipo_Token::ID),
            ]);
            temp.insert(Tipo_Token::ID_VOID, vec![
                AlphaPilha::Var("<Type>".to_string()),
                AlphaPilha::Token(Tipo_Token::ID),
            ]);
            assert_eq!(self.tabela.insert("<Param>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::ID_BOOL, vec![
                AlphaPilha::Var("<Var>".to_string()),
                AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)
            ]);
            temp.insert(Tipo_Token::ID_CHAR, vec![
                AlphaPilha::Var("<Var>".to_string()),
                AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)
            ]);
            temp.insert(Tipo_Token::ID_STR, vec![
                AlphaPilha::Var("<Var>".to_string()),
                AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)
            ]);
            temp.insert(Tipo_Token::ID_INT, vec![
                AlphaPilha::Var("<Var>".to_string()),
                AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)
            ]);
            temp.insert(Tipo_Token::ID_FLOAT, vec![
                AlphaPilha::Var("<Var>".to_string()),
                AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)
            ]);
            temp.insert(Tipo_Token::ID_VOID, vec![
                AlphaPilha::Var("<Var>".to_string()),
                AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)
            ]);
            assert_eq!(self.tabela.insert("<Var_Decl>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::ID_BOOL, vec![
                AlphaPilha::Var("<Type>".to_string()),
                AlphaPilha::Token(Tipo_Token::ID),
                AlphaPilha::Var("<VarOpt>".to_string())
            ]);
            temp.insert(Tipo_Token::ID_CHAR, vec![
                AlphaPilha::Var("<Type>".to_string()),
                AlphaPilha::Token(Tipo_Token::ID),
                AlphaPilha::Var("<VarOpt>".to_string())
            ]);
            temp.insert(Tipo_Token::ID_STR, vec![
                AlphaPilha::Var("<Type>".to_string()),
                AlphaPilha::Token(Tipo_Token::ID),
                AlphaPilha::Var("<VarOpt>".to_string())
            ]);
            temp.insert(Tipo_Token::ID_INT, vec![
                AlphaPilha::Var("<Type>".to_string()),
                AlphaPilha::Token(Tipo_Token::ID),
                AlphaPilha::Var("<VarOpt>".to_string())
            ]);
            temp.insert(Tipo_Token::ID_FLOAT, vec![
                AlphaPilha::Var("<Type>".to_string()),
                AlphaPilha::Token(Tipo_Token::ID),
                AlphaPilha::Var("<VarOpt>".to_string())
            ]);
            temp.insert(Tipo_Token::ID_VOID, vec![
                AlphaPilha::Var("<Type>".to_string()),
                AlphaPilha::Token(Tipo_Token::ID),
                AlphaPilha::Var("<VarOpt>".to_string())
            ]);
            assert_eq!(self.tabela.insert("<Var>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::SIMBOLO_IGUAL, vec![
                AlphaPilha::Token(Tipo_Token::SIMBOLO_IGUAL),
                AlphaPilha::Var("<Op_Or>".to_string())

            ]);
            temp.insert(Tipo_Token::PONTO_VIRGULA, vec![]);
            assert_eq!(self.tabela.insert("<VarOpt>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::ID_CHAR, vec![AlphaPilha::Token(Tipo_Token::ID_CHAR)]);
            temp.insert(Tipo_Token::ID_STR, vec![AlphaPilha::Token(Tipo_Token::ID_STR)]);
            temp.insert(Tipo_Token::ID_INT, vec![AlphaPilha::Token(Tipo_Token::ID_INT)]);
            temp.insert(Tipo_Token::ID_FLOAT, vec![AlphaPilha::Token(Tipo_Token::ID_FLOAT)]);
            temp.insert(Tipo_Token::ID_VOID, vec![AlphaPilha::Token(Tipo_Token::ID_VOID)]);
            temp.insert(Tipo_Token::ID_BOOL, vec![AlphaPilha::Token(Tipo_Token::ID_BOOL)]);

            assert_eq!(self.tabela.insert("<Type>".to_string(), temp), None);
        }

        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::ID_BOOL,  vec![AlphaPilha::Var("<Var_Decl>".to_string())]);
            temp.insert(Tipo_Token::ID_CHAR,  vec![AlphaPilha::Var("<Var_Decl>".to_string())]);
            temp.insert(Tipo_Token::ID_STR,   vec![AlphaPilha::Var("<Var_Decl>".to_string())]);
            temp.insert(Tipo_Token::ID_INT,   vec![AlphaPilha::Var("<Var_Decl>".to_string())]);
            temp.insert(Tipo_Token::ID_FLOAT, vec![AlphaPilha::Var("<Var_Decl>".to_string())]);
            temp.insert(Tipo_Token::ID_VOID,  vec![AlphaPilha::Var("<Var_Decl>".to_string())]);
            temp.insert(Tipo_Token::IF, vec![
                AlphaPilha::Token(Tipo_Token::IF),
                AlphaPilha::Token(Tipo_Token::PARENTESE_ESQUERDO),
                AlphaPilha::Var("<Expr>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO),
                AlphaPilha::Var("<Then_Stm>".to_string()),
                AlphaPilha::Token(Tipo_Token::ELSE),
                AlphaPilha::Var("<Then_Stm>".to_string()),
            ]);
            temp.insert(Tipo_Token::WHILE, vec![
                AlphaPilha::Token(Tipo_Token::WHILE),
                AlphaPilha::Token(Tipo_Token::PARENTESE_ESQUERDO),
                AlphaPilha::Var("<Expr>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO),
                AlphaPilha::Var("<Then_Stm>".to_string())
            ]);
            temp.insert(Tipo_Token::BREAK, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::CONTINUE, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::RETURN, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::PRINTK, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::PONTO_VIRGULA, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::CHAVE_ESQUERDA, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::OCTAL, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::HEX, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::INT, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::STR, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::CHAR, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::FLOAT, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::TRUE, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::FALSE, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::ID, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);

            assert_eq!(self.tabela.insert("<Stm>".to_string(), temp), None);
        }

        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::IF, vec![
                AlphaPilha::Token(Tipo_Token::IF),
                AlphaPilha::Token(Tipo_Token::PARENTESE_ESQUERDO),
                AlphaPilha::Var("<Expr>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO),
                AlphaPilha::Var("<Then_Stm>".to_string()),
                AlphaPilha::Token(Tipo_Token::ELSE),
                AlphaPilha::Var("<Then_Stm>".to_string()),
            ]);
            temp.insert(Tipo_Token::WHILE, vec![
                AlphaPilha::Token(Tipo_Token::WHILE),
                AlphaPilha::Token(Tipo_Token::PARENTESE_ESQUERDO),
                AlphaPilha::Var("<Expr>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO),
                AlphaPilha::Var("<Then_Stm>".to_string())
            ]);
            temp.insert(Tipo_Token::BREAK, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::CONTINUE, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::RETURN, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::PRINTK, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::PONTO_VIRGULA, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::CHAVE_ESQUERDA, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::OCTAL, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::HEX, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::INT, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::STR, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::CHAR, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::FLOAT, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::TRUE, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::FALSE, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::ID, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![AlphaPilha::Var("<Normal_Stm>".to_string())]);
            assert_eq!(self.tabela.insert("<Then_Stm>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::BREAK, vec![
                AlphaPilha::Token(Tipo_Token::BREAK)
            ]);
            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![AlphaPilha::Var("<Expr>".to_string()), AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![AlphaPilha::Var("<Expr>".to_string()), AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![AlphaPilha::Var("<Expr>".to_string()), AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)]);
            temp.insert(Tipo_Token::OCTAL, vec![AlphaPilha::Var("<Expr>".to_string()), AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)]);
            temp.insert(Tipo_Token::HEX, vec![AlphaPilha::Var("<Expr>".to_string()), AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)]);
            temp.insert(Tipo_Token::INT, vec![AlphaPilha::Var("<Expr>".to_string()), AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)]);
            temp.insert(Tipo_Token::STR, vec![AlphaPilha::Var("<Expr>".to_string()), AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)]);
            temp.insert(Tipo_Token::CHAR, vec![AlphaPilha::Var("<Expr>".to_string()), AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)]);
            temp.insert(Tipo_Token::FLOAT, vec![AlphaPilha::Var("<Expr>".to_string()), AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)]);
            temp.insert(Tipo_Token::TRUE, vec![AlphaPilha::Var("<Expr>".to_string()), AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)]);
            temp.insert(Tipo_Token::FALSE, vec![AlphaPilha::Var("<Expr>".to_string()), AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)]);
            temp.insert(Tipo_Token::ID, vec![AlphaPilha::Var("<Expr>".to_string()), AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![AlphaPilha::Var("<Expr>".to_string()), AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)]);

            temp.insert(Tipo_Token::CONTINUE, vec![
                AlphaPilha::Token(Tipo_Token::CONTINUE),
                AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)
            ]);
            temp.insert(Tipo_Token::RETURN, vec![
                AlphaPilha::Token(Tipo_Token::RETURN),
                AlphaPilha::Var("<Expr>".to_string()),
                AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)
            ]);
             temp.insert(Tipo_Token::PRINTK, vec![
                AlphaPilha::Token(Tipo_Token::PRINTK),
                AlphaPilha::Token(Tipo_Token::PARENTESE_ESQUERDO),
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO),
            ]
            );
            temp.insert(Tipo_Token::PONTO_VIRGULA, vec![
                AlphaPilha::Token(Tipo_Token::PONTO_VIRGULA)
            ]);
            temp.insert(Tipo_Token::CHAVE_ESQUERDA, vec![AlphaPilha::Var("<Block>".to_string())]);
            assert_eq!(self.tabela.insert("<Normal_Stm>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::CHAVE_ESQUERDA, vec![
                AlphaPilha::Token(Tipo_Token::CHAVE_ESQUERDA),
                AlphaPilha::Var("<Stm_List>".to_string()),
                AlphaPilha::Token(Tipo_Token::CHAVE_DIREITA)
            ]);

            assert_eq!(self.tabela.insert("<Block>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::IF, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::WHILE, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID_CHAR, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID_STR, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID_INT, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID_FLOAT, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID_VOID, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID_BOOL, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::BREAK, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::CONTINUE, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::RETURN, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::PRINTK, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::PONTO_VIRGULA, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAVE_ESQUERDA, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Stm>".to_string()),
                AlphaPilha::Var("<Stm_List>".to_string()),
            ]);

            temp.insert(Tipo_Token::CHAVE_DIREITA, vec![]);

            assert_eq!(self.tabela.insert("<Stm_List>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_Assign>".to_string()),
                AlphaPilha::Var("<ExprOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_Assign>".to_string()),
                AlphaPilha::Var("<ExprOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_Assign>".to_string()),
                AlphaPilha::Var("<ExprOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_Assign>".to_string()),
                AlphaPilha::Var("<ExprOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_Assign>".to_string()),
                AlphaPilha::Var("<ExprOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_Assign>".to_string()),
                AlphaPilha::Var("<ExprOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_Assign>".to_string()),
                AlphaPilha::Var("<ExprOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_Assign>".to_string()),
                AlphaPilha::Var("<ExprOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_Assign>".to_string()),
                AlphaPilha::Var("<ExprOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_Assign>".to_string()),
                AlphaPilha::Var("<ExprOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_Assign>".to_string()),
                AlphaPilha::Var("<ExprOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_Assign>".to_string()),
                AlphaPilha::Var("<ExprOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_Assign>".to_string()),
                AlphaPilha::Var("<ExprOpt>".to_string()),
            ]);

            assert_eq!(self.tabela.insert("<Expr>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::VIRGULA, vec![
                AlphaPilha::Token(Tipo_Token::VIRGULA),
                AlphaPilha::Var("<Op_Assign>".to_string()),
                AlphaPilha::Var("<ExprOpt>".to_string()),
            ]);

            assert_eq!(self.tabela.insert("<ExprRec>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::VIRGULA, vec![
                AlphaPilha::Var("<ExprRec>".to_string())
            ]);

            temp.insert(Tipo_Token::PARENTESE_DIREITO, vec![]);
            temp.insert(Tipo_Token::PONTO_VIRGULA, vec![]);
            assert_eq!(self.tabela.insert("<ExprOpt>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();

            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Var("<Op_AssignOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Var("<Op_AssignOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Var("<Op_AssignOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Var("<Op_AssignOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Var("<Op_AssignOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Var("<Op_AssignOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Var("<Op_AssignOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Var("<Op_AssignOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Var("<Op_AssignOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Var("<Op_AssignOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Var("<Op_AssignOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Var("<Op_AssignOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Var("<Op_AssignOpt2>".to_string()),
            ]);

            assert_eq!(self.tabela.insert("<Op_Assign>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::SIMBOLO_IGUAL, vec![
                AlphaPilha::Var("<Op_AssignOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::VIRGULA, vec![]);
            temp.insert(Tipo_Token::PARENTESE_DIREITO, vec![]);
            temp.insert(Tipo_Token::PONTO_VIRGULA, vec![]);
            assert_eq!(self.tabela.insert("<Op_AssignOpt2>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::SIMBOLO_IGUAL, vec![
                AlphaPilha::Token(Tipo_Token::SIMBOLO_IGUAL),
                AlphaPilha::Var("<Op_Assign>".to_string()),
            ]);

            assert_eq!(self.tabela.insert("<Op_AssignOpt>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();



            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);


            assert_eq!(self.tabela.insert("<Op_or>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();




            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_OR),
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_OR),
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_OR),
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_OR),
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_OR),
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_OR),
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_OR),
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_OR),
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_OR),
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_OR),
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_OR),
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_OR),
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_Or>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_OR),
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Var("<OpOrOpt>".to_string()),
            ]);
            assert_eq!(self.tabela.insert("<OpOrRec>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();

            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<OpOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<OpOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<OpOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<OpOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<OpOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<OpOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<OpOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<OpOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<OpOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<OpOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<OpOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<OpOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<OpOrRec>".to_string()),
            ]);


            temp.insert(Tipo_Token::SIMBOLO_D_OR, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_IGUAL, vec![]);
            temp.insert(Tipo_Token::PARENTESE_DIREITO, vec![]);
            temp.insert(Tipo_Token::VIRGULA, vec![]);
            temp.insert(Tipo_Token::PONTO_VIRGULA, vec![]);

            assert_eq!(self.tabela.insert("<OpOrOpt>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();



            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);

            assert_eq!(self.tabela.insert("<Op_And>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();


            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_AND),
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_AND),
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_AND),
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_AND),
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_AND),
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_AND),
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_AND),
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_AND),
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_AND),
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_AND),
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_AND),
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_AND),
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_And>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_AND),
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Var("<Op_AndOpt>".to_string()),
            ]);

            assert_eq!(self.tabela.insert("<OpAndRec>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();



            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<OpAndRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<OpAndRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<OpAndRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<OpAndRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<OpAndRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<OpAndRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<OpAndRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<OpAndRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<OpAndRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<OpAndRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<OpAndRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<OpAndRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<OpAndRec>".to_string()),
            ]);


            temp.insert(Tipo_Token::SIMBOLO_D_AND, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_OR, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_IGUAL, vec![]);
            temp.insert(Tipo_Token::PARENTESE_DIREITO, vec![]);
            temp.insert(Tipo_Token::VIRGULA, vec![]);
            temp.insert(Tipo_Token::PONTO_VIRGULA, vec![]);

            assert_eq!(self.tabela.insert("<Op_AndOpt>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();



            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);



            assert_eq!(self.tabela.insert("<Op_BinOr>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();

            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_BinOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_BinOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_BinOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_BinOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_BinOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_BinOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_BinOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_BinOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_BinOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_BinOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_BinOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_BinOrRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_BinOrRec>".to_string()),
            ]);


            temp.insert(Tipo_Token::SIMBOLO_OR, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_AND, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_OR, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_IGUAL, vec![]);
            temp.insert(Tipo_Token::PARENTESE_DIREITO, vec![]);
            temp.insert(Tipo_Token::VIRGULA, vec![]);
            temp.insert(Tipo_Token::PONTO_VIRGULA, vec![]);


            assert_eq!(self.tabela.insert("<Op_BinOrOpt>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();

            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_OR),
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_OR),
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_OR),
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_OR),
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_OR),
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_OR),
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_OR),
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_OR),
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_OR),
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_OR),
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_OR),
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_OR),
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_BinOr>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_OR),
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Var("<Op_BinOrOpt>".to_string()),
            ]);


            assert_eq!(self.tabela.insert("<Op_BinOrRec>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();

            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);

            assert_eq!(self.tabela.insert("<Op_BinAND>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();



            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_BinANDRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_BinANDRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_BinANDRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_BinANDRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_BinANDRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_BinANDRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_BinANDRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_BinANDRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_BinANDRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_BinANDRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_BinANDRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_BinANDRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_BinANDRec>".to_string()),
            ]);


            temp.insert(Tipo_Token::SIMBOLO_AND, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_OR, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_AND, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_OR, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_IGUAL, vec![]);
            temp.insert(Tipo_Token::PARENTESE_DIREITO, vec![]);
            temp.insert(Tipo_Token::VIRGULA, vec![]);
            temp.insert(Tipo_Token::PONTO_VIRGULA, vec![]);

            assert_eq!(self.tabela.insert("<Op_BinANDOpt>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();



            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_AND),
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_AND),
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_AND),
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_AND),
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_AND),
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_AND),
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_AND),
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_AND),
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_AND),
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_AND),
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_AND),
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_AND),
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_BinAND>".to_string()),
                AlphaPilha::Token(Tipo_Token::SIMBOLO_AND),
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_BinANDOpt>".to_string()),
            ]);

            assert_eq!(self.tabela.insert("<Op_BinANDRec>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();

            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);

            assert_eq!(self.tabela.insert("<Op_Equate>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();


            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_EquateRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_EquateRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_EquateRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_EquateRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_EquateRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_EquateRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_EquateRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_EquateRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_EquateRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_EquateRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_EquateRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_EquateRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_EquateRec>".to_string()),
            ]);


            temp.insert(Tipo_Token::SIMBOLO_D_IGUAL, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_DIFERENTE, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_AND, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_OR, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_AND, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_OR, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_IGUAL, vec![]);
            temp.insert(Tipo_Token::PARENTESE_DIREITO, vec![]);
            temp.insert(Tipo_Token::VIRGULA, vec![]);
            temp.insert(Tipo_Token::PONTO_VIRGULA, vec![]);

            assert_eq!(self.tabela.insert("<Op_EquateOpt2>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();

            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_Equate>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt>".to_string()),
                AlphaPilha::Var("<Op_EquateOpt2>".to_string()),
            ]);

            assert_eq!(self.tabela.insert("<Op_EquateRec>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();


            temp.insert(Tipo_Token::SIMBOLO_D_IGUAL, vec![
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_IGUAL),
                AlphaPilha::Var("<Op_Compare>".to_string()),
            ]);

            temp.insert(Tipo_Token::SIMBOLO_D_DIFERENTE, vec![
                AlphaPilha::Token(Tipo_Token::SIMBOLO_D_DIFERENTE),
                AlphaPilha::Var("<Op_Compare>".to_string()),
            ]);

            assert_eq!(self.tabela.insert("<Op_EquateOpt>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();
            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);


            assert_eq!(self.tabela.insert("<Op_Compare>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();



            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);


            temp.insert(Tipo_Token::SIMBOLO_MENOR_Q, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MENOR_IGUAL_Q, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MAIOR_IGUAL_Q, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MAIOR_Q, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_IGUAL, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_DIFERENTE, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_AND, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_OR, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_AND, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_OR, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_IGUAL, vec![]);
            temp.insert(Tipo_Token::PARENTESE_DIREITO, vec![]);
            temp.insert(Tipo_Token::VIRGULA, vec![]);
            temp.insert(Tipo_Token::PONTO_VIRGULA, vec![]);

            assert_eq!(self.tabela.insert("<Op_CompareOpt2>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();

            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_Compare>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt>".to_string()),
                AlphaPilha::Var("<Op_CompareOpt2>".to_string()),
            ]);


            assert_eq!(self.tabela.insert("<Op_CompareRec>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();


            temp.insert(Tipo_Token::SIMBOLO_MENOR_Q, vec![
                AlphaPilha::Token(Tipo_Token::SIMBOLO_MENOR_Q),
                AlphaPilha::Var("<Op_Add>".to_string()),
            ]);

            temp.insert(Tipo_Token::SIMBOLO_MAIOR_Q, vec![
                AlphaPilha::Token(Tipo_Token::SIMBOLO_MAIOR_Q),
                AlphaPilha::Var("<Op_Add>".to_string()),
            ]);

            temp.insert(Tipo_Token::SIMBOLO_MAIOR_IGUAL_Q, vec![
                AlphaPilha::Token(Tipo_Token::SIMBOLO_MAIOR_IGUAL_Q),
                AlphaPilha::Var("<Op_Add>".to_string()),
            ]);

            temp.insert(Tipo_Token::SIMBOLO_MENOR_IGUAL_Q, vec![
                AlphaPilha::Token(Tipo_Token::SIMBOLO_MENOR_IGUAL_Q),
                AlphaPilha::Var("<Op_Add>".to_string()),
            ]);

            assert_eq!(self.tabela.insert("<Op_CompareOpt>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();


            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);

            assert_eq!(self.tabela.insert("<Op_Add>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();

            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_CompareRec>".to_string()),
            ]);


            temp.insert(Tipo_Token::SIMBOLO_MAIS, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MENOR_Q, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MENOR_IGUAL_Q, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MAIOR_IGUAL_Q, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MAIOR_Q, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_IGUAL, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_DIFERENTE, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_AND, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_OR, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_AND, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_OR, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_IGUAL, vec![]);
            temp.insert(Tipo_Token::PARENTESE_DIREITO, vec![]);
            temp.insert(Tipo_Token::VIRGULA, vec![]);
            temp.insert(Tipo_Token::PONTO_VIRGULA, vec![]);



            assert_eq!(self.tabela.insert("<Op_AddOpt2>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();

            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_AddOpt>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_AddOpt>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_AddOpt>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_AddOpt>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_AddOpt>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_AddOpt>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_AddOpt>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_AddOpt>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_AddOpt>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_AddOpt>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_AddOpt>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_AddOpt>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_Add>".to_string()),
                AlphaPilha::Var("<Op_AddOpt>".to_string()),
                AlphaPilha::Var("<Op_AddOpt2>".to_string()),
            ]);


            assert_eq!(self.tabela.insert("<Op_AddRec>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();

            temp.insert(Tipo_Token::SIMBOLO_MAIS, vec![
                AlphaPilha::Token(Tipo_Token::SIMBOLO_MAIS),
                AlphaPilha::Var("<Op_Mult>".to_string()),
            ]);

            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Token(Tipo_Token::SIMBOLO_MENOS),
                AlphaPilha::Var("<Op_Mult>".to_string()),
            ]);

            assert_eq!(self.tabela.insert("<Op_AddOpt>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();

            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_Unary>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_Unary>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_Unary>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_Unary>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_Unary>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_Unary>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_Unary>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_Unary>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_Unary>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_Unary>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_Unary>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_Unary>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_Unary>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);

            assert_eq!(self.tabela.insert("<Op_Mult>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();

            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_MultOpt>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_MultOpt>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_MultOpt>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_MultOpt>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_MultOpt>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_MultOpt>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_MultOpt>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_MultOpt>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_MultOpt>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_MultOpt>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_MultOpt>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_MultOpt>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_Mult>".to_string()),
                AlphaPilha::Var("<Op_MultOpt>".to_string()),
                AlphaPilha::Var("<Op_MultOpt2>".to_string()),
            ]);

            assert_eq!(self.tabela.insert("<Op_MultRec>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();

            temp.insert(Tipo_Token::SIMBOLO_MULTI, vec![
                AlphaPilha::Token(Tipo_Token::SIMBOLO_MULTI),
                AlphaPilha::Var("<Op_Unary>".to_string()),

            ]);
            temp.insert(Tipo_Token::SIMBOLO_DIV, vec![
                AlphaPilha::Token(Tipo_Token::SIMBOLO_DIV),
                AlphaPilha::Var("<Op_Unary>".to_string()),
            ]);

            temp.insert(Tipo_Token::SIMBOLO_MOD, vec![
                AlphaPilha::Token(Tipo_Token::SIMBOLO_MOD),
                AlphaPilha::Var("<Op_Unary>".to_string()),
            ]);

            assert_eq!(self.tabela.insert("<Op_MultOpt>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();

            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Op_MultRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Op_MultRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Op_MultRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Op_MultRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Op_MultRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Op_MultRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Op_MultRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Op_MultRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Op_MultRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Op_MultRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Op_MultRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Op_MultRec>".to_string()),
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Op_MultRec>".to_string()),
            ]);


            temp.insert(Tipo_Token::SIMBOLO_MULTI, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_DIV, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MOD, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MAIS, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MENOR_Q, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MENOR_IGUAL_Q, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MAIOR_IGUAL_Q, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MAIOR_Q, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_IGUAL, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_DIFERENTE, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_AND, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_OR, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_AND, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_OR, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_IGUAL, vec![]);
            temp.insert(Tipo_Token::PARENTESE_DIREITO, vec![]);
            temp.insert(Tipo_Token::VIRGULA, vec![]);
            temp.insert(Tipo_Token::PONTO_VIRGULA, vec![]);

            assert_eq!(self.tabela.insert("<Op_MultOpt2>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();




            temp.insert(Tipo_Token::SIMBOLO_NOT, vec!{
                AlphaPilha::Token(Tipo_Token::SIMBOLO_NOT),
                AlphaPilha::Var("<Op_Unary>".to_string())
            });


            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec!{
                AlphaPilha::Token(Tipo_Token::SIMBOLO_BIT_NOT),
                AlphaPilha::Var("<Op_Unary>".to_string())
            });


            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec!{
                AlphaPilha::Token(Tipo_Token::SIMBOLO_MENOS),
                AlphaPilha::Var("<Op_Unary>".to_string())
            });





            temp.insert(Tipo_Token::OCTAL, vec!{
                AlphaPilha::Var("<Value>".to_string())
            });

            temp.insert(Tipo_Token::HEX, vec!{
                AlphaPilha::Var("<Value>".to_string())
            });

            temp.insert(Tipo_Token::INT, vec!{
                AlphaPilha::Var("<Value>".to_string())
            });

            temp.insert(Tipo_Token::STR, vec!{
                AlphaPilha::Var("<Value>".to_string())
            });

            temp.insert(Tipo_Token::CHAR, vec!{
                AlphaPilha::Var("<Value>".to_string())
            });

            temp.insert(Tipo_Token::FLOAT, vec!{
                AlphaPilha::Var("<Value>".to_string())
            });

            temp.insert(Tipo_Token::TRUE, vec!{
                AlphaPilha::Var("<Value>".to_string())
            });

            temp.insert(Tipo_Token::FALSE, vec!{
                AlphaPilha::Var("<Value>".to_string())
            });

            temp.insert(Tipo_Token::ID, vec!{
                AlphaPilha::Var("<Value>".to_string())
            });

            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec!{
                AlphaPilha::Var("<Value>".to_string())
            });

            assert_eq!(self.tabela.insert("<Op_Unary>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();

            temp.insert(Tipo_Token::OCTAL, vec!{
                AlphaPilha::Token(Tipo_Token::OCTAL)
            });
            temp.insert(Tipo_Token::HEX, vec!{
                AlphaPilha::Token(Tipo_Token::HEX)
            });
            temp.insert(Tipo_Token::INT, vec!{
                AlphaPilha::Token(Tipo_Token::INT)
            });
            temp.insert(Tipo_Token::STR, vec!{
                AlphaPilha::Token(Tipo_Token::STR)
            });
            temp.insert(Tipo_Token::CHAR, vec!{
                AlphaPilha::Token(Tipo_Token::CHAR)
            });
            temp.insert(Tipo_Token::FLOAT, vec!{
                AlphaPilha::Token(Tipo_Token::FLOAT)
            });
            temp.insert(Tipo_Token::TRUE, vec!{
                AlphaPilha::Token(Tipo_Token::TRUE)
            });
            temp.insert(Tipo_Token::FALSE, vec!{
                AlphaPilha::Token(Tipo_Token::FALSE)
            });
            temp.insert(Tipo_Token::ID, vec!{
                AlphaPilha::Token(Tipo_Token::ID),
                AlphaPilha::Var("<Id_Opt>".to_string())
            });
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec!{
                AlphaPilha::Token(Tipo_Token::PARENTESE_ESQUERDO),
                AlphaPilha::Var("<Expr>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO)
            });


            assert_eq!(self.tabela.insert("<Value>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();


            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec!{
                AlphaPilha::Token(Tipo_Token::PARENTESE_ESQUERDO),
                AlphaPilha::Var("<Id_Opt2>".to_string()),
            });


            temp.insert(Tipo_Token::SIMBOLO_MULTI, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_DIV, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MOD, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MAIS, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MENOR_Q, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MENOR_IGUAL_Q, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MAIOR_IGUAL_Q, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_MAIOR_Q, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_IGUAL, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_DIFERENTE, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_AND, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_OR, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_AND, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_D_OR, vec![]);
            temp.insert(Tipo_Token::SIMBOLO_IGUAL, vec![]);
            temp.insert(Tipo_Token::PARENTESE_DIREITO, vec![]);
            temp.insert(Tipo_Token::VIRGULA, vec![]);
            temp.insert(Tipo_Token::PONTO_VIRGULA, vec![]);

            assert_eq!(self.tabela.insert("<Id_Opt>".to_string(), temp), None);
        }


        {
            let mut temp = HashMap::new();

            temp.insert(Tipo_Token::SIMBOLO_NOT, vec![
                AlphaPilha::Var("<Expr>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO)
            ]);
            temp.insert(Tipo_Token::SIMBOLO_BIT_NOT, vec![
                AlphaPilha::Var("<Expr>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO)
            ]);
            temp.insert(Tipo_Token::SIMBOLO_MENOS, vec![
                AlphaPilha::Var("<Expr>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO)
            ]);
            temp.insert(Tipo_Token::OCTAL, vec![
                AlphaPilha::Var("<Expr>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO)
            ]);
            temp.insert(Tipo_Token::HEX, vec![
                AlphaPilha::Var("<Expr>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO)
            ]);
            temp.insert(Tipo_Token::INT, vec![
                AlphaPilha::Var("<Expr>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO)
            ]);
            temp.insert(Tipo_Token::STR, vec![
                AlphaPilha::Var("<Expr>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO)
            ]);
            temp.insert(Tipo_Token::CHAR, vec![
                AlphaPilha::Var("<Expr>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO)
            ]);
            temp.insert(Tipo_Token::FLOAT, vec![
                AlphaPilha::Var("<Expr>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO)
            ]);
            temp.insert(Tipo_Token::TRUE, vec![
                AlphaPilha::Var("<Expr>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO)
            ]);
            temp.insert(Tipo_Token::FALSE, vec![
                AlphaPilha::Var("<Expr>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO)
            ]);
            temp.insert(Tipo_Token::ID, vec![
                AlphaPilha::Var("<Expr>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO)
            ]);
            temp.insert(Tipo_Token::PARENTESE_ESQUERDO, vec![
                AlphaPilha::Var("<Expr>".to_string()),
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO)
            ]);



            temp.insert(Tipo_Token::PARENTESE_DIREITO, vec![
                AlphaPilha::Token(Tipo_Token::PARENTESE_DIREITO)
            ]);




            assert_eq!(self.tabela.insert("<Id_Opt2>".to_string(), temp), None);
        }

    }


    pub fn iniciar_analise(&mut self) {



        while self.match_token(Tipo_Token::EOF) {



            let topo = self.pilha.pop().unwrap();


            match topo {
                AlphaPilha::Token(t) => {
                    if self.match_token(t) {
                        self.consumir_token();
                    } else {
                        self.erro("1");
                    }
                },
                AlphaPilha::Var(v) => {
                    let variavel = self.tabela.get(&v);

                    match variavel {
                        Some(v) => {
                            let lista = v.get(&self.tokens[self.token_atual].token());


                            match lista {
                                Some(l) => {
                                    let mut ll = l.to_vec();
                                    ll.reverse();

                                    for i in ll {
                                        self.pilha.push(i);
                                    }


                                },
                                _ => { self.erro("3"); }
                            }


                        },
                        _ => { self.erro("2"); }
                    }
                }
            }

        }

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
}
