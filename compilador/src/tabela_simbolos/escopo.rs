use crate::tabela_simbolos::simbolo::Simbolo;
use crate::analisador_lexico::tipo_token::*;

use prettytable::Table;
use colored::*;

#[derive(Clone, PartialEq)]
pub struct Escopo {
    entradas: Vec<Simbolo>,
    subescopos: Vec<usize>,
    escopo_pai: usize,
    escopo_num: usize,
}

impl Escopo {
    pub fn novo() -> Escopo {
        Escopo {
            entradas: Vec::new(),
            subescopos: Vec::new(),
            escopo_pai: 0,
            escopo_num: 0,
        }
    }

    pub fn add_entrada(&mut self, s: Simbolo) {
        for i in 0..self.entradas.len() {
            match self.entradas[i].clone() {
                Simbolo::Var(n, _t, _l, _a, _b) => match s.clone() {
                    Simbolo::Var(n1, _t1, _l1, _a, _b) => {
                        if n1 == n {
                            panic!("escopo.rs 1");
                        }
                    }

                    Simbolo::Func(n1, _t1, _l1, _a1, _b1, _) => {
                        if n1 == n {
                            panic!("escopo.rs 2");
                        }
                    }
                },
                Simbolo::Func(n, _t, _l, _a, _b, _c) => match s.clone() {
                    Simbolo::Var(n1, _t1, _l1, _, _d) => {
                        if n1 == n {
                            panic!("escopo.rs 3");
                        }
                    }

                    Simbolo::Func(n1, _t1, _l1, _a1, _b1, _) => {
                        if n1 == n {
                            panic!("função {} já declarada na linha {}", n1, _b);
                        }
                    }
                },
            }
        }
        self.entradas.push(s);
    }

    pub fn add_subescopo(&mut self, e: usize) {
        self.subescopos.push(e);
    }

    pub fn add_como_pai(&mut self, escopo_pai: usize) {
        self.escopo_pai = escopo_pai;
    }

    //TODO:
    pub fn _pai(&self) -> usize {
        self.escopo_pai
    }

    //TODO:
    pub fn _escopo_num(&self) -> usize {
        self.escopo_num
    }

    pub fn set_escopo_num(&mut self, i: usize) {
        self.escopo_num = i;
    }

    pub fn _pegar(&self, _indice: usize) -> Option<Simbolo> {
        None
            //TODO:
    }

    pub fn _atualizar(&mut self, _indice: usize, _simbolo: Simbolo) -> bool {
        false
            //TODO:
    }

    pub fn printar(&self) {

        if self.entradas.len() == 0 { return; }

        println!("\n\nEscopo numero {}, filho de {}", self.escopo_num.to_string().green(), self.escopo_pai.to_string().purple());
        let mut table = Table::new();
        table.set_titles(row!["NOME", "TIPO", "F/V", "PARAMS", "LINHA", "ENTRADA"]);
        //table.add_row(row!["NOME", "TIPO", "F/V", "PARAMS", "LINHA", "ENTRADA"]);

        let mut nome_func: String = "".into();

        for i in 0..self.entradas.len() {

            match self.entradas[i].clone() {
                Simbolo::Func(nome, tipo, n_params, _params, linha, entrada) => {
                    table.add_row(row![nome, tipo, "F", n_params, linha, entrada]);
                }
                Simbolo::Var(nome, tipo, linha, func_nome, entrada) => {
                    table.add_row(row![nome, tipo, "V", "-----", linha, entrada]);
                    nome_func = func_nome.clone();
                }
            }

        }

        //table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        //table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.printstd();
        println!("\nEscopo de {}", nome_func.underline());
        println!("\n\n");
    }
}

#[derive(Clone)]
pub struct ListaEscopo {
    escopos: Vec<Escopo>,
}

impl ListaEscopo {
    pub fn nova() -> ListaEscopo {
        let mut t = Vec::new();
        t.push(Escopo::novo());
        ListaEscopo { escopos: t }
    }

    pub fn add_escopo(&mut self, indice: usize, e: Escopo) {
        for i in 0..self.escopos.len() {
            if self.escopos[i].escopo_num == indice {
                self.escopos[i].add_subescopo(e.escopo_num);
            }
        }

        self.escopos.push(e);
    }

    pub fn pai_de(&self, indice: usize) -> usize {
        for i in 0..self.escopos.len() {
            if self.escopos[i].escopo_num == indice {
                return self.escopos[i].escopo_pai;
            }
        }

        panic!("aaaaaaaaa");
    }

    pub fn add_simbolo_no_indice(&mut self, indice: usize, s: Simbolo) {
        for i in 0..self.escopos.len() {
            if self.escopos[i].escopo_num == indice {
                self.escopos[i].add_entrada(s);
                return;
            }
        }

        panic!("asaaad");
    }

    pub fn printar(&self) {
        for i in 0..self.escopos.len() {
            self.escopos[i].printar();
        }
    }

    pub fn lookup(&self, lexema: String, escopo: usize) -> Option<Tipo_Token> {

        for i in 0..self.escopos.len() {
            if self.escopos[i].escopo_num == escopo {

                let mut escopo_observado : usize = escopo;
                let mut ultimo_escopo_observado : usize;

                loop {

                    for j in 0..self.escopos[escopo_observado].entradas.len() {
                        match self.escopos[escopo_observado].entradas[j].clone() {
                            Simbolo::Var(a, b, _c, _d, _e) => if a == lexema { return Some(b) },
                            Simbolo::Func(a, b, _c, _d, _e, _f) => if a == lexema {return Some(b) },
                        }
                    }

                    ultimo_escopo_observado = escopo_observado;
                    escopo_observado = self.pai_de(escopo_observado);

                    if ultimo_escopo_observado == escopo_observado {
                        return None;
                    }

                }
            }
        }



        return None;
    }


    pub fn lista_params(&self, lexema: String) -> Option<Vec<Tipo_Token>> {

        for j in 0..self.escopos[0].entradas.len() {
            match self.escopos[0].entradas[j].clone() {
                Simbolo::Var(a, _b, _c, _d, _e) => if a == lexema { panic!("{} não é uma função", lexema); },
                Simbolo::Func(a, _b, _c, d, _e, _f) => if a == lexema {return Some(d) },
            }

        }

        None
    }

    pub fn existe(&self, s : Simbolo, escopo : usize) -> bool {

        for i in 0..self.escopos.len() {
            if self.escopos[i].escopo_num == escopo {

                let mut escopo_observado : usize = escopo;
                let mut ultimo_escopo_observado : usize;

                loop {

                    for j in 0..self.escopos[escopo_observado].entradas.len() {
                        match self.escopos[escopo_observado].entradas[j].clone() {

                            Simbolo::Var(a, _b, _c, _d, _e) => {
                                match s.clone() {
                                    Simbolo::Var(n, _, _, _, _) => if n == a { return true; },
                                    Simbolo::Func(n, _, _, _, _, _) => if n == a {return true; },
                                }
                            },
                            Simbolo::Func(a, _b, _c, _d, _e, _f) => {
                                match s.clone() {
                                    Simbolo::Var(n, _, _, _, _) => if n == a { return true; },
                                    Simbolo::Func(n, _, _, _, _, _) => if n == a {return true; },
                                }
                            },
                        }
                    }

                    ultimo_escopo_observado = escopo_observado;
                    escopo_observado = self.pai_de(escopo_observado);

                    if ultimo_escopo_observado == escopo_observado {
                        return false;
                    }

                }
            }
        }

        return false;
    }
}
