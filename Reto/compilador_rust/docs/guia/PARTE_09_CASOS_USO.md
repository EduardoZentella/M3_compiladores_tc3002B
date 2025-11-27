# PARTE 9: Casos de Uso Prácticos

[← Parte 8: Funciones](PARTE_08_FUNCIONES.md) | [Volver al índice](../../GUIA_TECNICA.md) | [Siguiente: Testing →](PARTE_10_TESTING.md)

## 9.1 Cómo Agregar un Nuevo Operador

### Ejemplo Completo: Agregar Operador Módulo `%`

Este es el proceso **completo** paso a paso para agregar el operador módulo que calcula el residuo de una división.

#### Paso 1: Token Léxico

**Archivo**: `src/lexico/token.rs`

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum TipoToken {
    // ... existentes ...
    Multiplicacion,
    Division,
    Modulo,  // ← AGREGAR
    // ...
}
```

**Archivo**: `src/lexico/mod.rs`

```rust
fn reconocer_operador(&mut self) -> Result<Option<Token>, String> {
    // ...
    let tipo = match ch {
        '*' => Some(TipoToken::Multiplicacion),
        '/' => Some(TipoToken::Division),
        '%' => Some(TipoToken::Modulo),  // ← AGREGAR
        // ...
    };
    // ...
}
```

#### Paso 2: Gramática

**Archivo**: `gramatica.txt`

Buscar la producción `<*/>`:

```
<*/> → *
<*/> → /
<*/> → %
```

**Regenerar tablas**:

```bash
cargo run --bin generador_slr
```

Esto actualiza `src/sintactico/tabla_slr.rs` automáticamente.

#### Paso 3: Conversión Token → Símbolo

**Archivo**: `src/sintactico/mod.rs`

Buscar función `token_a_simbolo`:

```rust
fn token_a_simbolo(token: &Token) -> String {
    match &token.tipo {
        // ...
        TipoToken::Multiplicacion => "*".to_string(),
        TipoToken::Division => "/".to_string(),
        TipoToken::Modulo => "%".to_string(),  // ← AGREGAR
        // ...
    }
}
```

#### Paso 4: Cubo Semántico

**Archivo**: `src/semantico/cubo_semantico.rs`

Agregar el operador al enum:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operador {
    Suma,
    Resta,
    Multiplicacion,
    Division,
    Modulo,  // ← AGREGAR
    // ...
}
```

Agregar reglas en la tabla:

```rust
fn inicializar_tabla() -> HashMap<(TipoDato, Operador, TipoDato), TipoDato> {
    let mut tabla = HashMap::new();

    // ... reglas existentes ...

    // Módulo
    tabla.insert((TipoDato::Entero, Operador::Modulo, TipoDato::Entero), TipoDato::Entero);
    // Nota: Módulo típicamente solo se define para enteros

    tabla
}
```

#### Paso 5: Operador de Cuádruplo

**Archivo**: `src/intermedio/cuadruplo.rs`

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum OperadorCuadruplo {
    Suma,
    Resta,
    Multiplicacion,
    Division,
    Modulo,  // ← AGREGAR
    // ...
}
```

Actualizar Display si existe:

```rust
impl std::fmt::Display for OperadorCuadruplo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // ...
            OperadorCuadruplo::Modulo => write!(f, "%"),
            // ...
        }
    }
}
```

#### Paso 6: Generador de Cuádruplos

**Archivo**: `src/intermedio/generador.rs`

En `generar_operaciones`, agregar conversión:

```rust
fn convertir_operador_cuadruplo(&self, op_token: &TipoToken) -> OperadorCuadruplo {
    match op_token {
        TipoToken::Suma => OperadorCuadruplo::Suma,
        TipoToken::Resta => OperadorCuadruplo::Resta,
        TipoToken::Multiplicacion => OperadorCuadruplo::Multiplicacion,
        TipoToken::Division => OperadorCuadruplo::Division,
        TipoToken::Modulo => OperadorCuadruplo::Modulo,  // ← AGREGAR
        // ...
    }
}
```

#### Paso 7: Máquina Virtual

**Archivo**: `src/vm/ejecutor.rs`

En `ejecutar_aritmetica`:

```rust
fn ejecutar_aritmetica(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
    let valor1 = self.leer_operando(&cuad.operando_izq)?;
    let valor2 = self.leer_operando(&cuad.operando_der)?;
    let resultado_dir = self.extraer_direccion(&cuad.resultado)?;

    let resultado = match cuad.operador {
        OperadorCuadruplo::Suma => valor1.operar_aritmetica(&valor2, |a, b| a + b),
        OperadorCuadruplo::Resta => valor1.operar_aritmetica(&valor2, |a, b| a - b),
        OperadorCuadruplo::Multiplicacion => valor1.operar_aritmetica(&valor2, |a, b| a * b),
        OperadorCuadruplo::Division => {
            if valor2.a_flotante() == 0.0 {
                return Err("Error: División por cero".to_string());
            }
            valor1.operar_aritmetica(&valor2, |a, b| a / b)
        },
        OperadorCuadruplo::Modulo => {  // ← AGREGAR
            if valor2.a_entero() == 0 {
                return Err("Error: Módulo por cero".to_string());
            }
            // Módulo solo para enteros
            Valor::Entero(valor1.a_entero() % valor2.a_entero())
        },
        _ => return Err(format!("Operador no aritmético: {:?}", cuad.operador)),
    };

    self.escribir_memoria(resultado_dir, resultado)?;
    Ok(())
}
```

#### Paso 8: Testing

**Crear**: `tests/programas/test_modulo.txt`

```paradox
programa test_modulo;

vars a, b, resultado : entero;

inicio {
    a = 10;
    b = 3;
    resultado = a % b;
    escribe(resultado);
}
fin
```

**Ejecutar**:

```bash
cargo run --bin compilador_rust -- tests/programas/test_modulo.txt
```

**Salida esperada**: `1` (10 % 3 = 1)

### Resumen del Proceso

| Paso | Archivo                       | Acción                          |
| ---- | ----------------------------- | ------------------------------- |
| 1    | `lexico/token.rs`             | Agregar enum variant            |
| 2    | `lexico/mod.rs`               | Reconocer símbolo `%`           |
| 3    | `gramatica.txt`               | Agregar a producción            |
| 4    | Regenerar tablas              | `cargo run --bin generador_slr` |
| 5    | `sintactico/mod.rs`           | Conversión token→símbolo        |
| 6    | `semantico/cubo_semantico.rs` | Reglas de tipos                 |
| 7    | `intermedio/cuadruplo.rs`     | Enum de operador                |
| 8    | `intermedio/generador.rs`     | Generación de cuádruplo         |
| 9    | `vm/ejecutor.rs`              | Implementar ejecución           |
| 10   | Testing                       | Crear y ejecutar test           |

## 9.2 Cómo Agregar un Nuevo Tipo de Dato

### Ejemplo: Agregar Tipo `booleano`

#### Paso 1: Definir el Tipo

**Archivo**: `src/semantico/tipos.rs` (o donde esté TipoDato)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TipoDato {
    Entero,
    Flotante,
    Char,
    Letrero,
    Booleano,  // ← AGREGAR
    Nula,
}
```

#### Paso 2: Token y Gramática

**Token** (`src/lexico/token.rs`):

```rust
pub enum TipoToken {
    // ...
    Entero,
    Flotante,
    Booleano,  // ← AGREGAR
    // ...
}
```

**Léxico** (`src/lexico/mod.rs`):

```rust
let tipo_token = match texto {
    // ...
    "entero" => TipoToken::Entero,
    "flotante" => TipoToken::Flotante,
    "booleano" => TipoToken::Booleano,  // ← AGREGAR
    // ...
};
```

**Gramática**:

```
<TIPO> → entero
<TIPO> → flotante
<TIPO> → booleano
```

Regenerar tablas.

#### Paso 3: Memoria Virtual

**Archivo**: `src/intermedio/memoria_virtual.rs`

Agregar rangos de memoria para booleano:

```rust
// Constantes de rangos
const GLOBAL_ENTERO_INICIO: usize = 1000;
const GLOBAL_ENTERO_FIN: usize = 1999;
const GLOBAL_FLOTANTE_INICIO: usize = 3000;
const GLOBAL_FLOTANTE_FIN: usize = 3999;
const GLOBAL_CHAR_INICIO: usize = 5000;
const GLOBAL_CHAR_FIN: usize = 5999;
const GLOBAL_BOOLEANO_INICIO: usize = 6000;  // ← AGREGAR
const GLOBAL_BOOLEANO_FIN: usize = 6999;     // ← AGREGAR

// Repetir para LOCAL_, TEMPORAL_, CONSTANTE_
```

Agregar contadores y pools:

```rust
pub struct MemoriaVirtual {
    global_entero: usize,
    global_flotante: usize,
    global_char: usize,
    global_booleano: usize,  // ← AGREGAR

    // ... mismo para local, temporal, constante ...

    temporales_disponibles_booleano: HashSet<usize>,  // ← AGREGAR
    tabla_constantes_booleano: HashMap<bool, usize>,  // ← AGREGAR
}
```

Actualizar `asignar_variable`:

```rust
pub fn asignar_variable(&mut self, tipo: TipoDato, segmento: TipoSegmento)
    -> Result<usize, String> {
    // ...
    match (segmento, tipo) {
        // ... casos existentes ...
        (TipoSegmento::Global, TipoDato::Booleano) => {  // ← AGREGAR
            if self.global_booleano > GLOBAL_BOOLEANO_FIN {
                return Err("Desbordamiento de memoria global para booleanos".to_string());
            }
            let dir = self.global_booleano;
            self.global_booleano += 1;
            Ok(dir)
        }
        // ... mismo para Local y Temporal ...
    }
}
```

#### Paso 4: Cubo Semántico

Agregar reglas para booleano:

```rust
// Operadores lógicos AND, OR, NOT
tabla.insert((TipoDato::Booleano, Operador::And, TipoDato::Booleano), TipoDato::Booleano);
tabla.insert((TipoDato::Booleano, Operador::Or, TipoDato::Booleano), TipoDato::Booleano);

// Comparaciones retornan booleano
tabla.insert((TipoDato::Entero, Operador::Mayor, TipoDato::Entero), TipoDato::Booleano);
// etc.
```

#### Paso 5: VM

**Archivo**: `src/vm/memoria.rs`

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Valor {
    Entero(i32),
    Flotante(f64),
    Letrero(usize),
    Booleano(bool),  // ← AGREGAR
}
```

Actualizar métodos:

```rust
impl Valor {
    pub fn a_bool(&self) -> bool {
        match self {
            Valor::Booleano(b) => *b,
            Valor::Entero(i) => *i != 0,
            Valor::Flotante(f) => *f != 0.0,
            _ => false,
        }
    }
}
```

## 9.3 Cómo Agregar una Estructura de Control

### Ejemplo: Agregar `for`

#### Paso 1: Gramática

```
<CICLO> → for id = <EXPRESIÓN> to <EXPRESIÓN> haz <CUERPO>
```

Regenerar tablas.

#### Paso 2: Punto Neurálgico

**Archivo**: `src/sintactico/acciones_semanticas.rs`

```rust
pub fn ejecutar_accion(&mut self, produccion: &str, ...) {
    match produccion {
        // ...
        "<CICLO> → for id = <EXPRESIÓN> to <EXPRESIÓN> haz <CUERPO>" => {
            self.generar_for()?;
        }
        // ...
    }
}
```

#### Paso 3: Generador

**Archivo**: `src/intermedio/generador.rs`

```rust
pub fn generar_for(&mut self) -> Result<(), String> {
    // PilaO tiene: [valor_inicial, valor_final]
    let valor_final = self.pilao.pop().ok_or("Falta valor final")?;
    let valor_inicial = self.pilao.pop().ok_or("Falta valor inicial")?;

    // Variable de control ya fue asignada
    // Generar:
    // 1. Asignar valor inicial
    // 2. Salvar dirección de inicio del loop
    // 3. Comparar variable con valor final
    // 4. GotoF al final si variable > final
    // 5. Cuerpo del loop
    // 6. Incrementar variable
    // 7. Goto al inicio
    // 8. Label de salida

    // Implementación...
    Ok(())
}
```

## 9.4 Cómo Modificar el Cubo Semántico

### Cambiar Regla: Permitir `entero = flotante` (truncamiento)

**Archivo**: `src/semantico/cubo_semantico.rs`

Actualmente rechazado. Para permitirlo:

```rust
pub fn validar_asignacion(&self, tipo_destino: TipoDato, tipo_fuente: TipoDato)
    -> Result<(), String> {
    match (tipo_destino, tipo_fuente) {
        // Mismo tipo: OK
        (a, b) if a == b => Ok(()),

        // Promoción entero → flotante: OK
        (TipoDato::Flotante, TipoDato::Entero) => Ok(()),

        // AGREGAR: Truncamiento flotante → entero con warning
        (TipoDato::Entero, TipoDato::Flotante) => {
            eprintln!("Warning: Truncando flotante a entero");
            Ok(())
        },

        // Resto: Error
        _ => Err(format!(
            "No se puede asignar {:?} a variable de tipo {:?}",
            tipo_fuente, tipo_destino
        )),
    }
}
```

## 9.5 Cómo Cambiar Rangos de Memoria

### Ejemplo: Expandir memoria global de enteros

**Archivo**: `src/intermedio/memoria_virtual.rs`

```rust
// ANTES
const GLOBAL_ENTERO_INICIO: usize = 1000;
const GLOBAL_ENTERO_FIN: usize = 1999;  // 1000 variables

// DESPUÉS
const GLOBAL_ENTERO_INICIO: usize = 1000;
const GLOBAL_ENTERO_FIN: usize = 2999;  // 2000 variables
```

**Archivo**: `src/vm/memoria.rs`

Actualizar las mismas constantes:

```rust
const GLOBAL_ENTERO_INICIO: usize = 1000;
const GLOBAL_ENTERO_FIN: usize = 2999;
```

**Importante**: Los rangos en `memoria_virtual.rs` y `vm/memoria.rs` **deben coincidir**.

### Reorganizar Completamente

Si quieres cambiar el esquema completo:

1. Decidir nuevo layout (ej: Global 0-9999, Local 10000-19999, etc.)
2. Actualizar TODAS las constantes en ambos archivos
3. Actualizar documentación
4. Ejecutar todos los tests para verificar

## 9.6 Cómo Optimizar Generación de Cuádruplos

### Ejemplo: Constant Folding (Plegado de Constantes)

Si detectas `3 + 5`, en lugar de generar cuádruplo, calcular directamente `8`.

**Archivo**: `src/intermedio/generador.rs`

```rust
pub fn generar_operaciones(&mut self, operador: &TipoToken) -> Result<(), String> {
    let operando_der = self.pilao.pop().ok_or("...")?;
    let operando_izq = self.pilao.pop().ok_or("...")?;
    let tipo_der = self.ptypes.pop().ok_or("...")?;
    let tipo_izq = self.ptypes.pop().ok_or("...")?;

    // OPTIMIZACIÓN: Constant folding
    if let (Operando::Direccion(dir_izq), Operando::Direccion(dir_der)) =
        (&operando_izq, &operando_der) {
        // Verificar si ambas direcciones son de constantes
        if self.es_constante(*dir_izq) && self.es_constante(*dir_der) {
            let val_izq = self.obtener_valor_constante(*dir_izq)?;
            let val_der = self.obtener_valor_constante(*dir_der)?;

            // Calcular resultado
            let resultado = match operador {
                TipoToken::Suma => val_izq + val_der,
                TipoToken::Resta => val_izq - val_der,
                // ...
            };

            // Guardar como nueva constante
            let dir_resultado = self.memoria_virtual.asignar_constante_entera(resultado)?;
            self.pilao.push(Operando::Direccion(dir_resultado));
            self.ptypes.push(tipo_resultado);

            return Ok(()); // ← No genera cuádruplo
        }
    }

    // Caso normal: generar cuádruplo
    // ...
}
```

---

## Resumen

En esta parte cubrimos:

- Agregar operador completo (10 pasos)
- Agregar tipo de dato (5 componentes)
- Agregar estructura de control
- Modificar cubo semántico
- Cambiar rangos de memoria
- Optimizaciones básicas

**Siguiente**: [PARTE 10: Testing y Debugging →](PARTE_10_TESTING.md)

[← Parte 8](PARTE_08_FUNCIONES.md) | [Volver al índice](../../GUIA_TECNICA.md)
