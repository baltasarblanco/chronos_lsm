use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::BufWriter;
use std::path::PathBuf;

// Guardamos 'f64' (flotante) para poder hacer matemáticas
#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    timestamp: u64,
    value: f64,
}

pub struct AudioDB {
    path: PathBuf,
    // Índice: Timestamp -> (Valor, Offset)
    // Guardamos el valor en RAM también para calcular rápido sin ir al disco
    index: Vec<(u64, f64)>,
}

impl AudioDB {
    pub fn new(path: &str) -> Result<Self, String> {
        let p = PathBuf::from(path);
        if p.exists() {
            fs::remove_file(&p).map_err(|e| e.to_string())?;
        }
        Ok(AudioDB {
            path: p,
            index: Vec::new(),
        })
    }

    pub fn add(&mut self, time: u64, value: f64) -> Result<(), String> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|e| e.to_string())?;
        let mut writer = BufWriter::new(file);

        let entry = Entry {
            timestamp: time,
            value,
        };
        bincode::serialize_into(&mut writer, &entry).map_err(|e| e.to_string())?;

        // Guardamos en RAM ordenado por tiempo
        self.index.push((time, value));
        Ok(())
    }
    // Función auxiliar pura (Matemática pura)
    // Esto hace el código principal mucho más legible
    fn lerp(t0: u64, y0: f64, t1: u64, y1: f64, target: u64) -> f64 {
        if t1 == t0 {
            return y0;
        } // Evitar división por cero
        let fraction = (target - t0) as f64 / (t1 - t0) as f64;
        y0 + fraction * (y1 - y0)
    }
    // --- 🔥 LA HABILIDAD DEL JEFE: INTERPOLACIÓN LINEAL ---
    // Si pido T=5, y tengo T=0 (Val=0) y T=10 (Val=100)...
    // Debe devolver 50.
    pub fn get_interpolated(&self, target_time: u64) -> Option<f64> {
        // 1. Caso borde: Base de datos vacía
        if self.index.is_empty() {
            return None;
        }

        // 2. Buscamos el punto de corte
        let i = self.index.partition_point(|(t, _)| *t <= target_time);

        // Manejo de casos borde (Start/End) usando 'match' o 'if' limpios
        // Esto es muy eficiente porque usa acceso directo por índice
        match i {
            0 => Some(self.index[0].1), // Antes del inicio
            len if len >= self.index.len() => Some(self.index.last()?.1), // Después del final
            _ => {
                // Caso central: Interpolación
                let (t0, y0) = self.index[i - 1];
                let (t1, y1) = self.index[i];
                Some(Self::lerp(t0, y0, t1, y1, target_time))
            }
        }
    }
}

fn main() {
    let mut db = AudioDB::new("audio_signal.db").unwrap();

    println!(">>> 🎹 GENERANDO ONDA DE CIERRA (RAMP)...");

    // Vamos a guardar puntos lejanos:
    // T=0    -> Valor 0.0
    // T=100  -> Valor 100.0
    // T=200  -> Valor 0.0

    db.add(0, 0.0).unwrap();
    println!("   📍 Punto Clave: T=0,   Val=0.0");

    db.add(100, 100.0).unwrap();
    println!("   📍 Punto Clave: T=100, Val=100.0");

    db.add(200, 0.0).unwrap();
    println!("   📍 Punto Clave: T=200, Val=0.0");

    println!("\n>>> 🎛️ SINTETIZANDO VALORES INTERMEDIOS...");

    // Queremos saber qué pasaba en T=50.
    // Como es una línea recta entre 0 y 100, debería ser 50.0.
    let val_50 = db.get_interpolated(50).unwrap();
    println!(
        "   ❓ Consulta T=50  (Esperado: 50.0) -> Resultado: {:.2}",
        val_50
    );

    // Queremos saber qué pasaba en T=150.
    // Entre 100 (Val 100) y 200 (Val 0), debería ser 50.0 bajando.
    let val_150 = db.get_interpolated(150).unwrap();
    println!(
        "   ❓ Consulta T=150 (Esperado: 50.0) -> Resultado: {:.2}",
        val_150
    );

    // Un valor raro: T=25
    let val_25 = db.get_interpolated(25).unwrap();
    println!(
        "   ❓ Consulta T=25  (Esperado: 25.0) -> Resultado: {:.2}",
        val_25
    );

    if val_50 == 50.0 && val_25 == 25.0 {
        println!("\n✨ ¡JEFE DERROTADO! Tu motor de audio sabe interpolar.");
    } else {
        println!("\n❌ FALLO EN LA MATRIZ. Revisa la fórmula.");
    }
}
