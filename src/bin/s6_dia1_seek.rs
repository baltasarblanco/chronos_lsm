use std::fs::OpenOptions;
use std::io::{Write, Read, Seek, SeekFrom};

fn main() {
    let ruta = "index_test.log";
    println!(">>> INICIANDO PRUEBAS DE NAVEGACIÓN EN DISCO <<<");

    // 1. ESCRITURA CON RASTREO (MAPPING)
    // Vamos a guardar donde empieza cada mensaje.
    let mut offsets = Vec::new(); // Aqui guardaremos las posiciones

    {
        let mut archivo = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true) // Borrar contenido viejo para empezar limpio
            .open(ruta)
            .unwrap();

        // MENSAJE 1
        let pos1 = archivo.stream_position().unwrap(); // ¿Donde estoy? (Byte 0)
        offsets.push(pos1);
        archivo.write_all(b"PrimerMensaje").unwrap();

        // MENSAJE 2
        let pos2 = archivo.stream_position().unwrap(); // ¿Donde estoy ahora?
        offsets.push(pos2);
        archivo.write_all(b"SECRETO_NUCLEAR").unwrap();

        // MENSAJE 3
        let pos3 = archivo.stream_position().unwrap();
        offsets.push(pos3);
        archivo.write_all(b"TercerMensaje").unwrap();

        println!("> Mapa generado: {:?}", offsets);
        // Deberia ser algo como [0, 13, 28] (dependiendo del largo del texto)
    }

    //  2. LECTURA QUIRÚRGICA (SEEKING)
    // Queremos leer SOLO el "SECRETO_NUCLEAR" (Mensaje 2).
    // No queremos leer el 1 ni el 3.
    {
        println!("\n>>> Intentando extracción quirúrgica del Mensaje 2...");
        let mut archivo = std::fs::File::open(ruta).unwrap();
        
        // A. OBTENEMOS LA COORDENADA
        let posicion_objetivo = offsets[1]; // El segundo elemento
        println!("> Saltando directamente al byte: {}", posicion_objetivo);
    
        // B. EL SALTO NINJA
        archivo.seek(SeekFrom::Start(posicion_objetivo)).unwrap();

        // C. LEEMOS (Sabemos que mide 15 bytes 'SECRETO_NUCLEAR')
        let mut buffer = [0u8; 15];
        archivo.read_exact(&mut buffer).unwrap();

        let texto = String::from_utf8_lossy(&buffer);
        println!("> ¡Dato extraído: '{}'", texto);
    }
}