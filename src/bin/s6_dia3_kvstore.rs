use std::collections::HashMap;
use std::fs::{OpenOptions, File};
use std::io::{Write, Read, Seek, SeekFrom, BufReader, BufWriter};
use std::path::PathBuf;
use serde::{Serialize, Deserialize, de::DeserializeOwned};


// ---- EL MOTOR DE DISCO (Simplificado de la Semana 5) ----
// (Aquí asumimos que LogEngine maneja la escritura cruda)
// Para este ejercicio, integraremos la lógica directamente en el KvStore
// para que veas la conexión clara entre Mapa y Archivo.

#[derive(Debug)]
pub struct KvStore {
    path: PathBuf,
    index: HashMap<String, u64>, // RAM: Clave -> Offset
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    key: String,
    value: String,
}

impl KvStore {
    // 1. CONSTRUCTOR: ARRANQUE DE FRÍO
    // En un sistema real, aquí leeríamos el archivo para reconstruir el indice (se hara mañana)
    // Hoy empezamos vacíos.
    pub fn new(path: &str) -> Self {
        KvStore {
            path: PathBuf::from(path),
            index: HashMap::new(),
        }
    }

    // 2. SET: ESCRIBIR Y MAPEAR
    pub fn set(&mut self, key: String, value: String) -> Result<(), String> {
        // A. Abrir archivo en modo Append
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|e| e.to_string())?;

        let mut writer = BufWriter::new(file);

        // ---- CORECCIÓN ----
        // Forzamos al cursor a ir hasta el final (End) y nos dice dónde cayó (el offset real)
        let current_offset = writer.seek(SeekFrom::End(0)).map_err(|e| e.to_string())?;
        // -------------------

        // B. Crear el paquete
        let entry = Entry { key: key.clone(), value };

        // C. Escribir en Disco (Serializado)
        // bincode escribe [LEN][DATA] automaticamente
        bincode::serialize_into(&mut writer, &entry).map_err(|e| e.to_string())?;
        
        // Asegurar que se guardó
        writer.flush().map_err(|e| e.to_string())?;

        // D. ACTUALIZAR EL MAPA (RAM)
        // "La clave X ahora vive en el offset Y"
        self.index.insert(key, current_offset);

        Ok(())

    
    }

    // 3. GET: LECTURA QUIRÚRGICA
    pub fn get(&self, key: &str) -> Result<Option<String>, String> {
        // A. Consultar el Mapa (RAM) - Velocidad 0(1)
        let offset = match self.index.get(key) {
            Some(&pos) => pos,
            None => return Ok(None), // No existe la clave
        };

        // B. Abrir archivo (Solo lectura)
        let mut file = File::open(&self.path).map_err(|e| e.to_string())?;

        // C. El SALTO (Seek) - Velocidad 0(1)
        file.seek(SeekFrom::Start(offset)).map_err(|e| e.to_string())?;

        // D. Leer solo ese registro
        // deserialize_from lee el largo y luego los datos. Sabe dónde parar.
        let entry: Entry = bincode::deserialize_from(&file).map_err(|e| e.to_string())?;
        
        Ok(Some(entry.value))


    }
} // < ----- CIERRE DEL impl KvStore ( linea importante ) 


fn main() {
    println!(">>> CHRONOS KV STORE V1 (RAM + DISCO) <<<");
    
    // Limpiamos pruebas anteriores
    let ruta = "mi_base_datos.db";
    let _ = std::fs::remove_file(ruta);

    let mut db = KvStore::new(ruta);

    println!(">>> 1. Insertando datos...");
    // Usamos .unwrap() aquí para simplificar el manejo de errores en el main
    db.set("usuario:1".to_string(), "Baltasar".to_string()).unwrap();
    db.set("config:color".to_string(), "Azul".to_string()).unwrap();

    println!(">>> Base de datos actual (RAM Index): {:?}", db.index);
    
    println!("\n>>> 2. Consultando datos (Velocidad Luz)...");

    let key = "config:color";
    match db.get(key).unwrap() {
        Some(val) => println!("   ✅ Encontrado: {} = {}", key, val),
        None => println!("   ❌ No encontrado"),

    }
    

}

