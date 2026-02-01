use std::collections::HashMap; // <---- LA ESTRELLA DE HOY
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Registro {
    clave: String,
    valor: String,
}

fn main() {
    let ruta = "chronos_indexed.db";
    println!(">>> INICIANDO SIMULACIÓN DE ÍNDICE (RAM + DISCO) <<<");

    // 1. EL ÍNDICE EN MEMORIA (Nuestra "Libreta")
    // Mapea String (Clave) -> u64 (Posicion en byte)
    let mut indice: HashMap<String, u64> = HashMap::new();

    // 2. FASE DE ESCRITURA (POPULANDO LA BASE DE DATOS)
    {
        let mut archivo = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(ruta)
            .unwrap();

        // Datos a insertar
        let datos = vec![
            Registro { clave: "user_1".into(), valor: "Baltasar".into() },
            Registro { clave: "config_theme".into(), valor: "Dark Mode".into() },
            Registro { clave: "server_status".into(), valor: "Online".into() },
    
        ];
        
        
        // BUCLE para guardar datos (user_1, config_theme, etc.)
        for dato in datos {

            // A. PREGUNTAMOS: ¿DÓNDE ESTÁ EL CURSOR? == ¿DONDE ESTOY PARADO? 
            let posicion_actual = archivo.stream_position().unwrap();

            // B. GUARDAMOS EN EL ÍNDICE (RAM) == ANOTAR EN LA LIBRETA (RAM)
            // "La clave X vive en la posición Y"
            indice.insert(dato.clave.clone(), posicion_actual);

            // C. ESCRIBIMOS EN DISCO
            bincode::serialize_into(&archivo, &dato).unwrap();

            println!("> Indexeando: '{}' -> Byte {}", dato.clave, posicion_actual);

        }
    }


    println!("\n>>> BASE DE DATOS CERRADA. ÍNDICE EN MEMORIA LISTO: {:?}", indice);

    // 3. FASE DE LECTURA RÁPIDA (Consulta)
    // Imagina que esto ocurre horas despues. Solo tenemos el archivo y el indice en RAM.
    {
        let mut archivo = std::fs::File::open(ruta).unwrap();
        let clave_buscada = "config_theme"; // <--- Queremos ESTE dato especifico

        println!("\n>>>> BUSCANDO CLAVE: '{}'...", clave_buscada);

        // PASO A: CONSULTA EL MAPA (RAM) - Velocidad: 0(1)
        match indice.get(clave_buscada) {
            Some(&offset) => {
                println!("> ¡Encontrado en índice! Saltando al byte: {}", offset);

                // PASO B: SALTO FÍSICO (DISC SEEK)
                archivo.seek(SeekFrom::Start(offset)).unwrap();

                // PASO C: LEER OBJETO (DESERIALIZE)
                // deserialize_from lee del archivo hasta completar el struct
                let resultado: Registro = bincode::deserialize_from(&archivo).unwrap();

                println!("> VALOR RECUPERADO: '{}'", resultado.valor);
                println!("> ✅ ÉXITO. Lectura quirúrgica completada.");
            
            },
            None => {
                println!("> ❌ Clave no encontrada en el índice.");

            }
        }
}
}
