// Definimos la forma de un Componente.
// #[derive(Debug)] permite imprimir la struct con {:?} para depurar.
#[derive(Debug)]
struct Componente {
    nombre: String,
    temperatura: i32,
    estado: String,
}

fn main() {
    // --- FASE 1: CREACIÓN (Ownership nace aquí) ---
    let c1 = Componente {
        nombre: String::from("Núcleo GPU"),
        temperatura: 85,
        estado: String::from("Recalentado"),
    };

    let c2 = Componente {
        nombre: String::from("Memoria RAM"),
        temperatura: 40,
        estado: String::from("Estable"),
    };

    println!(">>> Componentes fabricados.");

    // -- FASE 2: EL VECTOR (El nuevo dueño) ---
    // Creamos un vector mutable porque vamos a meterle cosas.
    let mut inventario: Vec<Componente> = Vec::new();

    // ¡MOVE SEMANTICS!
    // Al hacer push, c1 y c2 DEJAN DE EXISTIR en el main.
    // Ahora viven DENTRO del vector 'inventario'.
    inventario.push(c1);
    inventario.push(c2);

    // println!("{:?}", c1); // <--- ESTO FALLARÍA. c1 ya no es tuyo. Es del vector.

    // --- FASE 3: AUDITORÍA (Préstamos Inmutable &)
    // Pasamos una REFERENCIA del inventario.
    // No queremos perder el inventario, solo leerlo.
    auditar_sistema(&inventario);

    // --- FASE 4: MANTENIMIENTO (Préstamos Mutable &mut) ---
    // Pasamos una REFERENCIA MUTABLE para poder modificar los datos dentro.
    reparar_sistema(&mut inventario);

    // --- FASE 5: VERIFICACIÓN FINAL ---
    // Como solo PRESTAMOS el inventario en los pasos anteriores,
    // todavia somos los dueños aqui.
    println!(">>> Estado final del sistema:");
    auditar_sistema(&inventario);
}

// --- FUNCIONES AUXILIARES ---

// Recibe: Referencia de lectura (&Vec)
// Retorna: Nada
fn auditar_sistema(lista: &Vec<Componente>) {
    println!("--- AUDITORÍA ---");
    // Iteramos sobre la referencia de los componentes (&items)
    for item in lista {
        println!("Componente: {} | Temp: {}°C | Estado: {}",
            item.nombre, item.temperatura, item.estado);
        
    }
    println!("-------------------------");
}

// Recibe: Referencia mutable (&mut Vec).
// Retorna: Nada.
fn reparar_sistema(lista: &mut Vec<Componente>) {
    println!(">>> INICIANDO PROTOCOLO DE REPARACIÓN...");

    // Iteramos mutablemente sobre el vector
    for item in lista {
        if item.temperatura > 80 {
            println!("¡ ALERTA {} está crítico. Enfriando...", item.nombre);
            // MODIFICAMOS el dato original gracias a &mut
            item.temperatura = 45;
            item.estado = String::from("Estable");

        }
    }
}




