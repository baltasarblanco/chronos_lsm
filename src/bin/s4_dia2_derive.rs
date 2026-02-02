// 1. LA ETIQUETA MÁGICA 
// #[derive(...)] le dice a Rust:
// "Escribe automáticamente la implementación de estos Traits para mi struct".
// - Debug: Permite imprimir con {:?}
// - PartialEq: Permite comparar con ==
// - Clone: Permite hacer .clone()  (copia profunda)
#[derive(Debug, PartialEq, Clone)]
struct Usuario {
    nombre: String,
    nivel: i32,
    admin: bool,
}

fn main() {
    let u1 = Usuario {
        nombre: String::from("Baltasar"),
        nivel: 10,
        admin: true,
    };

    // MAGIA 1: CLONE
    // No escribimos el código de 'clone', pero Rust lo genero por nosotros.
    let mut u2 = u1.clone();
    u2.nombre = String::from("Clon Malvado");

    // MAGIA 2: DEBUG
    // Gracias a 'Debug', podemos imprimir la struct entera.
    // Antes tenías que escribir cada campo a mano.
    println!("Usuario 1: {:?}", u1);
    println!("Usuario 2: {:?}", u2);

    // MAGIA 3: PARTIAL EQ (Comparación)
    // Gracias a 'PartialEq', podemos usar ==
    if u1 == u2 {
        println!(">>> ¡Son idénticos!");
    } else { 
        println!(">>> Son diferentes(afortunadamente).");
    }


    // RETO DE COMPROBACIÓN:
    // Crea un u3 que sea idéntico a u1 y comprueba si == dice que es true.
    let u3 = Usuario {
        nombre: String::from("Baltasar"),
        nivel: 10,
        admin: true,
    };

    if u1 == u3 {
        println!(">>> u1 y u3 son gemelos idénticos.");
    }
}

