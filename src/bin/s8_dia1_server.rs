use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

// Esta función maneja a cada cliente (navegeador, terminal, script)
fn handle_client(mut stream: TcpStream) {
    // 1. Preparamos un buffer (un array de bytes) para escuchar
    let mut buffer = [0; 512];

    // 2. Leemos lo que nos mandan
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            if bytes_read == 0 {
                return;
            } // Conexión fantasma

            // Convertimos bytes a texto para verlo en consola
            let request = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("   📩 MENSAJE RECIBIDO: {}", request);

            // 3. Respondemos (Handhake)
            // Si entras por navegador, necesitas cabeceras HTTP.
            // Si es terminal pura, basta con el texto
            let response = "HTTP/1.1 200 OK\r\n\r\nHola desde Klyntar Systems!";

            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => {
            eprintln!("   ❌ Error leyendo: {}", e);
        }
    }
}

fn main() {
    // 1. BIND: Abrimos el puerto 8080 en localhost
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("-----------------------------------------");
    println!("🕷️ KLYNTAR SERVER ACTIVO EN PUERTO 8080");
    println!("   Esperando conexiones...");
    println!("-----------------------------------------");

    // 2. Bucle infinito: El servidor nunca duerme
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("   ✨ ¡Alguien tocó la puerta!");
                handle_client(stream);
            }
            Err(e) => {
                println!("   💀 Conexión fallida: {}", e);
            }
        }
    }
}
