use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

// Definimos un alias para no escribir todo este chorizo cada vez
// Una Base de Datos es: Un puntero compartido (Arc) a un Semáforo (Mutex) que protege un Mapa.
type Db = Arc<Mutex<HashMap<String, String>>>;

fn handle_client(mut stream: TcpStream, db: Db) {
    let mut buffer = [0; 521];

    // IP para logs
    let peer_addr = stream.peer_addr().unwrap();

    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }

                let raw_msg = String::from_utf8_lossy(&buffer[..bytes_read]);
                let command_line = raw_msg.trim();
                if command_line.is_empty() {
                    continue;
                }

                println!("  📩 [{}]: '{}'", peer_addr, command_line);

                let parts: Vec<&str> = command_line.split_whitespace().collect();

                let response = match parts.as_slice() {
                    // CASO SET: Escribir en el cerebro compartido
                    ["SET", key, value] => {
                        // 1. Pedimos permiso al semáforo (.lock())
                        // 2. Si hay panico en otro hilo .unrwap() nos avisa (ignoremos eso x ahora)
                        let mut map = db.lock().unwrap();

                        // 3. Escribimos (ahora somos dueños exclusivos del mapa por unos micronosegundos)
                        map.insert(key.to_string(), value.to_string());

                        // 4. Al terminar este bloque, el Mutex se libera solo (Drop)
                        format!("✅ Ok. Guardado '{}' en la Mente Colmena.\n", key)
                    }

                    // CASO GET: Leer del cerebro compartido
                    ["GET", key] => {
                        // 1. Bloqueamos para leer (seguridad ante todo)
                        let map = db.lock().unwrap();

                        match map.get(*key) {
                            Some(v) => format!("💎 VALOR ENCONTRADO: '{}'\n", v),
                            None => format!("🤷‍♂️ No sé qué es '{}'\n", key),
                        }
                    }

                    ["PING"] => "PONG\n".to_string(),
                    ["EXIT"] => break,
                    _ => format!("❌ ERROR: '{}'? No hablo ese idioma.\n", command_line),
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

    // 1. CREAMOS EL CEREBRO ORIGIANL
    // Es un HashMap vacío, protegido por una Mutex, envuelvo en un Arc.
    let global_db: Db = Arc::new(Mutex::new(HashMap::new()));

    println!("-----------------------------------------");
    println!("🧠 KLYNTAR v1.3 (HIVE MIND) ACTIVO");
    println!("   Todos los hilos comparten el mismo cerebro.");
    println!("-----------------------------------------");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // 2. CLONAMOS EL PUNTERO (NO EL DATO)
                // Esto crea una "referencia" nueva al MISMO lugar en memoria.
                // Es barato  Y RÁPIDO!
                let db_clone = Arc::clone(&global_db);

                thread::spawn(move || {
                    // Le pasamos el clon al hilo
                    handle_client(stream, db_clone);
                });
            }
            Err(e) => println!("Error de conexión: {}", e),
        }
    }
}
