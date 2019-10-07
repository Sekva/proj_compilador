use crate::tabela_simbolos::simbolo::Simbolo;

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
                Simbolo::Var(n, _t, _l, _) => match s.clone() {
                    Simbolo::Var(n1, _t1, _l1, _) => {
                        if n1 == n {
                            panic!("1");
                        }
                    }

                    Simbolo::Func(n1, _t1, _l1, _a1, _b1) => {
                        if n1 == n {
                            panic!("2");
                        }
                    }
                },
                Simbolo::Func(n, _t, _l, _a, _b) => match s.clone() {
                    Simbolo::Var(n1, _t1, _l1, _) => {
                        if n1 == n {
                            panic!("3");
                        }
                    }

                    Simbolo::Func(n1, _t1, _l1, _a1, _b1) => {
                        if n1 == n {
                            panic!("4");
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

    pub fn pai(&self) -> usize {
        self.escopo_pai
    }

    pub fn escopo_num(&self) -> usize {
        self.escopo_num
    }

    pub fn set_escopo_num(&mut self, i: usize) {
        self.escopo_num = i;
    }

    pub fn pegar(&self, indice: usize) -> Option<Simbolo> {
        None
            //TODO:
    }

    pub fn atualizar(&mut self, indice: usize, simbolo: Simbolo) -> bool {
        false
            //TODO:
    }

    pub fn printar(&self) {

        if self.entradas.len() == 0 { return; }

        println!("\n\nEscopo numero {}, filho de {}", self.escopo_num.to_string().green(), self.escopo_pai.to_string().purple());
        let mut table = Table::new();
        table.set_titles(row!["NOME", "TIPO", "F/V", "PARAMS", "LINHA"]);
        //table.add_row(row!["NOME", "TIPO", "F/V", "PARAMS", "LINHA"]);

        let mut nome_func: String = "".into();

        for i in 0..self.entradas.len() {

            match self.entradas[i].clone() {
                Simbolo::Func(nome, tipo, n_params, _params, linha) => {
                    table.add_row(row![nome, tipo, "F", n_params, linha]);
                }
                Simbolo::Var(nome, tipo, linha, func_nome) => {
                    table.add_row(row![nome, tipo, "V", "-----", linha]);
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
}
