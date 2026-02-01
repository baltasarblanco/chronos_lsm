// Hasta ahora, Chronos sufre de SÍNDROME de Díogenes Digital: GUARDA TODO.
// Si actualizas la clave "vida" de '100' a '90', el archivo guarda:
// 1. vida '100' (Basura)
// 2. vida '90' (Válido)

// El 'Garbage Collector' (GC) que vamos a escribir hoy hará lo siguiente:
// Leerá el archivo sucio -> Filtrará solo los datos que siguen VIVOS (los que coinciden con el índice actual!)
// -> Escribirá un archivo nuevo, limpio y comprimido -> Hará el "cambiazo" (Swap) de archivos.

// Prestar mucha atención a la función 'compact'. Es el corazon de los sistemas LSM
// (Log-Structured Merge-Trees) como el que usan Google (BigTable) o Facebook (Cassandra).

use std::collections::HashMap;
use std::fs::{self, OpenOptions, File}; // fs::self nos permite usar fs::rename
use std::io::{Write, Read, Seek, SeekFrom, BufReader, BufWriter};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    key: String,
    value: String,
}

#[derive(Debug)]
pub struct KvStore {
    path: PathBuf,
    index: HashMap<String, u64>,
}

impl KvStore {
    pub fn new(path: &str) -> Result<Self, String> {
        let path_buf = PathBuf::from(path);
        let mut index = HashMap::new();

        if path_buf.exists() {
            KvStore::load_index(&path_buf, &mut index)?;

        }

        Ok(KvStore {
            path: path_buf,
            index,
        })
    }

    fn load_index(path: &PathBuf, index: &mut HashMap<String, u64>) -> Result<(), String> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        let mut reader = BufReader::new(file);

        loop {
            let current_pos = reader.stream_position().map_err(|e| e.to_string())?;
            let resultado: Result<Entry, _> = bincode::deserialize_from(&mut reader);

            match resultado {
                Ok(entry) => { index.insert(entry.key, current_pos); }
                Err(_) => break,
            }
        }
        Ok(())
    }   

    pub fn set(&mut self, key: String, value: String) -> Result<(), String> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|e| e.to_string())?;

        let mut writer = BufWriter::new(file);
        let current_offset = writer.seek(SeekFrom::End(0)).map_err(|e| e.to_string())?;
        
        let entry = Entry { key: key.clone(), value };
        bincode::serialize_into(&mut writer, &entry).map_err(|e| e.to_string())?;

        // ¡Importante! Flush asegura que se escriba  en disco YA.
        writer.flush().map_err(|e| e.to_string())?;

        self.index.insert(key, current_offset);
        Ok(())

    }

    pub fn get(&self, key: &str) -> Result<Option<String>, String> {
        let offset = match self.index.get(key) {
            Some(&pos) => pos,
            None => return Ok(None),

        };

        let mut file = File::open(&self.path).map_err(|e| e.to_string())?;
        file.seek(SeekFrom::Start(offset)).map_err(|e| e.to_string())?;
        let entry: Entry = bincode::deserialize_from(&file).map_err(|e| e.to_string())?;

        Ok(Some(entry.value))

    
    }

    // --- 🧹 LA JOYA DE LA CORONA: COMPACTION ---
    pub fn compact(&mut self) -> Result<(), String> {
        println!(">>> 🧹 INICIANDO RECOLECCIÓN DE DATOS BASURA...");

        // 1. Crear archivo temporal ("choros.db.tmp")
        let mut temp_path = self.path.clone();
        temp_path.set_extension("db.tmp");

        let temp_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&temp_path)   
            .map_err(|e| e.to_string())?;

        let mut writer = BufWriter::new(temp_file);

        // 2. Abrir archivo original para leer
        let file = File::open(&self.path).map_err(|e| e.to_string())?;
        let mut reader = BufReader::new(file);

        // 3. Nueva variable para el nuevo índice (offset)
        let mut new_index = HashMap::new();
        let mut current_temp_offset = 0; // Empezamos en 0 en el nuevo archivo

        // 4. BARRIDO COMPLETO
        loop {
            // A. Recordar dónde estaba este registro en el archivo VIEJO.
            let old_pos = reader.stream_position().map_err(|e| e.to_string())?;

            // B. Intentar Leer
            let entry: Entry = match bincode::deserialize_from(&mut reader) {
                Ok(e) => e,
                Err(_) => break, // EOF
            };

            // C. EL JUICIO FINAL: ¿Este registo sigue vivo?
            // Comparamos la posicion vieja con lo que dice el índice actual
            // Si el índice dice que la clave esta en 'old_pos', entonces es la version actual.
            // Si el índice apunta a otro lado (mas adelante), esta version es vieja.
            if let Some(&latest_pos) = self.index.get(&entry.key) {
                if latest_pos == old_pos {
                    // ¡ES VALIDO! Lo salvamos

                    // Escribimos en el archivo temporal
                    bincode::serialize_into(&mut writer, &entry).map_err(|e| e.to_string())?;

                    // Actualizamos el NUEVO índice con la NUEVA posición
                    new_index.insert(entry.key.clone(), current_temp_offset);

                    // Calculamos cuánto avanzó el cursor en el archivo nuevo.
                    // (Truco: le preguntamos al writer dónde está ahora)
                    current_temp_offset = writer.stream_position().map_err(|e| e.to_string())?;

                }
            }
        }

        writer.flush().map_err(|e| e.to_string())?;

        // 5. EL CAMBIAZÓ ATÓMICO (Atomic Swap)
        // Borramos el viejo, renombramos el nuevo.
        // Rust necesita que soltemos los archivos (drop) antes de renombrar en Windows.
        // Como 'reader' y 'writer' salen de scope aqui, estamos seguros.

        // Pero para estar seguro 100% en Windows, hacemos un drop explicito (OPCIONAL, pero BUENA PRACTICA)
        drop(reader);
        drop(writer);

        fs::rename(&temp_path, &self.path).map_err(|e| e.to_string())?;

        // 6. Actualizar el cerebro de la DB
        self.index = new_index;


        println!("✨ LIMPIEZA COMPLETADA. Índice actualizado.");
        Ok(())
    }
}

fn main() {
    let ruta = "basurero_log.db";
    let _ = std::fs::remove_file(ruta); // Empezamos limpios
    let mut db = KvStore::new(ruta).unwrap();

    println!(">>> 1. GENERANDO BASURA (Simulando uso intenso)...");
    // Vamos a actualizar la MISMA clave 1000 veces
    for i in 0..1000 {
        db.set("contador".to_string(), format!("Valor {}", i)).unwrap();
    }
    db.set("dato_fijo".to_string(), "NO ME BORRES".to_string()).unwrap();

    // Verificamos tamaño antes
    let tamaño_antes = fs::metadata(ruta).unwrap().len();
    println!(">>> 📈 Tamaño del archivo ANTES: {} bytes", tamaño_antes);
    println!(">>> 🧠 Claves en memoria: {}", db.index.len()); // Deberian ser 2 claves

    // EL MOMENTO DE LA VERDAD 
    db.compact().unwrap();

    // Verificamos tamaño después
    let tamaño_despues = fs::metadata(ruta).unwrap().len();
    println!(">>> 📉 Tamaño del archivo DESPUÉS: {} bytes", tamaño_despues);

    // Verificamos integridad
    println!("\n>>> VERIFICANDO DATOS:");
    println!("  Contador: {:?}", db.get("dato_fijo").unwrap());



}