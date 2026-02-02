fn main() {
    println!(">>> MAPA DE MEMORIA DEL SISTEMA <<<");

    // 1. STACK (La pila)
    // Los enteros son simples, viven en el Stack.
    let a = 42;
    let b = 100;

    // 2. HEAP (El Montón)
    // Box::new fuerza a guardar un entero simple en el HEAP.
    // 'c' es un puntero en el Stack que apunta al Heap.
    let c = Box::new(999);

    // Un String es complejo:
    // - El puntero, la longitud y la capacidad viven en el STACK.
    // - Las letras reales ("Simbionte") viven en el HEAP.
    let s = String::from("Simbionte");

    // 3. STATIC (Texto en el binario)
    // Estos textos existen antes de que el programa corra.
    let lit = "Soy Eterno";

    println!("--- VARIABLES EN EL STACK (Direcciones altas) ---");
    // &a significa "la direccion de a"
    println!("a (Entero):      {:p}", &a);
    println!("b (Entero):      {:p}", &b);

    // Aquí imprimimos la dirección del PUNTERO 'c', no del dato.
    println!("c (Puntero Box): {:p}", &c);
    println!("s (Puntero Str): {:p}", &s);

    println!("\n--- DATOS EN EL HEAP (Direcciones bajas o lejanas) ---");
    // *c accede al dato. &*c obtiene la dirección del DATO real en el Heap
    println!("Dato de c (Box): {:p}", &*c);
    // s.as_ptr() nos da la direccion del buffer de texto en el Heap
    println!("Dato de s (TXT): {:p}", s.as_ptr());

    println!("\n--- DATOS STATIC (Zona de Código) ---");
    println!("Literal:          {:p}", lit);

}

