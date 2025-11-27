# PARTE 1: Arquitectura y Flujo General

[← Volver al índice](../../GUIA_TECNICA.md) | [Siguiente: Análisis Léxico →](PARTE_02_LEXICO.md)

## 1.1 Visión General del Compilador

El compilador Patito es un **compilador de un solo paso** que procesa el código fuente en fases secuenciales:

```
Código Fuente (.txt)
    ↓
┌─────────────────────┐
│  Análisis Léxico    │  → Genera stream de Tokens
│  (src/lexico)       │
└─────────────────────┘
    ↓
┌─────────────────────┐
│  Análisis Sintáctico│  → Parser SLR(1) con acciones semánticas
│  (src/sintactico)   │
└─────────────────────┘
    ↓ (en paralelo durante parsing)
┌─────────────────────┐
│  Análisis Semántico │  → Validaciones de tipos y alcances
│  (src/semantico)    │
└─────────────────────┘
    ↓ (genera durante parsing)
┌─────────────────────┐
│  Código Intermedio  │  → Cuádruplos (3 direcciones)
│  (src/intermedio)   │
└─────────────────────┘
    ↓
┌─────────────────────┐
│  Máquina Virtual    │  → Ejecuta cuádruplos
│  (src/vm)           │
└─────────────────────┘
    ↓
Salida del Programa
```

### Características Clave

1. **Un Solo Paso**: Las fases sintáctica, semántica y de generación de código ocurren simultáneamente
2. **Driven por Tablas**: El parser usa tablas ACTION y GOTO precalculadas
3. **Puntos Neurálgicos**: Acciones específicas se ejecutan en momentos clave del parsing
4. **Memoria Virtual**: Direcciones abstractas (1000-24999) traducidas en ejecución

## 1.2 Flujo de Datos Detallado

### Entrada: main.rs

**Archivo**: `src/main.rs`

```rust
fn main() {
    // 1. Leer archivo fuente
    let contenido = fs::read_to_string(archivo)?;

    // 2. Análisis léxico
    let tokens = AnalizadorLexico::new(&contenido).tokenizar()?;

    // 3. Crear contextos
    let mut contexto_semantico = ContextoSemantico::new();
    let mut generador = GeneradorCuadruplos::new();

    // 4. Parser SLR con acciones semánticas
    let mut parser = AnalizadorSintactico::new(tokens);
    parser.analizar(&mut contexto_semantico, &mut generador)?;

    // 5. Generar programa objeto
    let programa = generador.generar_programa("programa")?;

    // 6. Ejecutar en VM
    let mut vm = Ejecutor::new();
    vm.cargar_programa(programa)?;
    vm.ejecutar()?;
}
```

### Flujo Interno Durante el Parsing

```
Token Stream → AnalizadorSintactico
                    ↓
            ┌───────────────┐
            │ Tabla ACTION  │ ← Consulta con (estado, token)
            │ Tabla GOTO    │
            └───────────────┘
                    ↓
            Shift / Reduce / Accept
                    ↓
            [REDUCE detectado]
                    ↓
      ejecutar_accion_semantica(producción)
                    ↓
      ┌─────────────────────────────┐
      │ ContextoSemantico           │
      │  - Validar tipos            │
      │  - Verificar variables      │
      │  - Gestionar alcances       │
      └─────────────────────────────┘
                    ↓
      ┌─────────────────────────────┐
      │ GeneradorCuadruplos         │
      │  - Generar cuádruplos       │
      │  - Asignar memoria          │
      │  - Gestionar temporales     │
      └─────────────────────────────┘
                    ↓
            Continuar parsing...
```

## 1.3 Estructuras de Datos Principales

### 1.3.1 Token

**Archivo**: `src/lexico/token.rs`

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum TipoToken {
    // Palabras reservadas
    Programa, Inicio, Fin, Vars, Si, Sino, // ...

    // Identificadores y literales
    Identificador(String),
    ConstanteEntera(i32),
    ConstanteFlotante(f64),
    Letrero(String),

    // Operadores
    Suma, Resta, Multiplicacion, Division,
    Mayor, Menor, Igual, Diferente,
    Asignacion,

    // Delimitadores
    ParentesisAbre, ParentesisCierra,
    LlaveAbre, LlaveCierra,
    PuntoYComa, DosPuntos, Coma,

    FinArchivo,
}

pub struct Token {
    pub tipo: TipoToken,
    pub linea: usize,
}
```

**Uso**: Los tokens fluyen del léxico al sintáctico y contienen toda la información necesaria para el parsing.

### 1.3.2 Cuádruplo

**Archivo**: `src/intermedio/cuadruplo.rs`

```rust
#[derive(Debug, Clone)]
pub struct Cuadruplo {
    pub operador: OperadorCuadruplo,
    pub operando_izq: Operando,
    pub operando_der: Operando,
    pub resultado: Operando,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperadorCuadruplo {
    // Aritméticos
    Suma, Resta, Multiplicacion, Division,

    // Relacionales
    Mayor, Menor, Igual, Diferente,

    // Asignación y control
    Asignacion,
    Goto, GotoF, GotoV,

    // Funciones
    Era, Parametro, GoSub, Return, EndFunc,

    // I/O
    Lectura, Escritura,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operando {
    Direccion(usize),          // Dirección virtual (1000-24999)
    Variable(String),          // Nombre de variable o función
    ConstanteEntera(i32),      // Valor literal entero
    ConstanteFlotante(f64),    // Valor literal flotante
    Etiqueta(usize),           // Índice de cuádruplo para saltos
    Letrero(usize),            // Índice en tabla de strings
    Vacio,                     // No usado
}
```

**Formato**: `(operador, operando_izq, operando_der, resultado)`

**Ejemplos**:

```
(+, Dir(1000), Dir(1001), Dir(13000))     // suma = x + y
(=, Dir(13000), Vacio, Dir(1002))         // z = suma
(GotoF, Dir(13001), Vacio, Etiqueta(10))  // if (!cond) goto 10
(GoSub, Var("factorial"), Vacio, Dir(13002))  // temp = factorial()
```

### 1.3.3 Memoria Virtual

**Archivo**: `src/intermedio/memoria_virtual.rs`

```rust
pub struct MemoriaVirtual {
    // Contadores de direcciones por segmento y tipo
    global_entero: usize,      // 1000-1999
    global_flotante: usize,    // 3000-3999
    global_char: usize,        // 5000-5999

    local_entero: usize,       // 7000-8999
    local_flotante: usize,     // 9000-10999
    local_char: usize,         // 11000-12999

    temporal_entero: usize,    // 13000-14999
    temporal_flotante: usize,  // 15000-16999
    temporal_char: usize,      // 17000-18999

    constante_entero: usize,   // 19000-20999
    constante_flotante: usize, // 21000-22999
    constante_char: usize,     // 23000-24999

    // Pools AVAIL para reutilización de temporales
    temporales_disponibles_entero: HashSet<usize>,
    temporales_disponibles_flotante: HashSet<usize>,
    temporales_disponibles_char: HashSet<usize>,

    // Tablas de constantes (deduplicación)
    tabla_constantes_entero: HashMap<i32, usize>,
    tabla_constantes_flotante: HashMap<String, usize>,
    tabla_constantes_char: HashMap<char, usize>,
}
```

**Rangos de Memoria**:

| Segmento  | Entero      | Flotante    | Char        |
| --------- | ----------- | ----------- | ----------- |
| Global    | 1000-1999   | 3000-3999   | 5000-5999   |
| Local     | 7000-8999   | 9000-10999  | 11000-12999 |
| Temporal  | 13000-14999 | 15000-16999 | 17000-18999 |
| Constante | 19000-20999 | 21000-22999 | 23000-24999 |

### 1.3.4 Contexto Semántico

**Archivo**: `src/semantico/contexto_semantico.rs`

```rust
pub struct ContextoSemantico {
    // Directorio de funciones
    directorio_funciones: DirectorioFunciones,

    // Variables globales
    variables_globales: TablaVariables,

    // Variables locales (actual función)
    variables_locales: Option<TablaVariables>,

    // Cubo semántico (validación de tipos)
    cubo_semantico: CuboSemantico,

    // Función actual (None = scope global)
    funcion_actual: Option<String>,

    // Parámetros esperados en llamada
    parametros_esperados: Vec<(String, TipoDato)>,
    parametros_actuales: Vec<TipoDato>,
}
```

**Responsabilidades**:

- Validar tipos en operaciones
- Detectar variables no declaradas
- Detectar variables doblemente declaradas
- Gestionar alcances (global/local)
- Validar número de parámetros en llamadas

## 1.4 Convenciones del Proyecto

### 1.4.1 Nombres de Archivos

- **snake_case**: Todos los archivos `.rs`
- **Módulos**: Un directorio = un módulo con `mod.rs`
- **Tests**: Archivos con prefijo `test_` en `src/bin/`

### 1.4.2 Nomenclatura de Código

```rust
// Structs y Enums: PascalCase
struct MemoriaVirtual { }
enum TipoDato { }

// Funciones y variables: snake_case
fn asignar_variable() { }
let contador_temporales = 0;

// Constantes: SCREAMING_SNAKE_CASE
const GLOBAL_ENTERO_INICIO: usize = 1000;
const TEMPORAL_ENTERO_FIN: usize = 14999;

// Métodos públicos vs privados
pub fn metodo_publico() { }  // Expuesto al exterior
fn metodo_privado() { }      // Interno del módulo
```

### 1.4.3 Manejo de Errores

```rust
// Retornar Result para operaciones que pueden fallar
pub fn asignar_variable(&mut self, tipo: TipoDato) -> Result<usize, String> {
    if self.global_entero > GLOBAL_ENTERO_FIN {
        return Err("Desbordamiento de memoria global".to_string());
    }
    Ok(direccion)
}

// Propagar errores con ?
let direccion = self.memoria_virtual.asignar_variable(tipo)?;

// Panic solo para errores irrecuperables (bugs del compilador)
panic!("Estado inconsistente: no debería llegar aquí");
```

### 1.4.4 Comentarios y Documentación

````rust
/// Asigna una dirección virtual para una variable.
///
/// # Argumentos
/// * `tipo` - El tipo de dato de la variable
/// * `segmento` - El segmento donde se asignará (Global/Local/Temporal)
///
/// # Retorna
/// * `Ok(usize)` - La dirección virtual asignada
/// * `Err(String)` - Mensaje de error si hay desbordamiento
///
/// # Ejemplos
/// ```
/// let dir = mem.asignar_variable(TipoDato::Entero, TipoSegmento::Global)?;
/// ```
pub fn asignar_variable(&mut self, tipo: TipoDato, segmento: TipoSegmento)
    -> Result<usize, String> {
    // Implementación...
}
````

## 1.5 Puntos de Entrada para Modificaciones

### Quiero agregar un nuevo operador (+, -, etc.)

1. **Léxico** (`src/lexico/token.rs`): Agregar token
2. **Gramática** (`gramatica.txt`): Agregar a producción
3. **Cubo Semántico** (`src/semantico/cubo_semantico.rs`): Definir validación
4. **Cuádruplo** (`src/intermedio/cuadruplo.rs`): Agregar operador
5. **Generador** (`src/intermedio/generador.rs`): Generar cuádruplo
6. **VM** (`src/vm/ejecutor.rs`): Implementar ejecución

→ **Ver Parte 9, Sección 1 para ejemplo completo**

### Quiero cambiar rangos de memoria

1. **Memoria Virtual** (`src/intermedio/memoria_virtual.rs`): Constantes
2. **VM Memoria** (`src/vm/memoria.rs`): Constantes de traducción
3. **Regenerar tests**: Verificar que todo sigue funcionando

→ **Ver Parte 6, Sección 3**

### Quiero agregar una nueva estructura de control (for, switch, etc.)

1. **Gramática** (`gramatica.txt`): Definir sintaxis
2. **Regenerar tablas**: `cargo run --bin generador_slr`
3. **Acciones Semánticas** (`src/sintactico/acciones_semanticas.rs`): Punto neurálgico
4. **Generador** (`src/intermedio/generador.rs`): Métodos de generación
5. **Tests**: Crear programa de prueba

→ **Ver Parte 9, Sección 3**

## 1.6 Herramientas de Debugging

### Verbose Levels

```bash
# Sin verbose: solo output del programa
cargo run -- programa.txt

# Nivel 1: Fases principales
cargo run -- programa.txt -v

# Nivel 2: Estados del parser y acciones semánticas
cargo run -- programa.txt -vv

# Nivel 3: Debug completo (tokens, reduces, atributos)
cargo run -- programa.txt -vvv
```

### Inspeccionar Cuádruplos

Los cuádruplos se imprimen automáticamente después de compilar con verbose >= 1:

```
0: Cuadruplo { operador: Goto, operando_izq: Vacio, operando_der: Vacio, resultado: Etiqueta(4) }
1: Cuadruplo { operador: Asignacion, operando_izq: Direccion(7000), operando_der: Vacio, resultado: Direccion(13000) }
```

### Método de Estadísticas

```rust
// En GeneradorCuadruplos
generador.mostrar_cuadruplos();  // Imprime todos los cuádruplos
println!("{}", generador.memoria_virtual.obtener_estadisticas());
```

Salida:

```
Memoria Virtual - Temporales: E=3 F=1 C=0 | Pool Disponible: E=2 F=0 C=0 | Constantes: E=5 F=2 C=0
```

---

## Resumen

En esta parte cubrimos:

- Arquitectura general del compilador (5 fases)
- Flujo de datos entre módulos
- Estructuras de datos fundamentales (Token, Cuádruplo, Memoria)
- Convenciones de código
- Puntos de entrada para modificaciones comunes

**Siguiente**: [PARTE 2: Análisis Léxico →](PARTE_02_LEXICO.md)

[← Volver al índice](../../GUIA_TECNICA.md)
