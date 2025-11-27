# Directorio de Ejecutables (bin/)

## Descripción

Este directorio contiene programas ejecutables independientes para probar, validar y generar componentes del compilador. Cada archivo es un binario independiente que se puede ejecutar con `cargo run --bin <nombre>`.

## Archivos Ejecutables

### `generador.rs` (generador_slr)

**Propósito:** Generar automáticamente las tablas SLR a partir de la gramática.

**Ejecución:**

```bash
cargo run --bin generador_slr
```

**Proceso:**

1. Lee y parsea `gramatica.txt`
2. Calcula conjuntos FIRST y FOLLOW
3. Construye el autómata LR(0)
4. Genera las tablas ACTION y GOTO
5. Escribe `src/sintactico/tabla_slr.rs`

**Salida:**

```
=== Generador de Analizador Sintáctico SLR ===

✓ Gramática parseada correctamente!
  - Total de producciones: 70
  - Símbolo inicial: <ProgramaPrime>

Calculando conjuntos FIRST y FOLLOW...
✓ Conjuntos FIRST y FOLLOW calculados correctamente!
  - Total de FIRST calculados: 77
  - Total de FOLLOW calculados: 41

Construyendo autómata LR(0)...
✓ Autómata LR(0) construido correctamente!
  - Total de estados: 108
  - Total de transiciones: 240

Generando tablas SLR...
✓ Tablas SLR generadas correctamente!
  - Entradas en ACTION: 465
  - Entradas en GOTO: 124

Escribiendo archivo tabla_slr.rs...
✓ Archivo generado exitosamente en src/sintactico/tabla_slr.rs

=== Generación completada exitosamente ===
```

**Cuándo ejecutar:**

- Después de modificar `gramatica.txt`
- Cuando se agregan nuevas producciones
- Si hay conflictos en el parsing que requieren ajuste de gramática

---

### `test_gramatica.rs`

**Propósito:** Verificar que la gramática se parsea correctamente.

**Ejecución:**

```bash
cargo run --bin test_gramatica
```

**Qué prueba:**

- Lectura del archivo `gramatica.txt`
- Formato correcto de producciones
- Creación de la regla aumentada
- Estructura de la gramática

**Salida esperada:**

```
=== Test de Parsing de Gramática ===

✓ Gramática parseada correctamente
  - Símbolo inicial: <Programa>
  - Total de producciones: 70

Primeras 10 producciones:
  0: <ProgramaPrime> → <Programa>
  1: <Programa> → programa id ; <VARS_OPT> <FUNCS_LIST> inicio <CUERPO> fin
  2: <VARS_OPT> → <VARS>
  3: <VARS_OPT> → ε
  ...
```

---

### `test_first_follow.rs`

**Propósito:** Probar el cálculo de conjuntos FIRST y FOLLOW.

**Ejecución:**

```bash
cargo run --bin test_first_follow
```

**Qué prueba:**

- Cálculo de FIRST para cada no terminal
- Cálculo de FOLLOW para cada no terminal
- Corrección de los conjuntos

**Salida esperada:**

```
=== Test de Conjuntos FIRST y FOLLOW ===

✓ Gramática cargada: 70 producciones
✓ Conjuntos FIRST calculados: 77
✓ Conjuntos FOLLOW calculados: 41

Ejemplos de conjuntos FIRST:
  FIRST(<Programa>) = {programa}
  FIRST(<EXPRESION>) = {id, cte_ent, cte_flot, +, -, (}
  FIRST(<FACTOR>) = {id, cte_ent, cte_flot, +, -, (}
  ...

Ejemplos de conjuntos FOLLOW:
  FOLLOW(<Programa>) = {$}
  FOLLOW(<EXPRESION>) = {;, ), ,, >, <, ==, !=}
  FOLLOW(<TERMINO>) = {+, -, ;, ), ,, >, <, ==, !=}
  ...
```

**Utilidad:**

- Debugging de gramática
- Verificar que los conjuntos son correctos
- Detectar problemas antes de generar tablas

---

### `test_lr0.rs`

**Propósito:** Probar la construcción del autómata LR(0).

**Ejecución:**

```bash
cargo run --bin test_lr0
```

**Qué prueba:**

- Construcción del autómata LR(0)
- Estados generados
- Transiciones entre estados
- Función de clausura
- Función goto

**Salida esperada:**

```
=== Test de Autómata LR(0) ===

✓ Gramática cargada
✓ Conjuntos FIRST/FOLLOW calculados
✓ Autómata LR(0) construido

Estadísticas:
  - Total de estados: 108
  - Total de transiciones: 240

Estado 0 (inicial):
  <ProgramaPrime> → • <Programa>
  <Programa> → • programa id ; <VARS_OPT> <FUNCS_LIST> inicio <CUERPO> fin

Transiciones desde Estado 0:
  programa → Estado 1
  <Programa> → Estado 2

Estado 1:
  <Programa> → programa • id ; <VARS_OPT> <FUNCS_LIST> inicio <CUERPO> fin

  ...
```

**Utilidad:**

- Visualizar el autómata
- Debugging de conflictos
- Entender el proceso de parsing

---

### `test_sintactico.rs`

**Propósito:** Probar el analizador sintáctico completo.

**Ejecución:**

```bash
cargo run --bin test_sintactico
```

**Qué prueba:**

- Parsing de programas de prueba
- Manejo de construcciones válidas
- Detección de errores sintácticos
- Funcionamiento de las tablas SLR

**Tests incluidos:**

**Test 1: Programa mínimo**

```
programa test;
inicio {
}
fin
```

**Test 2: Programa con asignación**

```
programa test;
inicio {
    x = 5;
}
fin
```

**Salida esperada:**

```
=== Test del Analizador Sintáctico SLR ===

--- Test 1: Programa mínimo ---
✓ Test 1 PASADO

--- Test 2: Programa con asignación ---
✓ Test 2 PASADO
```

**En modo verbose:**

```
Estado: 0 | Token: 'programa' (tipo: programa) | Pila: [0]
  → Acción: Shift(1)
Estado: 1 | Token: 'test' (tipo: id) | Pila: [0, 1]
  → Acción: Shift(3)
  ...
```

---

### Tests del Análisis Semántico

#### `test_cubo_semantico.rs`

Propósito: Probar las reglas del cubo semántico

Ejecución:
```bash
cargo run --bin test_cubo_semantico
```

Qué prueba:
- Validación de operaciones aritméticas
- Validación de operaciones relacionales
- Promoción de tipos
- Detección de errores de tipo

#### `test_tabla_variables.rs`

Propósito: Probar la tabla de variables

Ejecución:
```bash
cargo run --bin test_tabla_variables
```

Qué prueba:
- Agregar variables
- Buscar variables
- Detectar variables duplicadas

#### `test_directorio_funciones.rs`

Propósito: Probar el directorio de funciones

Ejecución:
```bash
cargo run --bin test_directorio_funciones
```

Qué prueba:
- Agregar funciones
- Agregar variables a funciones
- Búsqueda en diferentes alcances

#### `test_contexto_semantico.rs`

Propósito: Probar el contexto semántico completo

Ejecución:
```bash
cargo run --bin test_contexto_semantico
```

Qué prueba:
- Flujo completo del análisis semántico
- Coordinación entre componentes

---

### Tests de Generación de Código Intermedio

#### `test_generador_expresiones.rs`

Propósito: Probar generación de cuádruplos para expresiones aritméticas

Ejecución:
```bash
cargo run --bin test_generador_expresiones
```

Qué prueba:
- Generación de cuádruplos para suma, resta, multiplicación, división
- Manejo de precedencia de operadores
- Uso de temporales

#### `test_generador_relacionales.rs`

Propósito: Probar generación de cuádruplos para expresiones relacionales

Ejecución:
```bash
cargo run --bin test_generador_relacionales
```

Qué prueba:
- Operadores relacionales (>, <, ==, !=)
- Generación correcta de cuádruplos

#### `test_generador_estatutos.rs`

Propósito: Probar generación de cuádruplos para estatutos lineales

Ejecución:
```bash
cargo run --bin test_generador_estatutos
```

Qué prueba:
- Asignación
- Lectura
- Escritura

#### `test_programa_completo.rs`

Propósito: Probar compilación completa de un programa

Ejecución:
```bash
cargo run --bin test_programa_completo
```

Qué prueba:
- Análisis léxico, sintáctico y semántico
- Generación de código intermedio
- Programa completo con múltiples características

---

## Flujo de Desarrollo

Orden recomendado para usar estos programas:

1. Modificar gramatica.txt
2. cargo run --bin test_gramatica (si pasa)
3. cargo run --bin test_first_follow (si los conjuntos son correctos)
4. cargo run --bin test_lr0 (si el autómata está bien)
5. cargo run --bin generador_slr (genera las tablas)
6. cargo run --bin test_sintactico (prueba el parser completo)
7. cargo run --bin test_cubo_semantico (prueba análisis semántico)
8. cargo run --bin test_generador_expresiones (prueba generación de código)
9. cargo run -- archivo.txt (usar el compilador completo)

## Resumen de Comandos

| Comando                                     | Propósito                    |
| ------------------------------------------- | ---------------------------- |
| `cargo run --bin generador_slr`             | Generar tablas SLR           |
| `cargo run --bin test_gramatica`            | Validar gramática            |
| `cargo run --bin test_first_follow`         | Ver FIRST/FOLLOW             |
| `cargo run --bin test_lr0`                  | Ver autómata                 |
| `cargo run --bin test_sintactico`           | Probar parser                |
| `cargo run --bin test_cubo_semantico`       | Probar cubo semántico        |
| `cargo run --bin test_tabla_variables`      | Probar tabla de variables    |
| `cargo run --bin test_directorio_funciones` | Probar directorio funciones  |
| `cargo run --bin test_contexto_semantico`   | Probar contexto semántico    |
| `cargo run --bin test_generador_expresiones`| Probar gen. expresiones      |
| `cargo run --bin test_generador_relacionales`| Probar gen. relacionales    |
| `cargo run --bin test_generador_estatutos`  | Probar gen. estatutos        |
| `cargo run --bin test_programa_completo`    | Probar compilación completa  |

## Tips de Debugging

Si el generador falla:
1. Ejecuta `test_gramatica` primero
2. Revisa errores de sintaxis en `gramatica.txt`
3. Verifica que no haya símbolos no definidos

Si hay conflictos Shift/Reduce:
1. Ejecuta `test_first_follow` para ver conjuntos
2. Revisa si hay ambigüedad en la gramática
3. Puede requerir factorizar o ajustar precedencia

Si el parser falla en test:
1. Ejecuta en modo verbose
2. Mira en qué estado falla
3. Revisa el autómata con `test_lr0`
4. Verifica que el token esperado esté en FOLLOW

Si la generación de cuádruplos falla:
1. Ejecuta tests individuales de generación
2. Verifica que el análisis semántico pase
3. Revisa los puntos neurálgicos en acciones_semanticas.rs

## Referencias

- Cargo Book - Configuring a Target
- Testing in Rust
