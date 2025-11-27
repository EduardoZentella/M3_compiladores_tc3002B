//! # Módulo de Generación de Código Intermedio
//!
//! Este módulo contiene las estructuras y algoritmos para generar código intermedio (cuádruplos).

pub mod cuadruplo;
pub mod generador;
pub mod memoria_virtual;
pub mod programa;

pub use cuadruplo::{Cuadruplo, OperadorCuadruplo, Operando};
pub use generador::GeneradorCuadruplos;
pub use memoria_virtual::{MemoriaVirtual, TipoSegmento};
pub use programa::{ProgramaObjeto, InfoFuncionPrograma};
