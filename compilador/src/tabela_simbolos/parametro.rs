#[derive(Clone,PartialEq)]
pub struct Parametro {
    nome : String,
    tipo : String,
}


#[derive(Clone,PartialEq)]
pub struct Parametros {
    lista : Vec<Parametro>,
}

impl Parametros {
    pub fn novo() -> Parametros {
        Parametros {
            lista : Vec::new(),
        }
    }
}


