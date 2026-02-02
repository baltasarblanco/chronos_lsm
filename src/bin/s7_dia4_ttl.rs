use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{Write, Seek, SeekFrom, BufWriter};
use std::path::{PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    key: String,
    value: String,
    timestamp: u64,
}

#[derive(Debug)]
pub struct ChronosDB {
    path: PathBuf,
    index: HashMap<String, Vec<(u64, u64)>>,
}

impl ChronosDB {
    pub fn new(path: &str) -> Result<Self, String> {
        let path_buf = PathBuf::from(path);
        if path_buf.exists() { fs::remove_file(&path_buf).map_err(|e| e.to_string())?; }
        Ok(ChronosDB { path: path_buf, index: HashMap::new() })
    }

    fn current_time() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
    }
     
    pub fn set(&mut self, key: String, value: String) -> Result<u64, String> {
        let now = ChronosDB::current_time();
        let file = OpenOptions::new().create(true).append(true).open(&self.path).map_err(|e| e.to_string())?;
        let mut writer = BufWriter::new(file);
        let offset = writer.seek(SeekFrom::End(0)).map_err(|e| e.to_string())?;

        let entry = Entry { key: key.clone(), value, timestamp: now };
        bincode::serialize_into(&mut writer, &entry).map_err(|e| e.to_string())?;
        writer.flush().map_err(|e| e.to_string())?;

        self.index.entry(key).or_insert(Vec::new()).push((now, offset));
        Ok(now)
    }

    // --- 🔥 DÍA 4: RETENTION POLICY (La Poda) ---
    // Elimina del índice todo lo que sea más viejo que 'retention_ms'
    pub fn prune_old_data(&mut self, retention_ms: u64) -> usize {
        let now = ChronosDB::current_time();
        let threshold = now - retention_ms; // El punto de corte (hace X tiempo)
        let mut total_deleted = 0;

        println!("   ✂️ PODANDO: Borrando todo lo anterior a T={} (Hace {}ms", threshold, retention_ms);

        // Iteramos sobre cada clave del índice
        for (_key, history) in self.index.iter_mut() {
            let original_len = history.len();

            // RETAIN: Es una función mágica de Rust.
            // Mantiene solo los elementos que cumplen la condicion (true)
            // Si devuelve false, los borra del vector.
            history.retain(|(timestamp, _)| *timestamp >= threshold);

            let deleted_count = original_len - history.len();
            total_deleted += deleted_count;
        }

        total_deleted // Devolvemos cúantos registros borramos

    }

    // Función auxiliar para contar cuántos datos tenemos en total
    pub fn total_records(&self) -> usize {
        self.index.values().map(|v| v.len()).sum()
    }

}

fn main() {
    let mut db = ChronosDB::new("ttl_Test.db").unwrap();

    println!(">>> ⏳ GENERANDO HISTORIA ANTIGUA...");

    // T1: Hace mucho tiempo (simulamos que pasó tiempo durmiendo poco)
    db.set("log_servidor".to_string(), "Inicio sistema".to_string()).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(50));

    // T2: Pasado medio
    db.set("log_servidor".to_string(), "Usuario conectado".to_string()).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(50));

    //  T3: Pasado reciente
    db.set("log_servidor".to_string(), "Usuario conectado".to_string()).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(50));

    // T4: Presente (Recién ocurrido)
    db.set("log_servidor".to_string(), "Apagando".to_string()).unwrap();

    println!("   📊 Total registros antes de limpiar: {}", db.total_records());

    // --- APLICAR RETENCIÓN ---
    println!("\n>>> 🧹 EJECUTANDO LIMPIEZA (TTL = 80ms");
    // Queremos borrar todo lo que tenga más de 80ms de antiguedad.
    // T1 y T2 deberpian morir. T3 y T4 deberían sobrevivir.

    let borrados = db.prune_old_data(80);

    println!("   🗑️ Registros eliminados: {}", borrados);
    println!("   📊 Total registros despúes de limpiar: {}", borrados);

    // Verificación
    if db.total_records() == 2 {
        println!("\n ✨¡EXITO! El sistema olvidó el pasado irrelevante.");
    } else {
        println!("\n❌ ALGO FALLÓ: Se borraron {} registros.", borrados);
    }
    
    }
