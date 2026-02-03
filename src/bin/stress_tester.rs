use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;

// Configuración del ataque
const N_THREADS: usize = 50; // 50 usuarios simultáneos
const N_REQUESTS: usize = 100; // Cada usuario hace 100 peticiones

fn main() {
    println!("🔥 INICIANDO PRUEBA DE ESTRÉS (WRITE STORM) 🔥");
    println!("   Objetivo: localhost:8080");
    println!("   Hilos (Usuarios): {}", N_THREADS);
    println!("   Peticiones por usuario: {}", N_REQUESTS);
    println!("------------------------------------------------");

    let mut handles = vec![];

    // 1. Lanzamos la Horda
    for id in 0..N_THREADS {
        let handle = thread::spawn(move || {
            // Cada hilo simula se un cliente distinto
            if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8080") {
                for i in 0..N_REQUESTS {
                    // Esto va de 0 a 99
                    let key = format!("user_{}", id); // user_1, user_2...
                    let value = format!("data_{}", i); // data_0, data_1...

                    let command = format!("SET {} {} \n", key, value);

                    // Intentamos escribir
                    if stream.write_all(command.as_bytes()).is_err() {
                        eprintln!("   ❌ Fallo al enviar comando dede Hilo {}", id);
                        break;
                    }

                    // Leemos la respuesta ( para no saturar el buffer TCP )
                    let mut buffer = [0; 521];
                    let _ = stream.read(&mut buffer);
                }
                println!("   ✅ Hilo {} completó sus {} escrituras.", id, N_REQUESTS);
            } else {
                eprintln!("   💀 Hilo {} no pudo conectarse al servidor.", id);
            }
        });
        handles.push(handle);
    }

    // 2. Esperamos a que todos terminen
    for handle in handles {
        handle.join().unwrap();
    }

    println!("------------------------------------------------");
    println!("🏁 PRUEBA FINALIZADA. Verifica si el servidor sigue vivo.");
}
