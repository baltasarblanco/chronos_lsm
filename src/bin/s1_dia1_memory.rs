fn main() {
    // --- ZONA 1: STACK (La Pila) ---
    // Aquí viven los datos simples (enteros, booleanos).
    // Son rápidos y baratos. Tienen el rasgo "Copy".
    let x = 5;
    let y = x; // Rust hace una FOTOCOPIA de x.
    println!("Stack: x sigue vivo ({}) y y también ({})", x, y);

    // --- ZONA 2: HEAP (El Montón) ---
    // Aquí viven los datos complejos (Texto dinámico, Vectores).
    // Son caros. NO tienen "Copy". Tienen "Move".
    let s1 = String::from("Simbionte");
    
    // EL MOVIMIENTO (The Move):
    // Rust transfiere la propiedad (ownership) de s1 a s2.
    // s1 muere instantáneamente. No se copia la data, se mueve el contrato.
    let s2 = s1; 

    // 

    // --- LA ZONA DE LA MUERTE ---
    // 1. Ejecuta primero con esto COMENTADO (con //).
    // 2. LUEGO: Quita las // y mira cómo el compilador te grita.
    println!("El fantasma de s1: {}", s1);

    println!("El nuevo dueño s2: {}", s2);
}