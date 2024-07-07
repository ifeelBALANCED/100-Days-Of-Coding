use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Regiao {
    pub id: i32,
    pub sigla: String,
    pub nome: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UF {
    pub id: i32,
    pub sigla: String,
    pub nome: String,
    pub regiao: Regiao,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mesorregiao {
    pub id: i32,
    pub nome: String,
    pub UF: UF,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Microrregiao {
    pub id: i32,
    pub nome: String,
    pub mesorregiao: Mesorregiao,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegiaoIntermediaria {
    pub id: i32,
    pub nome: String,
    pub UF: UF,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegiaoImediata {
    pub id: i32,
    pub nome: String,
    #[serde(rename = "regiao-intermediaria")]
    pub regiao_intermediaria: RegiaoIntermediaria,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Municipio {
    pub id: i32,
    pub nome: String,
    pub microrregiao: Microrregiao,
    #[serde(rename = "regiao-imediata")]
    pub regiao_imediata: RegiaoImediata,
}