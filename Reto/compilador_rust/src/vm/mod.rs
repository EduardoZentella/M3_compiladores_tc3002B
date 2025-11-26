//! # Máquina Virtual
//!
//! Ejecuta código intermedio (cuádruplos) generado por el compilador.
//!
//! ## Arquitectura
//!
//! - **Memoria Segmentada**: GLOBAL (1000-4999), LOCAL (5000-9999), TEMPORAL (10000-14999), CONSTANTE (15000-24999)
//! - **Stack Frames**: Cada función tiene su propio marco de memoria con contexto local y temporal
//! - **Tabla de Funciones**: Mapea nombres de funciones a direcciones de código
//!
//! ## Módulos
//!
//! - `memoria`: Gestión de segmentos de memoria, valores y marcos (stack frames)
//! - `ejecutor`: Máquina virtual que ejecuta cuádruplos
//! - `io`: Abstracción de entrada/salida (consola, mock para tests)

pub mod memoria;
pub mod ejecutor;
pub mod io;

// Re-exportar tipos principales para facilitar el uso
pub use memoria::{Valor, SegmentoMemoria, MarcoMemoria, TipoSegmento};
pub use ejecutor::{MaquinaVirtual, InfoFuncion};
pub use io::{SistemaIO, ConsolaIO, MockIO};
