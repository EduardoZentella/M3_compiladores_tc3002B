# Módulo de Generación de Código Intermedio

Este módulo implementa la **generación de cuádruplos** (código intermedio de tres direcciones) para el compilador de Patito, correspondiente a la **Entrega 3** del proyecto.

## Contenido

- [Estructura del Módulo](#estructura-del-módulo)
- [Componentes Principales](#componentes-principales)
- [Algoritmo de Generación](#algoritmo-de-generación)
- [Puntos Neurálgicos](#puntos-neurálgicos)
- [Ejemplos de Uso](#ejemplos-de-uso)
- [Integración con el Parser](#integración-con-el-parser)

---

## Estructura del Módulo

```
src/intermedio/
├── mod.rs              # Exportaciones del módulo
├── cuadruplo.rs        # Definición de cuádruplos y operadores
├── memoria.rs          # Gestor de variables temporales (AVAIL)
└── generador.rs        # Generador principal con pilas y algoritmos
```

---

## Componentes Principales

### 1. Cuadruplo (cuadruplo.rs)

Representa una instrucción de código intermedio:

```rust
pub struct Cuadruplo {
    pub operador: OperadorCuadruplo,
    pub operando_izq: Operando,
    pub operando_der: Operando,
    pub resultado: Operando,
}
```

**Formato**: `(operador, operando_izq, operando_der, resultado)`

**Ejemplo**: `(+, a, b, t1)` → `t1 = a + b`

#### Operadores Soportados

| Categoría        | Operadores                                   | Descripción                         |
| ---------------- | -------------------------------------------- | ----------------------------------- |
| **Aritméticos**  | `+`, `-`, `*`, `/`                           | Operaciones matemáticas             |
| **Relacionales** | `>`, `<`, `==`, `!=`                         | Comparaciones (retornan entero 0/1) |
| **Asignación**   | `=`                                          | Asignación de valores               |
| **E/S**          | `lee`, `escribe`                             | Lectura y escritura                 |
| **Control**      | `goto`, `gotof`, `gotov`                     | Saltos (futuras entregas)           |
| **Funciones**    | `era`, `param`, `gosub`, `return`, `endfunc` | Llamadas (futuras entregas)         |

#### Tipos de Operandos

```rust
pub enum Operando {
    Variable(String),          // Variable o identificador: "a", "resultado"
    ConstanteEntera(i32),      // Constante entera: 42, -10
    ConstanteFlotante(f64),    // Constante flotante: 3.14, 2.5
    Temporal(usize),           // Temporal: t0, t1, t2, ...
    Vacio,                     // Operando vacío: "-"
}
```

---

### 2. Gestor de Memoria (memoria.rs)

Administra la asignación y liberación de **variables temporales**.

```rust
pub struct GestorMemoria {
    contador_temporales: usize,
    temporales_disponibles: HashSet<usize>,  // AVAIL
    tipos_temporales: Vec<TipoDato>,
}
```

#### Funciones Principales

- `siguiente_temporal(tipo: TipoDato) -> usize`: Obtiene el siguiente temporal disponible (AVAIL.next())
- `liberar_temporal(temporal: usize)`: Retorna un temporal al pool de disponibles
- `obtener_tipo_temporal(temporal: usize) -> Option<TipoDato>`: Consulta el tipo de un temporal

**Optimización**: Reutiliza temporales liberados antes de crear nuevos.

---

### 3. Generador de Cuádruplos (generador.rs)

Implementa los **algoritmos de traducción** para expresiones y estatutos.

```rust
pub struct GeneradorCuadruplos {
    // Pilas para generación
    poper: Vec<OperadorCuadruplo>,     // Pila de operadores pendientes
    pilao: Vec<Operando>,              // Pila de operandos pendientes
    ptypes: Vec<TipoDato>,             // Pila de tipos

    // Cola de salida
    quad: VecDeque<Cuadruplo>,         // Cola de cuádruplos generados

    // Gestión y validación
    gestor_memoria: GestorMemoria,     // Gestor de temporales
    cubo_semantico: CuboSemantico,     // Validación de tipos
}
```

---

## Algoritmo de Generación

### Algoritmo para Operaciones Aritméticas y Relacionales

Basado en el algoritmo presentado en clase:

```
1. PilaO.Push(operando) y PTypes.Push(tipo)
2. POper.Push(operador)
3. Si POper.top() es el operador esperado:
   a. right_operand = PilaO.Pop()
   b. right_Type = PTypes.Pop()
   c. left_operand = PilaO.Pop()
   d. left_Type = PTypes.Pop()
   e. operator = POper.Pop()
   f. result_Type = Semantics[left_Type, operator, right_Type]
   g. Si result_Type != ERROR:
      - result = AVAIL.next()
      - generar quad = (operator, left_operand, right_operand, result)
      - Quad.Push(quad)
      - PilaO.Push(result)
      - PTypes.Push(result_Type)
      - Liberar temporales si fueron usados
   h. Sino: ERROR("Type mismatch")
```

### Precedencia de Operadores

- **Multiplicación/División** (`*`, `/`): Mayor precedencia
- **Suma/Resta** (`+`, `-`): Menor precedencia
- **Paréntesis**: Marca de fondo falso para alterar precedencia

---

## Puntos Neurálgicos

Los **puntos neurálgicos (PN)** son llamadas al generador de cuádruplos insertadas en la gramática del parser para traducir expresiones a código intermedio.

### PN1: Procesar Operando

**Ubicación**: Al reconocer un `id` o constante
**Acción**: `procesar_operando(nombre: &str)`

```rust
// PilaO.Push(operando)
// PTypes.Push(tipo)
generador.procesar_operando("a")?;
```

---

### PN2: Procesar Suma/Resta

**Ubicación**: Al reconocer `+` o `-`
**Acción**: `procesar_suma_resta(operador: &str)`

```rust
// POper.Push(+ o -)
generador.procesar_suma_resta("+")?;
```

---

### PN3: Procesar Multiplicación/División

**Ubicación**: Al reconocer `*` o `/`
**Acción**: `procesar_mult_div(operador: &str)`

```rust
// POper.Push(* o /)
generador.procesar_mult_div("*")?;
```

---

### PN4: Generar Cuádruplo Suma/Resta

**Ubicación**: Después de procesar término completo
**Acción**: `generar_suma_resta()`

```rust
// Si POper.top() == '+' o '-', generar cuádruplo
generador.generar_suma_resta()?;
```

---

### PN5: Generar Cuádruplo Multiplicación/División

**Ubicación**: Después de procesar factor completo
**Acción**: `generar_mult_div()`

```rust
// Si POper.top() == '*' o '/', generar cuádruplo
generador.generar_mult_div()?;
```

---

### PN6: Abrir Paréntesis

**Ubicación**: Al reconocer `(`
**Acción**: `abrir_parentesis()`

```rust
// POper.Push(fondo falso)
generador.abrir_parentesis();
```

---

### PN7: Cerrar Paréntesis

**Ubicación**: Al reconocer `)`
**Acción**: `cerrar_parentesis()`

```rust
// POper.Pop(fondo falso)
generador.cerrar_parentesis()?;
```

---

### PN8: Procesar Operador Relacional

**Ubicación**: Al reconocer `>`, `<`, `==`, `!=`
**Acción**: `procesar_relacional(operador: &str)`

```rust
// POper.Push(operador relacional)
generador.procesar_relacional(">")?;
```

---

### PN9: Generar Cuádruplo Relacional

**Ubicación**: Después de procesar expresión completa
**Acción**: `generar_relacional()`

```rust
// Si POper.top() es relacional, generar cuádruplo
generador.generar_relacional()?;
```

---

### Estatutos Lineales

#### Asignación

```rust
// variable = expresión;
generador.generar_asignacion("variable")?;
```

#### Lectura

```rust
// lee(variable);
generador.generar_lectura("variable")?;
```

#### Escritura

```rust
// escribe(expresión);
generador.generar_escritura()?;
```

---

## Tests Disponibles

Ejecuta los siguientes tests para verificar la generación de cuádruplos:

```bash
# Test de expresiones aritméticas
cargo run --release --bin test_generador_expresiones

# Test de expresiones relacionales
cargo run --release --bin test_generador_relacionales

# Test de estatutos lineales
cargo run --release --bin test_generador_estatutos

# Test de programa completo
cargo run --release --bin test_programa_completo
```

---

## Validaciones Implementadas

1. **Validación de Tipos**: Usa el cubo semántico para verificar compatibilidad de tipos
2. **Variables No Declaradas**: Verifica que las variables existan en el contexto semántico
3. **Promoción de Tipos**: `entero` → `flotante` en operaciones mixtas
4. **Asignación Estricta**: `entero = flotante` es error (truncación)
5. **Paréntesis Balanceados**: Detecta paréntesis sin cerrar

---

---

## Referencias

- Material de clase: "Semantics of Expressions and Intermediate Code"
- Gramática de Patito: `gramatica.txt`

---

## Autor

Eduardo Zentella Castillo
Tecnológico de Monterrey
8vo Semestre - Compiladores (TC3002B)
**Entrega 3: Generación de Código Intermedio**
