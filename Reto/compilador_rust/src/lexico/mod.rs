pub mod token;

use crate::lexico::token::{Token, TipoToken};
use regex::Regex;
use lazy_static::lazy_static;

// Estructura para definir las reglas de tokens
struct ReglaToken {
    tipo: TipoToken,
    patron: Regex,
}

// Función auxiliar para logging con niveles de verbose
fn verbose_log(mensaje: &str, nivel_requerido: usize, nivel_actual: usize) {
    if nivel_actual >= nivel_requerido {
        println!("{}", mensaje);
    }
}

// Lazy Static para inicializar las reglas de tokens
lazy_static! {
    // Vector de reglas de tokens en el orden correcto
    static ref REGLAS_TOKENS: Vec<ReglaToken> = vec![
        // Keywords
        ReglaToken { tipo: TipoToken::Programa, patron: Regex::new(r"^\bprograma\b").unwrap() },
        ReglaToken { tipo: TipoToken::Inicio, patron: Regex::new(r"^\binicio\b").unwrap() },
        ReglaToken { tipo: TipoToken::Fin, patron: Regex::new(r"^\bfin\b").unwrap() },
        ReglaToken { tipo: TipoToken::Vars, patron: Regex::new(r"^\bvars\b").unwrap() },
        ReglaToken { tipo: TipoToken::Entero, patron: Regex::new(r"^\bentero\b").unwrap() },
        ReglaToken { tipo: TipoToken::Flotante, patron: Regex::new(r"^\bflotante\b").unwrap() },
        ReglaToken { tipo: TipoToken::LetreroTipo, patron: Regex::new(r"^\bletrero\b").unwrap() },
        ReglaToken { tipo: TipoToken::Escribe, patron: Regex::new(r"^\bescribe\b").unwrap() },
        ReglaToken { tipo: TipoToken::Mientras, patron: Regex::new(r"^\bmientras\b").unwrap() },
        ReglaToken { tipo: TipoToken::Haz, patron: Regex::new(r"^\bhaz\b").unwrap() },
        ReglaToken { tipo: TipoToken::Si, patron: Regex::new(r"^si\b").unwrap() },
        ReglaToken { tipo: TipoToken::Entonces, patron: Regex::new(r"^entonces\b").unwrap() },
        ReglaToken { tipo: TipoToken::Sino, patron: Regex::new(r"^sino\b").unwrap() },
        ReglaToken { tipo: TipoToken::Nula, patron: Regex::new(r"^\bnula\b").unwrap() },
        ReglaToken { tipo: TipoToken::Regresa, patron: Regex::new(r"^\bregresa\b").unwrap() },

        // Constantes (deben ir antes de identificadores)
        ReglaToken { tipo: TipoToken::CteFlot, patron: Regex::new(r"^[0-9]+\.[0-9]+([eE][+-]?[0-9]+)?").unwrap() },
        ReglaToken { tipo: TipoToken::CteEnt, patron: Regex::new(r"^[0-9]+").unwrap() },
        ReglaToken { tipo: TipoToken::Letrero, patron: Regex::new(r#"^"[^"]*""#).unwrap() },

        // Identificadores
        ReglaToken { tipo: TipoToken::Id, patron: Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap() },

        // Operadores (los compuestos primero)
        ReglaToken { tipo: TipoToken::Diferente, patron: Regex::new(r"^!=").unwrap() },
        ReglaToken { tipo: TipoToken::Igual, patron: Regex::new(r"^==").unwrap() },
        ReglaToken { tipo: TipoToken::Mas, patron: Regex::new(r"^\+").unwrap() },
        ReglaToken { tipo: TipoToken::Menos, patron: Regex::new(r"^-").unwrap() },
        ReglaToken { tipo: TipoToken::Multi, patron: Regex::new(r"^\*").unwrap() },
        ReglaToken { tipo: TipoToken::Div, patron: Regex::new(r"^/").unwrap() },
        ReglaToken { tipo: TipoToken::MayorQue, patron: Regex::new(r"^>").unwrap() },
        ReglaToken { tipo: TipoToken::MenorQue, patron: Regex::new(r"^<").unwrap() },
        ReglaToken { tipo: TipoToken::Asignacion, patron: Regex::new(r"^=").unwrap() },

        // Separadores
        ReglaToken { tipo: TipoToken::PuntoYComa, patron: Regex::new(r"^;").unwrap() },
        ReglaToken { tipo: TipoToken::DosPuntos, patron: Regex::new(r"^:").unwrap() },
        ReglaToken { tipo: TipoToken::LlaveAbre, patron: Regex::new(r"^\{").unwrap() },
        ReglaToken { tipo: TipoToken::LlaveCierra, patron: Regex::new(r"^\}").unwrap() },
        ReglaToken { tipo: TipoToken::ParenAbre, patron: Regex::new(r"^\(").unwrap() },
        ReglaToken { tipo: TipoToken::ParenCierra, patron: Regex::new(r"^\)").unwrap() },
        ReglaToken { tipo: TipoToken::Coma, patron: Regex::new(r"^,").unwrap() },
    ];

    // Regex para espacios en blanco y saltos de linea
    static ref REGLAS_ESPACIOS: Regex = Regex::new(r"^[ \t\r\n]+").unwrap();
}

// Funcion de analisis lexico
pub fn analyze(input: &str, nivel_verbose: usize) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut linea = 1;
    let mut resto = input;

    while !resto.is_empty() {
        // Ignorar espacios en blanco y saltos de linea
        if let Some(mat) = REGLAS_ESPACIOS.find(resto) {
            let espacio = mat.as_str();
            linea += espacio.matches('\n').count();
            resto = &resto[mat.end()..];
            continue;
        }

        // Buscar coincidencias con las reglas de tokens
        let mut matched = false;
        for ReglaToken { tipo, patron } in REGLAS_TOKENS.iter() {
            if let Some(mat) = patron.find(resto) {
                let valor = mat.as_str().to_string();
                tokens.push(Token {
                    tipo: tipo.clone(),
                    valor: valor.clone(),
                    linea,
                });
                // Nivel 3: mostrar cada token encontrado
                verbose_log(&format!("Token encontrado: {:?} ('{}') en linea {}", tipo, valor, linea), 3, nivel_verbose);
                resto = &resto[mat.end()..];
                matched = true;
                break;
            }
        }

        // Si no se encontro ningun token valido, es un error lexico
        if !matched {
            let caracter = resto.chars().next().unwrap_or('?');
            return Err(format!("Error lexico en linea {}: simbolo no reconocido '{}'", linea, caracter));
        }
    }

    // Agregar token EOF al final
    tokens.push(Token {
        tipo: TipoToken::EOF,
        valor: "$".to_string(),
        linea,
    });

    Ok(tokens)
}

