//! # Directorio de Funciones
//! - Almacenar información sobre todas las funciones (incluyendo el alcance global)
//! - Mantener la tabla de variables de cada función
//! - Validar que no haya funciones duplicadas
//! - Proporcionar acceso a las funciones y sus variables

use crate::semantico::tipos::TipoRetorno;
use crate::semantico::tabla_variables::TablaVariables;
use std::collections::HashMap;

/// Entrada de una función en el directorio
/// Contiene toda la información asociada a una función declarada,
/// incluyendo su tabla de variables local.
#[derive(Debug, Clone)]
pub struct EntradaFuncion {
    /// Tipo de retorno de la función
    pub tipo_retorno: TipoRetorno,

    /// Tabla de variables locales de la función
    pub tabla_variables: TablaVariables,
}

impl EntradaFuncion {
    /// Crea una nueva entrada de función
    pub fn new(tipo_retorno: TipoRetorno) -> Self {
        EntradaFuncion {
            tipo_retorno,
            tabla_variables: TablaVariables::new(),
        }
    }
}

/// Directorio de Funciones
/// - El alcance global se representa como una función especial con el nombre del programa
/// - Cada función tiene su propia tabla de variables
/// - Búsquedas O(1) gracias al HashMap
pub struct DirectorioFunciones {
    funciones: HashMap<String, EntradaFuncion>,
}

impl DirectorioFunciones {
    /// Crea un nuevo directorio de funciones vacío
    pub fn new() -> Self {
        DirectorioFunciones {
            funciones: HashMap::new(),
        }
    }

    /// Agrega una función al directorio
    /// Lanza error si la función ya existe (declaración duplicada)
    pub fn agregar_funcion(&mut self, nombre: &str, tipo_retorno: TipoRetorno) -> Result<(), String> {
        // Validar que la función no exista
        if self.funciones.contains_key(nombre) {
            return Err(format!(
                "Error semántico: Función '{}' doblemente declarada",
                nombre
            ));
        }

        // Agregar la función
        self.funciones.insert(
            nombre.to_string(),
            EntradaFuncion::new(tipo_retorno),
        );

        Ok(())
    }

    /// Busca una función en el directorio
    pub fn buscar_funcion(&self, nombre: &str) -> Option<&EntradaFuncion> {
        self.funciones.get(nombre)
    }

    /// Busca una función de forma mutable
    pub fn buscar_funcion_mut(&mut self, nombre: &str) -> Option<&mut EntradaFuncion> {
        self.funciones.get_mut(nombre)
    }

    /// Verifica si una función existe
    pub fn existe_funcion(&self, nombre: &str) -> bool {
        self.funciones.contains_key(nombre)
    }

    /// Agrega una variable a una función específica
    pub fn agregar_variable(
        &mut self,
        nombre_funcion: &str,
        nombre_variable: &str,
        tipo: crate::semantico::tipos::TipoDato,
    ) -> Result<(), String> {
        // Buscar la función
        let funcion = self.funciones.get_mut(nombre_funcion)
            .ok_or_else(|| format!(
                "Error semántico: Función '{}' no existe",
                nombre_funcion
            ))?;

        // Agregar la variable a la tabla de variables de la función
        funcion.tabla_variables.agregar(nombre_variable, tipo)
    }

    /// Busca una variable en una función específica
    pub fn buscar_variable(
        &self,
        nombre_funcion: &str,
        nombre_variable: &str,
    ) -> Option<&crate::semantico::tabla_variables::EntradaVariable> {
        self.funciones
            .get(nombre_funcion)?
            .tabla_variables
            .buscar(nombre_variable)
    }

    /// Retorna el número de funciones en el directorio
    pub fn cantidad_funciones(&self) -> usize {
        self.funciones.len()
    }

    /// Imprime el directorio de funciones completo (útil para debugging)
    pub fn imprimir(&self) {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║           DIRECTORIO DE FUNCIONES                            ║");
        println!("╚══════════════════════════════════════════════════════════════╝");

        if self.funciones.is_empty() {
            println!("(vacío)");
            return;
        }

        // Ordenar funciones por nombre para imprimir
        let mut funciones: Vec<_> = self.funciones.iter().collect();
        funciones.sort_by_key(|(nombre, _)| *nombre);

        for (nombre, entrada) in funciones {
            println!("\n┌─────────────────────────────────────────────────────────────┐");
            println!("│ Función: {:<50} │", nombre);
            println!("│ Tipo de retorno: {:<43} │", entrada.tipo_retorno);
            println!("│ Variables locales: {:<39} │", entrada.tabla_variables.cantidad());
            println!("└─────────────────────────────────────────────────────────────┘");

            // Imprimir tabla de variables si no está vacía
            if entrada.tabla_variables.cantidad() > 0 {
                entrada.tabla_variables.imprimir(nombre);
            }
        }
    }
}

impl Default for DirectorioFunciones {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantico::tipos::{TipoDato, TipoRetorno};

    #[test]
    fn test_agregar_funcion() {
        let mut dir = DirectorioFunciones::new();

        // Agregar función exitosamente
        assert!(dir.agregar_funcion("main", TipoRetorno::Nula).is_ok());
        assert!(dir.agregar_funcion("calcular", TipoRetorno::Tipo(TipoDato::Entero)).is_ok());

        // Función duplicada debe fallar
        assert!(dir.agregar_funcion("main", TipoRetorno::Nula).is_err());
    }

    #[test]
    fn test_buscar_funcion() {
        let mut dir = DirectorioFunciones::new();
        dir.agregar_funcion("suma", TipoRetorno::Tipo(TipoDato::Entero)).unwrap();

        // Buscar función existente
        let func = dir.buscar_funcion("suma");
        assert!(func.is_some());
        assert_eq!(func.unwrap().tipo_retorno, TipoRetorno::Tipo(TipoDato::Entero));

        // Buscar función inexistente
        assert!(dir.buscar_funcion("inexistente").is_none());
    }

    #[test]
    fn test_agregar_variable() {
        let mut dir = DirectorioFunciones::new();
        dir.agregar_funcion("mi_func", TipoRetorno::Nula).unwrap();

        // Agregar variable a función existente
        assert!(dir.agregar_variable("mi_func", "x", TipoDato::Entero).is_ok());
        assert!(dir.agregar_variable("mi_func", "y", TipoDato::Flotante).is_ok());

        // Agregar variable duplicada debe fallar
        assert!(dir.agregar_variable("mi_func", "x", TipoDato::Entero).is_err());

        // Agregar variable a función inexistente debe fallar
        assert!(dir.agregar_variable("inexistente", "z", TipoDato::Entero).is_err());
    }

    #[test]
    fn test_buscar_variable() {
        let mut dir = DirectorioFunciones::new();
        dir.agregar_funcion("test", TipoRetorno::Nula).unwrap();
        dir.agregar_variable("test", "contador", TipoDato::Entero).unwrap();

        // Buscar variable existente
        let var = dir.buscar_variable("test", "contador");
        assert!(var.is_some());
        assert_eq!(var.unwrap().tipo, TipoDato::Entero);

        // Buscar variable inexistente
        assert!(dir.buscar_variable("test", "inexistente").is_none());

        // Buscar en función inexistente
        assert!(dir.buscar_variable("inexistente", "contador").is_none());
    }
}
