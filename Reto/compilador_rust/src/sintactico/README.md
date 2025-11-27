# Módulo Sintáctico

## Descripción

Este módulo implementa el analizador sintáctico SLR(1) del compilador. Valida que la secuencia de tokens generada por el análisis léxico cumple con la estructura gramatical del lenguaje.

## Archivos

### `mod.rs`

Implementa el analizador sintáctico basado en tablas SLR.

**Función principal: `analyze(tokens: &[Token], is_verbose: &bool)`**

```rust
pub fn analyze(tokens: &[Token], is_verbose: &bool) -> Result<(), String>
```

**Algoritmo SLR(1):**

```
Entrada: Cadena de tokens w, tablas ACTION y GOTO
Salida: Accept si w ∈ L(G), Error en caso contrario

1. Inicializar:
   - pila_estados ← [0]      // Stack de estados
   - cursor ← 0               // Posición en tokens[]

2. Repetir:
   a. estado_actual ← tope de pila_estados
   b. símbolo_actual ← tokens[cursor].tipo.as_grammar_string()

   c. accion ← ACTION[estado_actual, símbolo_actual]

   d. Casos:
      - Shift(s):
          * push(pila_estados, s)
          * cursor++

      - Reduce(r):
          * regla ← PRODUCCIONES[r]
          * pop pila_estados (regla.longitud_cuerpo veces)
          * estado_anterior ← tope de pila_estados
          * nuevo_estado ← GOTO[estado_anterior, regla.cabeza]
          * push(pila_estados, nuevo_estado)

      - Accept:
          * return Ok(())

      - Error:
          * return Err("Error de sintaxis...")
```

**Estados del Parser:**

```
┌─────────────┐
│ Estado n    │ ← Tope de la pila
├─────────────┤
│ Estado n-1  │
├─────────────┤
│    ...      │
├─────────────┤
│ Estado 0    │ ← Estado inicial
└─────────────┘

Cursor → [Token₁] [Token₂] [Token₃] ... [Token_n] [$]
```

### `tabla_slr.rs`

**Archivo auto-generado** que contiene las tablas de parsing SLR.

⚠️ **NO EDITAR MANUALMENTE** - Se genera con `cargo run --bin generador_slr`

**Estructuras:**

1. **`Accion`**: Enum que representa las acciones del parser

   ```rust
   pub enum Accion {
       Shift(usize),    // Empujar estado y avanzar
       Reduce(usize),   // Reducir usando producción
       Accept,          // Aceptar entrada
   }
   ```

2. **`Regla`**: Información de una producción para reducción

   ```rust
   pub struct Regla {
       pub id: usize,
       pub cabeza: String,          // No terminal izquierdo
       pub longitud_cuerpo: usize,  // Símbolos en el lado derecho
   }
   ```

3. **`TABLA_ACTION`**: Mapa de (estado, terminal) → Acción

   ```rust
   HashMap<(usize, String), Accion>
   ```

   Ejemplo:

   ```rust
   (0, "programa") → Shift(1)
   (5, ";") → Reduce(15)
   (10, "$") → Accept
   ```

4. **`TABLA_GOTO`**: Mapa de (estado, no_terminal) → Nuevo estado

   ```rust
   HashMap<(usize, String), usize>
   ```

   Ejemplo:

   ```rust
   (0, "<Programa>") → 2
   (5, "<EXPRESION>") → 18
   ```

5. **`PRODUCCIONES`**: Vector con información de todas las producciones
   ```rust
   Vec<Regla>
   ```

### `acciones_semanticas.rs`

Contiene la lógica de acciones semánticas ejecutadas durante el parsing bottom-up.

**Función principal:**
```rust
pub fn ejecutar_accion_semantica(
    cabeza: &str,
    cuerpo_len: usize,
    atributos: &[String],
    tokens: &[Token],
    contexto: &mut ContextoSemantico,
    generador: &mut GeneradorCuadruplos,
    nivel_verbose: usize,
) -> Result<String, String>
```

**Responsabilidades:**
- Ejecutar acciones semánticas al REDUCIR producciones
- Coordinar con el contexto semántico (variables, funciones, tipos)
- Invocar al generador de cuádruplos en puntos neuralágicos
- Sintetizar atributos para propagación en el parse tree

**Sistema de matching:**
Usa pattern matching sobre `(cabeza, cuerpo_len)` para identificar cada producción y ejecutar su acción específica.

## Flujo de Análisis Sintáctico

```
Tokens del Lexer
       ↓
  SLR Parser (analyze)
       ↓
   ┌───────────────┐
   │ Estado Actual │
   └───────┬───────┘
           │
     Consultar ACTION
           │
     ┌─────┴─────┬─────────┬─────────┐
     │           │         │         │
   Shift      Reduce    Accept    Error
     │           │         │         │
  Push(s)   Pop+Push   Return OK  Return Err
   next        goto
```

## Ejemplo de Parsing

**Input:** `x = 5 ;`

**Tokens:** `[Id("x"), Asignacion("="), CteEnt("5"), PuntoYComa(";")]`

| Paso | Pila Estados    | Token Actual | Acción     | Explicación             |
| ---- | --------------- | ------------ | ---------- | ----------------------- |
| 1    | [0]             | id           | Shift(15)  | Empujar estado 15       |
| 2    | [0,15]          | =            | Shift(90)  | Empujar estado 90       |
| 3    | [0,15,90]       | cte_ent      | Reduce(54) | Reducir ε → <+-\_OPT>   |
| 4    | [0,15,90,39]    | cte_ent      | Shift(51)  | Empujar estado 51       |
| 5    | [0,15,90,39,51] | ;            | Reduce(38) | Reducir cte_ent → <CTE> |
| 6    | ...             | ...          | ...        | Continuar reducciones   |
| n    | [0,2]           | $            | Accept     | ✓ Entrada válida        |

## Manejo de Errores

El parser detecta errores de sintaxis cuando:

1. **No hay entrada en ACTION**: Token inesperado en el estado actual

   ```
   Error de sintaxis en línea 5: token inesperado '}' (esperado en estado 23)
   ```

2. **No hay entrada en GOTO**: Error interno (gramática mal construida)
   ```
   Error fatal: GOTO no encontrado para estado 12 y no-terminal <EXPRESION>
   ```

**Información de error incluye:**

- Número de línea del token problemático
- Token inesperado
- Estado del parser
- (En modo verbose: pila de estados completa)

## Modo Verbose

Cuando `is_verbose = true`, el parser imprime cada paso:

```
=== Iniciando análisis sintáctico SLR ===
Total de tokens: 11

Estado: 0 | Token: 'programa' (tipo: programa) | Pila: [0]
  → Acción: Shift(1)

Estado: 1 | Token: 'test' (tipo: id) | Pila: [0, 1]
  → Acción: Shift(3)

Estado: 3 | Token: ';' (tipo: ;) | Pila: [0, 1, 3]
  → Acción: Shift(4)

...

Estado: 2 | Token: '$' (tipo: $) | Pila: [0, 2]
  → Acción: Accept

✓ Análisis sintáctico completado exitosamente
```

## Estadísticas de las Tablas

Para la gramática actual:

- **Estados del autómata**: 108
- **Transiciones**: 240
- **Entradas en ACTION**: 465
- **Entradas en GOTO**: 124
- **Producciones**: 70

## Ejemplo de Uso

```rust
use compilador_rust::lexico;
use compilador_rust::sintactico;

let codigo = "programa test; inicio { x = 5; } fin";

// Análisis léxico
let tokens = lexico::analyze(codigo, &false)?;

// Análisis sintáctico
match sintactico::analyze(&tokens, &true) {
    Ok(()) => println!("✓ Programa sintácticamente correcto"),
    Err(e) => eprintln!("✗ Error de sintaxis: {}", e),
}
```

## Regeneración de Tablas

Cuando modificas `gramatica.txt`, debes regenerar las tablas:

```bash
cargo run --bin generador_slr
```

Este proceso:

1. Lee la gramática
2. Calcula FIRST y FOLLOW
3. Construye el autómata LR(0)
4. Genera las tablas ACTION y GOTO
5. Escribe `tabla_slr.rs` con lazy_static

## Ventajas del Parsing SLR

- Determinista: Una sola acción posible en cada estado
- Eficiente: O(n) donde n = número de tokens
- Detección temprana: Errores detectados en cuanto ocurren
- Table-driven: Fácil de mantener y modificar (regenerar tablas)

## Limitaciones

- Gramáticas ambiguas: No puede parsear gramáticas con conflictos Shift/Reduce
- Precedencia: Requiere modificar la gramática para manejar precedencia de operadores
- Recuperación de errores: Actualmente solo detecta el primer error

## Referencias

- [SLR Parser - Wikipedia](https://en.wikipedia.org/wiki/Simple_LR_parser)
- Dragon Book, Capítulo 4.7: LR Parsers
- [LR Parsing Tutorial](https://web.stanford.edu/class/archive/cs/cs143/)
