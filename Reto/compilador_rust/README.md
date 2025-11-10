# Compilador Patito en Rust

## Descripción

Este proyecto implementa un **compilador completo** para el lenguaje de programación **Patito**, desarrollado en Rust. El compilador incluye las fases de análisis léxico, análisis sintáctico SLR(1), **análisis semántico**, **generación de código intermedio (cuádruplos)**, y generación automática de tablas de parsing.

## Características

### Análisis Léxico

- Tokenización completa con reconocimiento de palabras reservadas, identificadores, constantes y operadores
- Manejo de errores con números de línea

### Análisis Sintáctico SLR(1)

- Parser de tabla que valida la estructura sintáctica del código fuente
- Generación automática de autómatas LR(0) y tablas SLR
- Cálculo de conjuntos FIRST y FOLLOW
- Reportes detallados de errores sintácticos

### Análisis Semántico

- **Cubo Semántico**: Validación de tipos en todas las operaciones
- **Tabla de Variables**: Gestión de variables por alcance con HashMap (O(1))
- **Directorio de Funciones**: Gestión de funciones y alcances
- **Contexto Semántico**: Coordinación del análisis durante el parsing
- **Validaciones**:
  - Variables doblemente declaradas
  - Variables no declaradas
  - Funciones duplicadas
  - Tipos incompatibles en operaciones
  - Asignaciones con truncamiento (rechazadas)
  - Promoción de tipos (entero → flotante)

### Generación de Código Intermedio

- **Cuádruplos**: Código intermedio de tres direcciones
- **Pilas de Operadores y Operandos**: POper, PilaO, PTypes
- **Gestor de Memoria Temporal**: AVAIL con reutilización de temporales
- **Algoritmos de Traducción**:
  - Expresiones aritméticas (+, -, \*, /)
  - Expresiones relacionales (>, <, ==, !=)
  - Estatutos lineales (asignación, lectura, escritura)
- **9 Puntos Neurálgicos** documentados e implementados

## Estructura del Proyecto

```
compilador_rust/
├── src/
│   ├── bin/                    # Ejecutables y programas de prueba
│   ├── gramatica/              # Parsing y análisis de gramáticas
│   ├── lexico/                 # Análisis léxico (tokenización)
│   ├── sintactico/             # Análisis sintáctico SLR
│   ├── semantico/              # Análisis semántico
│   ├── intermedio/             # Generación de código intermedio
│   ├── lib.rs                  # Módulos públicos de la biblioteca
│   └── main.rs                 # Compilador principal
├── gramatica.txt               # Definición de la gramática del lenguaje
└── Cargo.toml                  # Configuración del proyecto
```

## Instalación

### Requisitos Previos

- Rust 1.70 o superior
- Cargo (incluido con Rust)

### Compilar el Proyecto

```bash
# Clonar el repositorio
git clone <repository-url>
cd compilador_rust

# Compilar el proyecto
cargo build --release
```

## Uso

### Compilar un Archivo

```bash
# Compilar un archivo con salida verbose
cargo run -- ruta/al/archivo.txt -v

# Compilar sin detalles
cargo run -- ruta/al/archivo.txt
```

### Ejecutar Tests

```bash
# Ejecutar todos los tests
cargo test

# ═══════════════════════════════════════════════════════════
# Tests de Análisis Léxico y Sintáctico
# ═══════════════════════════════════════════════════════════
cargo run --bin test_lexico

# Test del analizador sintáctico
cargo run --bin test_sintactico

# Test de FIRST y FOLLOW
cargo run --bin test_first_follow

# Test del autómata LR(0)
cargo run --bin test_lr0

# ═══════════════════════════════════════════════════════════
# Tests de Análisis Semántico
# ═══════════════════════════════════════════════════════════
cargo run --bin test_cubo_semantico
cargo run --bin test_tabla_variables
cargo run --bin test_directorio_funciones
cargo run --bin test_contexto_semantico

# ═══════════════════════════════════════════════════════════
# Tests de Generación de Código Intermedio (NUEVO)
# ═══════════════════════════════════════════════════════════
cargo run --release --bin test_generador_expresiones      # Expresiones aritméticas
cargo run --release --bin test_generador_relacionales     # Expresiones relacionales
cargo run --release --bin test_generador_estatutos        # Estatutos lineales
cargo run --release --bin test_programa_completo          # Programa completo
```

### Regenerar Tablas SLR

Si modificas la gramática en `gramatica.txt`, debes regenerar las tablas:

```bash
cargo run --bin generador_slr
```

## Documentación

### Módulos Principales

- [**src/intermedio/**](src/intermedio/README.md) - Generación de código intermedio (cuádruplos)
- [**src/semantico/**](src/semantico/README.md) - Análisis semántico y sus tablas
- [**src/gramatica/**](src/gramatica/README.md) - Parseo de gramáticas y cálculo de conjuntos
- [**src/lexico/**](src/lexico/README.md) - Análisis léxico y tokenización
- [**src/sintactico/**](src/sintactico/README.md) - Análisis sintáctico SLR
- [**src/bin/**](src/bin/README.md) - Programas ejecutables y utilidades

## Gramática del Lenguaje Patito

### Tipos de Datos

- `entero`: Números enteros
- `flotante`: Números de punto flotante
- `nula`: Tipo void (sin retorno)

### Palabras Reservadas

`programa`, `inicio`, `fin`, `vars`, `entero`, `flotante`, `si`, `sino`, `entonces`, `mientras`, `haz`, `escribe`, `nula`

### Operadores

- **Aritméticos**: `+`, `-`, `*`, `/`
- **Relacionales**: `>`, `<`, `==`, `!=`
- **Asignación**: `=`

### Estructuras de Control

- Condicionales: `si`/`entonces`/`sino`
- Ciclos: `mientras`/`haz`
- Funciones con parámetros y tipo de retorno

### Ejemplo de Programa

```
programa mi_programa;

vars x, y : entero;
vars total : flotante;

entero suma(a : entero, b : entero) {
    vars resultado : entero;
    {
        resultado = a + b;
    }
};

inicio {
    x = 5;
    y = 10;
    total = 3.14;

    si (x < y) entonces {
        escribe("x es menor que y");
    } sino {
        escribe("x es mayor o igual a y");
    };

    mientras (x < 10) haz {
        x = x + 1;
    }
}
fin
```

## Arquitectura del Compilador

### Fase 1: Análisis Léxico

1. Lee el archivo fuente carácter por carácter
2. Agrupa caracteres en tokens usando expresiones regulares
3. Clasifica tokens por tipo (palabra reservada, identificador, operador, etc.)
4. Genera una lista de tokens con información de línea

### Fase 2: Análisis Sintáctico

1. Lee la secuencia de tokens del análisis léxico
2. Usa un parser SLR(1) basado en tablas ACTION y GOTO
3. Valida que la estructura del programa cumple con la gramática
4. Reporta errores de sintaxis con ubicación precisa

### Fase 3: Análisis Semántico

1. **Puntos Neurálgicos**: Ejecuta acciones en puntos específicos de la gramática
2. **Directorio de Funciones**: Crea y mantiene información de funciones
3. **Tablas de Variables**: Gestiona variables por alcance (local y global)
4. **Cubo Semántico**: Valida tipos en todas las operaciones
5. **Validaciones**:
   - Variable doblemente declarada
   - Variable no declarada
   - Función duplicada
   - Tipos incompatibles

### Fase 4: Generación de Tablas

1. Lee la gramática desde `gramatica.txt`
2. Calcula conjuntos FIRST y FOLLOW
3. Construye el autómata LR(0)
4. Genera tablas ACTION y GOTO
5. Escribe el archivo `tabla_slr.rs` con las tablas

## Cubo Semántico

### Operadores Aritméticos

- **Regla**: Cualquier operación con `flotante` promueve a `flotante`
- `entero + entero = entero`
- `entero + flotante = flotante`
- `flotante + entero = flotante`

### Operadores Relacionales

- **Regla**: Siempre retornan `entero` (0 = falso, 1 = verdadero)
- Permiten comparaciones entre cualquier combinación de tipos

### Asignación

- **Regla**: Fuertemente tipado
- `entero = entero` ✓
- `flotante = flotante` ✓
- `flotante = entero` ✓ (promoción)
- `entero = flotante` ✗ (truncamiento no permitido)

## Tecnologías Utilizadas

- **Rust 2024 Edition**: Lenguaje de programación principal
- **regex**: Para el análisis léxico con expresiones regulares
- **lazy_static**: Para inicialización estática de tablas grandes
- **HashMap/HashSet**: Para estructuras de datos eficientes (O(1))

## Estado del Proyecto

- [x] Análisis léxico completo
- [x] Parser de gramáticas
- [x] Cálculo de FIRST y FOLLOW
- [x] Construcción de autómata LR(0)
- [x] Generación de tablas SLR
- [x] Análisis sintáctico SLR funcional
- [x] Análisis semántico
- [x] Generación de código intermedio
- [ ] Optimización
- [ ] Generación de código objeto

## Autores

Eduardo Zentella Castillo

## Licencia

Este proyecto es parte del curso de Compiladores en Tecnológico de Monterrey.

## Referencias

- Compilers: Principles, Techniques, and Tools (Dragon Book)
- The Rust Programming Language Book
- Documentación de SLR Parsing
- Notas del curso de Compiladores
