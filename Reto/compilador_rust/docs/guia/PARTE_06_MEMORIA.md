# PARTE 6: Memoria Virtual

[← Parte 5: Código Intermedio](PARTE_05_INTERMEDIO.md) | [Volver al índice](../../GUIA_TECNICA.md) | [Siguiente: Máquina Virtual →](PARTE_07_VM.md)

## 6.1 Arquitectura de Memoria Segmentada

### Esquema General

La memoria virtual del compilador usa **direcciones numéricas** (1000-24999) organizadas en **4 segmentos**:

```
┌──────────────────────────────────────────┐
│  SEGMENTO GLOBAL (1000-6999)             │  ← Variables globales
├──────────────────────────────────────────┤
│  SEGMENTO LOCAL (7000-12999)             │  ← Variables de función actual
├──────────────────────────────────────────┤
│  SEGMENTO TEMPORAL (13000-18999)         │  ← Temporales para expresiones
├──────────────────────────────────────────┤
│  SEGMENTO CONSTANTE (19000-24999)        │  ← Literales (5, 3.14, etc.)
└──────────────────────────────────────────┘
```

### Distribución por Tipo

Cada segmento se subdivide por **tipo de dato**:

```
SEGMENTO GLOBAL (1000-6999)
├── Enteros:   1000-2999 (2000 espacios)
├── Flotantes: 3000-4999 (2000 espacios)
└── Chars:     5000-6999 (2000 espacios)

SEGMENTO LOCAL (7000-12999)
├── Enteros:   7000-8999 (2000 espacios)
├── Flotantes: 9000-10999 (2000 espacios)
└── Chars:     11000-12999 (2000 espacios)

SEGMENTO TEMPORAL (13000-18999)
├── Enteros:   13000-14999 (2000 espacios)
├── Flotantes: 15000-16999 (2000 espacios)
└── Chars:     17000-18999 (2000 espacios)

SEGMENTO CONSTANTE (19000-24999)
├── Enteros:   19000-20999 (2000 espacios)
├── Flotantes: 21000-22999 (2000 espacios)
└── Chars:     23000-24999 (2000 espacios)
```

**Archivo**: `src/intermedio/memoria_virtual.rs`

```rust
// Límites de direcciones para cada segmento y tipo
const GLOBAL_ENTERO_INICIO: usize = 1000;
const GLOBAL_ENTERO_FIN: usize = 2999;
const GLOBAL_FLOTANTE_INICIO: usize = 3000;
const GLOBAL_FLOTANTE_FIN: usize = 4999;
const GLOBAL_CHAR_INICIO: usize = 5000;
const GLOBAL_CHAR_FIN: usize = 6999;

const LOCAL_ENTERO_INICIO: usize = 7000;
const LOCAL_ENTERO_FIN: usize = 8999;
const LOCAL_FLOTANTE_INICIO: usize = 9000;
const LOCAL_FLOTANTE_FIN: usize = 10999;
const LOCAL_CHAR_INICIO: usize = 11000;
const LOCAL_CHAR_FIN: usize = 12999;

const TEMPORAL_ENTERO_INICIO: usize = 13000;
const TEMPORAL_ENTERO_FIN: usize = 14999;
const TEMPORAL_FLOTANTE_INICIO: usize = 15000;
const TEMPORAL_FLOTANTE_FIN: usize = 16999;
const TEMPORAL_CHAR_INICIO: usize = 17000;
const TEMPORAL_CHAR_FIN: usize = 18999;

const CONSTANTE_ENTERO_INICIO: usize = 19000;
const CONSTANTE_ENTERO_FIN: usize = 20999;
const CONSTANTE_FLOTANTE_INICIO: usize = 21000;
const CONSTANTE_FLOTANTE_FIN: usize = 22999;
const CONSTANTE_CHAR_INICIO: usize = 23000;
const CONSTANTE_CHAR_FIN: usize = 24999;
```

### Estructura de MemoriaVirtual

```rust
pub struct MemoriaVirtual {
    // Contadores de direcciones por segmento y tipo
    global_entero: usize,
    global_flotante: usize,
    global_char: usize,

    local_entero: usize,
    local_flotante: usize,
    local_char: usize,

    temporal_entero: usize,
    temporal_flotante: usize,
    temporal_char: usize,

    constante_entero: usize,
    constante_flotante: usize,
    constante_char: usize,

    // Tablas de constantes (valor → dirección)
    tabla_constantes_entero: HashMap<i32, usize>,
    tabla_constantes_flotante: HashMap<String, usize>,
    tabla_constantes_char: HashMap<char, usize>,

    // Pools de temporales disponibles (AVAIL)
    temporales_disponibles_entero: HashSet<usize>,
    temporales_disponibles_flotante: HashSet<usize>,
    temporales_disponibles_char: HashSet<usize>,
}
```

## 6.2 Pool de Temporales (AVAIL)

### Problema

Sin reutilización, los temporales se agotan rápidamente:

```rust
x = a + b;  // temp1 = a + b
y = c + d;  // temp2 = c + d  ← temp1 ya no se usa
z = e + f;  // temp3 = e + f  ← temp2 ya no se usa
// ...
// Temporal 1000 = ...  ← AGOTADO!
```

### Solución: AVAIL

**AVAIL** es un pool de direcciones temporales liberadas que pueden reutilizarse:

```
┌─────────────────────────────────────┐
│  temporales_disponibles_entero      │
│  ├─ 13005                            │
│  ├─ 13012                            │
│  └─ 13023                            │
└─────────────────────────────────────┘
```

### Asignar Temporal

```rust
pub fn asignar_variable(&mut self, tipo: TipoDato, segmento: TipoSegmento)
    -> Result<usize, String> {

    match (segmento, tipo) {
        (TipoSegmento::Temporal, TipoDato::Entero) => {
            // Intentar reutilizar temporal disponible
            if let Some(&dir) = self.temporales_disponibles_entero.iter().next() {
                self.temporales_disponibles_entero.remove(&dir);
                return Ok(dir);
            }

            // Si no hay disponibles, asignar nuevo
            if self.temporal_entero > TEMPORAL_ENTERO_FIN {
                return Err("Error: Desbordamiento de temporales enteros".to_string());
            }
            let dir = self.temporal_entero;
            self.temporal_entero += 1;
            Ok(dir)
        }
        // ... mismo para flotante y char ...
    }
}
```

### Liberar Temporal

```rust
pub fn liberar_temporal(&mut self, direccion: usize) {
    // Determinar el tipo según el rango de dirección
    if direccion >= TEMPORAL_ENTERO_INICIO && direccion <= TEMPORAL_ENTERO_FIN {
        self.temporales_disponibles_entero.insert(direccion);
    } else if direccion >= TEMPORAL_FLOTANTE_INICIO && direccion <= TEMPORAL_FLOTANTE_FIN {
        self.temporales_disponibles_flotante.insert(direccion);
    } else if direccion >= TEMPORAL_CHAR_INICIO && direccion <= TEMPORAL_CHAR_FIN {
        self.temporales_disponibles_char.insert(direccion);
    }
}
```

### Ejemplo de Uso

```rust
// Expresión: x = a + b * c;

// 1. Asignar temp1 para b * c
let temp1 = memoria.asignar_variable(TipoDato::Entero, TipoSegmento::Temporal)?;
// temp1 = 13000

// 2. Generar cuádruplo: temp1 = b * c
quad.push(Cuadruplo::new(Multiplicacion, b, c, temp1));

// 3. Asignar temp2 para a + temp1
let temp2 = memoria.asignar_variable(TipoDato::Entero, TipoSegmento::Temporal)?;
// temp2 = 13001

// 4. Generar cuádruplo: temp2 = a + temp1
quad.push(Cuadruplo::new(Suma, a, temp1, temp2));

// 5. Liberar temp1 (ya no se usa)
memoria.liberar_temporal(temp1);

// 6. Asignar x = temp2
quad.push(Cuadruplo::new(Asignacion, temp2, Nulo, x));

// 7. Liberar temp2
memoria.liberar_temporal(temp2);

// Siguiente expresión puede reutilizar 13000 y 13001
```

## 6.3 Deduplicación de Constantes

### Problema

Sin deduplicación, cada uso de una constante crea nueva dirección:

```rust
x = 5;  // constante 5 → dirección 19000
y = 5;  // constante 5 → dirección 19001  ← DUPLICADO
z = 5;  // constante 5 → dirección 19002  ← DUPLICADO
```

### Solución: Tabla de Constantes

Usar **HashMap** para mapear valor → dirección:

```rust
tabla_constantes_entero: HashMap<i32, usize>
```

### Asignar Constante

```rust
pub fn asignar_constante_entera(&mut self, valor: i32) -> Result<usize, String> {
    // Si ya existe, retornar dirección existente
    if let Some(&direccion) = self.tabla_constantes_entero.get(&valor) {
        return Ok(direccion);
    }

    // Si no existe, crear nueva entrada
    if self.constante_entero > CONSTANTE_ENTERO_FIN {
        return Err("Error: Desbordamiento de constantes enteras".to_string());
    }

    let direccion = self.constante_entero;
    self.constante_entero += 1;

    self.tabla_constantes_entero.insert(valor, direccion);

    Ok(direccion)
}
```

### Constantes Flotantes

**Problema**: Comparar `f64` directamente es problemático por precisión.

**Solución**: Usar `String` como clave:

```rust
pub fn asignar_constante_flotante(&mut self, valor: f64) -> Result<usize, String> {
    let clave = format!("{:.10}", valor); // 10 decimales de precisión

    if let Some(&direccion) = self.tabla_constantes_flotante.get(&clave) {
        return Ok(direccion);
    }

    if self.constante_flotante > CONSTANTE_FLOTANTE_FIN {
        return Err("Error: Desbordamiento de constantes flotantes".to_string());
    }

    let direccion = self.constante_flotante;
    self.constante_flotante += 1;

    self.tabla_constantes_flotante.insert(clave, direccion);

    Ok(direccion)
}
```

### Ejemplo

```rust
let dir1 = memoria.asignar_constante_entera(42)?;
// dir1 = 19000, tabla: {42: 19000}

let dir2 = memoria.asignar_constante_entera(42)?;
// dir2 = 19000 (mismo que dir1, reutilizado)

let dir3 = memoria.asignar_constante_entera(100)?;
// dir3 = 19001, tabla: {42: 19000, 100: 19001}
```

## 6.4 Cómo Modificar Rangos de Memoria

### Ejemplo: Expandir Globales de Enteros

**Paso 1**: Modificar constantes en `memoria_virtual.rs`

```rust
// ANTES
const GLOBAL_ENTERO_INICIO: usize = 1000;
const GLOBAL_ENTERO_FIN: usize = 2999;  // 2000 espacios

// DESPUÉS
const GLOBAL_ENTERO_INICIO: usize = 1000;
const GLOBAL_ENTERO_FIN: usize = 3999;  // 3000 espacios
```

**Paso 2**: Ajustar rango de flotantes (para evitar overlap)

```rust
// ANTES
const GLOBAL_FLOTANTE_INICIO: usize = 3000;

// DESPUÉS
const GLOBAL_FLOTANTE_INICIO: usize = 4000;  // Empieza después de enteros
const GLOBAL_FLOTANTE_FIN: usize = 5999;
```

**Paso 3**: Ajustar chars

```rust
const GLOBAL_CHAR_INICIO: usize = 6000;
const GLOBAL_CHAR_FIN: usize = 7999;
```

**Paso 4**: Actualizar en `vm/memoria.rs`

**CRÍTICO**: Los rangos en VM deben coincidir:

```rust
// En src/vm/memoria.rs
const GLOBAL_ENTERO_INICIO: usize = 1000;
const GLOBAL_ENTERO_FIN: usize = 3999;  // ← MISMO que memoria_virtual.rs
const GLOBAL_FLOTANTE_INICIO: usize = 4000;
const GLOBAL_FLOTANTE_FIN: usize = 5999;
// ...
```

### Reorganizar Completamente

Si quieres cambiar el esquema completo (ej: global 0-9999):

```rust
// Nueva distribución
const GLOBAL_ENTERO_INICIO: usize = 0;
const GLOBAL_ENTERO_FIN: usize = 2999;
const GLOBAL_FLOTANTE_INICIO: usize = 3000;
const GLOBAL_FLOTANTE_FIN: usize = 5999;
const GLOBAL_CHAR_INICIO: usize = 6000;
const GLOBAL_CHAR_FIN: usize = 9999;

const LOCAL_ENTERO_INICIO: usize = 10000;
const LOCAL_ENTERO_FIN: usize = 12999;
// ...
```

**Actualizar en ambos archivos**:

- `src/intermedio/memoria_virtual.rs`
- `src/vm/memoria.rs`

## 6.5 Resetear Memoria Local

### Por Qué

Al finalizar una función, las direcciones locales deben liberarse para la siguiente:

```
Función A:
  x: entero → 7000
  y: flotante → 9000

Función B:
  a: entero → 7000  ← REUTILIZA (reseteo)
  b: flotante → 9000
```

### Implementación

```rust
pub fn resetear_local(&mut self) {
    self.local_entero = LOCAL_ENTERO_INICIO;
    self.local_flotante = LOCAL_FLOTANTE_INICIO;
    self.local_char = LOCAL_CHAR_INICIO;
}
```

**Cuándo llamar**:

```rust
// Al finalizar función
pub fn finalizar_funcion(&mut self) -> Result<(), String> {
    // ... generar EndFunc ...

    self.memoria_virtual.resetear_local();

    Ok(())
}
```

## 6.6 Obtener Información de Direcciones

### Determinar Tipo por Dirección

```rust
pub fn obtener_tipo_por_direccion(direccion: usize) -> Option<TipoDato> {
    match direccion {
        // Global
        GLOBAL_ENTERO_INICIO..=GLOBAL_ENTERO_FIN => Some(TipoDato::Entero),
        GLOBAL_FLOTANTE_INICIO..=GLOBAL_FLOTANTE_FIN => Some(TipoDato::Flotante),
        GLOBAL_CHAR_INICIO..=GLOBAL_CHAR_FIN => Some(TipoDato::Char),

        // Local
        LOCAL_ENTERO_INICIO..=LOCAL_ENTERO_FIN => Some(TipoDato::Entero),
        LOCAL_FLOTANTE_INICIO..=LOCAL_FLOTANTE_FIN => Some(TipoDato::Flotante),
        LOCAL_CHAR_INICIO..=LOCAL_CHAR_FIN => Some(TipoDato::Char),

        // Temporal
        TEMPORAL_ENTERO_INICIO..=TEMPORAL_ENTERO_FIN => Some(TipoDato::Entero),
        TEMPORAL_FLOTANTE_INICIO..=TEMPORAL_FLOTANTE_FIN => Some(TipoDato::Flotante),
        TEMPORAL_CHAR_INICIO..=TEMPORAL_CHAR_FIN => Some(TipoDato::Char),

        // Constante
        CONSTANTE_ENTERO_INICIO..=CONSTANTE_ENTERO_FIN => Some(TipoDato::Entero),
        CONSTANTE_FLOTANTE_INICIO..=CONSTANTE_FLOTANTE_FIN => Some(TipoDato::Flotante),
        CONSTANTE_CHAR_INICIO..=CONSTANTE_CHAR_FIN => Some(TipoDato::Char),

        _ => None,
    }
}
```

### Determinar Segmento

```rust
pub fn obtener_segmento(direccion: usize) -> Option<TipoSegmento> {
    match direccion {
        GLOBAL_ENTERO_INICIO..=GLOBAL_CHAR_FIN => Some(TipoSegmento::Global),
        LOCAL_ENTERO_INICIO..=LOCAL_CHAR_FIN => Some(TipoSegmento::Local),
        TEMPORAL_ENTERO_INICIO..=TEMPORAL_CHAR_FIN => Some(TipoSegmento::Temporal),
        CONSTANTE_ENTERO_INICIO..=CONSTANTE_CHAR_FIN => Some(TipoSegmento::Constante),
        _ => None,
    }
}
```

## 6.7 Debugging de Memoria

### Imprimir Estado

```rust
pub fn imprimir_estado(&self) {
    println!("\n╔══════════════════════════════════════════════════╗");
    println!("║        ESTADO DE MEMORIA VIRTUAL                 ║");
    println!("╚══════════════════════════════════════════════════╝\n");

    println!("SEGMENTO GLOBAL:");
    println!("  Enteros:   {} / {} ({} usados)",
             self.global_entero,
             GLOBAL_ENTERO_FIN,
             self.global_entero - GLOBAL_ENTERO_INICIO);
    println!("  Flotantes: {} / {} ({} usados)",
             self.global_flotante,
             GLOBAL_FLOTANTE_FIN,
             self.global_flotante - GLOBAL_FLOTANTE_INICIO);
    println!("  Chars:     {} / {} ({} usados)",
             self.global_char,
             GLOBAL_CHAR_FIN,
             self.global_char - GLOBAL_CHAR_INICIO);

    println!("\nSEGMENTO LOCAL:");
    println!("  Enteros:   {} / {} ({} usados)",
             self.local_entero,
             LOCAL_ENTERO_FIN,
             self.local_entero - LOCAL_ENTERO_INICIO);
    // ...

    println!("\nCONSTANTES:");
    println!("  Enteros únicos: {}", self.tabla_constantes_entero.len());
    println!("  Flotantes únicos: {}", self.tabla_constantes_flotante.len());

    println!("\nTEMPORALES DISPONIBLES (AVAIL):");
    println!("  Enteros: {}", self.temporales_disponibles_entero.len());
    println!("  Flotantes: {}", self.temporales_disponibles_flotante.len());
}
```

**Salida**:

```
╔══════════════════════════════════════════════════╗
║        ESTADO DE MEMORIA VIRTUAL                 ║
╚══════════════════════════════════════════════════╝

SEGMENTO GLOBAL:
  Enteros:   1003 / 2999 (3 usados)
  Flotantes: 3002 / 4999 (2 usados)
  Chars:     5000 / 6999 (0 usados)

SEGMENTO LOCAL:
  Enteros:   7000 / 8999 (0 usados)
  Flotantes: 9000 / 10999 (0 usados)
  Chars:     11000 / 12999 (0 usados)

CONSTANTES:
  Enteros únicos: 5
  Flotantes únicos: 2

TEMPORALES DISPONIBLES (AVAIL):
  Enteros: 3
  Flotantes: 1
```

---

## Resumen

En esta parte cubrimos:

- Arquitectura de memoria segmentada (4 segmentos × 3 tipos)
- Pool de temporales AVAIL (reutilización)
- Deduplicación de constantes (HashMap)
- Modificar rangos de memoria
- Resetear memoria local
- Obtener información de direcciones
- Debugging de memoria

**Siguiente**: [PARTE 7: Máquina Virtual →](PARTE_07_VM.md)

[← Parte 5](PARTE_05_INTERMEDIO.md) | [Volver al índice](../../GUIA_TECNICA.md)
