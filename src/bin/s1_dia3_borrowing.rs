fn main() {
    // --- EL DUEÑO ORIGINAL ---
    let s1 = String::from("Reactor Arc");

    // --- EL PRÉSTAMO (Borrowing) ---
    // Mira el '&'. Significa: "Toma una REFERENCIA a s1".
    // No le damos s1. Le damos una dirección para que vaya a ver s1.
    let tamaño = calcular_longitud(&s1);

    // --- LA MAGIA ---
    // Como solo PRESTAMOS s1, todavía es nuestro.
    // Ayer, esta línea hubiera explotado en rojo. Hoy funciona.
    println!("El '{}' tiene un tamaño de {} bytes.", s1, tamaño);
}

// --- LA FUNCIÓN VISITANTE ---
// Mira el tipo de dato: &String (Referencia a String).
// NO es un String. Es un puntero seguro hacia un String que vive en el otro lado.
fn calcular_longitud(s: &String) -> usize {
    // s.push_str(" ¡Explosion!"); // <---- ESTO FALLARÍA
    // Las referencia son INMUTABLES por defecto. Solo puedes mirar.

    s.len()
}  // Aqui 's' sale de scope. Pero como 's' no es dueño de nada, nada se borra.
  // El dato original (s1) sigue vivo en el main.
