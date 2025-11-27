# PARTE 4: Análisis Semántico

[← Parte 3: Sintáctico](PARTE_03_SINTACTICO.md) | [Volver al índice](../../GUIA_TECNICA.md) | [Siguiente: Código Intermedio →](PARTE_05_INTERMEDIO.md)

## 4.1 El Cubo Semántico

### Qué es el Cubo Semántico

El **cubo semántico** define las reglas de compatibilidad de tipos para operaciones.

```
┌───────────────────────────────────┐
│  TipoDato × Operador × TipoDato  │
│              ↓                    │
│         TipoDato Resultado        │
└───────────────────────────────────┘
```

**Ejemplo**:

- `entero + entero → entero`
- `entero + flotante → flotante`
- `flotante + flotante → flotante`
- `entero + letrero → ERROR`

### Estructura del Cubo

**Archivo**: `src/semantico/cubo_semantico.rs`

```rust
pub struct CuboSemantico {
    reglas: HashMap<(TipoDato, Operador, TipoDato), TipoDato>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operador {
    // Aritméticos
    Suma,
    Resta,
    Multiplicacion,
    Division,

    // Relacionales
    MayorQue,
    MenorQue,
    Igual,
    Diferente,

    // Asignación
    Asignacion,
}

impl CuboSemantico {
    pub fn new() -> Self {
        let mut cubo = CuboSemantico {
            reglas: HashMap::new(),
        };
        cubo.inicializar_reglas();
        cubo
    }
}
```

### Reglas Predefinidas

#### Operadores Aritméticos (+, -, \*, /)

**Regla de Promoción**: Cualquier operación con `flotante` promueve a `flotante`.

```rust
fn inicializar_reglas(&mut self) {
    let operadores_aritmeticos = [
        Operador::Suma,
        Operador::Resta,
        Operador::Multiplicacion,
        Operador::Division,
    ];

    for &op in &operadores_aritmeticos {
        // entero OP entero = entero
        self.agregar_regla(TipoDato::Entero, op, TipoDato::Entero, TipoDato::Entero);

        // entero OP flotante = flotante (promoción)
        self.agregar_regla(TipoDato::Entero, op, TipoDato::Flotante, TipoDato::Flotante);

        // flotante OP entero = flotante (promoción)
        self.agregar_regla(TipoDato::Flotante, op, TipoDato::Entero, TipoDato::Flotante);

        // flotante OP flotante = flotante
        self.agregar_regla(TipoDato::Flotante, op, TipoDato::Flotante, TipoDato::Flotante);
    }
}
```

**Tabla de Aritméticos**:

| Izq \ Der    | entero   | flotante |
| ------------ | -------- | -------- |
| **entero**   | entero   | flotante |
| **flotante** | flotante | flotante |

#### Operadores Relacionales (>, <, ==, !=)

**Regla**: Siempre retornan `entero` (0 = falso, 1 = verdadero).

```rust
let operadores_relacionales = [
    Operador::MayorQue,
    Operador::MenorQue,
    Operador::Igual,
    Operador::Diferente,
];

for &op in &operadores_relacionales {
    // Todas las combinaciones retornan entero
    self.agregar_regla(TipoDato::Entero, op, TipoDato::Entero, TipoDato::Entero);
    self.agregar_regla(TipoDato::Entero, op, TipoDato::Flotante, TipoDato::Entero);
    self.agregar_regla(TipoDato::Flotante, op, TipoDato::Entero, TipoDato::Entero);
    self.agregar_regla(TipoDato::Flotante, op, TipoDato::Flotante, TipoDato::Entero);
}
```

#### Operador de Asignación (=)

**Regla**: Fuertemente tipado con promoción `entero → flotante`.

```rust
// entero = entero ✓
self.agregar_regla(TipoDato::Entero, Operador::Asignacion, TipoDato::Entero, TipoDato::Entero);

// flotante = flotante ✓
self.agregar_regla(TipoDato::Flotante, Operador::Asignacion, TipoDato::Flotante, TipoDato::Flotante);

// flotante = entero ✓ (promoción)
self.agregar_regla(TipoDato::Flotante, Operador::Asignacion, TipoDato::Entero, TipoDato::Flotante);

// entero = flotante ✗ (truncamiento NO permitido)
// NO se agrega regla → validar() retorna Error
```

### Uso del Cubo

```rust
pub fn validar(&self, tipo1: TipoDato, operador: Operador, tipo2: TipoDato)
    -> ResultadoTipo {
    match self.reglas.get(&(tipo1, operador, tipo2)) {
        Some(&tipo_resultado) => ResultadoTipo::Ok(tipo_resultado),
        None => ResultadoTipo::Error,
    }
}
```

**Ejemplo de uso**:

```rust
let cubo = CuboSemantico::new();

// Validar: 5 + 3.14
let resultado = cubo.validar(
    TipoDato::Entero,
    Operador::Suma,
    TipoDato::Flotante
);

assert_eq!(resultado, ResultadoTipo::Ok(TipoDato::Flotante));
```

## 4.2 Cómo Agregar/Modificar Operadores

### Ejemplo: Agregar Operador Módulo (%)

**Paso 1**: Agregar al enum `Operador`

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

**Paso 2**: Agregar conversión desde string

```rust
impl Operador {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "+" => Some(Operador::Suma),
            // ...
            "%" => Some(Operador::Modulo),  // ← AGREGAR
            _ => None,
        }
    }
}
```

**Paso 3**: Agregar reglas en el cubo

```rust
fn inicializar_reglas(&mut self) {
    // ... operadores existentes ...

    // MÓDULO: solo para enteros
    self.agregar_regla(TipoDato::Entero, Operador::Modulo, TipoDato::Entero, TipoDato::Entero);

    // Si quieres permitir flotante:
    // self.agregar_regla(TipoDato::Flotante, Operador::Modulo, TipoDato::Flotante, TipoDato::Flotante);
}
```

### Ejemplo: Permitir Truncamiento `entero = flotante`

Por defecto, está **prohibido**. Para permitirlo con warning:

```rust
// En inicializar_reglas, agregar:
self.agregar_regla(
    TipoDato::Entero,
    Operador::Asignacion,
    TipoDato::Flotante,
    TipoDato::Entero  // Retorna entero (truncado)
);
```

Luego en la generación de código, agregar cuádruplo de conversión:

```rust
if tipo_destino == TipoDato::Entero && tipo_fuente == TipoDato::Flotante {
    eprintln!("Warning: Truncando flotante a entero");
    // Generar cuádruplo de conversión
}
```

## 4.3 Tabla de Variables y Alcances

### Estructura de Entrada

**Archivo**: `src/semantico/tabla_variables.rs`

```rust
#[derive(Debug, Clone)]
pub struct EntradaVariable {
    /// Tipo de dato de la variable
    pub tipo: TipoDato,

    /// Dirección de memoria virtual asignada
    pub direccion: usize,

    /// Indica si esta variable es un parámetro de función
    pub es_parametro: bool,

    /// Posición del parámetro (si aplica)
    pub posicion_parametro: Option<usize>,
}
```

### Tabla de Variables

```rust
#[derive(Debug, Clone)]
pub struct TablaVariables {
    variables: HashMap<String, EntradaVariable>,
}

impl TablaVariables {
    /// Agrega una variable
    pub fn agregar(&mut self, nombre: &str, tipo: TipoDato, direccion: usize)
        -> Result<(), String> {
        // Validar duplicados
        if self.variables.contains_key(nombre) {
            return Err(format!(
                "Error semántico: Variable '{}' doblemente declarada",
                nombre
            ));
        }

        self.variables.insert(
            nombre.to_string(),
            EntradaVariable::new(tipo, direccion),
        );

        Ok(())
    }

    /// Busca una variable
    pub fn buscar(&self, nombre: &str) -> Option<&EntradaVariable> {
        self.variables.get(nombre)
    }
}
```

### Alcances (Scopes)

Cada función tiene su propia tabla de variables:

```
┌─────────────────────────────────┐
│  Programa (Global)              │
│  ├─ Tabla Variables Globales    │
├─────────────────────────────────┤
│  Función "calcular"             │
│  ├─ Tabla Variables Locales     │
│  ├─ Parámetros: [a, b]          │
├─────────────────────────────────┤
│  Función "imprimir"             │
│  ├─ Tabla Variables Locales     │
└─────────────────────────────────┘
```

## 4.4 Directorio de Funciones

### Entrada de Función

**Archivo**: `src/semantico/directorio_funciones.rs`

```rust
#[derive(Debug, Clone)]
pub struct EntradaFuncion {
    /// Nombre de la función
    pub nombre: String,

    /// Tipo de retorno
    pub tipo_retorno: TipoRetorno,

    /// Tabla de variables locales
    pub tabla_variables: TablaVariables,

    /// Lista de parámetros (en orden)
    pub parametros: Vec<Parametro>,

    /// Dirección de inicio de la función en cuádruplos
    pub direccion_inicio: Option<usize>,

    /// Cantidad de variables locales (para ERA)
    pub num_variables_locales: usize,
}
```

### Directorio

```rust
pub struct DirectorioFunciones {
    funciones: HashMap<String, EntradaFuncion>,
}

impl DirectorioFunciones {
    /// Agrega una función
    pub fn agregar_funcion(&mut self, nombre: &str, tipo_retorno: TipoRetorno)
        -> Result<(), String> {
        if self.funciones.contains_key(nombre) {
            return Err(format!(
                "Error semántico: Función '{}' ya declarada",
                nombre
            ));
        }

        let funcion = EntradaFuncion::new(nombre.to_string(), tipo_retorno);
        self.funciones.insert(nombre.to_string(), funcion);

        Ok(())
    }

    /// Busca una función
    pub fn buscar(&self, nombre: &str) -> Option<&EntradaFuncion> {
        self.funciones.get(nombre)
    }
}
```

## 4.5 Contexto Semántico

### Qué es el Contexto

El **ContextoSemantico** coordina todos los componentes semánticos:

```rust
pub struct ContextoSemantico {
    /// Directorio de funciones
    pub dir_funciones: DirectorioFunciones,

    /// Cubo semántico
    pub cubo_semantico: CuboSemantico,

    /// Alcance actual (función en la que estamos)
    alcance_actual: String,

    /// Tipo actual durante declaración
    tipo_actual: Option<TipoDato>,

    /// Nombre del programa
    nombre_programa: String,

    /// Gestor de memoria virtual
    memoria_virtual: MemoriaVirtual,
}
```

### Flujo de Uso

#### 1. Inicializar Programa

```rust
contexto.inicializar_programa("mi_programa")?;
```

Esto:

- Establece `alcance_actual = "mi_programa"`
- Crea entrada en directorio de funciones para el programa

#### 2. Declarar Variables Globales

```rust
contexto.establecer_tipo_actual(TipoDato::Entero)?;
contexto.agregar_variable("x")?;
contexto.agregar_variable("y")?;
```

Esto:

- Asigna direcciones de memoria en segmento global
- Agrega variables a la tabla del programa

#### 3. Iniciar Función

```rust
contexto.iniciar_funcion("calcular", TipoRetorno::Entero)?;
```

Esto:

- Cambia `alcance_actual = "calcular"`
- Crea entrada en directorio de funciones

#### 4. Agregar Parámetros

```rust
contexto.agregar_parametro("a", TipoDato::Entero)?;
contexto.agregar_parametro("b", TipoDato::Flotante)?;
```

Esto:

- Asigna direcciones en segmento local
- Registra parámetros en la función

#### 5. Variables Locales

```rust
contexto.establecer_tipo_actual(TipoDato::Flotante)?;
contexto.agregar_variable("resultado")?;
```

Esto:

- Asigna dirección local
- Agrega a tabla de variables de la función

#### 6. Finalizar Función

```rust
contexto.finalizar_funcion()?;
```

Esto:

- Restablece `alcance_actual` al programa
- Resetea memoria local

## 4.6 Validaciones Semánticas

### Validación de Tipos en Operaciones

```rust
pub fn validar_operacion(
    &self,
    tipo_izq: TipoDato,
    operador: &str,
    tipo_der: TipoDato
) -> Result<TipoDato, String> {
    let op = Operador::from_str(operador)
        .ok_or_else(|| format!("Operador desconocido: {}", operador))?;

    match self.cubo_semantico.validar(tipo_izq, op, tipo_der) {
        ResultadoTipo::Ok(tipo_resultado) => Ok(tipo_resultado),
        ResultadoTipo::Error => Err(format!(
            "Error semántico: Operación inválida {} {} {}",
            tipo_izq, operador, tipo_der
        )),
    }
}
```

### Validación de Variables

```rust
pub fn buscar_variable(&self, nombre: &str) -> Result<&EntradaVariable, String> {
    // Buscar primero en alcance local (si no es el programa)
    if self.alcance_actual != self.nombre_programa {
        if let Some(var) = self.dir_funciones.buscar_variable(
            &self.alcance_actual,
            nombre
        ) {
            return Ok(var);
        }
    }

    // Buscar en alcance global
    self.dir_funciones.buscar_variable(&self.nombre_programa, nombre)
        .ok_or_else(|| format!(
            "Error semántico: Variable '{}' no declarada",
            nombre
        ))
}
```

### Validación de Funciones

```rust
pub fn validar_llamada_funcion(
    &self,
    nombre: &str,
    tipos_argumentos: &[TipoDato]
) -> Result<TipoRetorno, String> {
    let funcion = self.dir_funciones.buscar(nombre)
        .ok_or_else(|| format!(
            "Error semántico: Función '{}' no declarada",
            nombre
        ))?;

    // Validar cantidad de argumentos
    if tipos_argumentos.len() != funcion.parametros.len() {
        return Err(format!(
            "Error semántico: Función '{}' espera {} argumentos, recibió {}",
            nombre,
            funcion.parametros.len(),
            tipos_argumentos.len()
        ));
    }

    // Validar tipos de argumentos
    for (i, (tipo_arg, param)) in tipos_argumentos.iter()
        .zip(funcion.parametros.iter()).enumerate() {
        if *tipo_arg != param.tipo {
            return Err(format!(
                "Error semántico: Argumento {} de '{}' debe ser {:?}, recibió {:?}",
                i + 1,
                nombre,
                param.tipo,
                tipo_arg
            ));
        }
    }

    Ok(funcion.tipo_retorno)
}
```

## 4.7 Debugging Semántico

### Imprimir Cubo Semántico

```rust
contexto.cubo_semantico.imprimir();
```

**Salida**:

```
╔══════════════════════════════════════════════════════════════╗
║            CUBO SEMÁNTICO - LENGUAJE PATITO                  ║
╚══════════════════════════════════════════════════════════════╝

OPERADORES ARITMÉTICOS (+, -, *, /):
─────────────────────────────────────
  entero + entero = entero
  entero + flotante = flotante
  flotante + entero = flotante
  flotante + flotante = flotante

OPERADORES RELACIONALES (>, <, ==, !=):
────────────────────────────────────────
  entero > entero = entero
  entero > flotante = entero
  flotante > entero = entero
  flotante > flotante = entero

OPERADOR DE ASIGNACIÓN (=):
───────────────────────────
  entero = entero → entero ✓
  entero = flotante → ERROR ✗
  flotante = entero → flotante ✓
  flotante = flotante → flotante ✓
```

### Imprimir Directorio de Funciones

```rust
contexto.dir_funciones.imprimir();
```

**Salida**:

```
═══════════════════════════════════════
DIRECTORIO DE FUNCIONES
═══════════════════════════════════════

Función: mi_programa (Global)
  Tipo retorno: Nula
  Variables globales: 3
    x: entero @ 1000
    y: entero @ 1001
    z: flotante @ 3000

Función: calcular
  Tipo retorno: entero
  Parámetros: 2
    a: entero @ 7000
    b: flotante @ 10000
  Variables locales: 1
    resultado: flotante @ 10001
  Dirección inicio: 15
```

### Verbose Semántico

Agregar en validaciones:

```rust
if verbose >= 3 {
    println!("[SEM] Validando operación: {:?} {} {:?}",
             tipo_izq, operador, tipo_der);
    println!("[SEM] Tipo resultado: {:?}", tipo_resultado);
}
```

---

## Resumen

En esta parte cubrimos:

- Cubo semántico: estructura y reglas
- Agregar/modificar operadores en el cubo
- Tabla de variables y alcances
- Directorio de funciones
- Contexto semántico (coordinador)
- Validaciones (tipos, variables, funciones)
- Debugging semántico

**Siguiente**: [PARTE 5: Generación de Código Intermedio →](PARTE_05_INTERMEDIO.md)

[← Parte 3](PARTE_03_SINTACTICO.md) | [Volver al índice](../../GUIA_TECNICA.md)
