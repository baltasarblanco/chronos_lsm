// Esta función intenta buscar un usuario por ID.
// Devuelve Option<String>: "Tal vez un String, tal vez nada".
fn buscar_usuario(id: i32) -> Option<String> {
    if id == 1 {
        // Encontramos algo, lo envolvemos en Some
        Some(String::from("Spiderman"))
    } else if id == 99 {
        Some(String::from("Venom"))
    } else {
        // No encontramos nada. Devolvemos la "Nada explicita".
        None
    }

}

fn main() {
    let id_buscado = 50; // Prueba cambiar esto a 1, 99 o 50.

    // Llamamos a la función. 
    // 'resultado' NO es un string. Es un Option<Stringer>.
    // No podemos imprimirlo directamente ni usarlo todavia.
    let resultado = buscar_usuario(id_buscado);
    
    // FORMA 1: El método seguro (match))
    // Estamos obligados a manejar el caso de que este vacio.
    match resultado {
        Some(nombre) => println!("¡EXITO! Usuario encontrado: {}", nombre),
        None => println!("ERROR 404: El usuario {} no existe en la Matrix.", id_buscado),
    }

    // FORMA 2: El metodo Kamikaze (.unwrap)
    // Solo úsalo si estás 100% seguro de que NO va a fallar.
    // Si es None, el programa explota (Panic)
    // Descomenta para probar el peligro:

    let nombre_seguro = buscar_usuario(50).unwrap();
    println!("Nombre forzado: {}", nombre_seguro);

}

