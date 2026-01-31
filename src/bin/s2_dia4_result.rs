// Traemos herramientas para manejar archivos
use std::fs::File;
use std::io::Read;

fn main() {
    // Intentamos leer un archivo que (probablemente) no existe.
    let nombre_archivo = "archivo_secreto.txt";
 
    // leer_archivo devuelve un Result
    // Result<String, String> significa:
    // - Ok(String): "Toma, aqui esta el contenido".
    // - Err(String): "Falle, y aqui esta la razón en texto".
    let resultado = leer_archivo_seguro(nombre_archivo);

    match resultado {
        Ok(contenido) => println!(">>> ÉXITO: Leímos el archivo:\n{}", contenido),
        Err(error) => println!(">>> ALERTA: No se pudo leer. Razon: {}", error),
    }

}


// Función que devuelve un Result (Éxito o Error)
fn leer_archivo_seguro(path: &str) -> Result<String, String> {
    // Intentamos abrir el archivo. File::open devuleve un Result también.
    let archivo_result = File::open(path);

    // Usamos match para ver si se abrió
    let mut archivo = match archivo_result {
        Ok(f) => f, // Si abrio, capturamos el archivo 'f'
        Err(_) => return Err(String::from("El archivo no existe o no tengo permisos.")),
        // Si falló, terminamos la función AHORA MISMO y devolvemos el error.

    };

    // Si llegamos aquí, el archivo está abierto. Intentamos leerlo
    let mut contenido = String::new();

    // read_to_string tambien puede fallar (disco dañado, etc...)
    match archivo.read_to_string(&mut contenido) {
        Ok(_) => Ok(contenido), // Todo perfecto. Dwevolvemos el contenido envuelto en Ok.
        Err(_) => Err(String::from("Puedo abrirlo, pero no puedo leer los datos.")),
    }


}

