use rand::seq::SliceRandom;

struct Regra {
    regra : String,
    derivacoes : Vec<String>,
}

impl Regra {

    fn derivacao_aleatoria(&self, limitador : usize) -> String {

        if limitador > 1000 {
            for i in self.derivacoes.iter() {
                if &(*i.trim()) == "<PalavraVazia>" {
                    return (*i.as_str()).into();
                }
            }
        }


        self.derivacoes.choose(&mut rand::thread_rng()).unwrap().to_string()
    }

    fn nome(&self) -> String {
        self.regra.as_str().to_string()
    }

}


fn expandir(palavra : String, regras : &Vec<Regra>, limitador : usize) -> (bool, String) {

    for i in regras.iter() {
        if palavra.contains(&(*i.nome())) {
            return  (true, palavra.replacen(&(*i.nome()), &(*i.derivacao_aleatoria(limitador)), 1));
        }
    }

    (false, palavra)
}


fn alguma_variavel(palavra : &String, regras : &Vec<Regra>) -> bool {

    for i in regras.iter() {
        if palavra.contains(&(*i.nome())) {
            return true;
        }
    }

    return false;
}


fn main() {

    let mut regras = Vec::new();


    regras.push(
        Regra {
            regra : String::from("<PalavraVazia>"),
            derivacoes : vec![
                String::from("")
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Decls>"),
            derivacoes : vec![
                String::from("<Decl> <Decls>"),
                String::from("<PalavraVazia>")
            ]
        }
    );



    regras.push(
        Regra {
            regra : String::from("<Decl>"),
            derivacoes : vec![
                String::from("<Func Decl>"),
                String::from("<Var Decl>"),
            ]
        }
    );


    regras.push(
        Regra {
            regra : String::from("<Func Decl>"),
            derivacoes : vec![
                String::from("<Func Id> ( <Func ParamsOpt>"),
            ]
        }
    );



    regras.push(
        Regra {
            regra : String::from("<Func ParamsOpt>"),
            derivacoes : vec![
                String::from("<Params> ) <Block>"),
                String::from(") <Block>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Params>"),
            derivacoes : vec![
                String::from("<Param> <ParamsOpt>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<ParamsOpt>"),
            derivacoes : vec![
                String::from(" , <Params>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Param>"),
            derivacoes : vec![
                String::from("<Type> <Id>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Func Id>"),
            derivacoes : vec![
                String::from("<Type> <Id>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Var Decl>"),
            derivacoes : vec![
                String::from("<Type> <Var> <Var List> ; "),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Var>"),
            derivacoes : vec![
                String::from("<Id> <VarOpt>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<VarOpt>"),
            derivacoes : vec![
                String::from(" = <Op Or>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Var List>"),
            derivacoes : vec![
                String::from(" , <Var> <Var List>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Type>"),
            derivacoes : vec![
                String::from(" char "),
                String::from(" str "),
                String::from(" int "),
                String::from(" float "),
                String::from(" void "),
                String::from(" bool "),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Stm>"),
            derivacoes : vec![
                String::from("<Var Decl>"),
                String::from(" if   ( <Expr> ) <IfOpt>"),
                String::from("while ( <Expr> ) { <Stm> }"),
                String::from("<Normal Stm>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<IfOpt>"),
            derivacoes : vec![
                String::from("{ <Then Stm> } else { <Stm> }"),
                String::from("{ <Stm> }"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Then Stm>"),
            derivacoes : vec![
                String::from("if ( <Expr> ) { <Then Stm> } else { <Then Stm> }"),
                String::from("while ( <Expr> ) { <Then Stm> }"),
                String::from("<Normal Stm>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Normal Stm>"),
            derivacoes : vec![
                String::from("<Block>"),
                String::from("<Expr> ; "),
                String::from(" break ; "),
                String::from(" continue ; "),
                String::from(" return <Expr> ; "),
                String::from(" printk ( <Op Or> ) ; "),
                String::from(" ; "),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Block>"),
            derivacoes : vec![
                String::from(" { <Stm List> } "),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Stm List>"),
            derivacoes : vec![
                String::from("<Stm> <Stm List>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Expr>"),
            derivacoes : vec![
                String::from("<Op Assign> <ExprOpt>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<ExprRec>"),
            derivacoes : vec![
                String::from(" , <Op Assign> <ExprRecOpt>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<ExprOpt>"),
            derivacoes : vec![
                String::from("<ExprRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<ExprRecOpt>"),
            derivacoes : vec![
                String::from("<ExprRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op Assign>"),
            derivacoes : vec![
                String::from("<Op Or> <Op AssignOpt2>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op AssignOpt2>"),
            derivacoes : vec![
                String::from("<Op AssignOpt>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op AssignOpt>"),
            derivacoes : vec![
                String::from("=   <Op Assign>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op If>"),
            derivacoes : vec![
                String::from("<Op Or> <Op IfOpt>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op IfOpt>"),
            derivacoes : vec![
                String::from(" ? <Op If> : <Op If>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op Or>"),
            derivacoes : vec![
                String::from("<Op And> <OpOrOpt>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<OpOrRec>"),
            derivacoes : vec![
                String::from("<Op Or> || <Op And> <OpOrRecOpt>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<OpOrOpt>"),
            derivacoes : vec![
                String::from("<OpOrRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<OpOrRecOpt>"),
            derivacoes : vec![
                String::from("<OpOrRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op And>"),
            derivacoes : vec![
                String::from("<Op BinOr> <Op AndOpt>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<OpAndRec>"),
            derivacoes : vec![
                String::from("<Op And> && <Op BinOr> <OpAndRecOpt>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op AndOpt>"),
            derivacoes : vec![
                String::from("<OpAndRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<OpAndRecOpt>"),
            derivacoes : vec![
                String::from("<OpAndRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op BinOr>"),
            derivacoes : vec![
                String::from("<Op BinXOR> <Op BinOrOpt>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op BinOrOpt>"),
            derivacoes : vec![
                String::from("<Op BinOrRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op BinOrRec>"),
            derivacoes : vec![
                String::from("<Op BinOr> | <Op BinXOR> <Op BinOrRecOpt>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op BinOrRecOpt>"),
            derivacoes : vec![
                String::from("<Op BinOrRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op BinXOR>"),
            derivacoes : vec![
                String::from("<Op BinAND> <Op BinXOROpt>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op BinXOROpt>"),
            derivacoes : vec![
                String::from("<Op BinXORRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op BinXORRec>"),
            derivacoes : vec![
                String::from("<Op BinXOR> ^ <Op BinAND> <Op BinXORRecOpt>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op BinXORRecOpt>"),
            derivacoes : vec![
                String::from("<Op BinXORRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op BinAND>"),
            derivacoes : vec![
                String::from("<Op Equate> <Op BinANDOpt>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op BinANDOpt>"),
            derivacoes : vec![
                String::from("<Op BinANDRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op BinANDRec>"),
            derivacoes : vec![
                String::from("<Op BinAND> & <Op Equate> <Op BinANDRecOpt>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op BinANDRecOpt>"),
            derivacoes : vec![
                String::from("<Op BinANDRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op Equate>"),
            derivacoes : vec![
                String::from("<Op Compare> <Op EquateOpt2>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op EquateOpt2>"),
            derivacoes : vec![
                String::from("<Op EquateRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op EquateRec>"),
            derivacoes : vec![
                String::from("<Op Equate> <Op EquateOpt> <Op EquateRecOpt>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op EquateRecOpt>"),
            derivacoes : vec![
                String::from("<Op EquateRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op EquateOpt>"),
            derivacoes : vec![
                String::from(" == <Op Compare>"),
                String::from(" != <Op Compare>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op Compare>"),
            derivacoes : vec![
                String::from("<Op Add> <Op CompareOpt2>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op CompareOpt2>"),
            derivacoes : vec![
                String::from("<Op CompareRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op CompareRec>"),
            derivacoes : vec![
                String::from("<Op Compare> <Op CompareOpt> <Op CompareRecOpt>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op CompareRecOpt>"),
            derivacoes : vec![
                String::from("<Op CompareRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op CompareOpt>"),
            derivacoes : vec![
                String::from(" < <Op Add>"),
                String::from(" > <Op Add>"),
                String::from(" <= <Op Add>"),
                String::from(" >= <Op Add>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op Add>"),
            derivacoes : vec![
                String::from("<Op Mult> <Op AddOpt2>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op AddOpt2>"),
            derivacoes : vec![
                String::from("<Op AddRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op AddRec>"),
            derivacoes : vec![
                String::from("<Op Add> <Op AddOpt> <Op AddRecOpt>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op AddRecOpt>"),
            derivacoes : vec![
                String::from("<Op AddRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op AddOpt>"),
            derivacoes : vec![
                String::from(" + <Op Mult>"),
                String::from(" - <Op Mult>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op Mult>"),
            derivacoes : vec![
                String::from("<Op Unary> <Op MultOpt2>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op MultRec>"),
            derivacoes : vec![
                String::from("<Op Mult> <Op MultOpt> <Op MultRecOpt>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op MultRecOpt>"),
            derivacoes : vec![
                String::from("<Op MultRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op MultOpt>"),
            derivacoes : vec![
                String::from(" * <Op Unary>"),
                String::from(" / <Op Unary>"),
                String::from(" % <Op Unary>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op MultOpt2>"),
            derivacoes : vec![
                String::from("<Op MultRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Op Unary>"),
            derivacoes : vec![
                String::from(" !  <Op Unary>"),
                String::from(" ~  <Op Unary>"),
                String::from(" -  <Op Unary>"),
                String::from(" *  <Op Unary>"),
                String::from(" ++ <Op Unary>"),
                String::from(" -- <Op Unary>"),
                String::from("<Value>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Value>"),
            derivacoes : vec![
                String::from("<OctLiteral>"),
                String::from("<HexLiteral>"),
                String::from("<DecLiteral>"),
                String::from("<StringLiteral>"),
                String::from("<CharLiteral>"),
                String::from("<FloatLiteral>"),
                String::from("<BoolLiteral>"),
                String::from("<Id> <Id Opt>"),
                String::from(" ( <Expr> ) "),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Id Opt>"),
            derivacoes : vec![
                String::from(" ( <Id Opt2>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Id Opt2>"),
            derivacoes : vec![
                String::from("<Expr> ) "),
                String::from(" ) "),
            ]
        }
    );



    regras.push(
        Regra {
            regra : String::from("<Id>"),
            derivacoes : vec![
                String::from("<Id Head><Id Tail>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Id Tail>"),
            derivacoes : vec![
                String::from("<Id Head>"),
                String::from("<Digit>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Id Head>"),
            derivacoes : vec![
                String::from("<Letter>"),
                String::from("_"),
            ]
        }
    );


    regras.push(
        Regra {
            regra : String::from("<DecLiteral>"),
            derivacoes : vec![
                String::from("<Digit><DecLiteralOpt>"),
            ]
        }
    );


    regras.push(
        Regra {
            regra : String::from("<DecLiteralOpt>"),
            derivacoes : vec![
                String::from("<DecLiteral>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );


    regras.push(
        Regra {
            regra : String::from("<HexLiteral>"),
            derivacoes : vec![
                String::from("0x<Hex Number>"),
            ]
        }
    );


    regras.push(
        Regra {
            regra : String::from("<Hex Number>"),
            derivacoes : vec![
                String::from("<Hex Letter>"),
                String::from("<Digit>"),
                String::from("<Hex Letter><Hex NumberRec>"),
                String::from("<Digit><Hex NumberRec>"),
            ]
        }
    );


    regras.push(
        Regra {
            regra : String::from("<Hex NumberRec>"),
            derivacoes : vec![
                String::from("<Hex Number><Hex NumberOpt><Hex NumberRecOpt>"),
            ]
        }
    );


    regras.push(
        Regra {
            regra : String::from("<Hex NumberRecOpt>"),
            derivacoes : vec![
                String::from("<Hex NumberRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );


    regras.push(
        Regra {
            regra : String::from("<Hex NumberOpt>"),
            derivacoes : vec![
                String::from("<Digit>"),
                String::from("<Hex Letter>"),
            ]
        }
    );


    regras.push(
        Regra {
            regra : String::from("<Hex Letter>"),
            derivacoes : vec![
                String::from("a"),
                String::from("b"),
                String::from("c"),
                String::from("d"),
                String::from("e"),
                String::from("f"),
                String::from("A"),
                String::from("B"),
                String::from("C"),
                String::from("D"),
                String::from("E"),
                String::from("F"),
            ]
        }
    );


    regras.push(
        Regra {
            regra : String::from("<Digit>"),
            derivacoes : vec![
                String::from("0"),
                String::from("1"),
                String::from("2"),
                String::from("3"),
                String::from("4"),
                String::from("5"),
                String::from("6"),
                String::from("7"),
                String::from("8"),
                String::from("9"),
            ]
        }
    );


    regras.push(
        Regra {
            regra : String::from("<Letter>"),
            derivacoes : vec![
                String::from("a"),
                String::from("b"),
                String::from("c"),
                String::from("d"),
                String::from("e"),
                String::from("f"),
                String::from("g"),
                String::from("h"),
                String::from("i"),
                String::from("j"),
                String::from("k"),
                String::from("l"),
                String::from("m"),
                String::from("n"),
                String::from("o"),
                String::from("p"),
                String::from("q"),
                String::from("r"),
                String::from("s"),
                String::from("t"),
                String::from("u"),
                String::from("v"),
                String::from("w"),
                String::from("x"),
                String::from("y"),
                String::from("z"),
                String::from("A"),
                String::from("B"),
                String::from("C"),
                String::from("D"),
                String::from("E"),
                String::from("F"),
                String::from("G"),
                String::from("H"),
                String::from("I"),
                String::from("J"),
                String::from("K"),
                String::from("L"),
                String::from("M"),
                String::from("N"),
                String::from("O"),
                String::from("P"),
                String::from("Q"),
                String::from("R"),
                String::from("S"),
                String::from("T"),
                String::from("U"),
                String::from("V"),
                String::from("W"),
                String::from("X"),
                String::from("Y"),
                String::from("Z"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<BoolLiteral> "),
            derivacoes : vec![
                String::from(" false "),
                String::from(" true "),
            ]
        }
    );


    regras.push(
        Regra {
            regra : String::from("<OctLiteral>"),
            derivacoes : vec![
                String::from(" 0<Oct Number> "),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Oct Number>"),
            derivacoes : vec![
                String::from("<Octal Digit><Oct NumberOpt>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Oct NumberOpt>"),
            derivacoes : vec![
                String::from("<Oct NumberRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Oct NumberRec>"),
            derivacoes : vec![
                String::from("<Oct Number><Octal Digit><Oct NumberRecOpt>")
            ]
        }
    );


    regras.push(
        Regra {
            regra : String::from("<Oct NumberRecOpt>"),
            derivacoes : vec![
                String::from("<Oct NumberRec>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<Octal Digit>"),
            derivacoes : vec![
                String::from("0"),
                String::from("1"),
                String::from("2"),
                String::from("3"),
                String::from("4"),
                String::from("5"),
                String::from("6"),
                String::from("7"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<FloatLiteral>"),
            derivacoes : vec![
                String::from("<DecLiteral>.<DecLiteral>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<StringLiteral>"),
            derivacoes : vec![
                String::from("\"<CharArray>\""),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<CharArray>"),
            derivacoes : vec![
                String::from("<Letter><CharArray>"),
                String::from("<Digit><CharArray>"),
                String::from("<PalavraVazia>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<CharLiteral>"),
            derivacoes : vec![
                String::from("'<CharLiteralOpt>'"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<CharLiteralOpt>"),
            derivacoes : vec![
                String::from("<Letter>"),
                String::from("<Digit>"),
            ]
        }
    );

    regras.push(
        Regra {
            regra : String::from("<PalavraVazia>"),
            derivacoes : vec![
                String::from("")
            ]
        }
    );


    // regras.reverse();

    let mut palavra : String = String::from("<Value>");

    let mut limitador : usize = 0;

    let mut variaveis : bool;

    let (variaveis2, palavra2) = expandir(palavra, &regras, limitador);

    variaveis = variaveis2;
    palavra = palavra2;

    while variaveis {
        println!("");
        println!("{}", palavra);
        println!("");
        println!("limitador {}", limitador);

        let (variaveis2, palavra2) = expandir(palavra, &regras, limitador);


        variaveis = variaveis2;
        palavra = palavra2;

        limitador += 1;
        regras.shuffle(&mut rand::thread_rng());
    }

    let mut _rt = false;

    if alguma_variavel(&palavra, &regras) {
        variaveis = true;
        while variaveis {
            println!("");
            println!("{}", palavra);
            println!("");
            println!("limitador {}", limitador);

            let (variaveis2, palavra2) = expandir(palavra, &regras, limitador);


            variaveis = variaveis2;
            palavra = palavra2;

            limitador += 1;
        }
        _rt =true;
    }

    println!("");
    println!("{}", palavra);
    println!("");
}
