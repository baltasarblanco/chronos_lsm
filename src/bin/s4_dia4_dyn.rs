// 1. EL TRAIT (El Comportamiento Común) 
// Todo lo que ocurra en el sistema debe ser "Registrable".
trait Registrable {
    fn describir(&self) -> String;
    fn timestamp(&self) -> u64; // Tiempo Unix
}


// 2. EVENTO A: CAMBIO DE VALOR (Insert/Update)
struct CambioValor {
    tiempo: u64,
    clave: String,
    valor: String,
}

impl Registrable for CambioValor {
    fn describir(&self) -> String {
        format!("SET '{}' = '{}'", self.clave, self.valor)
    
    }

    fn timestamp(&self) -> u64 { self.tiempo }

}

// 3. EVENTO B: BORRADO (Delete)
struct Borrado {
    tiempo: u64,
    clave: String,
}

impl Registrable for Borrado {
    fn describir(&self) -> String {
        format!("DELTE '{}'", self.clave)
    }
    fn timestamp(&self) -> u64 { self.tiempo }

}

// 4. EVENTO C: ERROR DE SISTEMA
struct ErrorCritico {
    tiempo: u64,
    codigo: i32,
}

impl Registrable for ErrorCritico {
    fn describir(&self) -> String {
        format!("!!! ERROR CRITICO #{}", self.codigo)
    }
    fn timestamp(&self) -> u64 { self.tiempo }
}

fn main() {
    println!(">>> CHRONOS-LOG: INICIANDO BITÁCORA DE TIEMPO <<<");
    

    // AQUÍ ESTÁ LA MAGIA: Vec<Box<dyn Registrable>>
    // Traducido: "Un vector de Cajas. Dentro de cada caja hay ALGO (dyn)
    // que cumple el contrato Registrable"
    let mut linea_temporal: Vec<Box<dyn Registrable>> = Vec::new();

    // Insertamos eventos de tipo TOTALMENTE DISTINTOS
    linea_temporal.push(Box::new(CambioValor {
        tiempo: 1000,
        clave: String::from("saldo"),
        valor: String::from("100.00"),
    }));

    linea_temporal.push(Box::new(CambioValor {
        tiempo:1010,
        clave: String::from("saldo"),
        valor: String::from("5000.00"), // HFT: Alguien se hizo rico.
    }));

    linea_temporal.push(Box::new(ErrorCritico {
        tiempo: 1012,
        codigo: 404,
    }));

    // REPRODUCIMOS LA HISTORIA
    // Rust usa "Dynamic Dispatch" para saber qué método .describir() llamar
    // en tiempo de ejecución.
    println!("--- REPLAY DE EVENTOS ---");
    for evento in linea_temporal.iter() {
        println!("[T={}] -> {}", evento.timestamp(), evento.describir());
    }    
        
    // SIMULACIÓN DE "TIME TRAVEL" (GET saldo @ 1002)
    println!("\n--- VIAJE EN EL TIEMPO: BUSCANDO SALDO @ T=1002 ---");
    let target_time = 1002;
    let mut ultimo_valor = "N/A";

    for evento in linea_temporal.iter() {
        if evento.timestamp() > target_time {
            break; // Nos pasamos el tiempo objetivo
        }

        // Hacemos un "downcast" manual (truco sucio para simular lectura)
        // En la vida real usaríamos Enums para esto, pero hoy practicamos Traits.
        let descripcion = evento.describir();
        if descripcion.starts_with("SET 'saldo'") {
            ultimo_valor = " (Valor encontrado en el log)";
            // Aquí iria la lógica real de extracción
            println!(" > Encontrado evento relevante en T={}: {}", evento.timestamp(), descripcion);

        }

    }

        
}

