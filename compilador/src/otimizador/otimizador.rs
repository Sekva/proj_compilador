use crate::analisador_lexico::tipo_token::*;

use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum TipoComando {
    Goto,
    Label,
    Stm,
    Ret,
    If,
    Printk,
    Call,
    Funcao,
    Op,
    Nop,
    Assign,
    Assign_POP,
    Param,
    CloseFrame,
    OpenFrame,
}

impl fmt::Display for TipoComando {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

fn strip_characters(original: &str, to_strip: &str) -> String {
    let mut result = String::new();
    for c in original.chars() {
        if !to_strip.contains(c) {
            result.push(c);
        }
    }
    result
}

pub struct Otimizador {
    programa: Vec<(String, TipoComando, Tipo_Token)>,
    nivel: usize,
}

impl Otimizador {
    pub fn novo(nivel: usize, programa: Vec<(String, TipoComando, Tipo_Token)>) -> Otimizador {
        Otimizador {
            programa: programa,
            nivel: nivel,
        }
    }

    pub fn otimizar(&mut self) -> Vec<(String, TipoComando, Tipo_Token)> {
        if self.nivel == 1 {
            self.remover_rets(); // tira um ret acima do outro
            self.remover_gotos_desnecessarios(); //gera labels nao usados
            self.remover_labels_nao_usados(); // tira labels q nunca são usados
            self.gotos_non_sense();
        }

        self.programa.clone()
    }

    fn remover_rets(&mut self) {
        let mut novo_programa: Vec<(String, TipoComando, Tipo_Token)> = Vec::new();
        for i in 1..self.programa.len() {
            if !(self.programa[i].1 == TipoComando::Ret
                && self.programa[i - 1].1 == TipoComando::Ret)
            {
                novo_programa.push(self.programa[i].clone());
            }
        }
        self.programa = novo_programa;
    }

    fn remover_gotos_desnecessarios(&mut self) {
        let mut i = 1;

        while i < self.programa.len() {
            if self.programa[i].1 == TipoComando::Label
                && self.programa[i - 1].1 == TipoComando::Label
            {
                let para_ficar = self.programa[i].0.split_whitespace().next().unwrap();
                let para_remover = self.programa[i - 1].0.split_whitespace().next().unwrap();

                let para_ficar_ok = strip_characters(&para_remover.clone(), ":");
                let para_remover_ok = strip_characters(&para_ficar.clone(), ":");

                for j in 0..self.programa.len() {
                    if self.programa[j].1 == TipoComando::Goto {
                        let mut iter = self.programa[j].0.split_whitespace();
                        iter.next();
                        let lido = iter.next().unwrap();

                        if lido == para_remover_ok {
                            self.programa[j].0 = format!("GOTO {}", para_ficar_ok);
                        }
                    }
                }
            }

            i += 1;
        }
    }

    fn remover_labels_nao_usados(&mut self) {
        let mut i = 1;
        let mut tamanho = self.programa.len();

        while i < tamanho {
            if self.programa[i].1 == TipoComando::Label {
                let mut achado = false;

                let label =
                    strip_characters(self.programa[i].0.split_whitespace().next().unwrap(), ":");

                let mut j = 0;
                let mut tamanho2 = self.programa.len();
                while j < tamanho2 {
                    if self.programa[j].1 == TipoComando::Goto {
                        let mut iter = self.programa[j].0.split_whitespace();
                        iter.next();
                        let lido = iter.next().unwrap();

                        if lido == label {
                            achado = true;
                        }
                    }

                    if self.programa[j].1 == TipoComando::If {
                        let mut iter = self.programa[j].0.split_whitespace();
                        iter.next();
                        iter.next();
                        iter.next();
                        let lido = iter.next().unwrap();

                        if lido == label {
                            achado = true;
                        }
                    }

                    j += 1;
                    tamanho2 = self.programa.len();
                }

                if !achado {
                    self.programa.remove(i);
                }
            }

            i += 1;
            tamanho = self.programa.len();
        }
    }

    fn gotos_non_sense(&mut self) {
        let mut i = 1;
        let mut tamanho = self.programa.len();

        while i < tamanho {
            if self.programa[i].1 == TipoComando::Goto {
                let mut iter = self.programa[i].0.split_whitespace();
                iter.next();
                let label_goto = iter.next().unwrap();

                if self.programa[i + 1].1 == TipoComando::Label {
                    let iter2 = self.programa[i + 1].0.split_whitespace().next().unwrap();
                    let label = strip_characters(&iter2.clone(), ":");

                    if label_goto == label {
                        self.programa.remove(i);
                    }
                }
            }

            i += 1;
            tamanho = self.programa.len();
        }
    }
}
