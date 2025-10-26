# Compilador en Rust

## 📋 Descripción

Este proyecto implementa un **compilador completo** para un lenguaje de programación estructurado simple, desarrollado en Rust. El compilador incluye las fases de análisis léxico, análisis sintáctico SLR(1), y generación automática de tablas de parsing.

## 🎯 Características

- **Análisis Léxico**: Tokenización completa con reconocimiento de palabras reservadas, identificadores, constantes y operadores
- **Análisis Sintáctico SLR(1)**: Parser de tabla que valida la estructura sintáctica del código fuente
- **Generación Automática**: Construcción automática de autómatas LR(0) y tablas SLR
- **Cálculo de Conjuntos**: FIRST y FOLLOW sets para gramáticas libres de contexto
- **Manejo de Errores**: Reportes detallados de errores léxicos y sintácticos con números de línea

## 🏗️ Estructura del Proyecto

```
compilador_rust/
├── src/
│   ├── bin/              # Ejecutables y programas de prueba
│   ├── gramatica/        # Parsing y análisis de gramáticas
│   ├── lexico/           # Análisis léxico (tokenización)
│   ├── sintactico/       # Análisis sintáctico SLR
│   ├── lib.rs           # Módulos públicos de la biblioteca
│   └── main.rs          # Compilador principal
├── gramatica.txt        # Definición de la gramática del lenguaje
└── Cargo.toml           # Configuración del proyecto

```

## 🚀 Instalación

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

## 💻 Uso

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

# Test del analizador léxico
cargo run --bin test_lexico

# Test del analizador sintáctico
cargo run --bin test_sintactico

# Test de FIRST y FOLLOW
cargo run --bin test_first_follow

# Test del autómata LR(0)
cargo run --bin test_lr0
```

### Regenerar Tablas SLR

Si modificas la gramática en `gramatica.txt`, debes regenerar las tablas:

```bash
cargo run --bin generador_slr
```

## 📖 Documentación por Módulo

- [**src/gramatica/**](src/gramatica/README.md) - Parseo de gramáticas y cálculo de conjuntos
- [**src/lexico/**](src/lexico/README.md) - Análisis léxico y tokenización
- [**src/sintactico/**](src/sintactico/README.md) - Análisis sintáctico SLR
- [**src/bin/**](src/bin/README.md) - Programas ejecutables y utilidades

## 🔤 Gramática del Lenguaje

El lenguaje soporta:

- **Palabras reservadas**: `programa`, `inicio`, `fin`, `vars`, `entero`, `flotante`, `si`, `sino`, `mientras`, `haz`, `escribe`, `nula`
- **Operadores**: `+`, `-`, `*`, `/`, `=`, `==`, `!=`, `<`, `>`
- **Estructuras de control**: Condicionales (`si`/`sino`), ciclos (`mientras`)
- **Funciones**: Declaración y llamada con argumentos
- **Variables**: Declaración con tipos (`entero`, `flotante`)

### Ejemplo de Programa

```
programa ejemplo;

vars x, y : entero;

inicio {
    x = 5;
    y = x + 10;

    si (y > 10) entonces {
        escribe("y es mayor que 10");
    } sino {
        escribe("y es menor o igual a 10");
    };

    mientras (x < 10) haz {
        x = x + 1;
    }
}
fin
```

## 🧪 Arquitectura del Compilador

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

### Fase 3: Generación de Tablas (Offline)

1. Lee la gramática desde `gramatica.txt`
2. Calcula conjuntos FIRST y FOLLOW
3. Construye el autómata LR(0)
4. Genera tablas ACTION y GOTO
5. Escribe el archivo `tabla_slr.rs` con las tablas

## 🛠️ Tecnologías Utilizadas

- **Rust 2024 Edition**: Lenguaje de programación principal
- **regex**: Para el análisis léxico con expresiones regulares
- **lazy_static**: Para inicialización estática de tablas grandes
- **HashMap/HashSet**: Para estructuras de datos eficientes

## 📝 Estado del Proyecto

- [x] Análisis léxico completo
- [x] Parser de gramáticas
- [x] Cálculo de FIRST y FOLLOW
- [x] Construcción de autómata LR(0)
- [x] Generación de tablas SLR
- [x] Análisis sintáctico SLR funcional
- [ ] Análisis semántico
- [ ] Generación de código intermedio
- [ ] Optimización
- [ ] Generación de código objeto

## 👥 Autores

Eduardo Zentella Castillo

## 📄 Licencia

Este proyecto es parte del curso de Compiladores en Tecnológico de Monterrey.

## 🤝 Contribuciones

Este es un proyecto académico. Las contribuciones están limitadas a los miembros del equipo.

## 📚 Referencias

- Compilers: Principles, Techniques, and Tools (Dragon Book)
- The Rust Programming Language Book
- Documentación de SLR Parsing
