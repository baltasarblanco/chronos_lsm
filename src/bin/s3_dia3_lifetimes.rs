fn main() {
    let heroe1 = String::from("Thor");
    let resultado; // Declaramos variable sin vallor

    { // --- INICIO DE UN MUNDO PARALELO (Scope interno) ---
        let heroe2 = String::from("Hulk"); //heroe2 nace aqui
        
        // Pasamos Thor (vida larga) y Hulk (vida corta).
        // El lifeime 'a se ajusta al MÁS CORTO (Hulk).
        resultado = el_mas_largo(heroe1.as_str(), heroe2.as_str());


        println!("Dentro del scope: {}", resultado); // Esto funciona
    } // --- FIN DEL MUNDO PARALELO --- Hulk muere aqui.

    // --- EL CRIMEN ---
    // Intentamos usar 'resultado' aqui afuera.
    // Pero 'resultado' estaba atado a la vida de Hulk (por el 'a).
    // Como Hulk murió, 'resultado' tambien murió.
    println!("Fuera del scope: {}", resultado);
}


// CORRECCIÓN 2: ¡Aquí está la función que habías borrado!
// Sin esto, el programa no sabe cómo elegir.
fn el_mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x 
    } else {
        y 
    }
}
