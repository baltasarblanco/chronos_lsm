use std::collections::HashMap;
use std::fs::{OpenOptions, File};
use std::io::{Write, Read, Seek, SeekFrom, BufReader, BufWriter};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

// --- ESTRUCTURAS ---
#[derive(Debug)]
pub struct KvStore {
    path: PathBuf,
    index: HashMap<String, u64>, // El cerebro RAM
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    key: String,
    value: String,
}

impl KvStore {
    // 1. CONSTRUCTOR INTELIGENTE
    pub fn new(path: &str) -> Result<Self, String> {
        let path_buf = PathBuf::from(path);

        //  Iniciamos con el mapa vacio
        let mut index = HashMap::new();

        // SI EL ARCHIVO EXISTE, LO LEEMOS PARA RECUPERAR LA MEMORIA
        if path_buf.exists() {
            println!(">>> 📂 Archivo encontrado. Iniciando recuperación de índice...");
            KvStore::load_index(&path_buf, &mut index)?;
            println!(">>> 🧠 Memoria restaurada. {} claves cargadas.", index.len());
        } else {
            println!(">>> 🆕 Archivo nuevo. Base de datos vacia");
        
        }

        Ok(KvStore {
            path: path_buf,
            index,
        })
         
    }

    // --- FUNCIÓN PRIVADA DE RECUPERACIÓN  ( LA MAGIA DE HOY ) ---
    fn load_index(path: &PathBuf, index: &mut HashMap<String, u64>) -> Result<(), String> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        let mut reader = BufReader::new(file);


        loop {
            // A. CAPTURAR POSICIÓN ANTES DE LEER
            // Esto es crucial: necesitamos saber dónde EMPIEZA el registro.
            let current_pos = reader.stream_position().map_err(|e| e.to_string())?;

            // B. INTENTAR LEER UN REGISTRO
            // bincode::deserialize_from leerá exactamente un Entry si puede
            let resultado: Result<Entry, _> = bincode::deserialize_from(&mut reader);

            match resultado {
                Ok(entry) => {
                    // C. ÉXITO: GUARDAMOS EN EL MAPA
                    // "La clave X vive en la posición current_pos"
                    index.insert(entry.key, current_pos);
                    
                }
                Err(_) => {
                    // D. ERROR/EOF: ASUMIMOS QUE TERMINÓ EL ARCHIVO
                    // (En un sistema real distinguiríamos entre EOF y corrupción) 
                    break;

                }
            }

        }
        Ok(())
    }


    // 2. SET (Con tu correción de ayer)
    pub fn set(&mut self, key: String, value: String) -> Result<(), String> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|e| e.to_string())?;

        let mut writer = BufWriter::new(file);

    // Vamos al final real para obtener el offset correcto
        let current_offset = writer.seek(SeekFrom::End(0)).map_err(|e| e.to_string())?;

        let entry = Entry { key: key.clone(), value };
        bincode::serialize_into(&mut writer, &entry).map_err(|e| e.to_string())?;
        writer.flush().map_err(|e| e.to_string())?;

        self.index.insert(key, current_offset);
        Ok(())

    }

    // 3. GET (Lectura Quirúrgica)
    pub fn get(&self, key: &str) -> Result<Option<String>, String> {
        let offset = match self.index.get(key) {
            Some(&pos) => pos,
            None => return Ok(None),

        };

        let mut file = File::open(&self.path).map_err(|e| e.to_string())?;
        file.seek(SeekFrom::Start(offset)).map_err(|e| e.to_string())?;
    
        // Aqui bincode sabe leer SOLO lo necesario
        let entry: Entry = bincode::deserialize_from(&file).map_err(|e| e.to_string())?;
        Ok(Some(entry.value))
    }    
}

// ---- SIMULACIÓN DE PERSISTENCIA -----
fn main() {
    let ruta = "memoria_persistente.db";

    // FASE 1: ¿YA EXISTEN LOS DATOS?
    // Insertamos abrir la DB. Si el archivo existen, 'new' recuperará el índice.
    let mut db = KvStore::new(ruta).unwrap();

    // Verificamos si recordamos algo del pasado
    if let Ok(Some(valor)) = db.get("heroe_actual") {
        println!("\n >>> ¡BIENVENIDO DE NUEVO! DATOS RECUPERADOS:");
        println!("   🛡️ Heroe: {}", valor);

        // Si ya existe, agregamos algo nuevo para verficar que siga funcionando.
        println!(">>> Agregando nueva entrada...");
        db.set("ultima_sesion".to_string(), "Hoy".to_string()).unwrap();

    } else {
        println!("\n >>> PRIMERA EJECUCIÓN (O DATOS BORRADOS)");
        println!(">>> Guardando datos iniciales...");

        db.set("heroe_actual".to_string(), "Kirito".to_string()).unwrap();
        db.set("nivel".to_string(), "99".to_string()).unwrap();
        db.set("espada".to_string(), "Elucidator".to_string()).unwrap();

        println!(">>> Datos guardados. ¡AHORA CIERRE EL PROGRAMA Y VUELVE A EJECUTARLO!");

    }

    println!("\n>>> ESTADO FINAL DEL ÍNDICE RAM: {:?}", db.index);

}

