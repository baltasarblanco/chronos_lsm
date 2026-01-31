// --- RETO: LA CALCULADORA DE ENUMS ---

// 1. DEFINICIÓN: Rellena las variantes que faltan.
// Todas deben llevar dos números enteros (i32, i32).
enum Operacion {
    Sumar(i32, i32),
    Restar(i32, i32),       // <--- Rellena los tipos
    Multiplicar(i32, i32),  // <--- Rellena los tipos
    Dividir(i32, i32),      // <--- Rellena los tipos
}

fn main() {
    // 2. CREACIÓN: Instancia las operaciones.
    // He hecho la suma por ti. Haz tú la resta, multi y la división peligrosa.
    
    let op_suma = Operacion::Sumar(10, 5);
    let op_resta = Operacion::Restar(20, 8);
    let op_multi = Operacion::Multiplicar(5, 5); // Crea una multiplicación de 5 * 5
    let op_div_normal = Operacion::Dividir(20, 2); // Crea una división de 20 / 2
    let op_div_cero = Operacion::Dividir(10, 0);   // La trampa.

    // Ejecutamos la calculadora
    println!("Suma (10+5): {}", calcular(op_suma));
    println!("Resta (20-8): {}", calcular(op_resta));
    println!("Multi (5*5): {}", calcular(op_multi));
    println!("División (20/2): {}", calcular(op_div_normal));
    println!("División (10/0): {}", calcular(op_div_cero));
}

// 3. LÓGICA: El cerebro de la calculadora.
// Recibe una operación y devuelve un entero (i32).
fn calcular(op: Operacion) -> i32 {
    match op {
        // Caso 1: Suma (Ya hecho)
        Operacion::Sumar(a, b) => a + b,

        // Caso 2: Resta (Rellena la lógica)
        Operacion::Restar(a, b) => a - b, 

        // Caso 3: Multiplicación
        Operacion::Multiplicar(a, b) => a * b,

        // Caso 4: División con SEGURIDAD
        Operacion::Dividir(a, b) => {
            // AQUÍ ESTÁ LA PRUEBA DE FUEGO.
            // Si b es 0, no dividas. Devuelve 0 e imprime un error.
            if b == 0 {
                println!("¡ALERTA NUCLEAR! Intento de división por cero.");
                0 // ¿Qué número devolvemos si falla?
            } else {
                a / b // Haz la división normal
            }
        },
    }
}