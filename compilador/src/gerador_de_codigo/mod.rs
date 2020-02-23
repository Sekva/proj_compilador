use crate::analisador_lexico::tipo_token::Tipo_Token;
use crate::otimizador::otimizador::TipoComando;

use std::collections::HashMap;

fn strip_characters(original: &str, to_strip: &str) -> String {
    let mut result = String::new();
    for c in original.chars() {
        if !to_strip.contains(c) {
            result.push(c);
        }
    }
    result
}

/* O codigo fica cheio de ret, poderia por um só no final e dar jump pra ele */

pub struct Codigo {
    data_section: Vec<String>,
    bss_section: Vec<String>,
    text_section: Vec<String>,
    t_addr: Vec<(String, TipoComando, Tipo_Token)>,
}

impl Codigo {
    pub fn novo() -> Codigo {
        Codigo {
            data_section: Vec::new(),
            bss_section: Vec::new(),
            text_section: Vec::new(),
            t_addr: Vec::new(),
        }
    }

    pub fn gerar(&mut self, codigo: Vec<(String, TipoComando, Tipo_Token)>) -> String {
        self.t_addr = codigo.clone();
        self.preparar();

        for i in self.t_addr.clone() {
            match i.1 {
                TipoComando::Funcao => self.text_section.push(i.0),
                TipoComando::Label => self.text_section.push(i.0),
                TipoComando::Ret => self.text_section.push("ret1".into()),
                TipoComando::Call => {
                    let mut iter = i.0.split_whitespace();
                    iter.next();
                    let cod = strip_characters(iter.next().unwrap(), ",");
                    self.text_section.push(format!("call {}", cod));
                }
                TipoComando::OpenFrame => self.text_section.push("enter".into()),
                TipoComando::CloseFrame => self.text_section.push("leave".into()),
                TipoComando::Op => {
                    if i.2 == Tipo_Token::ID_INT {
                        let str3 = i.0.split(" ").collect::<Vec<&str>>();
                        self.text_section.push(format!("mov rax, {}", str3[2]));
                        self.text_section.push(format!("mov rdx, {}", str3[4]));

                        match str3[3] {
                            "+" => self.text_section.push(format!("add rax, rdx")),
                            "-" => self.text_section.push(format!("sub rax, rdx")),
                            _ => {}
                        }

                        self.text_section.push(format!("mov {}, rax", str3[4]));
                    }
                }

                _ => {}
            }
        }

        let mut ret_str = "".to_string();

        for i in self.text_section.clone() {
            ret_str = format!("{}\n{}", ret_str, i);
        }

        return ret_str;
    }

    fn preparar(&mut self) {
        self.acertar_nomes();
        self.text_section.push("section .text".into());
        self.text_section.push("global FUNC__CALL__start".into());
    }

    fn acertar_nomes(&mut self) {
        let mut frames = Vec::new();
        let mut frame = Vec::new();
    }
}
