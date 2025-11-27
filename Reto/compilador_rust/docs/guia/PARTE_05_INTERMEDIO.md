# PARTE 5: Generación de Código Intermedio

[← Parte 4: Semántico](PARTE_04_SEMANTICO.md) | [Volver al índice](../../GUIA_TECNICA.md) | [Siguiente: Memoria Virtual →](PARTE_06_MEMORIA.md)

## 5.1 Sistema de Cuádruplos

### Qué es un Cuádruplo

Un **cuádruplo** es una instrucción de código intermedio con formato:

```
(operador, operando_izq, operando_der, resultado)
```

**Ejemplo**:

```
a = b + c
```

Se traduce a:

```
(+, b, c, temp1)
(=, temp1, -, a)
```

### Estructura del Cuádruplo

**Archivo**: `src/intermedio/cuadruplo.rs`

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Cuadruplo {
    pub operador: OperadorCuadruplo,
    pub operando_izq: Operando,
    pub operando_der: Operando,
    pub resultado: Operando,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operando {
    Direccion(usize),    // Dirección de memoria (1000-24999)
    Letrero(usize),      // Índice en tabla de strings
    Nulo,                // Sin operando
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperadorCuadruplo {
    // Aritméticos
    Suma,           // +
    Resta,          // -
    Multiplicacion, // *
    Division,       // /

    // Relacionales
    MayorQue,       // >
    MenorQue,       // <
    Igual,          // ==
    Diferente,      // !=

    // Asignación
    Asignacion,     // =

    // E/S
    Lectura,        // lee
    Escritura,      // escribe

    // Control de flujo
    Goto,           // goto
    GotoF,          // goto falso (si condición es falsa)
    GotoV,          // goto verdadero (si condición es verdadera)

    // Funciones
    Era,            // Activation Record (reservar espacio)
    Parametro,      // Paso de parámetro
    GoSub,          // Llamada a subrutina
    EndFunc,        // Fin de función
    Return,         // Retorno de función
}
```

### Ejemplos de Cuádruplos

#### Aritmética

```paradox
x = 5 + 3 * 2;
```

Cuádruplos:

```
0: (*, 3, 2, temp1)        // temp1 = 3 * 2 = 6
1: (+, 5, temp1, temp2)    // temp2 = 5 + 6 = 11
2: (=, temp2, -, x)        // x = temp2
```

#### Condicional

```paradox
si (x > 5) entonces {
    y = 10;
} sino {
    y = 20;
}
```

Cuádruplos:

```
0: (>, x, 5, temp1)        // temp1 = (x > 5)
1: (GotoF, temp1, -, 4)    // Si false, saltar a 4
2: (=, 10, -, y)           // y = 10
3: (Goto, -, -, 5)         // Saltar a 5 (fin)
4: (=, 20, -, y)           // y = 20 (else)
5: ...                     // Continuar
```

#### Ciclo

```paradox
mientras (x < 10) haz {
    x = x + 1;
}
```

Cuádruplos:

```
0: (<, x, 10, temp1)       // temp1 = (x < 10)
1: (GotoF, temp1, -, 4)    // Si false, saltar a 4 (salir)
2: (+, x, 1, temp2)        // temp2 = x + 1
3: (=, temp2, -, x)        // x = temp2
4: (Goto, -, -, 0)         // Volver a 0 (evaluar condición)
5: ...                     // Continuar
```

## 5.2 Puntos Neurálgicos (PN)

Los **puntos neurálgicos** son acciones que se ejecutan durante el parsing para generar cuádruplos.

### PN1: Procesar Operando

**Cuándo**: Al reconocer un `id` o constante.

**Acción**:

```rust
pub fn procesar_operando(&mut self, nombre: &str) -> Result<(), String> {
    // Constante entera
    if let Ok(valor) = nombre.parse::<i32>() {
        let direccion = self.memoria_virtual.asignar_constante_entera(valor)?;
        self.pilao.push(Operando::Direccion(direccion));
        self.ptypes.push(TipoDato::Entero);
        return Ok(());
    }

    // Constante flotante
    if let Ok(valor) = nombre.parse::<f64>() {
        let direccion = self.memoria_virtual.asignar_constante_flotante(valor)?;
        self.pilao.push(Operando::Direccion(direccion));
        self.ptypes.push(TipoDato::Flotante);
        return Ok(());
    }

    // Variable: buscar en contexto
    let tipo = self.contexto.obtener_tipo_variable(nombre)?;
    let direccion = self.contexto.obtener_direccion_variable(nombre)?;

    self.pilao.push(Operando::Direccion(direccion));
    self.ptypes.push(tipo);

    Ok(())
}
```

### PN2: Procesar Operador

**Cuándo**: Al reconocer un operador (`+`, `-`, `*`, `/`, `>`, `<`, etc.).

**Acción**:

```rust
pub fn procesar_operador(&mut self, operador: OperadorCuadruplo) {
    self.poper.push(operador);
}
```

### PN3: Generar Operación

**Cuándo**: Al completar una expresión con operador.

**Acción**:

```rust
pub fn generar_operacion(&mut self) -> Result<(), String> {
    if self.poper.is_empty() {
        return Ok(()); // No hay operador pendiente
    }

    let operador = self.poper.pop().unwrap();
    let operando_der = self.pilao.pop().ok_or("Pila de operandos vacía")?;
    let operando_izq = self.pilao.pop().ok_or("Pila de operandos vacía")?;
    let tipo_der = self.ptypes.pop().ok_or("Pila de tipos vacía")?;
    let tipo_izq = self.ptypes.pop().ok_or("Pila de tipos vacía")?;

    // Validar tipos con cubo semántico
    let tipo_resultado = self.cubo_semantico.validar(tipo_izq, operador, tipo_der)?;

    // Asignar temporal para resultado
    let direccion_temporal = self.memoria_virtual.asignar_variable(
        tipo_resultado,
        TipoSegmento::Temporal
    )?;

    // Generar cuádruplo
    let cuad = Cuadruplo {
        operador,
        operando_izq,
        operando_der,
        resultado: Operando::Direccion(direccion_temporal),
    };

    self.quad.push_back(cuad);

    // Resultado queda en la pila para siguientes operaciones
    self.pilao.push(Operando::Direccion(direccion_temporal));
    self.ptypes.push(tipo_resultado);

    Ok(())
}
```

### PN4: Generar Asignación

**Cuándo**: Al completar `id = expresion;`.

**Acción**:

```rust
pub fn generar_asignacion(&mut self, nombre_variable: &str) -> Result<(), String> {
    // Obtener resultado de la expresión
    let operando_fuente = self.pilao.pop().ok_or("Pila vacía")?;
    let tipo_fuente = self.ptypes.pop().ok_or("Pila vacía")?;

    // Obtener tipo y dirección de la variable destino
    let tipo_destino = self.contexto.obtener_tipo_variable(nombre_variable)?;
    let direccion_destino = self.contexto.obtener_direccion_variable(nombre_variable)?;

    // Validar compatibilidad de tipos (asignación)
    self.cubo_semantico.validar(
        tipo_destino,
        Operador::Asignacion,
        tipo_fuente
    )?;

    // Generar cuádruplo de asignación
    let cuad = Cuadruplo {
        operador: OperadorCuadruplo::Asignacion,
        operando_izq: operando_fuente,
        operando_der: Operando::Nulo,
        resultado: Operando::Direccion(direccion_destino),
    };

    self.quad.push_back(cuad);

    Ok(())
}
```

### PN5: Iniciar Condición

**Cuándo**: Al detectar `si (`.

**Acción**:

```rust
pub fn iniciar_condicion(&mut self) {
    self.en_condicion += 1;
}
```

### PN6: GotoF de Condición

**Cuándo**: Al terminar de evaluar la expresión booleana: `si (exp) entonces`.

**Acción**:

```rust
pub fn generar_gotof(&mut self) -> Result<(), String> {
    let resultado_exp = self.pilao.pop().ok_or("Pila vacía")?;
    let tipo_exp = self.ptypes.pop().ok_or("Pila vacía")?;

    // Verificar que sea expresión booleana (entero en nuestro caso)
    if tipo_exp != TipoDato::Entero {
        return Err("Condición debe ser de tipo entero (booleano)".to_string());
    }

    // Generar GotoF con destino pendiente
    let cuad = Cuadruplo {
        operador: OperadorCuadruplo::GotoF,
        operando_izq: resultado_exp,
        operando_der: Operando::Nulo,
        resultado: Operando::Direccion(9999), // Placeholder
    };

    let direccion_cuad = self.quad.len();
    self.quad.push_back(cuad);

    // Guardar dirección para hacer FILL después
    self.pjumps.push(direccion_cuad);

    self.en_condicion -= 1;

    Ok(())
}
```

### PN7: FILL de GOTO (Final de If)

**Cuándo**: Al terminar el cuerpo del `entonces` (antes del `sino` o `;`).

**Acción**:

```rust
pub fn fill_goto_if(&mut self) -> Result<(), String> {
    let dir_gotof = self.pjumps.pop().ok_or("Pila de saltos vacía")?;

    // Hacer FILL: actualizar destino del GotoF
    let destino = self.quad.len(); // Dirección actual

    if let Some(cuad) = self.quad.get_mut(dir_gotof) {
        cuad.resultado = Operando::Direccion(destino);
    } else {
        return Err("Error: dirección de FILL inválida".to_string());
    }

    Ok(())
}
```

### PN8: Else

**Cuándo**: Al detectar `sino`.

**Acción**:

```rust
pub fn generar_else(&mut self) -> Result<(), String> {
    // Generar Goto para saltar el else (desde el then)
    let cuad = Cuadruplo {
        operador: OperadorCuadruplo::Goto,
        operando_izq: Operando::Nulo,
        operando_der: Operando::Nulo,
        resultado: Operando::Direccion(9999), // Placeholder
    };

    let dir_goto = self.quad.len();
    self.quad.push_back(cuad);

    // Hacer FILL del GotoF (ahora apunta al else)
    let dir_gotof = self.pjumps.pop().ok_or("Pila vacía")?;
    let destino_else = self.quad.len();

    if let Some(cuad) = self.quad.get_mut(dir_gotof) {
        cuad.resultado = Operando::Direccion(destino_else);
    }

    // Guardar Goto para hacer FILL al final del else
    self.pjumps.push(dir_goto);

    Ok(())
}
```

### PN9: Iniciar Ciclo

**Cuándo**: Al detectar `mientras (`.

**Acción**:

```rust
pub fn iniciar_ciclo(&mut self) {
    // Guardar dirección de inicio del ciclo
    let dir_inicio = self.quad.len();
    self.pjumps.push(dir_inicio);
    self.en_condicion += 1;
}
```

### PN10: Finalizar Ciclo

**Cuándo**: Al terminar el cuerpo del `mientras`.

**Acción**:

```rust
pub fn finalizar_ciclo(&mut self) -> Result<(), String> {
    // Generar Goto de regreso al inicio
    let dir_inicio = self.pjumps.pop().ok_or("Pila vacía")?;

    let cuad_goto = Cuadruplo {
        operador: OperadorCuadruplo::Goto,
        operando_izq: Operando::Nulo,
        operando_der: Operando::Nulo,
        resultado: Operando::Direccion(dir_inicio),
    };

    self.quad.push_back(cuad_goto);

    // Hacer FILL del GotoF (para salir del ciclo)
    let dir_gotof = self.pjumps.pop().ok_or("Pila vacía")?;
    let destino_salida = self.quad.len();

    if let Some(cuad) = self.quad.get_mut(dir_gotof) {
        cuad.resultado = Operando::Direccion(destino_salida);
    }

    Ok(())
}
```

## 5.3 Pilas de Traducción

### POper: Pila de Operadores

Almacena operadores pendientes durante el parsing de expresiones.

```rust
poper: Vec<OperadorCuadruplo>
```

**Uso**:

```rust
self.poper.push(OperadorCuadruplo::Suma);
let op = self.poper.pop().unwrap();
```

### PilaO: Pila de Operandos

Almacena operandos (direcciones de memoria) durante el parsing.

```rust
pilao: Vec<Operando>
```

**Uso**:

```rust
self.pilao.push(Operando::Direccion(1000));
let operando = self.pilao.pop().unwrap();
```

### PTypes: Pila de Tipos

Almacena tipos de operandos para validación semántica.

```rust
ptypes: Vec<TipoDato>
```

**Uso**:

```rust
self.ptypes.push(TipoDato::Entero);
let tipo = self.ptypes.pop().unwrap();
```

### PJumps: Pila de Saltos

Almacena direcciones de cuádruplos con saltos pendientes (para hacer FILL).

```rust
pjumps: Vec<usize>
```

**Uso**:

```rust
self.pjumps.push(15); // Dirección del GotoF
let dir = self.pjumps.pop().unwrap();
```

## 5.4 Agregar Nuevos Tipos de Cuádruplos

### Ejemplo: Agregar `AND` y `OR` lógicos

**Paso 1**: Agregar operadores

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperadorCuadruplo {
    // ... existentes ...
    And,  // ← AGREGAR
    Or,   // ← AGREGAR
}
```

**Paso 2**: Agregar conversión

```rust
impl OperadorCuadruplo {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            // ...
            "&&" => Some(OperadorCuadruplo::And),  // ← AGREGAR
            "||" => Some(OperadorCuadruplo::Or),   // ← AGREGAR
            _ => None,
        }
    }
}
```

**Paso 3**: Agregar Display

```rust
impl fmt::Display for OperadorCuadruplo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // ...
            OperadorCuadruplo::And => write!(f, "&&"),
            OperadorCuadruplo::Or => write!(f, "||"),
            // ...
        }
    }
}
```

**Paso 4**: Agregar al cubo semántico

```rust
// En cubo_semantico.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operador {
    // ...
    And,
    Or,
}

// En inicializar_reglas:
self.agregar_regla(TipoDato::Entero, Operador::And, TipoDato::Entero, TipoDato::Entero);
self.agregar_regla(TipoDato::Entero, Operador::Or, TipoDato::Entero, TipoDato::Entero);
```

**Paso 5**: Implementar en VM

```rust
// En vm/ejecutor.rs
match cuad.operador {
    // ...
    OperadorCuadruplo::And => {
        let val1 = self.leer_operando(&cuad.operando_izq)?;
        let val2 = self.leer_operando(&cuad.operando_der)?;
        let resultado = Valor::Entero(
            if val1.a_entero() != 0 && val2.a_entero() != 0 { 1 } else { 0 }
        );
        self.escribir_memoria(dir_resultado, resultado)?;
    }
    OperadorCuadruplo::Or => {
        let val1 = self.leer_operando(&cuad.operando_izq)?;
        let val2 = self.leer_operando(&cuad.operando_der)?;
        let resultado = Valor::Entero(
            if val1.a_entero() != 0 || val2.a_entero() != 0 { 1 } else { 0 }
        );
        self.escribir_memoria(dir_resultado, resultado)?;
    }
}
```

## 5.5 Debugging de Cuádruplos

### Imprimir Lista de Cuádruplos

```rust
pub fn imprimir_cuadruplos(&self) {
    println!("\n╔══════════════════════════════════════════════════╗");
    println!("║           CUÁDRUPLOS GENERADOS                   ║");
    println!("╚══════════════════════════════════════════════════╝\n");

    for (i, cuad) in self.quad.iter().enumerate() {
        println!("{:>3}: {} {} {} {}",
                 i,
                 cuad.operador,
                 Self::operando_str(&cuad.operando_izq),
                 Self::operando_str(&cuad.operando_der),
                 Self::operando_str(&cuad.resultado));
    }
}

fn operando_str(op: &Operando) -> String {
    match op {
        Operando::Direccion(dir) => format!("@{}", dir),
        Operando::Letrero(idx) => format!("STR[{}]", idx),
        Operando::Nulo => "-".to_string(),
    }
}
```

**Salida ejemplo**:

```
╔══════════════════════════════════════════════════╗
║           CUÁDRUPLOS GENERADOS                   ║
╚══════════════════════════════════════════════════╝

  0: Goto - - @10
  1: = @19000 - @7000
  2: = @19001 - @7001
  3: + @7000 @7001 @13000
  4: = @13000 - @1000
  5: EndFunc - - -
  6: = @19002 - @1001
  7: = @19003 - @1002
  8: GoSub - - @1
  9: Escritura - - @1000
 10: ...
```

### Verbose durante Generación

```rust
pub fn generar_operacion(&mut self) -> Result<(), String> {
    // ...

    let cuad = Cuadruplo { /* ... */ };

    if verbose >= 2 {
        println!("[CUAD #{}] Generado: {:?}", self.quad.len(), cuad);
    }

    self.quad.push_back(cuad);

    Ok(())
}
```

---

## Resumen

En esta parte cubrimos:

- Sistema de cuádruplos (estructura y operadores)
- Puntos neurálgicos (PN1-PN10)
- Pilas de traducción (POper, PilaO, PTypes, PJumps)
- Agregar nuevos tipos de cuádruplos
- Debugging de cuádruplos

**Siguiente**: [PARTE 6: Memoria Virtual →](PARTE_06_MEMORIA.md)

[← Parte 4](PARTE_04_SEMANTICO.md) | [Volver al índice](../../GUIA_TECNICA.md)
