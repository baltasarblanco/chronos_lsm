use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    loop {
        // Leemos del stream. Esto "bloquea" hasta que llega data.
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("   👋 El cliente colgó el teléfono.");
                    break;
                }

                // 1. Limpieza de datos
                // Convertimos bytes a String
                let raw_msg = String::from_utf8_lossy(&buffer[..bytes_read]);
                // .trim() quita los espacios y saltos de línea basura del final
                let command_line = raw_msg.trim();

                println!("   📩 COMANDO RECIBIDO: '{}'", command_line);

                // 2. EL CEREBRO (PARSER)
                // Dividimos la frase en palabras: "SET clave_valor" -> ["SET", "clave", "valor"]
                let parts: Vec<&str> = command_line.split_whitespace().collect();

                let response = match parts.as_slice() {
                    ["SET", key, value] => format!("✅ OK. Guardando '{}' -> '{}'\n", key, value),
                    ["GET", key] => format!("🔍 Buscando valor para '{}' ... \n", key),
                    ["PING"] => "PONG\n".to_string(),
                    ["EXIT"] | ["QUIT"] => {
                        let _ = stream.write_all(b"Bye!\n");
                        break; // El cliente pide salir
                    }
                    _ => format!("❌ ERROR: No entiendo '{}'\n", command_line),
                };

                // Respondemos y VOLVEMOS AL INICIO DEL LOOP esperando el siguiente comando
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("   ❌ Error respondiendo: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("   ❌ Error de la conexion: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("-----------------------------------------");
    println!("🤖 KLYNTAR v1.1 (MODO CHAT) ACTIVO");
    println!("   Usa: nc 127.0.0.1 8080");
    println!("-----------------------------------------");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("   ✨ ¡Nuevo cliente conectadoa!");
                handle_client(stream);
            }
            Err(e) => println!("Error de conexion: {}", e),
        }
    }
}
