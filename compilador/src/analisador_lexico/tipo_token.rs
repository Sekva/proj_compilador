use std::fmt;

#[derive(Copy,Clone,PartialEq, Eq, Hash, Debug)]
pub enum Tipo_Token {

    PARENTESE_ESQUERDO,
    PARENTESE_DIREITO,

    CHAVE_ESQUERDA,
    CHAVE_DIREITA,

    VIRGULA,
    PONTO_VIRGULA,

    SIMBOLO_IGUAL,

    SIMBOLO_D_OR,
    SIMBOLO_D_AND,

    SIMBOLO_OR,
    SIMBOLO_AND,

    SIMBOLO_D_IGUAL,
    SIMBOLO_D_DIFERENTE,

    SIMBOLO_MENOR_Q,
    SIMBOLO_MAIOR_Q,
    SIMBOLO_MENOR_IGUAL_Q,
    SIMBOLO_MAIOR_IGUAL_Q,

    SIMBOLO_MAIS,
    SIMBOLO_MENOS,

    SIMBOLO_MULTI,
    SIMBOLO_DIV,
    SIMBOLO_MOD,

    SIMBOLO_NOT,
    SIMBOLO_BIT_NOT,

    ID,
    STR,
    CHAR,
    INT,
    BOOL,
    FLOAT,
    OCTAL,
    HEX,
    VOID,

    ID_STR,
    ID_CHAR,
    ID_INT,
    ID_BOOL,
    ID_FLOAT,
    ID_VOID,

    ELSE,
    FALSE,
    FUNC,
    IF,
    PRINTK,
    RETURN,
    RETURNS,
    TRUE,
    AS,
    WHILE,
    BREAK,
    CONTINUE,



  EOF,
  VAZIO

}


impl fmt::Display for Tipo_Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Tipo_Token::ID => "ID",
            Tipo_Token::AS => "AS",
            Tipo_Token::BOOL => "BOOL",
            Tipo_Token::BREAK => "BREAK",
            Tipo_Token::CHAR => "CHAR",
            Tipo_Token::CHAVE_DIREITA => "CHAVE_DIREITA",
            Tipo_Token::CHAVE_ESQUERDA => "CHAVE_ESQUERDA",
            Tipo_Token::CONTINUE => "CONTINUE",
            Tipo_Token::ELSE => "ELSE",
            Tipo_Token::EOF => "EOF",
            Tipo_Token::FALSE => "FALSE",
            Tipo_Token::FLOAT => "FLOAT",
            Tipo_Token::FUNC => "FUNC",
            Tipo_Token::HEX => "HEX",
            Tipo_Token::ID_BOOL => "ID_BOOL",
            Tipo_Token::ID_CHAR => "ID_CHAR",
            Tipo_Token::ID_FLOAT => "ID_FLOAT",
            Tipo_Token::ID_INT => "ID_INT",
            Tipo_Token::ID_STR => "ID_STR",
            Tipo_Token::ID_VOID => "ID_VOID",
            Tipo_Token::IF => "IF",
            Tipo_Token::INT => "INT",
            Tipo_Token::OCTAL => "OCTAL",
            Tipo_Token::PARENTESE_DIREITO => "PARENTESE_DIREITO",
            Tipo_Token::PARENTESE_ESQUERDO => "PARENTESE_ESQUERDO",
            Tipo_Token::PONTO_VIRGULA => "PONTO_VIRGULA",
            Tipo_Token::PRINTK => "PRINTK",
            Tipo_Token::RETURN => "RETURN",
            Tipo_Token::RETURNS => "RETURNS",
            Tipo_Token::SIMBOLO_AND => "SIMBOLO_AND",
            Tipo_Token::SIMBOLO_BIT_NOT => "SIMBOLO_BIT_NOT",
            Tipo_Token::SIMBOLO_D_AND => "SIMBOLO_D_AND",
            Tipo_Token::SIMBOLO_D_DIFERENTE => "SIMBOLO_D_DIFERENTE",
            Tipo_Token::SIMBOLO_D_IGUAL => "SIMBOLO_D_IGUAL",
            Tipo_Token::SIMBOLO_D_OR => "SIMBOLO_D_OR",
            Tipo_Token::SIMBOLO_DIV => "SIMBOLO_DIV",
            Tipo_Token::SIMBOLO_IGUAL => "SIMBOLO_IGUAL",
            Tipo_Token::SIMBOLO_MAIOR_IGUAL_Q => "SIMBOLO_MAIOR_IGUAL_Q",
            Tipo_Token::SIMBOLO_MAIOR_Q => "SIMBOLO_MAIOR_Q",
            Tipo_Token::SIMBOLO_MAIS => "SIMBOLO_MAIS",
            Tipo_Token::SIMBOLO_MENOR_IGUAL_Q => "SIMBOLO_MENOR_IGUAL_Q",
            Tipo_Token::SIMBOLO_MENOR_Q => "SIMBOLO_MENOR_Q",
            Tipo_Token::SIMBOLO_MENOS => "SIMBOLO_MENOS",
            Tipo_Token::SIMBOLO_MOD => "SIMBOLO_MOD",
            Tipo_Token::SIMBOLO_MULTI => "SIMBOLO_MULTI",
            Tipo_Token::SIMBOLO_NOT => "SIMBOLO_NOT",
            Tipo_Token::SIMBOLO_OR => "SIMBOLO_OR",
            Tipo_Token::STR => "STR",
            Tipo_Token::TRUE => "TRUE",
            Tipo_Token::VIRGULA => "VIRGULA",
            Tipo_Token::VOID => "VOID",
            Tipo_Token::WHILE => "WHILE",
            _ => ""
        };
        write!(f, "{}", printable)
    }
}
