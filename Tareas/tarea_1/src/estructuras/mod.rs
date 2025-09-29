// src/estructuras/mod.rs
//! # Módulo de Estructuras de Datos Fundamentales
//!
//! Este módulo proporciona implementaciones robustas y eficientes de las principales
//! estructuras de datos utilizadas en ciencias de la computación y desarrollo de software.
//!
//! ## Estructuras implementadas:
//!
//! ### 1. Stack (Pila) - LIFO
//! - **Principio**: Last In, First Out
//! - **Operaciones principales**: push, pop, peek
//! - **Uso típico**: Evaluación de expresiones, call stacks, undo/redo
//!
//! ### 2. Queue (Cola) - FIFO
//! - **Principio**: First In, First Out
//! - **Operaciones principales**: encolar, desencolar, frente
//! - **Uso típico**: Sistemas de colas, BFS, buffering
//!
//! ### 3. HashMap (Tabla Hash/Diccionario)
//! - **Principio**: Mapeo clave-valor con acceso rápido
//! - **Operaciones principales**: insertar, obtener, remover
//! - **Uso típico**: Cachés, índices, contadores, configuraciones
//!
//! ## Características comunes:
//! - Manejo robusto de errores con tipos específicos
//! - Capacidad configurable (limitada o ilimitada)
//! - Operaciones eficientes con complejidades bien definidas
//! - Interfaz consistente y fácil de usar
//! - Documentación completa y tests exhaustivos

// Declarar submódulos
pub mod queue;
pub mod stack;
pub mod hashmap;

// Reexportar elementos públicos para facilitar el uso
pub use queue::{Cola, ErrorCola, ResultadoCola};
pub use stack::{Stack, ErrorStack, ResultadoStack};
pub use hashmap::{MiHashMap, ErrorHashMap, ResultadoHashMap};

#[cfg(test)]
mod tests_cola;

#[cfg(test)]
mod tests_stack;

#[cfg(test)]
mod tests_hashmap;