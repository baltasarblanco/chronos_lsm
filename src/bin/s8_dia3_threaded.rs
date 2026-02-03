use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread; // IMPORTANTE: La libreria de los hilos!!

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    // Obtenemos la direccion IP para saber quién es quién en los logs
    let peer_addr = stream.peer_addr().unwrap();
    println!("   🧵 [Hilo nuevo] Atendiendo a: {}", peer_addr);

    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("   👋 [{}] Colgó.", peer_addr);
                    break;
                }
                let raw_msg = String::from_utf8_lossy(&buffer[..bytes_read]);
                let command_line = raw_msg.trim();
                if command_line.is_empty() {
                    continue;
                }

                println!("   📩 [De {}]: '{}'", peer_addr, command_line);

                let parts: Vec<&str> = command_line.split_whitespace().collect();
                let response = match parts.as_slice() {
                    ["SET", key, value] => format!("✅ OK. Guardando '{}' -> '{}'\n", key, value),
                    ["GET", key] => format!("🔍 Buscando valor para '{}'...\n", key),
                    ["PING"] => "PONG\n".to_string(),
                    ["EXIT"] => {
                        let _ = stream.write(b"Bye!\n");
                        break;
                    }
                    _ => format!("❌ ERROR: No entiendo '{}'\n", command_line),
                };

                if let Err(_) = stream.write_all(response.as_bytes()) {
                    break;
                }
            }
            Err(_) => {
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("-----------------------------------------");
    println!("🕷️ KLYNTAR v1.2 (MULTITHREADED) ACTIVO");
    println!("   Capacidad: Ilimitada (hasta que reviente la RAM)");
    println!("-----------------------------------------");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("   ✨ ¡Nueva conexión! Desplegando clon...");

                //  🔥 LA MAGIA OCURRE AQUI 🔥
                // thread::spawn crae un universo paralelo para este clinete
                // 'move' significa: "Toma la variable 'stream' y llevatela  contigo, ya no es mia".
                thread::spawn(move || {
                    handle_client(stream);
                });

                // El bucle principal 'for' termina inmediatamente esta vuelta
                // y vuelve arriba a esperar al siguiente cliente.
                // NO SE BLOQUEA.
            }
            Err(e) => println!("Error de conexión: {}", e),
        }
    }
}
