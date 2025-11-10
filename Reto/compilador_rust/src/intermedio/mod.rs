//! # Módulo de Generación de Código Intermedio
//! 
//! Este módulo implementa la generación de cuádruplos (código intermedio)
//! para el compilador de Patito.

mod cuadruplo;
mod memoria;
mod generador;

pub use cuadruplo::{Cuadruplo, OperadorCuadruplo};
pub use memoria::GestorMemoria;
pub use generador::GeneradorCuadruplos;
