/// Estructura persona.
#[derive(Debug)]
pub struct Persona {
    pub nombre: String,
    pub edad: i32,
}

pub struct Operaciones;

/// Implementación de funciones.
impl Operaciones {
    pub fn new() -> Operaciones {
        Operaciones {}
    }

    /// Saludar
    pub fn saludo(&self, saludo: &String) -> String {
        // self.
        println!("Nombre {}, saludo", saludo);
        String::from("Algo para devolver")
    }
}