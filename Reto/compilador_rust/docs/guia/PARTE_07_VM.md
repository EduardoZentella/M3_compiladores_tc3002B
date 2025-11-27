# PARTE 7: Máquina Virtual

[← Parte 6: Memoria Virtual](PARTE_06_MEMORIA.md) | [Volver al índice](../../GUIA_TECNICA.md) | [Siguiente: Funciones →](PARTE_08_FUNCIONES.md)

## 7.1 Ejecutor de Cuádruplos

### Estructura de la Máquina Virtual

**Archivo**: `src/vm/ejecutor.rs`

```rust
pub struct MaquinaVirtual {
    /// Cuádruplos a ejecutar
    cuadruplos: Vec<Cuadruplo>,

    /// Instruction Pointer (IP) - dirección del cuádruplo actual
    ip: usize,

    /// Memoria global (permanente durante toda la ejecución)
    memoria_global: SegmentoMemoria,

    /// Memoria de constantes (permanente)
    memoria_constantes: SegmentoMemoria,

    /// Pila de marcos de memoria (stack frames para funciones)
    pila_marcos: Vec<MarcoMemoria>,

    /// Tabla de funciones (nombre → InfoFuncion)
    tabla_funciones: HashMap<String, InfoFuncion>,

    /// Marco temporal para parámetros (antes de GoSub)
    marco_temporal: Option<MarcoMemoria>,

    /// Flag de ejecución
    ejecutando: bool,

    /// Sistema de E/S
    io: Box<dyn SistemaIO>,

    /// Tabla de strings literales
    tabla_strings: Vec<String>,
}
```

### Ciclo de Ejecución

```rust
pub fn ejecutar(&mut self) -> Result<(), String> {
    // Crear marco inicial para el programa principal
    self.pila_marcos.push(MarcoMemoria::new("main".to_string(), 0));

    while self.ejecutando && self.ip < self.cuadruplos.len() {
        let cuadruplo = &self.cuadruplos[self.ip].clone();
        self.ejecutar_cuadruplo(cuadruplo)?;
        self.ip += 1;
    }

    Ok(())
}
```

**Flujo**:

```
┌──────────────────────────────┐
│  1. Crear marco principal    │
├──────────────────────────────┤
│  2. Mientras IP < total:     │
│     a) Leer cuádruplo[IP]    │
│     b) Ejecutar cuádruplo    │
│     c) IP++                  │
├──────────────────────────────┤
│  3. Finalizar                │
└──────────────────────────────┘
```

## 7.2 Segmentos de Memoria en Ejecución

### SegmentoMemoria

**Archivo**: `src/vm/memoria.rs`

```rust
#[derive(Debug, Clone)]
pub struct SegmentoMemoria {
    enteros: HashMap<usize, i32>,
    flotantes: HashMap<usize, f64>,
}

impl SegmentoMemoria {
    pub fn leer_entero(&self, offset: usize) -> Result<i32, String> {
        self.enteros.get(&offset).copied()
            .ok_or_else(|| format!("Dirección {} no inicializada", offset))
    }

    pub fn escribir_entero(&mut self, offset: usize, valor: i32) {
        self.enteros.insert(offset, valor);
    }

    pub fn leer_flotante(&self, offset: usize) -> Result<f64, String> {
        self.flotantes.get(&offset).copied()
            .ok_or_else(|| format!("Dirección {} no inicializada", offset))
    }

    pub fn escribir_flotante(&mut self, offset: usize, valor: f64) {
        self.flotantes.insert(offset, valor);
    }
}
```

### Valores en la VM

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Valor {
    Entero(i32),
    Flotante(f64),
    Letrero(usize),  // Índice en tabla de strings
}

impl Valor {
    pub fn a_entero(&self) -> i32 {
        match self {
            Valor::Entero(v) => *v,
            Valor::Flotante(v) => *v as i32,
            Valor::Letrero(_) => 0,
        }
    }

    pub fn a_flotante(&self) -> f64 {
        match self {
            Valor::Entero(v) => *v as f64,
            Valor::Flotante(v) => *v,
            Valor::Letrero(_) => 0.0,
        }
    }

    pub fn operar_aritmetica<F>(&self, otro: &Valor, op: F) -> Valor
    where
        F: Fn(f64, f64) -> f64,
    {
        let resultado = op(self.a_flotante(), otro.a_flotante());

        // Si ambos son enteros, devolver entero
        if matches!(self, Valor::Entero(_)) && matches!(otro, Valor::Entero(_)) {
            Valor::Entero(resultado as i32)
        } else {
            Valor::Flotante(resultado)
        }
    }

    pub fn operar_relacional<F>(&self, otro: &Valor, op: F) -> Valor
    where
        F: Fn(f64, f64) -> bool,
    {
        let resultado = op(self.a_flotante(), otro.a_flotante());
        Valor::Entero(if resultado { 1 } else { 0 })
    }
}
```

### Traducir Direcciones

La VM necesita determinar qué segmento usar para cada dirección:

```rust
pub enum TipoSegmento {
    Global,
    Local,
    Temporal,
    Constante,
}

pub fn traducir_direccion(direccion: usize) -> (TipoSegmento, usize) {
    match direccion {
        GLOBAL_INICIO..=GLOBAL_FIN => {
            (TipoSegmento::Global, direccion - GLOBAL_INICIO)
        },
        LOCAL_INICIO..=LOCAL_FIN => {
            (TipoSegmento::Local, direccion - LOCAL_INICIO)
        },
        TEMPORAL_INICIO..=TEMPORAL_FIN => {
            (TipoSegmento::Temporal, direccion - TEMPORAL_INICIO)
        },
        CONSTANTE_INICIO..=CONSTANTE_FIN => {
            (TipoSegmento::Constante, direccion - CONSTANTE_INICIO)
        },
        _ => panic!("Dirección inválida: {}", direccion),
    }
}
```

## 7.3 Stack de Marcos para Funciones

### MarcoMemoria

Cada función tiene su propio **marco de memoria** con:

- Memoria local
- Memoria temporal
- Dirección de retorno (IP)

```rust
#[derive(Debug, Clone)]
pub struct MarcoMemoria {
    /// Nombre de la función (para debugging)
    nombre_funcion: String,

    /// Dirección de retorno (IP donde continuar después de EndFunc)
    direccion_retorno: usize,

    /// Memoria local de la función
    memoria_local: SegmentoMemoria,

    /// Memoria temporal de la función
    memoria_temporal: SegmentoMemoria,

    /// Dirección donde guardar el valor de retorno (si aplica)
    direccion_resultado: Option<usize>,
}

impl MarcoMemoria {
    pub fn new(nombre: String, dir_retorno: usize) -> Self {
        MarcoMemoria {
            nombre_funcion: nombre,
            direccion_retorno: dir_retorno,
            memoria_local: SegmentoMemoria::new(),
            memoria_temporal: SegmentoMemoria::new(),
            direccion_resultado: None,
        }
    }
}
```

### Pila de Marcos

```
┌────────────────────────────────┐
│  Marco: main                   │  ← Tope (función actual)
│  ├─ Local: {...}               │
│  ├─ Temporal: {...}            │
│  └─ Ret: 0                     │
├────────────────────────────────┤
│  Marco: calcular               │
│  ├─ Local: {a, b, resultado}   │
│  ├─ Temporal: {temp1, temp2}   │
│  └─ Ret: 25                    │
├────────────────────────────────┤
│  Marco: main (programa)        │
│  ├─ Local: (vacío)             │
│  └─ Ret: 0                     │
└────────────────────────────────┘
```

**Operaciones**:

```rust
// Empujar marco (al llamar función)
self.pila_marcos.push(marco);

// Obtener marco actual
let marco_actual = self.pila_marcos.last_mut().ok_or("Sin marco")?;

// Desempilar marco (al retornar)
let marco = self.pila_marcos.pop().ok_or("Pila vacía")?;
```

## 7.4 Implementación de Operaciones

### Operaciones Aritméticas

```rust
fn ejecutar_aritmetica(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
    let valor1 = self.leer_operando(&cuad.operando_izq)?;
    let valor2 = self.leer_operando(&cuad.operando_der)?;
    let dir_resultado = self.extraer_direccion(&cuad.resultado)?;

    let resultado = match cuad.operador {
        OperadorCuadruplo::Suma => {
            valor1.operar_aritmetica(&valor2, |a, b| a + b)
        },
        OperadorCuadruplo::Resta => {
            valor1.operar_aritmetica(&valor2, |a, b| a - b)
        },
        OperadorCuadruplo::Multiplicacion => {
            valor1.operar_aritmetica(&valor2, |a, b| a * b)
        },
        OperadorCuadruplo::Division => {
            if valor2.a_flotante() == 0.0 {
                return Err("Error: División por cero".to_string());
            }
            valor1.operar_aritmetica(&valor2, |a, b| a / b)
        },
        _ => unreachable!(),
    };

    self.escribir_memoria(dir_resultado, resultado)?;

    Ok(())
}
```

### Operaciones Relacionales

```rust
fn ejecutar_relacional(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
    let valor1 = self.leer_operando(&cuad.operando_izq)?;
    let valor2 = self.leer_operando(&cuad.operando_der)?;
    let dir_resultado = self.extraer_direccion(&cuad.resultado)?;

    let resultado = match cuad.operador {
        OperadorCuadruplo::MayorQue => {
            valor1.operar_relacional(&valor2, |a, b| a > b)
        },
        OperadorCuadruplo::MenorQue => {
            valor1.operar_relacional(&valor2, |a, b| a < b)
        },
        OperadorCuadruplo::Igual => {
            valor1.operar_relacional(&valor2, |a, b| a == b)
        },
        OperadorCuadruplo::Diferente => {
            valor1.operar_relacional(&valor2, |a, b| a != b)
        },
        _ => unreachable!(),
    };

    self.escribir_memoria(dir_resultado, resultado)?;

    Ok(())
}
```

### Asignación

```rust
fn ejecutar_asignacion(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
    let valor = self.leer_operando(&cuad.operando_izq)?;
    let dir_destino = self.extraer_direccion(&cuad.resultado)?;

    self.escribir_memoria(dir_destino, valor)?;

    Ok(())
}
```

### Saltos

```rust
fn ejecutar_goto(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
    let destino = self.extraer_direccion(&cuad.resultado)?;
    self.ip = destino - 1; // -1 porque el loop hace IP++
    Ok(())
}

fn ejecutar_gotof(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
    let condicion = self.leer_operando(&cuad.operando_izq)?;
    let destino = self.extraer_direccion(&cuad.resultado)?;

    if condicion.a_entero() == 0 { // Falso
        self.ip = destino - 1;
    }

    Ok(())
}

fn ejecutar_gotov(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
    let condicion = self.leer_operando(&cuad.operando_izq)?;
    let destino = self.extraer_direccion(&cuad.resultado)?;

    if condicion.a_entero() != 0 { // Verdadero
        self.ip = destino - 1;
    }

    Ok(())
}
```

### E/S

```rust
fn ejecutar_escribe(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
    match &cuad.resultado {
        Operando::Letrero(idx) => {
            // Imprimir string literal
            let texto = &self.tabla_strings[*idx];
            self.io.escribir(texto);
        },
        _ => {
            // Imprimir valor numérico
            let valor = self.leer_operando(&cuad.resultado)?;
            match valor {
                Valor::Entero(v) => self.io.escribir(&v.to_string()),
                Valor::Flotante(v) => self.io.escribir(&v.to_string()),
                Valor::Letrero(idx) => self.io.escribir(&self.tabla_strings[idx]),
            }
        }
    }

    Ok(())
}

fn ejecutar_lectura(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
    let dir_destino = self.extraer_direccion(&cuad.resultado)?;
    let entrada = self.io.leer()?;

    // Intentar parsear
    let valor = Valor::desde_string(&entrada)?;

    self.escribir_memoria(dir_destino, valor)?;

    Ok(())
}
```

## 7.5 Agregar Nuevas Operaciones a la VM

### Ejemplo: Agregar `AND` y `OR`

**Paso 1**: Ya agregado en cuádruplo (Parte 5).

**Paso 2**: Implementar en VM

```rust
fn ejecutar_cuadruplo(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
    match cuad.operador {
        // ... operadores existentes ...

        OperadorCuadruplo::And | OperadorCuadruplo::Or => {
            self.ejecutar_logico(cuad)?;
        }

        // ...
    }
    Ok(())
}

fn ejecutar_logico(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
    let valor1 = self.leer_operando(&cuad.operando_izq)?;
    let valor2 = self.leer_operando(&cuad.operando_der)?;
    let dir_resultado = self.extraer_direccion(&cuad.resultado)?;

    let resultado = match cuad.operador {
        OperadorCuadruplo::And => {
            let a = valor1.a_entero() != 0;
            let b = valor2.a_entero() != 0;
            Valor::Entero(if a && b { 1 } else { 0 })
        },
        OperadorCuadruplo::Or => {
            let a = valor1.a_entero() != 0;
            let b = valor2.a_entero() != 0;
            Valor::Entero(if a || b { 1 } else { 0 })
        },
        _ => unreachable!(),
    };

    self.escribir_memoria(dir_resultado, resultado)?;

    Ok(())
}
```

## 7.6 Debugging de la VM

### Verbose durante Ejecución

```rust
pub fn ejecutar(&mut self, verbose: bool) -> Result<(), String> {
    self.pila_marcos.push(MarcoMemoria::new("main".to_string(), 0));

    while self.ejecutando && self.ip < self.cuadruplos.len() {
        let cuadruplo = &self.cuadruplos[self.ip].clone();

        if verbose {
            println!("[VM IP={}] {:?}", self.ip, cuadruplo);
        }

        self.ejecutar_cuadruplo(cuadruplo)?;

        if verbose {
            self.imprimir_estado_memoria();
        }

        self.ip += 1;
    }

    Ok(())
}
```

### Imprimir Estado de Memoria

```rust
fn imprimir_estado_memoria(&self) {
    println!("\n─── Estado de Memoria ───");
    println!("Pila de marcos: {} nivel(es)", self.pila_marcos.len());

    if let Some(marco) = self.pila_marcos.last() {
        println!("Marco actual: {}", marco.nombre_funcion);
        println!("  Locales: {} variable(s)", marco.memoria_local.enteros.len() + marco.memoria_local.flotantes.len());
        println!("  Temporales: {} variable(s)", marco.memoria_temporal.enteros.len() + marco.memoria_temporal.flotantes.len());
    }

    println!("Globales: {} variable(s)", self.memoria_global.enteros.len() + self.memoria_global.flotantes.len());
    println!("─────────────────────────\n");
}
```

**Ejecutar con verbose**:

```bash
VERBOSE=VM cargo run -- programa.txt
```

---

## Resumen

En esta parte cubrimos:

- Estructura de la Máquina Virtual
- Ciclo de ejecución de cuádruplos
- Segmentos de memoria en runtime
- Stack de marcos para funciones
- Implementación de operaciones (aritmética, relacional, E/S, saltos)
- Agregar nuevas operaciones a la VM
- Debugging de la VM

**Siguiente**: [PARTE 8: Funciones →](PARTE_08_FUNCIONES.md)

[← Parte 6](PARTE_06_MEMORIA.md) | [Volver al índice](../../GUIA_TECNICA.md)
