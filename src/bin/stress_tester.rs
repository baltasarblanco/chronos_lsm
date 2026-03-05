// Este código va a abrir 50 conexiones TCP simultáneas y bombardeara el servidor midiendo los milisegundos exactos.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Instant;

fn main() {
    let num_threads = 50;
    let requests_per_thread = 100; // 50 * 100 = 5,000 peticiones en total

    println!("🚀 Iniciando Benchmark Chronos...");
    println!(
        "  Lanzando {} hilos con {} peticiones cada uno por red TCP.",
        num_threads, requests_per_thread
    );

    // Arrancamos el cronómetro
    let start_time = Instant::now();
    let mut handles = vec![];

    for i in 0..num_threads {
        let handle = thread::spawn(move || {
            // Cada hilo sed conecta como un cliente independiente a tu servidor
            if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8080") {
                for j in 0..requests_per_thread {
                    // Enviamos un comando SET
                    let msg = format!("SET banch_key_{}_{} benchmark_value\n", i, j);
                    let _ = stream.write_all(msg.as_bytes());

                    // Leemos el "OK" de respuesta para confirmar
                    let mut buffer = [0; 512];
                    let _ = stream.read(&mut buffer);
                }
            } else {
                println!(
                    "❌ Error: El hilo {} no pudo conectarse al servidor. ¿Está encendido?",
                    i
                );
            }
        });
        handles.push(handle);
    }

    // Esperamos a que todos los hilos terminen su ataque
    for handle in handles {
        handle.join().unwrap();
    }

    // Paramos el cronómetro
    let duration = start_time.elapsed();
    let total_request = num_threads * requests_per_thread;
    let ops_per_second = (total_request as f64 / duration.as_secs_f64()) as u64;

    println!("--------------------------------------------------");
    println!("⏱️  Tiempo total: {:?}", duration);
    println!(
        "🔥 Rendimiento: {} operaciones por segundo (ops/sec)",
        ops_per_second
    );
    println!("--------------------------------------------------");
}
