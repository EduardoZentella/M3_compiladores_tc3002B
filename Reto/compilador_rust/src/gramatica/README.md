# Módulo de Gramática

## Descripción

Este módulo contiene todas las funcionalidades relacionadas con el análisis y procesamiento de gramáticas libres de contexto. Incluye el parseo de gramáticas, cálculo de conjuntos FIRST y FOLLOW, y construcción del autómata LR(0).

## Archivos

### `mod.rs`

Define las estructuras de datos fundamentales para representar gramáticas:

- **`Simbolo`**: Enum que representa símbolos terminales y no terminales
- **`Produccion`**: Estructura que representa una regla de producción
- **`Gramatica`**: Estructura principal que contiene todas las producciones

```rust
pub enum Simbolo {
    Terminal(String),    // Tokens del lenguaje (id, +, ;, etc.)
    NoTerminal(String),  // Símbolos de la gramática (<EXPRESION>, <TERMINO>, etc.)
    Epsilon,             // Cadena vacía (ε)
}

pub struct Produccion {
    pub id: usize,
    pub cabeza: Simbolo,         // Lado izquierdo de la producción
    pub cuerpo: Vec<Simbolo>,    // Lado derecho de la producción
}
```

### `parser.rs`

Parsea archivos de gramática en formato texto y los convierte a estructuras de datos internas.

**Funcionalidades:**

- Lee archivos con formato `<NoTerminal> → símbolo1 símbolo2 ...`
- Crea automáticamente la regla aumentada `<S'> → <S>`
- Maneja producciones épsilon
- Valida sintaxis de la gramática

**Formato esperado:**

```
<Programa> → programa id ; <Cuerpo> fin
<Cuerpo> → { <Estatutos> }
<Estatutos> → <Estatuto> <Estatutos>
<Estatutos> → ε
```

### `first_follow.rs`

Calcula los conjuntos FIRST y FOLLOW para cada símbolo no terminal de la gramática.

**Conjuntos FIRST:**

- FIRST(X) = conjunto de terminales que pueden aparecer al inicio de derivaciones desde X
- Usado para determinar qué producción aplicar en el parsing

**Conjuntos FOLLOW:**

- FOLLOW(X) = conjunto de terminales que pueden aparecer inmediatamente después de X
- Usado para construir la tabla SLR y detectar reducciones

**Algoritmo:**

1. Inicializar conjuntos vacíos
2. Iterar hasta que no haya cambios (punto fijo)
3. Aplicar reglas de cálculo según la gramática
4. Retornar mapas de FIRST y FOLLOW

### `lr0.rs`

Construye el autómata LR(0) necesario para el análisis sintáctico SLR.

**Estructuras:**

- **`ItemLR0`**: Representa un ítem LR(0) (producción con punto)
  ```
  A → α • β  (regla_id: usize, punto: usize)
  ```
- **`EstadoLR0`**: Conjunto de ítems LR(0)

**Algoritmo:**

1. **Inicio**: Crear estado inicial con ítem S' → • S
2. **Clausura**: Agregar ítems derivados cuando el punto está antes de un no terminal
3. **Goto**: Calcular transiciones moviendo el punto sobre símbolos
4. **Iteración**: Repetir hasta explorar todos los estados posibles

**Resultado:**

- Lista de estados (conjuntos de ítems)
- Mapa de transiciones: (estado, símbolo) → nuevo_estado

## Flujo de Procesamiento

```
gramatica.txt
     ↓
  parser.rs → Gramatica
     ↓
  first_follow.rs → (FIRST, FOLLOW)
     ↓
  lr0.rs → Autómata LR(0)
     ↓
  [Usado por generador_slr para crear tablas]
```

## Conceptos Clave

### Gramática Libre de Contexto

Una gramática G = (N, T, P, S) donde:

- **N**: Conjunto de símbolos no terminales
- **T**: Conjunto de símbolos terminales
- **P**: Conjunto de producciones (reglas)
- **S**: Símbolo inicial

### Gramática Aumentada

Se agrega una nueva producción `S' → S` donde S' es el nuevo símbolo inicial. Esto permite detectar la aceptación del input cuando se reduce esta regla.

### Ítem LR(0)

Representa una posición en una producción durante el parsing:

- `A → α • β` significa que hemos visto α y esperamos ver β
- El punto (•) indica la posición actual en la derivación

### Clausura

Dado un conjunto de ítems, la clausura agrega todos los ítems que pueden derivarse:

- Si tenemos `A → α • B β`, agregamos `B → • γ` para todas las producciones de B

### Función Goto

Calcula el siguiente estado al leer un símbolo:

- `goto(I, X)` = clausura de ítems `A → αX • β` donde `A → α • X β` está en I

## Ejemplo de Uso

```rust
use compilador_rust::gramatica::parser::parsear_gramatica;
use compilador_rust::gramatica::first_follow::calcular_first_follow;
use compilador_rust::gramatica::lr0::construir_automata;

// 1. Parsear gramática
let gramatica = parsear_gramatica("gramatica.txt")?;

// 2. Calcular FIRST y FOLLOW
let (first_sets, follow_sets) = calcular_first_follow(&gramatica);

// 3. Construir autómata LR(0)
let (estados, transiciones) = construir_automata(&gramatica);

println!("Estados: {}", estados.len());
println!("Transiciones: {}", transiciones.len());
```

## Complejidad

- **Parseo**: O(n) donde n = número de líneas en el archivo
- **FIRST/FOLLOW**: O(|N| × |P|²) en el peor caso
- **Autómata LR(0)**: O(|P| × |G|) donde G es el tamaño de la gramática

## Manejo de Errores

El módulo detecta y reporta:

- Producciones malformadas en el archivo de gramática
- Símbolos no definidos
- Gramáticas ambiguas o con conflictos (en construcción del autómata)

## Referencias

- Dragon Book, Capítulo 4: Syntax Analysis
- Engineering a Compiler, Sección sobre LR Parsing
- [LR Parser - Wikipedia](https://en.wikipedia.org/wiki/LR_parser)
