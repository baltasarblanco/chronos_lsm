// 1. DEFINIMOS EL TRAIT (El contrato)
// Cualquier que quiera ser "Notificable" DEBE implementar estas funciones.
pub trait Notificable {
    // Firma: Solo decimos cómo se llama y qué devuelve. No ponemos el código.
    fn crear_mensaje(&self) -> String;

    // Método por defecto: Si el usuario no lo programa, usa este.
    fn resumen(&self) -> String {
        String::from("(Sin resumen disponible)")
    }
}

// 2. ESTRUCTURA A: CORREO ELECTRÓNICO
struct Email {
    de: String,
    para: String,
    asunto: String,
    contenido: String,
}

// 3. ESTRUCTURA B: SMS (Mensaje de Texto)
struct SMS {
    numero: String,
    contenido: String,

}

// 4. IMPLEMENTACIÓN (Firmar el contrato)
// Enseñamos al Email a ser "Notificable"
impl Notificable for Email {
    fn crear_mensaje(&self) -> String {
        format!("EMAIL de {}:\nAsunto: {}\n{}", self.de, self.asunto, self.contenido)

    }
    
    fn resumen(&self) -> String {
        format!("Email de {} sobre {}", self.de, self.asunto)
    }
}

// Enseñamos alSMS a ser "Notificable"
impl Notificable for SMS {
    fn crear_mensaje(&self) -> String {
        format!("SMS del {}: {}", self.numero, self.contenido)
    
    }
    // No implementamos 'resumen', usará el de defecto.
}

fn main() {
    let mi_email = Email {
        de: String::from("jefe@chronos.com"),
        para: String::from("tu@chronos.com"),
        asunto: String::from("Aumento de sueldo"),
        contenido: String::from("Felicidades por sobrevivir a la Semana 3."),
    };

    let mi_sms = SMS {
        numero: String::from("+54911223344"),
        contenido: String::from("Compra leche."),
    };
    
    println!("--- SISTEMA DE NOTIFICACIONES ---");

    // Podemos llamar a los métodos del Trait como si fueran nativos
    enviar_notificacion(mi_email);
    enviar_notificacion(mi_sms);
}



// FUNCION MÁGICA (POLIMORFISMO)
// Recibe "Cualqueir cosa (T) que implemente el Trait Notificable".
// Esta función funciona para Emails, SMS, Señales de Humo, etc.
fn enviar_notificacion<T: Notificable>(item: T) {
    println!("ENVIANDO: {}", item.resumen());
    println!("Cuerpo: {}\n", item.crear_mensaje());
}