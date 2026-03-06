// Definimos todos los comandos válidos en Chronos
pub enum Command {
    Set(String, String), // SET requiere una llave y un valor
    Get(String),         // GET requiere solo una llave
    Compact,
    Ping,
    Unknown,
}

// Esta función toma el texto sucio de la red y lo convierte en un 'Command'
pub fn parse(input: &str) -> Command {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    match parts.as_slice() {
        ["SET", key, value] => Command::Set(key.to_string(), value.to_string()),
        ["GET", key] => Command::Get(key.to_string()),
        ["COMPACT"] => Command::Compact,
        ["PING"] => Command::Ping,
        _ => Command::Unknown,
    }
}
