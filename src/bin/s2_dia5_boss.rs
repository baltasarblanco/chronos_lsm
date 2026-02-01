 use std::io::{self, Write}; // Traemos herramientas de entrada/salida

// 1. EL LENGUAJE INTERNO (Lo seguro)
#[derive(Debug)]
enum Comando {
    Publicar(String, String), // PUT key value
    Obtener(String),          // GET key
    Borrar(String),           // DELETE key
    Salir,
}

fn main() {
    println!(">>> BIENVENIDO A CHRONOS SHELL v0.1");
    println!(">>> Comando disponibles: PUT <k> <v>, GET <k>, DELETE <k>, EXIT");

    loop {
        // A. MOSTRAR PROMPT (El cursor parpadeante)
        println!("> ");
        io::stdout().flush().unwrap(); // Forzamos que se pinte el > en pantalla

        // B. LEER TECLADO
        let mut input_usuario = String::new();
        io::stdin().read_line(&mut input_usuario).unwrap();

        // C. PROCESAR (El núcleo de la misión)
        match parsear_comando(&input_usuario) {
            Ok(comando) => {
                // Si todo salió bien, ejecutamos la lógica
                match comando {
                    Comando::Salir => {
                        println!("Apagando sistemas...");
                        break; // Rompe el loop infinito y sale del programa
                    },
                    // Aqui simulamos que la base de datos trabaja
                    _ => println!(">>> EJECUTANDO: {:?}", comando),
                }
            
        
            },
            Err(error) => {
                // Si el usuario escribió basura, le avisamos sin crashear
                println!("!!! ERROR DE SINTAXIS: {}", error);

            }
        }
    }
}


// 2. EL TRADUCTOR (De Texto Sucio a Comando Puro)
// Recibe: "&str" (referencia al texto del usuario)
// Deuvuelve: Result<Comando, String>
fn parsear_comando(input: &str) -> Result<Comando, String> {
    // LIMPIEZA:
    // 1. trim(): Quita espacios al inicio y final ("   PUT    " -> "PUT")
    // 2. split_whitespace(): Corta el texto en palabras ignorando espacios multiples.
    // 3. collect(): Convierte el iterador en un Vector de palabras.
    let partes: Vec<&str> = input.trim().split_whitespace().collect();
    

    // ANÁLISIS (Slice Patterns):
    // Miramos el vector 'partes' como si fuera una rebanada (slice).
    match partes.as_slice() {
        // Caos: El usuario escribío "PUT", una clave y un valor.
        ["PUT", clave, valor] => {
            Ok(Comando::Publicar(clave.to_string(), valor.to_string()))
        },

        // Caso: El usuario escribío "GET" y una clave.
        ["GET", clave] => {
            Ok(Comando::Obtener(clave.to_string()))
        
        },

        // Caso: DELETE y clave
        ["DELETE", clave] => {
            Ok(Comando::Borrar(clave.to_string()))
        
        },

        // Caso: Solo EXIT
        ["EXIT"] => Ok(Comando::Salir),

        // Caso: Lsita vacía (usuario dio Enter sin escribir nada)
        [] => Err(String::from("No escribiste nada.")),

        // Caso: Cualquier otra cosa (Comando desconocido o argumentos incorrectos)
        _ => Err(String::from("Comando no reconocido o falta de argumentos")),
    }

}