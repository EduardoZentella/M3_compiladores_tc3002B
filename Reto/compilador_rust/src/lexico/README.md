# Módulo Léxico

## Descripción

Este módulo implementa el analizador léxico (lexer) del compilador. Su función principal es convertir el código fuente (cadena de caracteres) en una secuencia de tokens que serán procesados por el analizador sintáctico.

## Archivos

### `token.rs`

Define los tipos de tokens reconocidos por el lenguaje y la estructura Token.

**Enumeración `TipoToken`:**

```rust
pub enum TipoToken {
    // Palabras reservadas
    Programa, Inicio, Fin, Vars, Entero, Flotante,
    Escribe, Mientras, Haz, Si, Sino, Nula,

    // Identificadores y constantes
    Id,          // nombres de variables, funciones
    CteEnt,      // constantes enteras: 42, 1000
    CteFlot,     // constantes flotantes: 3.14, 1.5e-10
    Letrero,     // cadenas de texto: "Hola mundo"

    // Operadores aritméticos
    Mas, Menos, Multi, Div,

    // Operadores relacionales
    MayorQue, MenorQue, Diferente, Igual,

    // Asignación
    Asignacion,

    // Separadores
    PuntoYComa, DosPuntos, LlaveAbre, LlaveCierra,
    ParenAbre, ParenCierra, Coma,

    // Fin de archivo
    EOF,
}
```

**Método `as_grammar_string()`:**
Convierte cada tipo de token a su representación en la gramática. Esto es crucial para que el analizador sintáctico pueda buscar en las tablas ACTION y GOTO.

Ejemplo:

```rust
TipoToken::Mas.as_grammar_string()      // → "+"
TipoToken::Id.as_grammar_string()       // → "id"
TipoToken::Programa.as_grammar_string() // → "programa"
```

**Estructura `Token`:**

```rust
pub struct Token {
    pub tipo: TipoToken,    // Tipo del token
    pub valor: String,      // Valor literal (ej: "mientras", "x", "42")
    pub linea: usize,       // Número de línea para errores
}
```

### `mod.rs`

Implementa el analizador léxico principal.

**Estructura `ReglaToken`:**
Asocia cada tipo de token con su patrón de expresión regular:

```rust
struct ReglaToken {
    tipo: TipoToken,
    patron: Regex,
}
```

**Patrones de Reconocimiento:**

1. **Palabras Reservadas**: Se reconocen con `\b` (word boundary) para evitar falsos positivos

   ```rust
   Regex::new(r"^\bprograma\b")  // Reconoce "programa" pero no "programador"
   ```

2. **Constantes Flotantes**: Deben estar antes que enteras para prioridad

   ```rust
   r"^[0-9]+\.[0-9]+([eE][+-]?[0-9]+)?"  // 3.14, 1.5e10, 2.0e-5
   ```

3. **Constantes Enteras**: Solo dígitos

   ```rust
   r"^[0-9]+"  // 0, 42, 1000
   ```

4. **Identificadores**: Letra o _ seguido de letras, dígitos o _

   ```rust
   r"^[a-zA-Z_][a-zA-Z0-9_]*"  // x, contador_1, _temp
   ```

5. **Operadores Compuestos**: Deben estar antes de los simples

   ```rust
   r"^!="   // antes de r"^="
   r"^=="   // antes de r"^="
   ```

6. **Letreros**: Cadenas entre comillas dobles
   ```rust
   r#"^"[^"]*""#  // "Hola mundo", "Error en línea"
   ```

**Función `analyze(input: &str, is_verbose: &bool)`:**

Algoritmo principal del análisis léxico:

```
1. Inicializar lista de tokens vacía
2. Para cada carácter en el input:
   a. Saltar espacios en blanco y contar líneas
   b. Intentar hacer match con cada patrón (en orden)
   c. Si hay match:
      - Crear token con tipo, valor y línea
      - Avanzar en el input
   d. Si no hay match:
      - Reportar error léxico con la línea
3. Agregar token EOF al final
4. Retornar lista de tokens
```

## Flujo de Análisis Léxico

```
archivo.txt
     ↓
"programa test; inicio { x = 5; } fin"
     ↓
analyze()
     ↓
[
  Token { tipo: Programa, valor: "programa", linea: 1 },
  Token { tipo: Id, valor: "test", linea: 1 },
  Token { tipo: PuntoYComa, valor: ";", linea: 1 },
  Token { tipo: Inicio, valor: "inicio", linea: 1 },
  Token { tipo: LlaveAbre, valor: "{", linea: 1 },
  Token { tipo: Id, valor: "x", linea: 1 },
  Token { tipo: Asignacion, valor: "=", linea: 1 },
  Token { tipo: CteEnt, valor: "5", linea: 1 },
  Token { tipo: PuntoYComa, valor: ";", linea: 1 },
  Token { tipo: LlaveCierra, valor: "}", linea: 1 },
  Token { tipo: Fin, valor: "fin", linea: 1 },
  Token { tipo: EOF, valor: "$", linea: 1 },
]
```

## Prioridad de Patrones

El orden en `REGLAS_TOKENS` es crítico:

1. Palabras reservadas primero: Evita que "programa" sea reconocido como identificador
2. Constantes antes de identificadores: `123abc` debe dar error, no ser "123" + "abc"
3. Operadores compuestos antes de simples: `!=` antes que `!` y `=`
4. Patrones más específicos primero: `3.14` debe ser flotante, no "3" + ".14"

## Manejo de Errores

El lexer detecta:

- **Símbolos no reconocidos**: Caracteres que no pertenecen al lenguaje

  ```
  Error léxico en línea 5: simbolo no reconocido '@'
  ```

- **Cadenas sin cerrar**: Detectadas implícitamente (no hacen match con el patrón)
  ```
  Error léxico en línea 3: simbolo no reconocido '"'
  ```

## Ejemplo de Uso

```rust
use compilador_rust::lexico::analyze;

let codigo = r#"
programa test;
vars x : entero;
inicio {
    x = 42;
    escribe("El valor es", x);
}
fin
"#;

match analyze(codigo, &true) {
    Ok(tokens) => {
        println!("Tokens generados: {}", tokens.len());
        for token in tokens {
            println!("{:?}", token);
        }
    }
    Err(error) => {
        eprintln!("Error: {}", error);
    }
}
```

## Complejidad

- **Tiempo**: O(n) donde n = longitud del input

  - Cada carácter se procesa una vez
  - Los regex son compilados una sola vez (lazy_static)

- **Espacio**: O(t) donde t = número de tokens
  - Se almacena cada token encontrado

## Optimizaciones

1. **lazy_static**: Los regex se compilan una sola vez al inicio
2. **Cadenas prestadas**: Se usa `&str` en lugar de `String` cuando es posible
3. **Match temprano**: Se detiene en el primer patrón que coincide

## Extensibilidad

Para agregar un nuevo tipo de token:

1. Agregar variante en `TipoToken`
2. Agregar caso en `as_grammar_string()`
3. Agregar `ReglaToken` en `REGLAS_TOKENS` (en el orden correcto)
4. Actualizar la gramática si es necesario

Ejemplo - agregar operador módulo:

```rust
// En token.rs
pub enum TipoToken {
    // ...
    Modulo,  // %
    // ...
}

impl TipoToken {
    pub fn as_grammar_string(&self) -> &str {
        match self {
            // ...
            TipoToken::Modulo => "%",
            // ...
        }
    }
}

// En mod.rs
lazy_static! {
    static ref REGLAS_TOKENS: Vec<ReglaToken> = vec![
        // ...
        ReglaToken { tipo: TipoToken::Modulo, patron: Regex::new(r"^%").unwrap() },
        // ...
    ];
}
```

## Referencias

- [Lexical Analysis - Dragon Book](https://en.wikipedia.org/wiki/Lexical_analysis)
- [Rust Regex Documentation](https://docs.rs/regex/)
- [Token (parser) - Wikipedia](https://en.wikipedia.org/wiki/Lexical_analysis#Token)
