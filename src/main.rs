use csv::{ ReaderBuilder, StringRecord };
use std::collections::{ HashMap };
use std::{fs};

const FILE_NAME: &str = "history.csv";

#[derive(Debug)]
struct DatoHistoria {
    tipo_dato: String,
    texto: String,
    tag: String,
    vida: i32,
    opciones: Vec<DatoHistoria>,
}

impl DatoHistoria {
    fn new(row: StringRecord) -> DatoHistoria {
        
        let vida = row.get(3).unwrap().trim();
        let vida : i32 = vida.parse().unwrap_or(0);

        let dato_historia = DatoHistoria {
            tipo_dato : row.get(0).unwrap().trim().to_string(),
            tag       : row.get(1).unwrap().trim().to_string(),
            texto     : row.get(2).unwrap().trim().to_string(),
            opciones  : vec![],
            vida,
        };
        dato_historia
    }
}

fn main() {

    let mut vida: i32 = 100;
    let mut current_tag = "INICIO".to_string();
    let mut last_record : String = String::new();

    let content = fs::read_to_string(FILE_NAME).unwrap();

    let mut rdr = ReaderBuilder::new()
                                    .delimiter(b';')
                                    .from_reader(content.as_bytes());

    let mut datos_historia: HashMap<String, DatoHistoria> = HashMap::new();

    for result in rdr.records() {
        let result = result.unwrap();
        let data = DatoHistoria::new(result);

        if data.tipo_dato == "SITUACION" {

            let record_tag = data.tag.clone();

            datos_historia.insert(record_tag.clone(), data);
            last_record = record_tag;

        } else if data.tipo_dato == "OPCION" {
            if let Some(data_selected) = datos_historia.get_mut(&last_record) {
                ( *data_selected ).opciones.push(data);
            }
        }

    }


    loop {
        
        println!("Tienes {} de vida,", vida);

        if let Some(data) = datos_historia.get(&current_tag) {

            println!("{:?}", data.texto);

            for ( index, option) in data.opciones.iter().enumerate() {
                println!("[{}] {}", index, option.texto)
            }

            let mut user_input = String::new();
            std::io::stdin().read_line(&mut user_input).unwrap();

            let option_selected = user_input.trim().parse().unwrap_or(99);

            if let Some( option ) = &data.opciones.get(option_selected) {
                current_tag = option.tag.clone();
            } else {
                println!("Comando no valido");
            }
            println!("data.vida {}", data.vida);

            vida += data.vida;
            println!("");
        } else {
            println!("Fallo");
            break;
        }
        if vida <= 0 { 
            println!("Has perdido");
            break; 
        }

    }

    println!("Chao");

}
