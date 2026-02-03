use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

// 🏛️ LA ESTRUCTURA DEL MOTOR
// Guarde el estado en RAM y el manejador del archivo en disco.
struct Engine {
    map: HashMap<String, String>,
    log_file: File,
}

impl Engine {
    // 🔥 EL RITUAL DE RESURRECIÓN (Recuperación)
    fn new(filepath: &str) -> io::Result<Self> {
        let path = filepath.to_string();

        // 1. Abrimos el archivo en modo "Append" (Agregar al final) y "Create" (Si no existe)
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&path)?;

        let mut map = HashMap::new();

        // 2. REPLAY: Leemos el historial para reconstruir la memoria
        println!("   📜 Rehidratando memoria desde '{}'...", path);
        let reader = BufReader::new(file.try_clone()?);

        for (_i, line) in reader.lines().enumerate() {
            if let Ok(record) = line {
                // Formato simple: "CLAVE_VALOR",
                if let Some((k, v)) = record.split_once(',') {
                    map.insert(k.to_string(), v.to_string());
                }
            }
        }
        println!("   ✅ Memoria restaurada: {} registros vivos.", map.len());

        // Devolvemos el motor listo con el archivo abierto para escribir nuevos datos
        Ok(Engine {
            map,
            log_file: file,
        })
    }

    // 💾 ESCRITURA: RAM + DISCO (Atomicidad simulada)
    fn set(&mut self, key: &str, value: &str) -> io::Result<()> {
        // 1. Escribir en RAM
        self.map.insert(key.to_string(), value.to_string());

        // 2. Escribir en DISCO (Persistencia)
        writeln!(self.log_file, "{},{}", key, value)?;

        // Opcional: self.log_file.flush()?; // Fuerza al disco inmediatamente (más lento, más seguro)
        Ok(())
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }
}

// Ahora el "Cerebro" protege a todo el Motor, no solo al mapa.
type Db = Arc<Mutex<Engine>>;

fn handle_client(mut stream: TcpStream, db: Db) {
    let mut buffer = [0; 512];
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

                println!("   📩 [{}]: '{}'", peer_addr, command_line);
                let parts: Vec<&str> = command_line.split_whitespace().collect();

                let _response = match parts.as_slice() {
                    ["SET", key, value] => {
                        // Bloqueamos el Mutex para tener acceso exclusivo
                        let mut engine = db.lock().unwrap();

                        match engine.set(key, value) {
                            Ok(_) => format!("💾 OK. Persistido '{}' en Disco y RAM.\n", key),
                            Err(e) => format!("🔥ERROR DE DISCO: {}\n", e),
                        }
                    }
                    ["GET", key] => {
                        let engine = db.lock().unwrap();
                        match engine.get(key) {
                            Some(v) => format!("💎 VALOR: '{}'\n", v),
                            None => format!("🤷‍♂️ No existe '{}'\n", key),
                        }
                    }
                    ["PING"] => "PONG\n".to_string(),
                    ["EXIT"] => {
                        break;
                    }
                    _ => "❌ COMANDO DESCONOCIDO\n".to_string(),
                };
            }
            Err(_) => {
                break;
            }
        }
    }
}

fn main() {
    // Iniciamos el Motor leyendo del archivo "chronos.db"
    let engine = Engine::new("chronos.db").expect("Fallo crítico al iniciar DB");
    let global_db = Arc::new(Mutex::new(engine));

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("-----------------------------------------");
    println!("💀 KLYNTAR v2.0 (PERSISTENTE) ACTIVO");
    println!("   Logs de WAL: ./chronos.db");
    println!("-----------------------------------------");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let db_clone = Arc::clone(&global_db);
                thread::spawn(move || {
                    handle_client(stream, db_clone);
                });
            }
            Err(e) => println!("Erro de conexión: {}", e),
        }
    }
}
