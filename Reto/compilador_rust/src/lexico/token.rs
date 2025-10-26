//! # Definición de Tokens
//!
//! Este módulo define los tipos de tokens reconocidos por el analizador léxico
//! y proporciona utilidades para trabajar con ellos.
//!
//! ## Estructura
//! - `TipoToken`: Enum con todos los tipos de tokens del lenguaje
//! - `Token`: Estructura que representa un token con su tipo, valor y línea
//!


/// Tipos de tokens reconocidos por el lenguaje.
///
/// Esta enumeración clasifica todos los elementos léxicos que puede
/// reconocer el compilador. Cada variante representa una categoría
/// sintáctica específica.
///
/// # Categorías
/// - **Keywords**: Palabras reservadas del lenguaje
/// - **Identificadores**: Nombres de variables, funciones, etc.
/// - **Constantes**: Valores literales (enteros, flotantes, cadenas)
/// - **Operadores**: Símbolos para operaciones aritméticas y lógicas
/// - **Separadores**: Símbolos de puntuación y delimitadores
/// - **EOF**: Marcador de fin de archivo
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TipoToken {
    // ═══════════════════════════════════════════════════════════
    // PALABRAS RESERVADAS (Keywords)
    // ═══════════════════════════════════════════════════════════
    /// Palabra reservada: `programa` - Inicia la declaración del programa
    Programa,
    /// Palabra reservada: `inicio` - Marca el inicio del cuerpo principal
    Inicio,
    /// Palabra reservada: `fin` - Marca el fin del programa
    Fin,
    /// Palabra reservada: `vars` - Inicia declaración de variables
    Vars,
    /// Palabra reservada: `entero` - Tipo de dato entero
    Entero,
    /// Palabra reservada: `flotante` - Tipo de dato de punto flotante
    Flotante,
    /// Palabra reservada: `escribe` - Función de salida (print)
    Escribe,
    /// Palabra reservada: `mientras` - Inicio de ciclo while
    Mientras,
    /// Palabra reservada: `haz` - Cuerpo del ciclo mientras
    Haz,
    /// Palabra reservada: `si` - Condicional if
    Si,
    /// Palabra reservada: `sino` - Condicional else
    Sino,
    /// Palabra reservada: `nula` - Tipo void (sin retorno)
    Nula,

    // ═══════════════════════════════════════════════════════════
    // IDENTIFICADORES Y CONSTANTES
    // ═══════════════════════════════════════════════════════════
    /// Identificador: nombres de variables, funciones, etc.
    /// Formato: [a-zA-Z_][a-zA-Z0-9_]*
    /// Ejemplos: `x`, `contador`, `_temp`, `suma_total`
    Id,

    /// Constante entera: números enteros
    /// Formato: [0-9]+
    /// Ejemplos: `0`, `42`, `1000`
    CteEnt,

    /// Constante flotante: números con punto decimal
    /// Formato: [0-9]+\.[0-9]+([eE][+-]?[0-9]+)?
    /// Ejemplos: `3.14`, `1.5e10`, `2.0e-5`
    CteFlot,

    /// Letrero: cadena de texto entre comillas dobles
    /// Formato: "[^"]*"
    /// Ejemplos: `"Hola mundo"`, `"Error en línea"`
    Letrero,

    // ═══════════════════════════════════════════════════════════
    // OPERADORES ARITMÉTICOS
    // ═══════════════════════════════════════════════════════════
    /// Operador: `+` (suma)
    Mas,
    /// Operador: `-` (resta)
    Menos,
    /// Operador: `*` (multiplicación)
    Multi,
    /// Operador: `/` (división)
    Div,

    // ═══════════════════════════════════════════════════════════
    // OPERADORES RELACIONALES
    // ═══════════════════════════════════════════════════════════
    /// Operador relacional: `>` (mayor que)
    MayorQue,
    /// Operador relacional: `<` (menor que)
    MenorQue,
    /// Operador relacional: `!=` (diferente de)
    Diferente,
    /// Operador relacional: `==` (igual a)
    Igual,

    // ═══════════════════════════════════════════════════════════
    // ASIGNACIÓN
    // ═══════════════════════════════════════════════════════════
    /// Operador de asignación: `=`
    Asignacion,

    // ═══════════════════════════════════════════════════════════
    // SEPARADORES Y DELIMITADORES
    // ═══════════════════════════════════════════════════════════
    /// Separador: `;` (punto y coma) - Termina sentencias
    PuntoYComa,
    /// Separador: `:` (dos puntos) - Declara tipos
    DosPuntos,
    /// Delimitador: `{` (abre llave) - Inicia bloque
    LlaveAbre,
    /// Delimitador: `}` (cierra llave) - Termina bloque
    LlaveCierra,
    /// Delimitador: `(` (abre paréntesis) - Agrupa expresiones
    ParenAbre,
    /// Delimitador: `)` (cierra paréntesis) - Cierra agrupación
    ParenCierra,
    /// Separador: `,` (coma) - Separa elementos en listas
    Coma,

    // ═══════════════════════════════════════════════════════════
    // TOKENS ESPECIALES
    // ═══════════════════════════════════════════════════════════
    /// Marcador de fin de archivo (End Of File)
    /// Se agrega automáticamente al final de la lista de tokens
    EOF,
}

impl TipoToken {
    /// Convierte el tipo de token a su representación en la gramática.
    ///
    /// Este método es **crucial** para el análisis sintáctico, ya que las tablas
    /// SLR usan estas representaciones como claves para buscar acciones.
    ///
    /// # Por qué existe este método
    /// El parser sintáctico necesita comparar tokens con símbolos terminales
    /// de la gramática. La gramática usa strings como "id", "+", "programa", etc.
    /// Este método proporciona esa conversión de manera centralizada.
    ///
    /// # Retorna
    /// Una referencia a un string estático que representa el token en la gramática.
    pub fn as_grammar(&self) -> &str {
        match self {
            // Keywords
            TipoToken::Programa => "programa",
            TipoToken::Inicio => "inicio",
            TipoToken::Fin => "fin",
            TipoToken::Vars => "vars",
            TipoToken::Entero => "entero",
            TipoToken::Flotante => "flotante",
            TipoToken::Escribe => "escribe",
            TipoToken::Mientras => "mientras",
            TipoToken::Haz => "haz",
            TipoToken::Si => "si",
            TipoToken::Sino => "sino",
            TipoToken::Nula => "nula",

            // Identificadores y Constantes
            TipoToken::Id => "id",
            TipoToken::CteEnt => "cte_ent",
            TipoToken::CteFlot => "cte_flot",
            TipoToken::Letrero => "letrero",

            // Operadores
            TipoToken::Mas => "+",
            TipoToken::Menos => "-",
            TipoToken::Multi => "*",
            TipoToken::Div => "/",
            TipoToken::MayorQue => ">",
            TipoToken::MenorQue => "<",
            TipoToken::Diferente => "!=",
            TipoToken::Igual => "==",
            TipoToken::Asignacion => "=",

            // Separadores
            TipoToken::PuntoYComa => ";",
            TipoToken::DosPuntos => ":",
            TipoToken::LlaveAbre => "{",
            TipoToken::LlaveCierra => "}",
            TipoToken::ParenAbre => "(",
            TipoToken::ParenCierra => ")",
            TipoToken::Coma => ",",

            // EOF
            TipoToken::EOF => "$",
        }
    }
}

/// Representa un token individual reconocido por el analizador léxico.
///
/// Un token es la unidad básica del análisis léxico. Contiene toda la
/// información necesaria para el análisis sintáctico y el reporte de errores.
///
/// # Campos
/// - `tipo`: Clasificación del token (palabra reservada, operador, etc.)
/// - `valor`: Texto literal tal como aparece en el código fuente
/// - `linea`: Número de línea donde aparece (para mensajes de error)
///
/// # Por qué guardamos el valor
/// Aunque el `tipo` clasifica el token, el `valor` es necesario para:
/// - Identificadores: saber qué variable es (`x`, `contador`, etc.)
/// - Constantes: saber el valor numérico (`42`, `3.14`, etc.)
/// - Letreros: saber el contenido de la cadena
/// - Debugging y mensajes de error descriptivos
#[derive(Debug, Clone)]
pub struct Token {
    /// Tipo/clasificación del token
    pub tipo: TipoToken,

    /// Valor literal del token como aparece en el código fuente
    pub valor: String,

    /// Número de línea donde aparece el token (1-indexed)
    /// Usado para reportar errores con ubicación precisa
    pub linea: usize,
}