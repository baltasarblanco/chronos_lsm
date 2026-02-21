use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;

const DB_PATH: &str = "chronos_v3.db";

struct Engine {
    map: HashMap<String, String>,
    log_file: File,
}

impl Engine {
    fn new(filepath: &str) -> io::Result<Self> {
        let path = filepath.to_string();
        // Abrimos el archivo en modo Append
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&path)?;

        let mut map = HashMap::new();
        println!("   📜 Rehidradanto memoria desde '{}'...", path);
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

    // 🧹 EL COMPACTADOR: La nueva función estrella
    fn compact(&mut self) -> io::Result<()> {
        let temp_path = "chronos_temp.db";
        println!("   🧹 Iniciando Compactación (Garbage Colecction)...");

        // 1. Crear el archivo temporal limpio
        let mut temp_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(temp_path)?;

        // 2. Volcar SOLO la memoria actual (que ya está limpia) al disco
        for (key, value) in &self.map {
            writeln!(temp_file, "{},{}", key, value)?;
        }

        // Forzamos que se escriba todo en disco físico
        temp_file.sync_all()?;

        // 3. Reemplazo Atómica (El truco de magia)
        // Renombrar es una operación atómica en el sistema operativo.
        // Si falla la luz aqui, no perdemos datos ( o tenemos el viejo o el nuevo.)
        fs::rename(temp_path, DB_PATH)?;

        // 4. Reabrir el archivo log apuntando al nuevo archivo limpio
        self.log_file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(DB_PATH)?;

        println!("   ✨ Compactación terminada. Basura eliminada.");
        Ok(())
    }
}

type Db = Arc<RwLock<Engine>>;

fn handle_client(mut stream: TcpStream, db: Db) {
    let mut buffer = [0; 512];
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

                let parts: Vec<&str> = command_line.split_whitespace().collect();

                let response = match parts.as_slice() {
                    ["SET", key, value] => {
                        let mut engine = db.write().unwrap();
                        match engine.set(key, value) {
                            Ok(_) => "OK\n".to_string(),
                            Err(e) => format!("ERR {}\n", e),
                        }
                    }
                    ["GET", key] => {
                        let engine = db.read().unwrap();
                        match engine.get(key) {
                            Some(v) => format!("{}\n", v),
                            None => "NULL\n".to_string(),
                        }
                    }
                    ["COMPACT"] => {
                        // <---- COMANDO NUEVO
                        // Necesitamos Write Lock porque vamos a tocar el archivo
                        let mut engine = db.write().unwrap();
                        match engine.compact() {
                            Ok(_) => "OK_COMPACTED\n".to_string(),
                            Err(e) => format!("ERR_COMPACT {}\n", e),
                        }
                    }
                    ["PING"] => "PONG\n".to_string(),
                    _ => "UNKNOWN\n".to_string(),
                };
                if stream.write_all(response.as_bytes()).is_err() {
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
    // Usamos el mismo DB_PATH
    let engine = Engine::new(DB_PATH).expect("Fallo DB");
    let global_db = Arc::new(RwLock::new(engine));

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("🚀 CHRONOS v3.1 (CON COMPACTADOR) LISTO");
    println!("   Comandos: SET, GET, PING, COMPACT");

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
