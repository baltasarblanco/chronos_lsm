use std::collections::HashMap;
use std::fs::{self, OpenOptions, File};
use std::io::{Write, Seek, SeekFrom, BufWriter};
use std::path::PathBuf;
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
    pub fn set (&mut self, key: String, value:String) -> Result<u64, String> {
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

    fn read_at(&self, offset: u64) -> Result<String, String> {
        let mut file = File::open(&self.path).map_err(|e| e.to_string())?;
        file.seek(SeekFrom::Start(offset)).map_err(|e| e.to_string())?;
        let entry: Entry = bincode::deserialize_from(&file).map_err(|e| e.to_string())?;
        Ok(entry.value)
    }

    // --- 🔥 DÍA 3: RANGE QUERY (La Novedad) ---
    // Devuelve una lista de (Tiempo, Valor) para graficar 
    pub fn get_range(&self, key: &str, start: u64, end: u64) -> Vec<(u64, String)> {
         let mut resultados = Vec::new();

         if let Some(history) = self.index.get(key) {
            // 1. FILTRADO INTELIGENTE
            // Interamos sobre el historial y tomamos solo lo que está en el rango.
            // (En el futuro, esto se optimiza con Búsqueda Binaria para no recorrer.
            // pero por ahora un filter() es suficiente para entender la lógica).
            for &(time, offset) in history {
                if time >= start && time <= end {
                    // Es un candidato válido, vamos al disco a leer el valor
                    if let Ok(val) = self.read_at(offset) {
                        resultados.push((time, val));
                    }
                }
            }

         }
         resultados
        }
}

fn main() {
    let mut db = ChronosDB::new("range_test.db").unwrap();

    println!(">>> 📊 GENRANDO DATOS DE MERCADO...");

    // Simulamos precios de una acción cada 10ms
    let t_inicio = db.set("accion_aapl".to_string(), "150".to_string()).unwrap(); // T0
    std::thread::sleep(std::time::Duration::from_millis(10));
    
    let t_medio_1 = db.set("accion_aapl".to_string(), "155".to_string()).unwrap(); // T1
    std::thread::sleep(std::time::Duration::from_millis(10));

    let _t_medio_2 = db.set("accion_aapl".to_string(), "160".to_string()).unwrap(); // T2
    std::thread::sleep(std::time::Duration::from_millis(10));

    let t_fin = db.set("accion_aapl".to_string(), "145".to_string()).unwrap(); // T3
    
    println!("   Datos generados desde T={} hasta T={}", t_inicio, t_fin);

    // --- PRUEBA DE RANGO ---
    println!("\n 🔎 CONSULTA: Dame los valores intermedios (excluyendo al inicio y el final)");

    // Definimos un rango que solo atrape a t_medio1 y t_medio_2
    // Sumamos/Restamos 1ms para asegurarnos de que el rango sea estricto
    let query_start = t_inicio + 1;
    let query_end = t_fin - 1;

    let historia = db.get_range("accion_aapl", query_start, query_end);

    for (t, v) in historia {
        println!("   📈 En T={} el valor fue: ${}", t, v);
    }

}


