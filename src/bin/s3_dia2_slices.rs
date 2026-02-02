fn main() {
    println!(">>> LABORATORIO DE SLICES <<<");

    // --- EXPERIMENTO 1: EL ARRAY (Buffer de Memoria) ---
    // Un array fijo de 10 números. Vive en el Stack.
    let datos_brutos: [i32; 10] = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    println!("Array original (ocupa 40 bytes): {:?}", datos_brutos);
    
    // CREAMOS UN SLICE (la ventana)
    // &datos_brutos[2..5] significa:
    // "Mirame desde el índice 2 (incluido) hasta el 5 (excluido)".
    // Es decir: índices 2, 3 y 4.
    let ventana: &[i32] = &datos_brutos[2..5];

    println!("La ventana ve: {:?}", ventana);
    println!("Longitud de la ventana: {}", ventana.len());
    // ¡Aca son la MISMA dirección! Costo de memoria: Casi cero.

    // --- EXPERIMENTO 2: STRNG SLICES (&str) ---
    // Un String en el Heap.
    let frase = String::from("Rust es Velocidad");

    // Queremos solo la palabra "Velocidad"
    // "Rust" ocupa 0..5 (R-u-s-t-espacio).
    // "es" ocupa 5..8.
    // "Velocidad" empieza en el 8.
    let palabra = &frase[8..]; // Desde el 8 hasta el final.
    
    println!("\nFrase completa: '{}'", frase);
    println!("Slice (Palabra): '{}'", palabra);

    // ADVERTENCIA FINAL:
    // Los SLices dependen del dueño original.
    // Si matas al dueño, el Slice muerte.
    // Descomenta la línea de abajo para ver el error de LIFETIME (Spoiler del Día 3):

     // drop(frase); // Matamos al dueño (frase).
     // println!("¿Sobrevive el slice? {}", palabra); // ¡ERROR!
}

