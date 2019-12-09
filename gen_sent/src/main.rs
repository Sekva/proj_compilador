use rand::seq::SliceRandom;
use rand::Rng;

use std::fs::File;
use std::io::{prelude::*, BufReader};

static mut LIMITADOR: u64 = 0;
static mut LIMITE: u64 = 4000;

struct Regra {
    regra: String,
    derivacoes: Vec<String>,
}

impl Regra {
    fn derivacao_aleatoria(&self) -> String {
        let num: f64 = rand::thread_rng().gen_range(-50.0, 50.0);

        unsafe {
            if num > 30.0 || LIMITADOR > LIMITE {
                for i in self.derivacoes.iter() {
                    if &(*i.trim()) == "EPSILON" {
                        return (*i.as_str()).into();
                    }
                }
            }
        }

        self.derivacoes
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string()
    }

    fn nome(&self) -> String {
        self.regra.as_str().to_string()
    }
}

fn remover_vazio(palavra: String) -> String {
    palavra.replace("EPSILON", "")
}

fn espaco(palavra: String) -> String {
    palavra.replace("  ", " ")
}

fn quebra(palavra: String) -> String {
    palavra
        .replace(";", ";\n")
        .replace("}", "}\n")
        .replace("{", "{\n")
        .replace("~ ~", "")
        .replace("- -", "")
        .replace("! !", "")
}

fn expandir(palavra: String, regras: &Vec<Regra>) -> (bool, String) {
    for i in regras.iter() {
        if palavra.contains(&(*i.nome())) {
            return (
                true,
                palavra.replacen(&(*i.nome()), &(*i.derivacao_aleatoria()), 1),
            );
        }
    }

    (false, palavra)
}

fn main() {
    let mut regras = Vec::new();

    let file = File::open("../ansi_c_modificado_EBNF").unwrap();

    for line in BufReader::new(file).lines() {
        let linha = String::from(line.unwrap());

        let v: Vec<&str> = linha.split("->").collect();

        let nome_regra = String::from(String::from(v[0]).trim());

        let derivadas = String::from(v[1]);

        let mut derivadas_lista: Vec<String> = Vec::new();

        let t: Vec<&str> = derivadas.split('|').collect();

        for i in t {
            derivadas_lista.push(String::from(i));
        }

        regras.push(Regra {
            regra: nome_regra,
            derivacoes: derivadas_lista,
        });
    }

    regras.reverse();

    let mut palavra: String = String::from("<Func_Decl>");

    let mut variaveis: bool;

    let (variaveis2, palavra2) = expandir(palavra, &regras);

    variaveis = variaveis2;
    palavra = palavra2;

    while variaveis {
        println!("");
        println!("{}", palavra);
        println!("");

        unsafe {
            println!(" llllllllllllll {}", LIMITADOR);
        }

        let (variaveis2, palavra2) = expandir(palavra, &regras);

        variaveis = variaveis2;
        palavra = palavra2;

        palavra = remover_vazio(palavra);
        palavra = espaco(palavra);

        print!("\r{}", palavra.len());
        regras.shuffle(&mut rand::thread_rng());

        unsafe {
            LIMITADOR = LIMITADOR + 1;
        }
    }

    palavra = quebra(palavra);
    palavra = remover_vazio(palavra);
    println!("");
    println!("{}", palavra);
    println!("");
}
