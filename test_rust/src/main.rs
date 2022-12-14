use estructuras::{Operaciones, Persona};

mod estructuras;

fn main() {

    //let nombre = "Eduardo";
    //let mut nombre: &str = "Eduardo"; //mut para poder modificar variable porque por defecto es inmutable.

    let nombre: String = String::from("Eduardo");

    let primera_persona = estructuras::Persona {
        nombre: String::from("Eduardo"),
        edad: 38,
    };

    let operaciones: Operaciones = Operaciones::new();

    operaciones.saludo(&primera_persona.nombre);

    println!("Final");
}
