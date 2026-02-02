fn main() {
    // --- PASO 1: EL SUJETO ---
    // Para que algo pueda cambiar, el DUEÑO debe permitirlo.
    // 'mut' es el permiso explícito de cambio.
    let mut s = String::from("Hulk");

    println!("Antes del experimento: {}", s);

    // --- PASO 2: LA INYECCIÓN (&mut) ---
    // Creamos una Referencia Mutable.
    // Le damos permiso a la función para modificar 's'.
    inyectar_gama(&mut s );
    
    println!("Después del experimento: {}", s);

    // --- PASO 3: EL PECADO MORTAL (La Carrera de Datos) ---
    // Rust te protege de la estupidez.
    // Intenta descomentar las siguentes 3 lineas para ver el error:

    // let r1 = &mut s; // Primer Escritor
    // let r2 = &mut s; // ¡SEGUNDO ESCRITOR! (Ilegal).
    // println!("{}, {}", r1, r2) // El compilador detiene el universo aquí. 
}

// --- LA CÁMARA GAMMA ---
// Recibe '&mut String'. Puede leer Y escribir.
fn inyectar_gama(texto: &mut String) {
    texto.push_str(" SMASH!"); // Modificamos el dato original.
}

