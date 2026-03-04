// EL CORAZON (El punto de ENTRADA)
// Main.rs será pequeñito, limpio y elegante. El único trabajo es unir el motor y el servidor.

// Le decimos a Rust que busque los otros dos archivos

mod engine;
mod server;

use engine::{Engine, DB_PATH};
use std::sync::{Arc, RwLock};

fn main() {
    println!("⏳ Iniciando Chronos DB...");

    // 1. Instanciamos el Motor
    let engine = Engine::new(DB_PATH).expect("Fallo crítico al iniciar la DB");

    // 2. Lo envolvemos en nuestra barrera de hilos
    let global_db = Arc::new(RwLock::new(engine));

    // 3. Arrancamos el Servidor TCP
    server::start_server(global_db);
}
