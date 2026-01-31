#[derive(Debug)]
enum Moneda {
    Dolar,
    Centavo,
    Peso,
    BilleteDeDiez,
    BilleteRaro(String), // Un billete de colecciíon con nombre
}

fn main() {
    let bolsillo = vec![
        Moneda::Dolar,
        Moneda::Centavo,
        Moneda::Peso,
        Moneda::BilleteDeDiez,
        Moneda::BilleteRaro(String::from("Patacón")),
    ];

    let total = contar_dinero(bolsillo);
    println!("Tienes un valor total de: ${}", total);
}

fn contar_dinero(monedas: Vec<Moneda>) -> i32 {
    let mut acumulado = 0;

    // Iteramos sobre cada moneda del vector
    for moneda in monedas {
        // MATCH: El oráculo que decide cuánto vale cada cosa
        let valor = match moneda {
            Moneda::Dolar => 2500,
            Moneda::Centavo => 1,
            Moneda::Peso => 100, // Asumimos 100 centavos
            Moneda::BilleteDeDiez => 1000,

            // DESTRUCTURING: Extraemos el dato interno 'nombre'
            Moneda::BilleteRaro(nombre) => {
                println!("¡Atención! Encontré una rareza: {}", nombre);
                // Si es Patacón, no vale nada (broma de economía argentina)
                if nombre == "Patacón" {
                    0
                } else {
                    5000 // Otros raros valen mucho
                }
            
            }
        }; // El punto y coma aquí es vital ( el match devuelve un valor)

        acumulado += valor;
    }
    

    acumulado
}
