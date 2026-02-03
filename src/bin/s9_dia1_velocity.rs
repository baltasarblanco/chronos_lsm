use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock}; // El secreto de la velocidad
use std::thread;
use std::time::Duration;

struct Engine {
    map: HashMap<String, String>,
    log_file: File,
}

impl Engine {
    fn new(filepath: &str) -> io::Result<Self> {
        let path = filepath.to_string();
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&path)?;

        let mut map = HashMap::new();
        println!("   📜 Rehidratando memoria desde '{}'...", path);
        let reader = BufReader::new(file.try_clone()?);

        for line in reader.lines() {
            if let Ok(record) = line {
                if let Some((k, v)) = record.split_once(',') {
                    map.insert(k.to_string(), v.to_string());
                }
            }
        }
        println!("   ✅ Memoria restaurada: {} registros.", map.len());
        Ok(Engine {
            map,
            log_file: file,
        })
    }

    fn set(&mut self, key: &str, value: &str) -> io::Result<()> {
        self.map.insert(key.to_string(), value.to_string());
        writeln!(self.log_file, "{},{}", key, value)?;
        Ok(())
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }
}

// 🧬 EL ADN CONCURRENTE
type Db = Arc<RwLock<Engine>>;

fn handle_client(mut stream: TcpStream, db: Db) {
    let mut buffer = [0; 512];
    // Manejo seguro de la dirección del cliente
    let peer_addr = match stream.peer_addr() {
        Ok(addr) => addr.to_string(),
        Err(_) => "Unknown".to_string(),
    };

    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                } // Conexión cerrada

                let raw_msg = String::from_utf8_lossy(&buffer[..bytes_read]);
                let command_line = raw_msg.trim();
                if command_line.is_empty() {
                    continue;
                }

                println!("   📩 [{}]: '{}'", peer_addr, command_line);
                let parts: Vec<&str> = command_line.split_whitespace().collect();

                let response = match parts.as_slice() {
                    ["SET", key, value] => {
                        // 🔒 WRITE LOCK (Exclusivo - Bloquea todo)
                        // Usamos .write().unwrap() para obtener acceso mutable
                        let mut engine = db.write().unwrap();
                        match engine.set(key, value) {
                            Ok(_) => format!("💾 OK. Guardado '{}'.\n", key),
                            Err(e) => format!("🔥 ERROR: {}\n", e),
                        }
                    }

                    ["GET", key] => {
                        // 🔓 READ LOCK (Compartido - Permite otros lectores)
                        // Usamos .read().unwrap() para obtener acceso de lectura
                        let engine = db.read().unwrap();

                        // 🐢 SIMULACIÓN DE CARGA (La Tortuga)
                        if *key == "heavy" {
                            println!("   🐢 [Hilo] Iniciando operación pesada (5s)...");
                            thread::sleep(Duration::from_secs(5));
                            println!("   🐇 [Hilo] Operación pesada terminada.");
                        }

                        match engine.get(key) {
                            Some(v) => format!("💎 VALOR: '{}'\n", v),
                            None => format!("🤷‍♂️ No existe '{}'\n", key),
                        }
                    }

                    ["PING"] => "PONG\n".to_string(),

                    ["EXIT"] => {
                        break;
                    }

                    _ => "❌ ERROR: Comando desconocido\n".to_string(),
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
    let engine = Engine::new("chronos_v3.db").expect("Fallo DB");
    // Envolvemos el motor en Arc<RwLock<...>>
    let global_db = Arc::new(RwLock::new(engine));

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("-----------------------------------------");
    println!("🚀 KLYNTAR v3.0 (HIGH PERFORMANCE) ACTIVO");
    println!("   Modo: Read-Write Lock (Concurrencia Real)");
    println!("-----------------------------------------");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let db_clone = Arc::clone(&global_db);
                thread::spawn(move || {
                    handle_client(stream, db_clone);
                });
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
