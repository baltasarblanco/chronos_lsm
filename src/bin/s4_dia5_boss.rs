use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};

// 1. LAS ESTRUCTURAS 
// Añadimos #[derive(Serialize, Deserialize)]
// Esto le enseña a Rust cómo convertir estos datos a ceros y unos AUTOMÁTICAMENTE.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Evento {
    timestamp: u64,
    tipo: String, // "SET", "DELETE"
    clave: String,
    valor: Option<String>, // Option porque un DELETE no tiene valor
}


fn main() {
    println!(">>> CHRONOS STORAGE ENGINE v0.1 <<<");

    let ruta_archivo = "chronos_db.bin";

    // --- FASE 1: ESCRITURA (Serialización) ---
    {

        println!(">>> Generando eventos en memoria...");
        let eventos = vec![
            Evento { timestamp: 1000, tipo: "SET".into(), clave: "user:1".into(), valor: Some("Batman".into()) },
            Evento { timestamp: 1005, tipo: "DELETE".into(), clave: "user:1".into(), valor: None }, // Tombstone
            Evento { timestamp: 1010, tipo: "SET".into(), clave: "user:2".into(), valor: Some("Robin".into()) },
    
        ];

        println!(">>> Guardando en disco (Formato Binario)...");
        // Creamos el archivo
        let archivo = File::create(ruta_archivo).unwrap();
        let buffer = BufWriter::new(archivo);

        // LA MAGIA DE BINCODE:
        // serialize_into(donde_escribir, que_escrbiir)
        bincode::serialize_into(buffer, &eventos).unwrap();

        println!(">>> ¡Datos persistidos con éxito!");

    } // Aquí 'eventos' muere y el archivo se cierra.

    // --- FASE 2: LECTURA (Deserialización) ---
    {

        println!("\n>>> Reiniciando sistema... leyendo disco...");

        // Abrimos el archivo
        let archivo = File::open(ruta_archivo).unwrap();
        let buffer = BufReader::new(archivo);

        // LA MAGIA INVERSA: 
        // Recuperamos el Vec<Eventos> completo desde los bytes crudos.
        let eventos_recuperados: Vec<Evento> = bincode::deserialize_from(buffer).unwrap();

        println!(">>> Datos recuperamos.");
        for e in eventos_recuperados {
            println!("  [T={}] {} Key='{}' Val={:?}", e.timestamp, e.tipo, e.clave, e.valor);
        }
    }
}


