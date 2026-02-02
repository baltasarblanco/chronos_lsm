fn main() {
    // --- ACTO 1: EL ATAQUE DE LOS CLONES ----
    let s1 = String::from("Simbionte Original");

    // SOLUCIÓN AL DÍA 1: .clone()
    // Aquí NO movemos el puntero.
    // Rust va al Heap, copia bit por bit los datos, y crea una memoria nueva para s2.
    // Es lento (relativamente), pero ambos sobreviven.
    let s2 = s1.clone();

    println!("Acto 1: s1 es '{}' y s2 es '{}'. ¡Ambos viven!", s1, s2);

    // --- ACTO 2: EL AGUJERO NEGRO (Funciones) ---
    // CUIDADO: Pasar una variable a una función funciona IGUAL que asignarla.
    // Se MUEVE. La función se la traga.
    let s3 = String::from("Datos Volátiles");

    destruir_variable(s3); // s3 entra aqui y NUNCA regresa.

    // SI DESCOMENTAS ESTO, EXPLOTA:
    // println!("¿Sigue vivo s3? {}", s3); // Error: s3 se movío a la función.

    // --- ACTO 3: EL BOOMERANG (Devolver Ownership) ---
    let s4 = String::from("Datos Reciclables");

    // Le damos s4 a la funcion, y la función nos devuelve la propiedad.
    // La capturamos en 's5'.
    let s5 = tomar_y_devolver(s4);

    println!("Acto 3: s4 murío, pero su alma volvió en s5: '{}'", s5);


}

// --- FUNCIONES AUXILIARES ---

fn destruir_variable(texto: String) {
    println!(">>> La función recibió: '{}'", texto);
    // Cuando esta funcion termina (Aqui abajo), 'texto' sale de scope.
    // Rust llama a 'drop' y libera la memoria. ¡Adios!

} // <--- Aquí muere el dato.

fn tomar_y_devolver(texto: String) -> String {
    println!(">>> La función tomó: '{}'", texto);
    texto // Devolvemos la variable al exterior. ¡Resurrección!
}


