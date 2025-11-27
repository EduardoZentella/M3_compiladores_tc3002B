# PARTE 3: Análisis Sintáctico SLR(1)

[← Parte 2: Léxico](PARTE_02_LEXICO.md) | [Volver al índice](../../GUIA_TECNICA.md) | [Siguiente: Semántico →](PARTE_04_SEMANTICO.md)

## 3.1 Cómo Funciona el Parser SLR(1)

### Conceptos Fundamentales

El compilador usa un **parser SLR(1) bottom-up** que construye el árbol de derivación desde las hojas hacia la raíz.

```
┌─────────────────────────────────────────┐
│  Tokens → Parser SLR → Árbol Sintáctico │
│                                         │
│  [programa, id, ;, vars, ...]           │
│         ↓                               │
│  Tablas ACTION + GOTO                   │
│         ↓                               │
│  Acciones: Shift / Reduce / Accept      │
└─────────────────────────────────────────┘
```

### Tablas SLR

El parser usa dos tablas generadas automáticamente:

1. **Tabla ACTION**: `(estado, terminal) → Acción`

   - `Shift(n)`: Empujar estado n y avanzar token
   - `Reduce(n)`: Aplicar producción n
   - `Accept`: Programa válido

2. **Tabla GOTO**: `(estado, no_terminal) → estado`
   - Define transiciones después de reducir

### Estructura del Parser

**Archivo**: `src/sintactico/mod.rs`

```rust
pub fn analyze(tokens: &[Token], nivel_verbose: usize)
    -> Result<GeneradorCuadruplos, String> {

    let mut pila_estados: Vec<usize> = vec![0]; // Inicia en estado 0
    let mut cursor = 0; // Posición en tokens
    let mut contexto = ContextoSemantico::new();
    let mut generador = GeneradorCuadruplos::new();

    loop {
        let estado_actual = *pila_estados.last().unwrap();

        // Obtener símbolo del token actual
        let token_str = if cursor < tokens.len() {
            tokens[cursor].tipo.as_grammar().to_string()
        } else {
            "$".to_string() // Fin de entrada
        };

        // Consultar ACTION[estado, símbolo]
        let accion = TABLA_ACTION.get(&(estado_actual, token_str.clone()));

        match accion {
            Some(Accion::Shift(nuevo_estado)) => {
                // Empujar estado y avanzar
                pila_estados.push(*nuevo_estado);
                cursor += 1;
            }
            Some(Accion::Reduce(num_prod)) => {
                // Aplicar producción: pop estados, push nuevo estado
                let regla = &REGLAS[*num_prod];

                // Desempilar tantos estados como símbolos en el cuerpo
                for _ in 0..regla.longitud_cuerpo {
                    pila_estados.pop();
                }

                // ACCIÓN SEMÁNTICA aquí
                ejecutar_accion_semantica(
                    &regla.cabeza,
                    &mut contexto,
                    &mut generador,
                    &tokens,
                    cursor
                )?;

                // Consultar GOTO[estado_tope, no_terminal]
                let estado_tope = *pila_estados.last().unwrap();
                let nuevo_estado = TABLA_GOTO
                    .get(&(estado_tope, regla.cabeza.clone()))
                    .ok_or("Error: transición GOTO no encontrada")?;

                pila_estados.push(*nuevo_estado);
            }
            Some(Accion::Accept) => {
                // ¡Programa sintácticamente correcto!
                return Ok(generador);
            }
            None => {
                return Err(format!(
                    "Error sintáctico en estado {} con token '{}'",
                    estado_actual, token_str
                ));
            }
        }
    }
}
```

### Ejemplo de Ejecución

**Programa**:

```paradox
programa test;
inicio {
    x = 5;
}
fin
```

**Traza (simplificada)**:

| Paso | Pila Estados | Token    | Acción    | Producción       |
| ---- | ------------ | -------- | --------- | ---------------- |
| 1    | [0]          | programa | Shift(1)  | -                |
| 2    | [0,1]        | id       | Shift(2)  | -                |
| 3    | [0,1,2]      | ;        | Shift(3)  | -                |
| 4    | [0,1,2,3]    | inicio   | Reduce(2) | `<VARS_OPT> → ε` |
| 5    | [0,1,2,4]    | inicio   | Shift(10) | -                |
| ...  | ...          | ...      | ...       | ...              |

## 3.2 Cómo Modificar la Gramática

### Archivo de Gramática

**Ubicación**: `gramatica.txt`

Este archivo define todas las producciones del lenguaje.

```paradox
<Programa> → programa id ; <VARS_OPT> <FUNCS_LIST> inicio <CUERPO> fin
<VARS_OPT> → <VARS> <VARS_OPT>
<VARS_OPT> → ε

<VARS> → vars <VAR_LIST> : <TIPO> ;
<TIPO> → entero
<TIPO> → flotante
<TIPO> → letrero

<ESTATUTO> → <ASIGNA>
<ESTATUTO> → <CONDICIÓN>
<ESTATUTO> → <CICLO>
<ASIGNA> → id = <EXPRESIÓN> ;

<EXPRESIÓN> → <EXP> <EXPRESION_PRIMA>
<EXP> → <TÉRMINO> <EXP_PRIMA>
<TÉRMINO> → <FACTOR> <TERMINO_PRIMA>
```

### Ejemplo: Agregar `do-while`

**Paso 1**: Agregar producción en `gramatica.txt`

```paradox
<CICLO> → mientras ( <EXPRESIÓN> ) haz <CUERPO>
<CICLO> → hacer <CUERPO> mientras ( <EXPRESIÓN> ) ;
```

**Paso 2**: Agregar tokens en `src/lexico/token.rs`

```rust
pub enum TipoToken {
    // ... existentes ...
    Mientras,
    Hacer,  // ← AGREGAR
    Haz,
}
```

**Paso 3**: Reconocer palabra en `src/lexico/mod.rs`

```rust
let tipo_token = match texto {
    "mientras" => TipoToken::Mientras,
    "hacer" => TipoToken::Hacer,  // ← AGREGAR
    "haz" => TipoToken::Haz,
    // ...
};
```

**Paso 4**: Mapear token a símbolo en `src/lexico/token.rs`

```rust
impl TipoToken {
    pub fn as_grammar(&self) -> &str {
        match self {
            // ...
            TipoToken::Mientras => "mientras",
            TipoToken::Hacer => "hacer",  // ← AGREGAR
            TipoToken::Haz => "haz",
            // ...
        }
    }
}
```

**Paso 5**: Regenerar tablas SLR

```bash
cargo run --bin generador_slr
```

Esto actualiza automáticamente `src/sintactico/tabla_slr.rs`.

**Paso 6**: Agregar acción semántica

**Archivo**: `src/sintactico/acciones_semanticas.rs`

```rust
pub fn ejecutar_accion_semantica(
    produccion: &str,
    contexto: &mut ContextoSemantico,
    generador: &mut GeneradorCuadruplos,
    tokens: &[Token],
    cursor: usize,
) -> Result<(), String> {
    match produccion {
        // ... existentes ...

        "<CICLO> → hacer <CUERPO> mientras ( <EXPRESIÓN> ) ;" => {
            // Generar cuádruplos para do-while
            // 1. Guardar dirección de inicio
            // 2. Ejecutar cuerpo
            // 3. Evaluar condición
            // 4. GotoV al inicio si true
            generador.generar_dowhile()?;
        }

        _ => {}
    }
    Ok(())
}
```

## 3.3 Regenerar Tablas SLR

### El Generador

**Archivo**: `src/bin/generador.rs`

Este programa:

1. Lee `gramatica.txt`
2. Calcula FIRST y FOLLOW
3. Construye autómata LR(0)
4. Genera tablas ACTION y GOTO
5. Escribe `src/sintactico/tabla_slr.rs`

### Proceso de Generación

```rust
// En generador.rs (simplificado)
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Parsear gramática
    let gramatica = parser::parsear_gramatica("gramatica.txt")?;

    // 2. Calcular FIRST y FOLLOW
    let first = calcular_first(&gramatica);
    let follow = calcular_follow(&gramatica, &first);

    // 3. Construir autómata LR(0)
    let automata = construir_lr0(&gramatica);

    // 4. Generar tablas SLR
    let (tabla_action, tabla_goto) = generar_tablas_slr(
        &automata,
        &gramatica,
        &follow
    );

    // 5. Escribir archivo
    escribir_tabla_slr(tabla_action, tabla_goto, "src/sintactico/tabla_slr.rs")?;

    Ok(())
}
```

### Ejecutar Generador

```bash
cargo run --bin generador_slr
```

**Salida esperada**:

```
Parseando gramática...
Calculando FIRST...
Calculando FOLLOW...
Construyendo autómata LR(0)...
  Estado 0: 15 ítems
  Estado 1: 8 ítems
  ...
Generando tabla ACTION...
Generando tabla GOTO...
Escribiendo src/sintactico/tabla_slr.rs...
✓ Tablas SLR generadas exitosamente
```

### Verificar Tablas Generadas

```bash
# Ver parte de las tablas
head -50 src/sintactico/tabla_slr.rs
```

Deberías ver:

```rust
// ==========================================
// ARCHIVO GENERADO AUTOMÁTICAMENTE
// Creado por 'generador_slr'
// NO EDITAR MANUALMENTE
// ==========================================

lazy_static! {
    pub static ref TABLA_ACTION: HashMap<(usize, String), Accion> = {
        let mut m = HashMap::new();
        m.insert((0, "programa".to_string()), Accion::Shift(1));
        m.insert((1, "id".to_string()), Accion::Shift(2));
        // ... muchas más entradas ...
    };
}
```

## 3.4 Acciones Semánticas Durante Parsing

### Cuándo se Ejecutan

Las acciones semánticas se ejecutan al **REDUCIR** una producción (bottom-up):

```rust
Some(Accion::Reduce(num_prod)) => {
    let regla = &REGLAS[*num_prod];

    // Desempilar estados
    for _ in 0..regla.longitud_cuerpo {
        pila_estados.pop();
    }

    // ← AQUÍ se ejecuta la acción semántica
    ejecutar_accion_semantica(
        &regla.cabeza,
        &mut contexto,
        &mut generador,
        &tokens,
        cursor
    )?;

    // Consultar GOTO y empujar estado
    // ...
}
```

### Archivo de Acciones

**Ubicación**: `src/sintactico/acciones_semanticas.rs`

```rust
pub fn ejecutar_accion_semantica(
    produccion: &str,
    contexto: &mut ContextoSemantico,
    generador: &mut GeneradorCuadruplos,
    tokens: &[Token],
    cursor: usize,
) -> Result<(), String> {
    match produccion {
        // === Programa ===
        "<Programa> → programa id ; <VARS_OPT> <FUNCS_LIST> inicio <CUERPO> fin" => {
            generador.finalizar_programa()?;
        }

        // === Variables ===
        "<VARS> → vars <VAR_LIST> : <TIPO> ;" => {
            // Las variables ya fueron registradas durante el parsing
        }

        // === Asignación ===
        "<ASIGNA> → id = <EXPRESIÓN> ;" => {
            generador.generar_asignacion()?;
        }

        // === Operaciones ===
        "<EXP_PRIMA> → <+-> <TÉRMINO> <EXP_PRIMA>" => {
            generador.generar_operacion_aditiva()?;
        }

        "<TERMINO_PRIMA> → <*/> <FACTOR> <TERMINO_PRIMA>" => {
            generador.generar_operacion_multiplicativa()?;
        }

        // === Condiciones ===
        "<CONDICIÓN> → si ( <EXPRESIÓN> ) entonces <CUERPO> <SINO_OPT> ;" => {
            generador.finalizar_condicion()?;
        }

        // === Ciclos ===
        "<CICLO> → mientras ( <EXPRESIÓN> ) haz <CUERPO>" => {
            generador.finalizar_ciclo()?;
        }

        // === Funciones ===
        "<FUNCS> → <FUNC_ARGS> <CUERPO> ;" => {
            generador.finalizar_funcion()?;
        }

        "<LLAMADA> → <LLAMADA_ARGS>" => {
            generador.generar_llamada_funcion()?;
        }

        _ => {
            // Producción sin acción semántica asociada
        }
    }

    Ok(())
}
```

### Puntos Neurálgicos

Algunas acciones requieren ejecutarse **antes** de reducir. Para esto se usan puntos neurálgicos detectados durante el shift:

```rust
Some(Accion::Shift(nuevo_estado)) => {
    // Detectar tokens especiales
    if cursor < tokens.len() {
        let token_actual = &tokens[cursor].tipo;

        // Punto neurálgico: inicio de condición
        if matches!(token_actual, TipoToken::Si) {
            if cursor + 1 < tokens.len() &&
               matches!(tokens[cursor + 1].tipo, TipoToken::ParenAbre) {
                generador.iniciar_condicion();
            }
        }

        // Punto neurálgico: inicio de ciclo
        if matches!(token_actual, TipoToken::Mientras) {
            generador.guardar_direccion_ciclo();
        }

        // Punto neurálgico: operador
        if matches!(token_actual, TipoToken::Suma | TipoToken::Resta) {
            generador.push_operador(token_actual);
        }
    }

    pila_estados.push(*nuevo_estado);
    cursor += 1;
}
```

## 3.5 Debugging del Parser

### Nivel 1: Verbose Básico

```bash
VERBOSE=1 cargo run -- programa.txt
```

Muestra:

- Total de tokens
- Aceptación o error

### Nivel 2: Verbose Detallado

```bash
VERBOSE=2 cargo run -- programa.txt
```

Muestra:

- Estado actual
- Token actual y su tipo
- Acción (Shift/Reduce)
- Pila de estados

**Salida ejemplo**:

```
Estado: 0 | Token: 'programa' (tipo: programa) | Pila: [0]
  → Acción: Shift(1)
Estado: 1 | Token: 'test' (tipo: id) | Pila: [0, 1]
  → Acción: Shift(2)
Estado: 2 | Token: ';' (tipo: ;) | Pila: [0, 1, 2]
  → Acción: Shift(3)
```

### Nivel 3: Verbose Completo

```bash
VERBOSE=3 cargo run -- programa.txt
```

Adicional a nivel 2:

- Producciones al reducir
- Pilas semánticas
- Cuádruplos generados

### Errores Comunes

#### Error: "Símbolo inesperado"

```
Error sintáctico en estado 15 con token 'entonces'
```

**Causa**: Token no esperado en ese estado.

**Debug**:

1. Ver estado 15 en tablas SLR
2. Verificar qué tokens son válidos
3. Revisar gramática para esa producción

**Solución**: Corregir programa o gramática.

---

#### Error: "Transición GOTO no encontrada"

```
Error: transición GOTO no encontrada para <EXPRESIÓN> desde estado 42
```

**Causa**: Tablas incompletas (error en generador).

**Debug**:

```bash
cargo run --bin generador_slr
```

Buscar warnings sobre conflictos.

**Solución**: Resolver conflicto shift/reduce o reduce/reduce en gramática.

---

#### Error: "Pila de estados vacía"

**Causa**: Bug en el parser (no debería pasar).

**Debug**: Revisar lógica de pop en `Reduce`.

## 3.6 Conflictos en Gramáticas

### Conflicto Shift/Reduce

Ocurre cuando el parser no sabe si hacer shift o reduce.

**Ejemplo**:

```
<EXPRESIÓN> → id
<EXPRESIÓN> → <EXPRESIÓN> + id
```

Entrada: `id + id`

Al ver `+`, ¿reducir `id` a `<EXPRESIÓN>` o hacer shift de `+`?

**Solución**: Refactorizar gramática con recursión a la derecha/izquierda clara.

### Conflicto Reduce/Reduce

Ocurre cuando el parser puede reducir por dos producciones diferentes.

**Ejemplo**:

```
<A> → id
<B> → id
```

Al ver `id`, ¿reducir a `<A>` o `<B>`?

**Solución**: Unificar producciones o agregar contexto.

### Detectar Conflictos

Al ejecutar `cargo run --bin generador_slr`, buscar:

```
⚠ ADVERTENCIA: Conflicto shift/reduce en estado 23
  Token: '+'
  Shift a estado 30
  Reduce por producción 15
```

## 3.7 Testing del Parser

### Test Básico

**Archivo**: `src/bin/test_sintactico.rs`

```rust
use compilador_rust::sintactico;
use compilador_rust::lexico::AnalizadorLexico;

#[test]
fn test_programa_minimo() {
    let codigo = r#"
programa test;
inicio {
}
fin
"#;

    let mut lexico = AnalizadorLexico::new(codigo);
    let tokens = lexico.tokenizar().unwrap();

    let resultado = sintactico::analyze(&tokens, 0);
    assert!(resultado.is_ok(), "Parser falló: {:?}", resultado);
}

#[test]
fn test_error_sintactico() {
    let codigo = r#"
programa test;
inicio
  x = 5
fin
"#; // Falta ';' después de x=5

    let mut lexico = AnalizadorLexico::new(codigo);
    let tokens = lexico.tokenizar().unwrap();

    let resultado = sintactico::analyze(&tokens, 0);
    assert!(resultado.is_err());
}
```

**Ejecutar**:

```bash
cargo test test_programa_minimo
cargo test test_error_sintactico
```

---

## Resumen

En esta parte cubrimos:

- Funcionamiento del parser SLR(1) bottom-up
- Tablas ACTION y GOTO
- Cómo modificar la gramática
- Regenerar tablas con generador_slr
- Acciones semánticas durante reduce
- Puntos neurálgicos
- Debugging con verbose
- Conflictos y soluciones
- Testing del parser

**Siguiente**: [PARTE 4: Análisis Semántico →](PARTE_04_SEMANTICO.md)

[← Parte 2](PARTE_02_LEXICO.md) | [Volver al índice](../../GUIA_TECNICA.md)
