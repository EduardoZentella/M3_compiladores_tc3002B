//! # Tabla de Variables
//! - Almacenar información sobre variables declaradas
//! - Validar que no haya declaraciones duplicadas
//! - Proporcionar acceso rápido al tipo de una variable

use crate::semantico::tipos::TipoDato;
use std::collections::HashMap;

/// Entrada de una variable en la tabla
/// Contiene toda la información asociada a una variable declarada.
#[derive(Debug, Clone)]
pub struct EntradaVariable {
    /// Tipo de dato de la variable (entero o flotante)
    pub tipo: TipoDato,
}

impl EntradaVariable {
    /// Crea una nueva entrada de variable
    pub fn new(tipo: TipoDato) -> Self {
        EntradaVariable { tipo }
    }
}

/// Tabla de Variables
///
/// Almacena las variables declaradas en un alcance específico (función o global).
/// Usa un HashMap para búsquedas O(1).
#[derive(Debug, Clone)]
pub struct TablaVariables {
    variables: HashMap<String, EntradaVariable>,
}

impl TablaVariables {
    /// Crea una nueva tabla de variables vacía
    pub fn new() -> Self {
        TablaVariables {
            variables: HashMap::new(),
        }
    }

    /// Agrega una variable a la tabla
    /// Lanza error si la variable ya existe en la tabla (declaración duplicada)
    pub fn agregar(&mut self, nombre: &str, tipo: TipoDato) -> Result<(), String> {
        // Validar que la variable no exista
        if self.variables.contains_key(nombre) {
            return Err(format!(
                "Error semántico: Variable '{}' doblemente declarada",
                nombre
            ));
        }

        // Agregar la variable
        self.variables.insert(
            nombre.to_string(),
            EntradaVariable::new(tipo),
        );

        Ok(())
    }

    /// Busca una variable en la tabla
    pub fn buscar(&self, nombre: &str) -> Option<&EntradaVariable> {
        self.variables.get(nombre)
    }

    /// Verifica si una variable existe en la tabla
    pub fn existe(&self, nombre: &str) -> bool {
        self.variables.contains_key(nombre)
    }

    /// Retorna el número de variables en la tabla
    pub fn cantidad(&self) -> usize {
        self.variables.len()
    }

    /// Retorna todas las variables como un vector de tuplas (nombre, tipo)
    pub fn listar(&self) -> Vec<(String, TipoDato)> {
        self.variables
            .iter()
            .map(|(nombre, entrada)| (nombre.clone(), entrada.tipo))
            .collect()
    }

    /// Imprime la tabla de variables (útil para debugging)
    pub fn imprimir(&self, titulo: &str) {
        println!("\n┌─────────────────────────────────────┐");
        println!("│ TABLA DE VARIABLES: {:<16}│", titulo);
        println!("├──────────────────┬──────────────────┤");
        println!("│ Variable         │ Tipo             │");
        println!("├──────────────────┼──────────────────┤");

        if self.variables.is_empty() {
            println!("│ (vacía)          │                  │");
        } else {
            let mut vars: Vec<_> = self.variables.iter().collect();
            vars.sort_by_key(|(nombre, _)| *nombre);

            for (nombre, entrada) in vars {
                println!("│ {:<16} │ {:<16} │", nombre, entrada.tipo);
            }
        }

        println!("└──────────────────┴──────────────────┘");
    }
}

impl Default for TablaVariables {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agregar_variable() {
        let mut tabla = TablaVariables::new();

        // Agregar variable exitosamente
        assert!(tabla.agregar("x", TipoDato::Entero).is_ok());
        assert!(tabla.agregar("y", TipoDato::Flotante).is_ok());

        // Variable duplicada debe fallar
        assert!(tabla.agregar("x", TipoDato::Entero).is_err());
    }

    #[test]
    fn test_buscar_variable() {
        let mut tabla = TablaVariables::new();
        tabla.agregar("contador", TipoDato::Entero).unwrap();

        // Buscar variable existente
        let entrada = tabla.buscar("contador");
        assert!(entrada.is_some());
        assert_eq!(entrada.unwrap().tipo, TipoDato::Entero);

        // Buscar variable inexistente
        assert!(tabla.buscar("inexistente").is_none());
    }

    #[test]
    fn test_existe() {
        let mut tabla = TablaVariables::new();
        tabla.agregar("total", TipoDato::Flotante).unwrap();

        assert!(tabla.existe("total"));
        assert!(!tabla.existe("inexistente"));
    }

    #[test]
    fn test_cantidad() {
        let mut tabla = TablaVariables::new();
        assert_eq!(tabla.cantidad(), 0);

        tabla.agregar("a", TipoDato::Entero).unwrap();
        assert_eq!(tabla.cantidad(), 1);

        tabla.agregar("b", TipoDato::Flotante).unwrap();
        assert_eq!(tabla.cantidad(), 2);
    }
}
