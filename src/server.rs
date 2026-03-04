// Los BRAZOS (El Servidor TCP)
// ESTE ARCHIVO se encargará de escuchar conexiones, manejar la red y crear hilos (threads).
// NO sabe como GUARDAR datos. Simplemente le pide al ENGINE que lo haga!!!
// Es decir, tiene funciones especificas que luego POSTERIORMENTE solicitara al ENGINE (el cerebro)
// QUE las GUARDE en sus datos.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;

// Importamos el motor que acabamos de crear
use crate::engine::Engine;

// Creamos un tipo de dato público para que sea fácil de escribir
pub type Db = Arc<RwLock<Engine>>;

pub fn start_server(db: Db) {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("🚀 CHRONOS SERVER LISTO EN PUERTO 8080");
    println!("   Comandos: SET, GET, PING, COMPACT");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let db_clone = Arc::clone(&db);
                thread::spawn(move || {
                    handle_client(stream, db_clone);
                });
            }
            Err(e) => println!("Error de conexión: {}", e),
        }
    }
}

// Esta función es privada (no tiene pub) porque solo se usa dentro de este archivo
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
