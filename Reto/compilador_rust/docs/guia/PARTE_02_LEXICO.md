# PARTE 2: Análisis Léxico

[← Parte 1: Arquitectura](PARTE_01_ARQUITECTURA.md) | [Volver al índice](../../GUIA_TECNICA.md) | [Siguiente: Análisis Sintáctico →](PARTE_03_SINTACTICO.md)

## 2.1 Cómo Funciona el Tokenizador

El análisis léxico convierte el texto fuente en una secuencia de tokens usando **expresiones regulares**.

**Archivo principal**: `src/lexico/mod.rs`

### Proceso de Tokenización

```rust
pub struct AnalizadorLexico {
    contenido: String,
    posicion: usize,
    linea_actual: usize,
}

impl AnalizadorLexico {
    pub fn tokenizar(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while self.posicion < self.contenido.len() {
            // 1. Saltar espacios en blanco
            self.saltar_espacios();

            // 2. Intentar reconocer cada patrón
            if let Some(token) = self.reconocer_numero()? {
                tokens.push(token);
            } else if let Some(token) = self.reconocer_identificador_o_palabra()? {
                tokens.push(token);
            } else if let Some(token) = self.reconocer_letrero()? {
                tokens.push(token);
            } else if let Some(token) = self.reconocer_operador()? {
                tokens.push(token);
            } else {
                return Err(format!("Carácter inválido en línea {}", self.linea_actual));
            }
        }

        tokens.push(Token::new(TipoToken::FinArchivo, self.linea_actual));
        Ok(tokens)
    }
}
```

### Patrones de Reconocimiento

#### 2.1.1 Números

```rust
fn reconocer_numero(&mut self) -> Result<Option<Token>, String> {
    lazy_static! {
        static ref RE_ENTERO: Regex = Regex::new(r"^\d+").unwrap();
        static ref RE_FLOTANTE: Regex = Regex::new(r"^\d+\.\d+").unwrap();
    }

    let resto = &self.contenido[self.posicion..];

    // Primero intentar flotante (más específico)
    if let Some(cap) = RE_FLOTANTE.find(resto) {
        let texto = cap.as_str();
        let valor: f64 = texto.parse().unwrap();
        self.posicion += texto.len();
        return Ok(Some(Token::new(
            TipoToken::ConstanteFlotante(valor),
            self.linea_actual
        )));
    }

    // Luego intentar entero
    if let Some(cap) = RE_ENTERO.find(resto) {
        let texto = cap.as_str();
        let valor: i32 = texto.parse().unwrap();
        self.posicion += texto.len();
        return Ok(Some(Token::new(
            TipoToken::ConstanteEntera(valor),
            self.linea_actual
        )));
    }

    Ok(None)
}
```

**Orden importante**: Flotante antes que entero, sino `3.14` se tokeniza como `3`, `.`, `14`

#### 2.1.2 Identificadores y Palabras Reservadas

```rust
fn reconocer_identificador_o_palabra(&mut self) -> Result<Option<Token>, String> {
    lazy_static! {
        static ref RE_ID: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
    }

    let resto = &self.contenido[self.posicion..];

    if let Some(cap) = RE_ID.find(resto) {
        let texto = cap.as_str();
        self.posicion += texto.len();

        // Verificar si es palabra reservada
        let tipo_token = match texto {
            "programa" => TipoToken::Programa,
            "inicio" => TipoToken::Inicio,
            "fin" => TipoToken::Fin,
            "vars" => TipoToken::Vars,
            "entero" => TipoToken::Entero,
            "flotante" => TipoToken::Flotante,
            "si" => TipoToken::Si,
            "sino" => TipoToken::Sino,
            "entonces" => TipoToken::Entonces,
            "mientras" => TipoToken::Mientras,
            "haz" => TipoToken::Haz,
            "escribe" => TipoToken::Escribe,
            "regresa" => TipoToken::Regresa,
            "nula" => TipoToken::Nula,
            _ => TipoToken::Identificador(texto.to_string()),
        };

        return Ok(Some(Token::new(tipo_token, self.linea_actual)));
    }

    Ok(None)
}
```

#### 2.1.3 Strings (Letreros)

```rust
fn reconocer_letrero(&mut self) -> Result<Option<Token>, String> {
    let resto = &self.contenido[self.posicion..];

    if resto.starts_with('"') {
        self.posicion += 1; // Saltar comilla inicial
        let mut contenido_string = String::new();

        while self.posicion < self.contenido.len() {
            let ch = self.contenido.chars().nth(self.posicion).unwrap();

            if ch == '"' {
                self.posicion += 1; // Saltar comilla final
                return Ok(Some(Token::new(
                    TipoToken::Letrero(contenido_string),
                    self.linea_actual
                )));
            }

            if ch == '\n' {
                return Err(format!(
                    "String no cerrado en línea {}",
                    self.linea_actual
                ));
            }

            contenido_string.push(ch);
            self.posicion += 1;
        }

        return Err("String no cerrado al final del archivo".to_string());
    }

    Ok(None)
}
```

#### 2.1.4 Operadores

```rust
fn reconocer_operador(&mut self) -> Result<Option<Token>, String> {
    let resto = &self.contenido[self.posicion..];

    // Operadores de dos caracteres (primero)
    if resto.starts_with("==") {
        self.posicion += 2;
        return Ok(Some(Token::new(TipoToken::Igual, self.linea_actual)));
    }
    if resto.starts_with("!=") {
        self.posicion += 2;
        return Ok(Some(Token::new(TipoToken::Diferente, self.linea_actual)));
    }

    // Operadores de un caracter
    let ch = resto.chars().next().unwrap();
    let tipo = match ch {
        '+' => Some(TipoToken::Suma),
        '-' => Some(TipoToken::Resta),
        '*' => Some(TipoToken::Multiplicacion),
        '/' => Some(TipoToken::Division),
        '>' => Some(TipoToken::Mayor),
        '<' => Some(TipoToken::Menor),
        '=' => Some(TipoToken::Asignacion),
        '(' => Some(TipoToken::ParentesisAbre),
        ')' => Some(TipoToken::ParentesisCierra),
        '{' => Some(TipoToken::LlaveAbre),
        '}' => Some(TipoToken::LlaveCierra),
        '[' => Some(TipoToken::CorcheteAbre),
        ']' => Some(TipoToken::CorcheteCierra),
        ';' => Some(TipoToken::PuntoYComa),
        ':' => Some(TipoToken::DosPuntos),
        ',' => Some(TipoToken::Coma),
        _ => None,
    };

    if let Some(t) = tipo {
        self.posicion += 1;
        return Ok(Some(Token::new(t, self.linea_actual)));
    }

    Ok(None)
}
```

**Orden importante**: Operadores de 2 caracteres (`==`, `!=`) antes que de 1 (`=`, `!`)

## 2.2 Agregar Nuevos Tokens

### Ejemplo: Agregar operador módulo `%`

#### Paso 1: Agregar a TipoToken

**Archivo**: `src/lexico/token.rs`

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum TipoToken {
    // ... operadores existentes ...
    Suma,
    Resta,
    Multiplicacion,
    Division,
    Modulo,  // ← NUEVO
    // ...
}
```

#### Paso 2: Agregar reconocimiento en léxico

**Archivo**: `src/lexico/mod.rs`

```rust
fn reconocer_operador(&mut self) -> Result<Option<Token>, String> {
    // ... código existente ...

    let ch = resto.chars().next().unwrap();
    let tipo = match ch {
        '+' => Some(TipoToken::Suma),
        '-' => Some(TipoToken::Resta),
        '*' => Some(TipoToken::Multiplicacion),
        '/' => Some(TipoToken::Division),
        '%' => Some(TipoToken::Modulo),  // ← NUEVO
        // ... resto ...
    };
    // ...
}
```

#### Paso 3: Actualizar conversión a símbolo (para parser)

**Archivo**: `src/sintactico/mod.rs` (buscar `token_a_simbolo`)

```rust
fn token_a_simbolo(token: &Token) -> String {
    match &token.tipo {
        TipoToken::Suma => "+".to_string(),
        TipoToken::Resta => "-".to_string(),
        TipoToken::Multiplicacion => "*".to_string(),
        TipoToken::Division => "/".to_string(),
        TipoToken::Modulo => "%".to_string(),  // ← NUEVO
        // ...
    }
}
```

#### Paso 4: Actualizar gramática

**Archivo**: `gramatica.txt`

```
<*/> → *
<*/> → /
<*/> → %
```

#### Paso 5: Regenerar tablas

```bash
cargo run --bin generador_slr
```

#### Paso 6: Continuar con cubo semántico y generador

Ver **Parte 9, Sección 1** para el resto del proceso.

## 2.3 Modificar Palabras Reservadas

### Ejemplo: Cambiar `mientras` por `while`

#### Paso 1: Modificar reconocimiento

**Archivo**: `src/lexico/mod.rs`

```rust
fn reconocer_identificador_o_palabra(&mut self) -> Result<Option<Token>, String> {
    // ...
    let tipo_token = match texto {
        "programa" => TipoToken::Programa,
        // ...
        "while" => TipoToken::Mientras,  // ← CAMBIAR de "mientras"
        // ...
    };
    // ...
}
```

#### Paso 2: Actualizar gramática

**Archivo**: `gramatica.txt`

```
<CICLO> → while ( <EXPRESIÓN> ) haz <CUERPO>
```

#### Paso 3: Regenerar tablas

```bash
cargo run --bin generador_slr
```

#### Paso 4: Actualizar tests

Todos los programas de prueba que usen `mientras` deben cambiarse a `while`.

## 2.4 Manejo de Errores Léxicos

### Errores Comunes

```rust
// 1. Carácter inválido
"✓"  → Error: "Carácter inválido en línea 5"

// 2. String no cerrado
"Hola  → Error: "String no cerrado en línea 3"

// 3. Número mal formado
3.14.15  → Tokeniza como: 3.14, ., 15 (error sintáctico posterior)
```

### Mejorar Mensajes de Error

```rust
fn reconocer_letrero(&mut self) -> Result<Option<Token>, String> {
    // ...
    if ch == '\n' {
        return Err(format!(
            "String no cerrado en línea {}. Los strings deben estar en una sola línea.",
            self.linea_actual
        ));
    }
    // ...
}
```

### Recuperación de Errores

Actualmente el léxico **falla al primer error**. Para permitir múltiples errores:

```rust
pub struct AnalizadorLexico {
    // ...
    errores: Vec<String>,  // Acumular errores
}

impl AnalizadorLexico {
    pub fn tokenizar(&mut self) -> Result<Vec<Token>, Vec<String>> {
        // ...
        if let Err(e) = self.reconocer_numero() {
            self.errores.push(e);
            self.posicion += 1;  // Saltar carácter problemático
            continue;
        }
        // ...

        if self.errores.is_empty() {
            Ok(tokens)
        } else {
            Err(self.errores.clone())
        }
    }
}
```

## 2.5 Testing del Léxico

### Test Unitario

**Crear**: `src/bin/test_lexico.rs`

```rust
use compilador_rust::lexico::AnalizadorLexico;
use compilador_rust::lexico::token::TipoToken;

fn main() {
    // Test 1: Números
    let codigo = "42 3.14";
    let mut lexico = AnalizadorLexico::new(codigo);
    let tokens = lexico.tokenizar().unwrap();

    assert_eq!(tokens[0].tipo, TipoToken::ConstanteEntera(42));
    assert_eq!(tokens[1].tipo, TipoToken::ConstanteFlotante(3.14));
    println!("✓ Test números pasó");

    // Test 2: Palabras reservadas
    let codigo = "si entonces sino";
    let mut lexico = AnalizadorLexico::new(codigo);
    let tokens = lexico.tokenizar().unwrap();

    assert_eq!(tokens[0].tipo, TipoToken::Si);
    assert_eq!(tokens[1].tipo, TipoToken::Entonces);
    assert_eq!(tokens[2].tipo, TipoToken::Sino);
    println!("✓ Test palabras reservadas pasó");

    // Test 3: Operadores
    let codigo = "+ - * / == !=";
    let mut lexico = AnalizadorLexico::new(codigo);
    let tokens = lexico.tokenizar().unwrap();

    assert_eq!(tokens[0].tipo, TipoToken::Suma);
    assert_eq!(tokens[3].tipo, TipoToken::Division);
    assert_eq!(tokens[4].tipo, TipoToken::Igual);
    assert_eq!(tokens[5].tipo, TipoToken::Diferente);
    println!("✓ Test operadores pasó");

    println!("\n✅ Todos los tests léxicos pasaron");
}
```

Ejecutar:

```bash
cargo run --bin test_lexico
```

## 2.6 Debugging del Léxico

### Imprimir Todos los Tokens

```rust
pub fn tokenizar(&mut self) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    while self.posicion < self.contenido.len() {
        // ... tokenización ...

        if cfg!(debug_assertions) {
            println!("Token: {:?}", tokens.last().unwrap());
        }
    }

    tokens
}
```

### Verbose Level

En `main.rs`:

```rust
if verbose >= 3 {
    println!("\n=== TOKENS GENERADOS ===");
    for (i, token) in tokens.iter().enumerate() {
        println!("{:3}: {:?}", i, token);
    }
}
```

---

## Resumen

En esta parte cubrimos:

- Cómo funciona el tokenizador con regex
- Proceso de reconocimiento (números, IDs, strings, operadores)
- Cómo agregar nuevos tokens
- Cómo modificar palabras reservadas
- Manejo de errores léxicos
- Testing del léxico

**Siguiente**: [PARTE 3: Análisis Sintáctico →](PARTE_03_SINTACTICO.md)

[← Parte 1](PARTE_01_ARQUITECTURA.md) | [Volver al índice](../../GUIA_TECNICA.md)
