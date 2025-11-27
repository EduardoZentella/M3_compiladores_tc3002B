# Módulo de Análisis Semántico

## Descripción General

Este módulo implementa el análisis semántico completo para el lenguaje Patito, incluyendo:

1. Cubo Semántico: Validación de tipos para operaciones
2. Tabla de Variables: Gestión de variables por alcance
3. Directorio de Funciones: Gestión de funciones y sus variables locales
4. Contexto Semántico: Coordinación del análisis durante el parsing

## Arquitectura

```
src/semantico/
├── mod.rs                      # Módulo principal
├── tipos.rs                    # Definición de tipos (TipoDato, TipoRetorno)
├── cubo_semantico.rs          # Cubo semántico para validación de tipos
├── tabla_variables.rs         # Tabla de variables por alcance
├── directorio_funciones.rs    # Directorio de funciones global
└── contexto.rs                # Contexto semántico durante parsing
```

## Estructuras de Datos

### 1. Cubo Semántico

El **Cubo Semántico** define las reglas de tipos para todas las operaciones del lenguaje.

#### Tipos de Datos

```rust
enum TipoDato {
    Entero,    // entero
    Flotante,  // flotante
}
```

#### Operadores Soportados

```rust
enum Operador {
    // Aritméticos
    Suma, Resta, Multiplicacion, Division,

    // Relacionales
    MayorQue, MenorQue, Igual, Diferente,

    // Asignación
    Asignacion,
}
```

#### Reglas del Cubo Semántico

**Operadores Aritméticos (+, -, \*, /)**

| Operando 1 | Operador | Operando 2 | Resultado |
| ---------- | -------- | ---------- | --------- |
| entero     | + - \* / | entero     | entero    |
| entero     | + - \* / | flotante   | flotante  |
| flotante   | + - \* / | entero     | flotante  |
| flotante   | + - \* / | flotante   | flotante  |

**Regla**: Cualquier operación con `flotante` promueve el resultado a `flotante`.

**Operadores Relacionales (>, <, ==, !=)**

| Operando 1 | Operador  | Operando 2 | Resultado |
| ---------- | --------- | ---------- | --------- |
| entero     | > < == != | entero     | entero    |
| entero     | > < == != | flotante   | entero    |
| flotante   | > < == != | entero     | entero    |
| flotante   | > < == != | flotante   | entero    |

**Regla**: Todos los operadores relacionales retornan `entero` (0 = falso, 1 = verdadero).

**Operador de Asignación (=)**

| Variable (izq) | Operador | Expresión (der) | Resultado                           |
| -------------- | -------- | --------------- | ----------------------------------- |
| entero         | =        | entero          | entero ✓                            |
| flotante       | =        | flotante        | flotante ✓                          |
| flotante       | =        | entero          | flotante ✓ (promoción)              |
| entero         | =        | flotante        | ERROR ✗ (truncamiento no permitido) |

**Regla**: El lenguaje es **fuertemente tipado**. No se permite asignación de `flotante` a `entero`.

#### Uso del Cubo Semántico

```rust
use compilador_rust::semantico::{CuboSemantico, TipoDato};
use compilador_rust::semantico::cubo_semantico::{Operador, ResultadoTipo};

let cubo = CuboSemantico::new();

// Validar operación: entero + flotante
let resultado = cubo.validar(
    TipoDato::Entero,
    Operador::Suma,
    TipoDato::Flotante
);

match resultado {
    ResultadoTipo::Ok(tipo) => println!("Resultado: {}", tipo), // flotante
    ResultadoTipo::Error => println!("Error: tipos incompatibles"),
}
```

### 2. Tabla de Variables

La **Tabla de Variables** almacena las variables declaradas en un alcance específico.

#### Estructura

```rust
pub struct TablaVariables {
    variables: HashMap<String, EntradaVariable>,
}

pub struct EntradaVariable {
    pub tipo: TipoDato,
    // Campos futuros: direccion, dimensiones, etc.
}
```

#### Características

- **Búsqueda O(1)** usando HashMap
- **Validación de declaraciones duplicadas**
- **Acceso rápido al tipo de variables**

#### Operaciones Principales

```rust
let mut tabla = TablaVariables::new();

// Agregar variable
tabla.agregar("x", TipoDato::Entero)?;  // Ok
tabla.agregar("x", TipoDato::Flotante)?; // Error: duplicada

// Buscar variable
if let Some(entrada) = tabla.buscar("x") {
    println!("Tipo: {}", entrada.tipo);
}

// Verificar existencia
if tabla.existe("x") {
    println!("Variable existe");
}
```

### 3. Directorio de Funciones

El **Directorio de Funciones** es la estructura de más alto nivel que mantiene información sobre todas las funciones.

#### Estructura

```rust
pub struct DirectorioFunciones {
    funciones: HashMap<String, EntradaFuncion>,
}

pub struct EntradaFuncion {
    pub tipo_retorno: TipoRetorno,
    pub tabla_variables: TablaVariables,
    // Campos futuros: parametros, direccion_inicio, etc.
}
```

#### Características

- **Alcance Global**: Representado como función con el nombre del programa
- **Funciones**: Cada una con su propia tabla de variables
- **Búsqueda O(1)** usando HashMap

#### Operaciones Principales

```rust
let mut dir = DirectorioFunciones::new();

// Crear alcance global
dir.agregar_funcion("mi_programa", TipoRetorno::Nula)?;

// Agregar función
dir.agregar_funcion("suma", TipoRetorno::Tipo(TipoDato::Entero))?;

// Agregar variable a función
dir.agregar_variable("suma", "x", TipoDato::Entero)?;

// Buscar función
if let Some(func) = dir.buscar_funcion("suma") {
    println!("Tipo retorno: {}", func.tipo_retorno);
}

// Buscar variable en función
if let Some(var) = dir.buscar_variable("suma", "x") {
    println!("Tipo: {}", var.tipo);
}
```

### 4. Contexto Semántico

El **Contexto Semántico** mantiene el estado durante el análisis sintáctico.

#### Estructura

```rust
pub struct ContextoSemantico {
    pub dir_funciones: DirectorioFunciones,
    pub cubo_semantico: CuboSemantico,
    alcance_actual: String,
    tipo_actual: Option<TipoDato>,
    nombre_programa: String,
}
```

#### Responsabilidades

- Rastrear el **alcance actual** (función en la que estamos)
- Rastrear el **tipo actual** (durante declaraciones)
- Facilitar **búsqueda de variables** (local → global)
- Coordinar el análisis semántico

#### Uso Básico

```rust
let mut ctx = ContextoSemantico::new();

// PN1: Inicializar programa
ctx.inicializar_programa("mi_programa")?;

// PN2+PN3: Declarar variable global
ctx.establecer_tipo_actual(TipoDato::Entero);
ctx.agregar_variable("x")?;

// PN4: Iniciar función
ctx.iniciar_funcion("suma", TipoRetorno::Tipo(TipoDato::Entero))?;

// PN3: Agregar variable local
ctx.establecer_tipo_actual(TipoDato::Entero);
ctx.agregar_variable("a")?;

// PN7: Buscar variable (local → global)
let tipo = ctx.obtener_tipo_variable("a")?;

// PN6: Finalizar función
ctx.finalizar_funcion();
```

## Puntos Neurálgicos (PN)

Los **puntos neurálgicos** son los lugares en la gramática donde se ejecutan acciones semánticas.

### PN1: Inicializar Programa

**Gramática**: `<Programa> → programa id; ...`
**Punto**: Después de reconocer `id`

```rust
// Acción en el parser:
ctx.inicializar_programa(&nombre_programa)?;
```

**Tareas**:

1. Crear entrada en DirFunc para alcance global
2. Establecer alcance actual = nombre del programa
3. Crear tabla de variables para alcance global

### PN2: Establecer Tipo Actual

**Gramática**: `<VARS> → vars <VAR_LIST> : <TIPO>;`
**Punto**: Después de reconocer `<TIPO>`

```rust
// Acción en el parser:
let tipo = TipoDato::from_str(&token_tipo).unwrap();
ctx.establecer_tipo_actual(tipo);
```

**Tareas**:

1. Guardar el tipo en `tipo_actual` del contexto

### PN3: Agregar Variable

**Gramática**: `<VAR_LIST> → id <VAR_LIST'>`
**Punto**: Al reconocer cada `id`

```rust
// Acción en el parser:
ctx.agregar_variable(&nombre_variable)?;
```

**Tareas**:

1. Agregar variable a tabla del alcance actual
2. Validar que no esté duplicada

### PN4: Iniciar Función

**Gramática**: `<FUNCS> → <TIPO_OPT> id (...) {...};`
**Punto**: Después de `<TIPO_OPT>` e `id`

```rust
// Acción en el parser:
let tipo_ret = TipoRetorno::from_str(&tipo_str).unwrap();
ctx.iniciar_funcion(&nombre_funcion, tipo_ret)?;
```

**Tareas**:

1. Agregar función a DirFunc
2. Validar que no esté duplicada
3. Cambiar alcance actual a la nueva función
4. Crear tabla de variables para la función

### PN5: Agregar Parámetros

**Gramática**: `<ARG_LIST> → id : <TIPO> <ARG_LIST'>`
**Punto**: Al procesar cada parámetro

```rust
// Acción en el parser:
ctx.establecer_tipo_actual(tipo_param);
ctx.agregar_variable(&nombre_param)?;
```

**Tareas**:

1. Agregar parámetro como variable en la función actual
2. Validar que no haya parámetros duplicados

### PN6: Finalizar Función

**Gramática**: `<FUNCS> → ... };`
**Punto**: Al finalizar la función

```rust
// Acción en el parser:
ctx.finalizar_funcion();
```

**Tareas**:

1. Restaurar alcance actual al programa global

### PN7: Validar Uso de Variable

**Gramática**: `<ASIGNA> → id = ...` y `<FACTOR> → id`
**Punto**: Al usar un `id`

```rust
// Acción en el parser:
let tipo = ctx.obtener_tipo_variable(&nombre_var)?;
// Usar 'tipo' para validación en cubo semántico
```

**Tareas**:

1. Buscar variable en alcance local
2. Si no existe, buscar en alcance global
3. Si no existe en ninguno, error "Variable no declarada"
4. Retornar tipo para validación de expresiones

## Pruebas

El módulo incluye 4 programas de prueba completos:

### 1. Test del Cubo Semántico

```bash
cargo run --bin test_cubo_semantico
```

Prueba todas las reglas del cubo semántico.

### 2. Test de Tabla de Variables

```bash
cargo run --bin test_tabla_variables
```

Prueba agregar, buscar y validar variables.

### 3. Test del Directorio de Funciones

```bash
cargo run --bin test_directorio_funciones
```

Prueba funciones, variables locales y globales.

### 4. Test del Contexto Semántico

```bash
cargo run --bin test_contexto_semantico
```

Simula el flujo completo del análisis semántico.

### Ejecutar Tests Unitarios

```bash
cargo test --lib semantico
```

## Ejemplo de Uso Completo

```rust
use compilador_rust::semantico::ContextoSemantico;

fn analizar_programa() -> Result<(), String> {
    let mut ctx = ContextoSemantico::new();

    // programa mi_programa;
    ctx.inicializar_programa("mi_programa")?;

    // vars x, y : entero;
    ctx.establecer_tipo_actual(TipoDato::Entero);
    ctx.agregar_variable("x")?;
    ctx.agregar_variable("y")?;

    // vars total : flotante;
    ctx.establecer_tipo_actual(TipoDato::Flotante);
    ctx.agregar_variable("total")?;

    // entero suma(a : entero, b : entero) {
    ctx.iniciar_funcion("suma", TipoRetorno::Tipo(TipoDato::Entero))?;
    ctx.establecer_tipo_actual(TipoDato::Entero);
    ctx.agregar_variable("a")?;  // parámetro
    ctx.agregar_variable("b")?;  // parámetro

    //     vars resultado : entero;
    ctx.agregar_variable("resultado")?;  // local

    //     { resultado = a + b; }
    let tipo_a = ctx.obtener_tipo_variable("a")?;     // entero
    let tipo_b = ctx.obtener_tipo_variable("b")?;     // entero

    // Validar con cubo semántico
    let resultado_suma = ctx.cubo_semantico.validar(
        tipo_a,
        Operador::Suma,
        tipo_b
    );

    // };
    ctx.finalizar_funcion();

    // inicio { x = 5; y = 10; } fin
    let tipo_x = ctx.obtener_tipo_variable("x")?;  // entero

    // Imprimir estado final
    ctx.imprimir();

    Ok(())
}
```

## Validaciones Implementadas

### Variables

- Variable doblemente declarada (mismo alcance)
- Variable no declarada (uso antes de declaración)
- Búsqueda en alcances (local a global)

### Funciones

- Función doblemente declarada
- Función no declarada (al llamar)
- Parámetros duplicados

### Tipos

- Validación de tipos en operaciones aritméticas
- Validación de tipos en operaciones relacionales
- Validación de tipos en asignaciones
- Promoción de tipos (entero a flotante)
- Rechazo de truncamiento (flotante a entero)

## Referencias

- Gramática: `gramatica.txt`
- Material de clase sobre análisis semántico
