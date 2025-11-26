//! # Definición de Tipos de Datos
use std::fmt;

/// Tipos de datos básicos del lenguaje Patito
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TipoDato {
    /// Tipo de dato entero
    Entero,
    /// Tipo de dato flotante (punto flotante)
    Flotante,
    /// Tipo de dato caracter
    Char,
    /// Tipo de dato string/letrero
    Letrero,
}

impl fmt::Display for TipoDato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TipoDato::Entero => write!(f, "entero"),
            TipoDato::Flotante => write!(f, "flotante"),
            TipoDato::Char => write!(f, "char"),
            TipoDato::Letrero => write!(f, "letrero"),
        }
    }
}

impl TipoDato {
    /// Convierte una cadena de texto a un TipoDato
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "entero" => Some(TipoDato::Entero),
            "flotante" => Some(TipoDato::Flotante),
            "char" => Some(TipoDato::Char),
            "letrero" => Some(TipoDato::Letrero),
            _ => None,
        }
    }
}

/// Tipos de retorno para funciones en Patito
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TipoRetorno {
    /// La función retorna un tipo de dato específico
    Tipo(TipoDato),
    /// La función no retorna valor (void)
    Nula,
}

impl fmt::Display for TipoRetorno {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TipoRetorno::Tipo(t) => write!(f, "{}", t),
            TipoRetorno::Nula => write!(f, "nula"),
        }
    }
}

impl TipoRetorno {
    /// Convierte una cadena de texto a un TipoRetorno
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "nula" => Some(TipoRetorno::Nula),
            _ => TipoDato::from_str(s).map(TipoRetorno::Tipo),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tipo_dato_from_str() {
        assert_eq!(TipoDato::from_str("entero"), Some(TipoDato::Entero));
        assert_eq!(TipoDato::from_str("flotante"), Some(TipoDato::Flotante));
        assert_eq!(TipoDato::from_str("char"), Some(TipoDato::Char));
        assert_eq!(TipoDato::from_str("invalido"), None);
    }

    #[test]
    fn test_tipo_retorno_from_str() {
        assert_eq!(
            TipoRetorno::from_str("entero"),
            Some(TipoRetorno::Tipo(TipoDato::Entero))
        );
        assert_eq!(
            TipoRetorno::from_str("flotante"),
            Some(TipoRetorno::Tipo(TipoDato::Flotante))
        );
        assert_eq!(TipoRetorno::from_str("nula"), Some(TipoRetorno::Nula));
        assert_eq!(TipoRetorno::from_str("invalido"), None);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", TipoDato::Entero), "entero");
        assert_eq!(format!("{}", TipoDato::Flotante), "flotante");
        assert_eq!(format!("{}", TipoDato::Char), "char");
        assert_eq!(format!("{}", TipoRetorno::Nula), "nula");
        assert_eq!(
            format!("{}", TipoRetorno::Tipo(TipoDato::Entero)),
            "entero"
        );
    }
}
