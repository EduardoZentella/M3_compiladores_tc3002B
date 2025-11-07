//! # Cubo Semántico
//! El Cubo Semántico define las reglas de tipos para:
//! - Operadores aritméticos: +, -, *, /
//! - Operadores relacionales: >, <, ==, !=
//! - Operador de asignación: =

use crate::semantico::tipos::TipoDato;
use std::collections::HashMap;

/// Operadores soportados por el lenguaje Patito
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operador {
    // Aritméticos
    Suma,
    Resta,
    Multiplicacion,
    Division,

    // Relacionales
    MayorQue,
    MenorQue,
    Igual,
    Diferente,

    // Asignación
    Asignacion,
}

impl Operador {
    /// Convierte una cadena a un Operador
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "+" => Some(Operador::Suma),
            "-" => Some(Operador::Resta),
            "*" => Some(Operador::Multiplicacion),
            "/" => Some(Operador::Division),
            ">" => Some(Operador::MayorQue),
            "<" => Some(Operador::MenorQue),
            "==" => Some(Operador::Igual),
            "!=" => Some(Operador::Diferente),
            "=" => Some(Operador::Asignacion),
            _ => None,
        }
    }
}

/// Resultado de una validación de tipos
#[derive(Debug, Clone, PartialEq)]
pub enum ResultadoTipo {
    /// Operación válida con el tipo resultante
    Ok(TipoDato),
    /// Operación inválida (tipos incompatibles)
    Error,
}

/// Cubo Semántico para validación de tipos
///
/// # Reglas principales:
/// - **Aritméticas**: Cualquier operación con `flotante` promueve a `flotante`
/// - **Relacionales**: Siempre retornan `entero` (0 o 1 como booleano)
/// - **Asignación**: Fuertemente tipado, solo permite `flotante = entero` (promoción)
pub struct CuboSemantico {
    reglas: HashMap<(TipoDato, Operador, TipoDato), TipoDato>,
}

impl CuboSemantico {
    /// Crea un nuevo Cubo Semántico con todas las reglas predefinidas
    pub fn new() -> Self {
        let mut cubo = CuboSemantico {
            reglas: HashMap::new(),
        };

        cubo.inicializar_reglas();
        cubo
    }

    /// Inicializa todas las reglas del cubo semántico
    fn inicializar_reglas(&mut self) {
        // OPERADORES ARITMÉTICOS (+, -, *, /)
        // Regla: Cualquier operación con flotante promueve a flotante
        let operadores_aritmeticos = [
            Operador::Suma,
            Operador::Resta,
            Operador::Multiplicacion,
            Operador::Division,
        ];

        for &op in &operadores_aritmeticos {
            // entero OP entero = entero
            self.agregar_regla(TipoDato::Entero, op, TipoDato::Entero, TipoDato::Entero);

            // entero OP flotante = flotante
            self.agregar_regla(TipoDato::Entero, op, TipoDato::Flotante, TipoDato::Flotante);

            // flotante OP entero = flotante
            self.agregar_regla(TipoDato::Flotante, op, TipoDato::Entero, TipoDato::Flotante);

            // flotante OP flotante = flotante
            self.agregar_regla(TipoDato::Flotante, op, TipoDato::Flotante, TipoDato::Flotante);
        }

        // OPERADORES RELACIONALES (>, <, ==, !=)
        // Regla: Siempre retornan entero (0 = falso, 1 = verdadero)
        let operadores_relacionales = [
            Operador::MayorQue,
            Operador::MenorQue,
            Operador::Igual,
            Operador::Diferente,
        ];

        for &op in &operadores_relacionales {
            // entero OP entero = entero (booleano)
            self.agregar_regla(TipoDato::Entero, op, TipoDato::Entero, TipoDato::Entero);

            // entero OP flotante = entero (booleano)
            self.agregar_regla(TipoDato::Entero, op, TipoDato::Flotante, TipoDato::Entero);

            // flotante OP entero = entero (booleano)
            self.agregar_regla(TipoDato::Flotante, op, TipoDato::Entero, TipoDato::Entero);

            // flotante OP flotante = entero (booleano)
            self.agregar_regla(TipoDato::Flotante, op, TipoDato::Flotante, TipoDato::Entero);
        }

        // OPERADOR DE ASIGNACIÓN (=)
        // Regla: Fuertemente tipado
        // entero = entero
        self.agregar_regla(TipoDato::Entero, Operador::Asignacion, TipoDato::Entero, TipoDato::Entero);

        // flotante = flotante
        self.agregar_regla(TipoDato::Flotante, Operador::Asignacion, TipoDato::Flotante, TipoDato::Flotante);

        // flotante = entero (promoción permitida)
        self.agregar_regla(TipoDato::Flotante, Operador::Asignacion, TipoDato::Entero, TipoDato::Flotante);
            }

    /// Agrega una regla al cubo semántico
    fn agregar_regla(&mut self, tipo1: TipoDato, op: Operador, tipo2: TipoDato, resultado: TipoDato) {
        self.reglas.insert((tipo1, op, tipo2), resultado);
    }

    /// Valida una operación y retorna el tipo resultante
    ///
    /// # Argumentos
    /// * `tipo1` - Tipo del operando izquierdo
    /// * `operador` - Operador a aplicar
    /// * `tipo2` - Tipo del operando derecho
    ///
    /// # Retorna
    /// * `ResultadoTipo::Ok(tipo)` - Si la operación es válida
    /// * `ResultadoTipo::Error` - Si los tipos son incompatibles
    pub fn validar(&self, tipo1: TipoDato, operador: Operador, tipo2: TipoDato) -> ResultadoTipo {
        match self.reglas.get(&(tipo1, operador, tipo2)) {
            Some(&tipo_resultado) => ResultadoTipo::Ok(tipo_resultado),
            None => ResultadoTipo::Error,
        }
    }

    /// Imprime el cubo semántico completo
    pub fn imprimir(&self) {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║            CUBO SEMÁNTICO - LENGUAJE PATITO                  ║");
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        println!("OPERADORES ARITMÉTICOS (+, -, *, /):");
        println!("─────────────────────────────────────");
        self.imprimir_operador(Operador::Suma, "+");

        println!("\nOPERADORES RELACIONALES (>, <, ==, !=):");
        println!("────────────────────────────────────────");
        self.imprimir_operador(Operador::MayorQue, ">");

        println!("\nOPERADOR DE ASIGNACIÓN (=):");
        println!("───────────────────────────");
        self.imprimir_asignacion();
    }

    fn imprimir_operador(&self, op: Operador, simbolo: &str) {
        let tipos = [TipoDato::Entero, TipoDato::Flotante];

        for &t1 in &tipos {
            for &t2 in &tipos {
                if let Some(&resultado) = self.reglas.get(&(t1, op, t2)) {
                    println!("  {} {} {} = {}", t1, simbolo, t2, resultado);
                }
            }
        }
    }

    fn imprimir_asignacion(&self) {
        let tipos = [TipoDato::Entero, TipoDato::Flotante];

        for &t1 in &tipos {
            for &t2 in &tipos {
                match self.reglas.get(&(t1, Operador::Asignacion, t2)) {
                    Some(&resultado) => {
                        println!("  {} = {} → {} ✓", t1, t2, resultado);
                    }
                    None => {
                        println!("  {} = {} → ERROR ✗", t1, t2);
                    }
                }
            }
        }
    }
}

impl Default for CuboSemantico {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operadores_aritmeticos() {
        let cubo = CuboSemantico::new();

        // entero + entero = entero
        assert_eq!(
            cubo.validar(TipoDato::Entero, Operador::Suma, TipoDato::Entero),
            ResultadoTipo::Ok(TipoDato::Entero)
        );

        // entero + flotante = flotante (promoción)
        assert_eq!(
            cubo.validar(TipoDato::Entero, Operador::Suma, TipoDato::Flotante),
            ResultadoTipo::Ok(TipoDato::Flotante)
        );

        // flotante * entero = flotante
        assert_eq!(
            cubo.validar(TipoDato::Flotante, Operador::Multiplicacion, TipoDato::Entero),
            ResultadoTipo::Ok(TipoDato::Flotante)
        );

        // flotante / flotante = flotante
        assert_eq!(
            cubo.validar(TipoDato::Flotante, Operador::Division, TipoDato::Flotante),
            ResultadoTipo::Ok(TipoDato::Flotante)
        );
    }

    #[test]
    fn test_operadores_relacionales() {
        let cubo = CuboSemantico::new();

        // Todos los operadores relacionales retornan entero
        assert_eq!(
            cubo.validar(TipoDato::Entero, Operador::MayorQue, TipoDato::Entero),
            ResultadoTipo::Ok(TipoDato::Entero)
        );

        assert_eq!(
            cubo.validar(TipoDato::Flotante, Operador::MenorQue, TipoDato::Entero),
            ResultadoTipo::Ok(TipoDato::Entero)
        );

        assert_eq!(
            cubo.validar(TipoDato::Entero, Operador::Igual, TipoDato::Flotante),
            ResultadoTipo::Ok(TipoDato::Entero)
        );
    }

    #[test]
    fn test_asignacion() {
        let cubo = CuboSemantico::new();

        // entero = entero
        assert_eq!(
            cubo.validar(TipoDato::Entero, Operador::Asignacion, TipoDato::Entero),
            ResultadoTipo::Ok(TipoDato::Entero)
        );

        // flotante = flotante
        assert_eq!(
            cubo.validar(TipoDato::Flotante, Operador::Asignacion, TipoDato::Flotante),
            ResultadoTipo::Ok(TipoDato::Flotante)
        );

        // flotante = entero (promoción)
        assert_eq!(
            cubo.validar(TipoDato::Flotante, Operador::Asignacion, TipoDato::Entero),
            ResultadoTipo::Ok(TipoDato::Flotante)
        );

        // entero = flotante (truncamiento no permitido)
        assert_eq!(
            cubo.validar(TipoDato::Entero, Operador::Asignacion, TipoDato::Flotante),
            ResultadoTipo::Error
        );
    }
}
