//! # Sistema de E/S
//!
//! Abstracción de entrada/salida para la Máquina Virtual.
//! Permite inyectar diferentes implementaciones (consola, mock, archivo, etc.)

use std::io::{self, Write};

/// Trait para sistemas de entrada/salida
pub trait SistemaIO {
    /// Lee una línea de entrada
    fn leer_linea(&mut self) -> Result<String, String>;

    /// Escribe una línea de salida
    fn escribir_linea(&mut self, msg: &str);
}

/// Implementación de E/S usando consola estándar (stdin/stdout)
pub struct ConsolaIO;

impl ConsolaIO {
    pub fn new() -> Self {
        ConsolaIO
    }
}

impl SistemaIO for ConsolaIO {
    fn leer_linea(&mut self) -> Result<String, String> {
        print!("> ");
        io::stdout().flush().map_err(|e| e.to_string())?;

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .map_err(|e| format!("Error al leer entrada: {}", e))?;

        Ok(buffer.trim().to_string())
    }

    fn escribir_linea(&mut self, msg: &str) {
        println!("{}", msg);
    }
}

/// Implementación de E/S mock para testing
pub struct MockIO {
    /// Buffer de entrada (las líneas se consumen en orden FIFO)
    entrada: Vec<String>,
    /// Buffer de salida (se acumulan todas las escrituras)
    salida: Vec<String>,
    /// Índice actual en el buffer de entrada
    indice_entrada: usize,
}

impl MockIO {
    /// Crea un nuevo MockIO con entrada predefinida
    pub fn new(entradas: Vec<String>) -> Self {
        MockIO {
            entrada: entradas,
            salida: Vec::new(),
            indice_entrada: 0,
        }
    }

    /// Crea un MockIO vacío (sin entrada)
    pub fn vacio() -> Self {
        MockIO {
            entrada: Vec::new(),
            salida: Vec::new(),
            indice_entrada: 0,
        }
    }

    /// Agrega una línea de entrada al buffer
    pub fn agregar_entrada(&mut self, linea: String) {
        self.entrada.push(linea);
    }

    /// Obtiene todas las salidas generadas
    pub fn obtener_salidas(&self) -> &[String] {
        &self.salida
    }

    /// Obtiene la última salida generada
    pub fn ultima_salida(&self) -> Option<&String> {
        self.salida.last()
    }

    /// Limpia el buffer de salida
    pub fn limpiar_salida(&mut self) {
        self.salida.clear();
    }
}

impl SistemaIO for MockIO {
    fn leer_linea(&mut self) -> Result<String, String> {
        if self.indice_entrada >= self.entrada.len() {
            return Err("MockIO: No hay más líneas de entrada disponibles".to_string());
        }

        let linea = self.entrada[self.indice_entrada].clone();
        self.indice_entrada += 1;
        Ok(linea)
    }

    fn escribir_linea(&mut self, msg: &str) {
        self.salida.push(msg.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_io_lectura() {
        let mut mock = MockIO::new(vec![
            "42".to_string(),
            "3.14".to_string(),
        ]);

        assert_eq!(mock.leer_linea().unwrap(), "42");
        assert_eq!(mock.leer_linea().unwrap(), "3.14");
        assert!(mock.leer_linea().is_err());
    }

    #[test]
    fn test_mock_io_escritura() {
        let mut mock = MockIO::vacio();

        mock.escribir_linea("Hola");
        mock.escribir_linea("Mundo");

        assert_eq!(mock.obtener_salidas(), &["Hola", "Mundo"]);
        assert_eq!(mock.ultima_salida().unwrap(), "Mundo");
    }

    #[test]
    fn test_mock_io_agregar_entrada() {
        let mut mock = MockIO::vacio();
        mock.agregar_entrada("100".to_string());
        mock.agregar_entrada("200".to_string());

        assert_eq!(mock.leer_linea().unwrap(), "100");
        assert_eq!(mock.leer_linea().unwrap(), "200");
    }
}
