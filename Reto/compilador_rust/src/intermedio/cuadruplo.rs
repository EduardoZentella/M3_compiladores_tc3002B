//! # Cuádruplo
//!
//! Representa una instrucción de código intermedio en forma de cuádruplo.
//! Formato: (operador, operando_izq, operando_der, resultado)

use std::fmt;

/// Operadores para cuádruplos
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperadorCuadruplo {
    // Aritméticos
    Suma,           // +
    Resta,          // -
    Multiplicacion, // *
    Division,       // /

    // Relacionales
    MayorQue,       // >
    MenorQue,       // <
    Igual,          // ==
    Diferente,      // !=

    // Asignación
    Asignacion,     // =

    // E/S
    Lectura,        // lee
    Escritura,      // escribe

    // Control de flujo
    Goto,           // goto
    GotoF,          // goto falso (condicional)
    GotoV,          // goto verdadero (condicional)

    // Funciones
    Era,            // Activation Record
    Parametro,      // Paso de parámetro
    GoSub,          // Llamada a subrutina
}

impl OperadorCuadruplo {
    /// Convierte una cadena a un OperadorCuadruplo
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "+" => Some(OperadorCuadruplo::Suma),
            "-" => Some(OperadorCuadruplo::Resta),
            "*" => Some(OperadorCuadruplo::Multiplicacion),
            "/" => Some(OperadorCuadruplo::Division),
            ">" => Some(OperadorCuadruplo::MayorQue),
            "<" => Some(OperadorCuadruplo::MenorQue),
            "==" => Some(OperadorCuadruplo::Igual),
            "!=" => Some(OperadorCuadruplo::Diferente),
            "=" => Some(OperadorCuadruplo::Asignacion),
            "lee" => Some(OperadorCuadruplo::Lectura),
            "escribe" => Some(OperadorCuadruplo::Escritura),
            "goto" => Some(OperadorCuadruplo::Goto),
            "gotof" => Some(OperadorCuadruplo::GotoF),
            "gotov" => Some(OperadorCuadruplo::GotoV),
            "era" => Some(OperadorCuadruplo::Era),
            "param" => Some(OperadorCuadruplo::Parametro),
            "gosub" => Some(OperadorCuadruplo::GoSub),
            _ => None,
        }
    }

    /// Convierte el operador del cubo semántico al operador de cuádruplo
    pub fn from_operador_semantico(op: crate::semantico::cubo_semantico::Operador) -> Self {
        use crate::semantico::cubo_semantico::Operador;

        match op {
            Operador::Suma => OperadorCuadruplo::Suma,
            Operador::Resta => OperadorCuadruplo::Resta,
            Operador::Multiplicacion => OperadorCuadruplo::Multiplicacion,
            Operador::Division => OperadorCuadruplo::Division,
            Operador::MayorQue => OperadorCuadruplo::MayorQue,
            Operador::MenorQue => OperadorCuadruplo::MenorQue,
            Operador::Igual => OperadorCuadruplo::Igual,
            Operador::Diferente => OperadorCuadruplo::Diferente,
            Operador::Asignacion => OperadorCuadruplo::Asignacion,
        }
    }
}

impl fmt::Display for OperadorCuadruplo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OperadorCuadruplo::Suma => "+",
            OperadorCuadruplo::Resta => "-",
            OperadorCuadruplo::Multiplicacion => "*",
            OperadorCuadruplo::Division => "/",
            OperadorCuadruplo::MayorQue => ">",
            OperadorCuadruplo::MenorQue => "<",
            OperadorCuadruplo::Igual => "==",
            OperadorCuadruplo::Diferente => "!=",
            OperadorCuadruplo::Asignacion => "=",
            OperadorCuadruplo::Lectura => "lee",
            OperadorCuadruplo::Escritura => "escribe",
            OperadorCuadruplo::Goto => "goto",
            OperadorCuadruplo::GotoF => "gotof",
            OperadorCuadruplo::GotoV => "gotov",
            OperadorCuadruplo::Era => "era",
            OperadorCuadruplo::Parametro => "param",
            OperadorCuadruplo::GoSub => "gosub",
        };
        write!(f, "{}", s)
    }
}

/// Representa un operando en un cuádruplo
#[derive(Debug, Clone, PartialEq)]
pub enum Operando {
    /// Dirección virtual de memoria
    Direccion(usize),
    /// Variable o identificador (para depuración/compatibilidad)
    Variable(String),
    /// Constante entera
    ConstanteEntera(i32),
    /// Constante flotante
    ConstanteFlotante(f64),
    /// Temporal (t1, t2, t3, ...)
    Temporal(usize),
    /// Vacío (para operaciones unarias o sin operando)
    Vacio,
    /// Etiqueta de salto (dirección de cuádruplo)
    Etiqueta(usize),
    /// Pendiente (para saltos que se llenarán después)
    Pendiente,
}

impl fmt::Display for Operando {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operando::Direccion(dir) => write!(f, "@{}", dir),
            Operando::Variable(nombre) => write!(f, "{}", nombre),
            Operando::ConstanteEntera(valor) => write!(f, "{}", valor),
            Operando::ConstanteFlotante(valor) => write!(f, "{}", valor),
            Operando::Temporal(num) => write!(f, "t{}", num),
            Operando::Vacio => write!(f, "-"),
            Operando::Etiqueta(dir) => write!(f, "L{}", dir),
            Operando::Pendiente => write!(f, "?"),
        }
    }
}

/// Cuádruplo: (operador, operando_izq, operando_der, resultado)
#[derive(Debug, Clone)]
pub struct Cuadruplo {
    pub operador: OperadorCuadruplo,
    pub operando_izq: Operando,
    pub operando_der: Operando,
    pub resultado: Operando,
}

impl Cuadruplo {
    /// Crea un nuevo cuádruplo
    pub fn new(
        operador: OperadorCuadruplo,
        operando_izq: Operando,
        operando_der: Operando,
        resultado: Operando,
    ) -> Self {
        Cuadruplo {
            operador,
            operando_izq,
            operando_der,
            resultado,
        }
    }
}

impl fmt::Display for Cuadruplo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.operador, self.operando_izq, self.operando_der, self.resultado
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operador_from_str() {
        assert_eq!(OperadorCuadruplo::from_str("+"), Some(OperadorCuadruplo::Suma));
        assert_eq!(OperadorCuadruplo::from_str("=="), Some(OperadorCuadruplo::Igual));
        assert_eq!(OperadorCuadruplo::from_str("escribe"), Some(OperadorCuadruplo::Escritura));
        assert_eq!(OperadorCuadruplo::from_str("invalid"), None);
    }

    #[test]
    fn test_cuadruplo_display() {
        let quad = Cuadruplo::new(
            OperadorCuadruplo::Suma,
            Operando::Variable("a".to_string()),
            Operando::Variable("b".to_string()),
            Operando::Temporal(1),
        );
        assert_eq!(quad.to_string(), "(+, a, b, t1)");
    }

    #[test]
    fn test_operando_display() {
        assert_eq!(Operando::Variable("x".to_string()).to_string(), "x");
        assert_eq!(Operando::ConstanteEntera(42).to_string(), "42");
        assert_eq!(Operando::ConstanteFlotante(3.14).to_string(), "3.14");
        assert_eq!(Operando::Temporal(5).to_string(), "t5");
        assert_eq!(Operando::Vacio.to_string(), "-");
    }
}
