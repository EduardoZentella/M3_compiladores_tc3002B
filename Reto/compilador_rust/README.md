# Compilador Patito en Rust

## Descripción

Este proyecto implementa un **compilador completo y funcional** para el lenguaje de programación **Patito**, desarrollado en Rust. El compilador incluye todas las fases: análisis léxico, análisis sintáctico SLR(1), análisis semántico, generación de código intermedio (cuádruplos), y máquina virtual para ejecución.

## Características Completas

### Análisis Léxico

- Tokenización completa con reconocimiento de palabras reservadas, identificadores, constantes y operadores
- Manejo de errores con números de línea
- Soporte para literales: enteros, flotantes, caracteres y strings

### Análisis Sintáctico SLR(1)

- Parser de tabla que valida la estructura sintáctica del código fuente
- Generación automática de autómatas LR(0) y tablas SLR
- 141 estados, 654 entradas ACTION, 175 entradas GOTO
- Cálculo de conjuntos FIRST y FOLLOW
- Reportes detallados de errores sintácticos

### Análisis Semántico

- **Cubo Semántico**: Validación de tipos en todas las operaciones
- **Tabla de Variables**: Gestión de variables por alcance con HashMap (O(1))
- **Directorio de Funciones**: Gestión completa de funciones, parámetros y retornos
- **Contexto Semántico**: Coordinación del análisis durante el parsing
- **Validaciones**:
  - Variables doblemente declaradas
  - Variables no declaradas
  - Funciones duplicadas
  - Tipos incompatibles en operaciones
  - Asignaciones con truncamiento (rechazadas)
  - Promoción de tipos (entero → flotante)
  - Número correcto de parámetros en llamadas

### Generación de Código Intermedio

- **Cuádruplos**: Código intermedio de tres direcciones
- **Pilas de Operadores y Operandos**: POper, PilaO, PTypes
- **Memoria Virtual Unificada**: Sistema de direcciones virtuales optimizado
  - Global: 1000-6999 (variables globales)
  - Local: 7000-12999 (variables/parámetros de funciones)
  - Temporal: 13000-18999 (con pool AVAIL de reutilización)
  - Constante: 19000-24999 (con deduplicación HashMap)
- **Estructuras de Control**:
  - Condicionales (IF/ELSE) con saltos GOTOF y GOTO
  - Ciclos (WHILE) con saltos hacia atrás
  - Anidamiento ilimitado de estructuras
- **Funciones**:
  - Declaración con parámetros múltiples
  - Variables locales
  - Valores de retorno
  - Recursión completa
  - Cuádruplos: ERA, PARAM, GOSUB, RETURN, ENDFUNC
- **Optimizaciones**:
  - Pool de temporales (AVAIL) para reutilización de memoria
  - Deduplicación de constantes
  - GOTO inicial para saltar definiciones de funciones

### Máquina Virtual

- Ejecutor de cuádruplos con 4 segmentos de memoria
- Soporte completo para todas las operaciones
- Stack de marcos para llamadas a funciones
- Marco temporal para parámetros (ERA)
- Ejecución inline de funciones hasta ENDFUNC
- Manejo de valores de retorno mediante temporales

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
├── tests/
│   └── programas/              # Programas de prueba en Patito
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

## Resultados de Tests - Todos Pasando

| Test                       | Descripción                       | Resultado Esperado | Estado |
| -------------------------- | --------------------------------- | ------------------ | ------ |
| 00_test_simple             | Asignación y escritura básica     | 10                 | SI     |
| 01_expresiones_aritmeticas | Operaciones aritméticas           | 25, 7, 30          | SI     |
| 02_entrada_salida          | Lectura y escritura               | 25                 | SI     |
| 03_decisiones              | Condicionales IF/ELSE             | "Ambos correctos"  | SI     |
| 04_ciclos                  | Ciclo WHILE                       | 1                  | SI     |
| 05_factorial_funcion       | Factorial recursivo (5!)          | 120                | SI     |
| 06_fibonacci_funcion       | Fibonacci recursivo fib(8)        | 21                 | SI     |
| test_funcion_simple        | Función con parámetro             | 5                  | SI     |
| 09_multiples_funciones     | 2 funciones, múltiples parámetros | 40, 375            | SI     |
| 10_programa_completo       | Programa complejo con todo        | 95                 | SI     |

**100% de tests pasando** - El compilador está completamente funcional.

## Uso

### Compilar un Archivo

```bash
# Compilar sin debug (solo output del programa)
cargo run -- ruta/al/archivo.txt

# Compilar con verbose nivel 1 (-v): Fases principales del compilador
cargo run -- ruta/al/archivo.txt -v

# Compilar con verbose nivel 2 (-vv): + Acciones semánticas y estados del parser
cargo run -- ruta/al/archivo.txt -vv

# Compilar con verbose nivel 3 (-vvv): + Debug completo (tokens, reduces, atributos)
cargo run -- ruta/al/archivo.txt -vvv
```

### Niveles de Verbose

- **Nivel 0** (sin flag): Solo muestra el output del programa y cuádruplos generados
- **Nivel 1** (`-v`): Muestra las fases principales del compilador (léxico, sintáctico, VM)
- **Nivel 2** (`-vv`): Añade estados del parser y acciones semánticas importantes
- **Nivel 3** (`-vvv`): Debug completo con cada token, reduce, atributos y detalles de cuádruplos

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
# Tests de Generación de Código Intermedio
# ═══════════════════════════════════════════════════════════
cargo run --release --bin test_generador_expresiones      # Expresiones aritméticas
cargo run --release --bin test_generador_relacionales     # Expresiones relacionales
cargo run --release --bin test_generador_estatutos        # Estatutos lineales
cargo run --release --bin test_programa_completo          # Programa completo

# Tests de memoria virtual
cargo test --lib memoria                                  # Tests unitarios de memoria virtual

# ═══════════════════════════════════════════════════════════
# Tests de Control de Flujo (programas de prueba)
# ═══════════════════════════════════════════════════════════
# Los programas de test están en tests/programas/
# Ver tests/programas/README.md para lista completa

cargo run --bin compilador_rust tests/programas/test_if_const.txt         # IF simple
cargo run --bin compilador_rust tests/programas/test_if_else_const.txt    # IF-ELSE
cargo run --bin compilador_rust tests/programas/test_while_const.txt      # WHILE
cargo run --bin compilador_rust tests/programas/test_complejo_const.txt   # Estructuras anidadas
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

### Programas de Prueba

- [**tests/programas/**](tests/programas/README.md) - Programas de ejemplo y tests en Patito

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
programa factorial;

vars n, resultado : entero;

entero factorial(num : entero) {
    vars res : entero;
    si (num == 0) entonces {
        res = 1;
    } sino {
        res = num * factorial(num - 1);
    };
    regresa res;
};

inicio {
    n = 5;
    resultado = factorial(n);
    escribe("Factorial de 5 es:");
    escribe(resultado);
}
fin
```

**Salida**: `Factorial de 5 es: 120`

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

### Fase 4: Generación de Código Intermedio

1. **Puntos Neurálgicos**: Ejecuta acciones en puntos específicos del reduce
2. **Pilas de Traducción**:
   - POper: Pila de operadores pendientes
   - PilaO: Pila de operandos
   - PTypes: Pila de tipos de operandos
3. **Generación de Cuádruplos**:
   - Expresiones aritméticas y relacionales
   - Asignaciones y estatutos
   - Saltos condicionales (GOTOF) e incondicionales (GOTO)
   - Funciones (ERA, PARAM, GOSUB, RETURN, ENDFUNC)
4. **Optimizaciones**:
   - Pool de temporales (AVAIL) para reutilización
   - Deduplicación de constantes
   - Liberación inmediata de temporales después de uso

### Fase 5: Máquina Virtual

1. **Carga del Programa**:
   - Cuádruplos
   - Mapa de funciones con direcciones de inicio
   - Constantes en memoria
   - Tabla de strings
2. **Ejecución**:
   - Stack de marcos para funciones
   - Memoria segmentada (global, local, temporal, constante)
   - Ejecución secuencial con saltos
   - Manejo de llamadas inline hasta ENDFUNC
3. **I/O**:
   - Escritura a consola
   - (Lectura reservada para implementación futura)

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
- [x] Análisis semántico completo
- [x] Generación de código intermedio
- [x] Memoria virtual unificada (direcciones 1000-24999)
- [x] Pool de temporales (AVAIL)
- [x] Estructuras de control (IF/ELSE, WHILE)
- [x] Anidamiento ilimitado de estructuras
- [x] Funciones con parámetros y retorno
- [x] Recursión completa
- [x] Variables locales en funciones
- [x] Máquina virtual funcional
- [x] Todos los tests pasando (10/10)

**El compilador está 100% funcional y probado.**

## Documentación Técnica Completa

Para entender cómo funciona internamente el compilador y poder hacer cualquier modificación (agregar operadores, cambiar manejo de memoria, modificar cubo semántico, etc.), consulta:

**[GUÍA TÉCNICA COMPLETA](GUIA_TECNICA.md)** - Documento exhaustivo con todos los detalles de implementación

## Autores

Eduardo Zentella Castillo

## Licencia

Este proyecto es parte del curso de Compiladores en Tecnológico de Monterrey.

## Referencias

- Compilers: Principles, Techniques, and Tools (Dragon Book)
- The Rust Programming Language Book
- Documentación de SLR Parsing
- Notas del curso de Compiladores
