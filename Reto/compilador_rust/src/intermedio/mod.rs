//! # Módulo de Generación de Código Intermedio
//!
//! Este módulo contiene las estructuras y algoritmos para generar código intermedio (cuádruplos).

pub mod cuadruplo;
pub mod generador;
pub mod memoria;
pub mod memoria_virtual;

pub use cuadruplo::{Cuadruplo, OperadorCuadruplo, Operando};
pub use generador::GeneradorCuadruplos;
pub use memoria::GestorMemoria;
pub use memoria_virtual::{MemoriaVirtual, TipoSegmento};
