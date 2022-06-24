use csv::{ ReaderBuilder, StringRecord };
use std::{fs, result};

const FILE_NAME: &str = "history.csv";

struct DatoHistoria {
    tipo_dato: String,
    texto: String,
    tag: String,
    vida: i32,
}

impl DatoHistoria {
    fn new(row: StringRecord) -> DatoHistoria {
        
        let vida: i32 = row.get(3).unwrap().trim().parse().unwrap_or(0);
        let dato = DatoHistoria {
            tipo_dato : row.get(0).unwrap().to_string(),
            tag       : row.get(1).unwrap().to_string(),
            texto     : row.get(2).unwrap().to_string(),
            vida,
        };
        dato
    }
}

fn main() {

    let content = fs::read_to_string(FILE_NAME).unwrap();
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    let mut datos_historia: Vec<DatoHistoria> = vec![];

    for result in rdr.records() {
        
        let result = result.unwrap();
        
        let dato = DatoHistoria::new(result);

        datos_historia.push(dato);
    }

}
