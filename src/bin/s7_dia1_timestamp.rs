use std::collections::HashMap;
use std::fs::{OpenOptions, File};
use std::io::{Write, Read, Seek, SeekFrom, BufReader, BufWriter};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH}; // <---- NUEVO: El tiempo
use serde::{Serialize, Deserialize};

// --- ESTRUCUTRA ESPACIO-TEMPORAL ---
#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    key: String,
    value: String,
    timestamp: u64, // <--- NUEVO: Milisegundos desde 1970
}

#[derive(Debug)]
pub struct TimeTravelStore {
    path: PathBuf,
    // EL ÍNDICE AHORA ES UN HISTORIAL
    // Clave -> Lista de (Tiempo, Offset)
    index: HashMap<String, Vec<(u64, u64)>>,
}

impl TimeTravelStore {
    pub fn new(path: &str) -> Result<Self, String> {
        let path_buf = PathBuf::from(path);
        let index = HashMap::new();
        // (Por hoy saltamos el 'load_index' para enfocarnos en la escritura temporal,
        // mañana conectaremos todo)

        Ok(TimeTravelStore {
            path: path_buf,
            index,

        })
    }
    
    // --- FUNCIÓN AUXILIAR PARA OBTENER TIEMPO ACTUAL ---
    fn current_time() -> u64 {
        let start = SystemTime::now();
        stat.duration_since(UNIX_EPOCH);
            .exect("Time went backwards")   
            .as_millis() as u64
    }

    // 1. SET: AHORA GUARDA CON SELLO DE TIEMPO
    pub fn set(&mut self, key: String, value: String) -> Result<(), String> {
        let now = TimeTravelStore::current_time();

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|e| e.to_string())?;

        let mut writer = BufWriter::new(file);
        let current_offset = writer.seek(SeekFrom::End(0)).map_err(|e| e.to_string())?;

        // Creamos la entrada con TIEMPO
        let entry = Entry {
            key: key.clone(),
            value,
            timestamp: now
        }

        bincode::serialize_into(&mut writer, &entry).map_err(|e| e.to_string())?;
        writer.flush().map_err(|e| e.to_string())?;

        // 2. ACTUALIZAR EL HISTORIAL (NO SOBREESCRIBIR)
        // Buscamos la lista de historia de esta clave. Si no existe, creamos una vacía.
        let history = self.index.entry(key).or_insert(Vec::new());

        // Agregamos el nuevo evento al final de la historia
        history.push((now, current_offset));

        println!("> ⏱️ Guardado en T={}: Offset={}", now, current_offset);
        Ok(())


    }
    

    // 3. GET (ClÁSICO): DEVUELVE EL ÚLTIMO VALOR (EL PRESENTE)
    pub fn get_latest(&self, key, &str) -> Result<Option<String>, String> {
        // Buscamos la historia
        if let Some(history) = self.index.get(key) {
            // .last() nos da el elemento mas reciente del vector
            if let Some(&(_time, offset)) = history.last() {
                // Leemos el disco
                return self.read_at(offset);
            }
        }
        Ok(None)
    }

    // --- FUNCIÓN DE LECTURA PRIVADA (DRY) ---
    fn read_at(&self, offset: u64) -> Result<Option<String>, String> {
        let mut file = File::open(&self.path).map_err(|e| e.to_string())?;
        file.seek(SeekFrom::Start(offset)).map_err(|e| e.to_string())?;
        let entry: Entry = bincode::deserialize_from(&file).map_err(|e| e.to_string())?;
        Ok(Some(entry.value))
    }
}

fn main() {
    let ruta = "chronos_time.db";
    let _ = std::fs::remove_file(ruta);
    let mud db = TimeTravelStore::new(ruta).unwrap();

    println!(">>> 🕰️ INICIANDO SIMULACIÓN TEMPORAL <<<");

    // T=0 (Aprox)
    db.set("precio_btc".to_string(), "50,000".to_string()).unwrap();

    // Simulamos que pasa un poco de tiempo (dormimos el hilo 10ms)
    std:thread::sleep(std::time::Duration::from_millis(10));

    // T=10
    db.set("precio_btc".to_string(), "51,000".to_string()).unwrap();
    
    std::thread::sleep(std::time::Duration::from_millis(10));

    // T=20
    db.set("precio_btc".to_string(), "49,000".to_string()).unwrap();

    println!(">>> 📜 HISTORIAL EN RAM PARA 'precios_btc':");
    if let Some(historia) = db.index.iter().enumarate() {
        for (i, (t, off)) in historia.iter().enumarate() {
            println!("   Evento #{}: Tiempo={}ms, Offset={}", i, t, off);

        }
    
    }
    
    println!(">>>\n 🔎 CONSULTA ACTUAL (Get Latest)");
    let valor = db.get_latest("precio_btc").unwrap().unwrap();
    println!("   Valor actual: {}", valor);
}