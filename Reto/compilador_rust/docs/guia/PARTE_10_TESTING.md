# PARTE 10: Testing y Debugging

[← Parte 9: Casos de Uso](PARTE_09_CASOS_USO.md) | [Volver al índice](../../GUIA_TECNICA.md)

## 10.1 Estrategia de Testing

### Niveles de Testing

```
┌─────────────────────────────────────┐
│  1. Tests Unitarios                 │  ← Componentes individuales
│     - Léxico                        │
│     - Cubo semántico                │
│     - Memoria virtual               │
├─────────────────────────────────────┤
│  2. Tests de Integración            │  ← Interacción entre módulos
│     - Parser + Léxico               │
│     - Semántico + Cuádruplos        │
│     - VM + Memoria                  │
├─────────────────────────────────────┤
│  3. Tests End-to-End                │  ← Programas completos
│     - tests/programas/00-10         │
└─────────────────────────────────────┘
```

## 10.2 Tests Unitarios

### Test del Analizador Léxico

**Archivo**: `src/bin/test_lexico.rs` (crear si no existe)

```rust
use compilador_rust::lexico::{AnalizadorLexico, TipoToken};

#[test]
fn test_reconocer_numeros() {
    let mut lexico = AnalizadorLexico::new("123 45.67 890");

    let tokens = lexico.tokenizar().unwrap();

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].tipo, TipoToken::CteEntera);
    assert_eq!(tokens[0].valor, "123");

    assert_eq!(tokens[1].tipo, TipoToken::CteFlotante);
    assert_eq!(tokens[1].valor, "45.67");

    assert_eq!(tokens[2].tipo, TipoToken::CteEntera);
    assert_eq!(tokens[2].valor, "890");
}

#[test]
fn test_reconocer_palabras_clave() {
    let mut lexico = AnalizadorLexico::new("programa vars inicio fin");

    let tokens = lexico.tokenizar().unwrap();

    assert_eq!(tokens[0].tipo, TipoToken::Programa);
    assert_eq!(tokens[1].tipo, TipoToken::Vars);
    assert_eq!(tokens[2].tipo, TipoToken::Inicio);
    assert_eq!(tokens[3].tipo, TipoToken::Fin);
}

#[test]
fn test_error_lexico() {
    let mut lexico = AnalizadorLexico::new("123.45.67");

    let resultado = lexico.tokenizar();
    assert!(resultado.is_err());
    assert!(resultado.unwrap_err().contains("formato inválido"));
}
```

**Ejecutar**:

```bash
cargo test test_reconocer_numeros
cargo test test_reconocer_palabras_clave
cargo test test_error_lexico
```

### Test del Cubo Semántico

**Archivo**: `src/bin/test_cubo.rs`

```rust
use compilador_rust::semantico::{CuboSemantico, TipoDato, Operador};

#[test]
fn test_suma_enteros() {
    let cubo = CuboSemantico::new();

    let resultado = cubo.obtener_tipo_resultado(
        TipoDato::Entero,
        Operador::Suma,
        TipoDato::Entero
    );

    assert!(resultado.is_ok());
    assert_eq!(resultado.unwrap(), TipoDato::Entero);
}

#[test]
fn test_division_flotantes() {
    let cubo = CuboSemantico::new();

    let resultado = cubo.obtener_tipo_resultado(
        TipoDato::Flotante,
        Operador::Division,
        TipoDato::Flotante
    );

    assert!(resultado.is_ok());
    assert_eq!(resultado.unwrap(), TipoDato::Flotante);
}

#[test]
fn test_operacion_invalida() {
    let cubo = CuboSemantico::new();

    let resultado = cubo.obtener_tipo_resultado(
        TipoDato::Entero,
        Operador::Suma,
        TipoDato::Letrero
    );

    assert!(resultado.is_err());
}

#[test]
fn test_comparacion_retorna_booleano() {
    let cubo = CuboSemantico::new();

    let resultado = cubo.obtener_tipo_resultado(
        TipoDato::Entero,
        Operador::Mayor,
        TipoDato::Entero
    );

    // Si tienes tipo Booleano:
    assert_eq!(resultado.unwrap(), TipoDato::Entero); // ← Ajustar según tu implementación
}
```

### Test de Memoria Virtual

**Archivo**: `src/bin/test_memoria.rs`

```rust
use compilador_rust::intermedio::{MemoriaVirtual, TipoDato, TipoSegmento};

#[test]
fn test_asignar_variables_globales() {
    let mut mem = MemoriaVirtual::new();

    let dir1 = mem.asignar_variable(TipoDato::Entero, TipoSegmento::Global).unwrap();
    let dir2 = mem.asignar_variable(TipoDato::Entero, TipoSegmento::Global).unwrap();

    assert_eq!(dir1, 1000);
    assert_eq!(dir2, 1001);
}

#[test]
fn test_pool_temporales() {
    let mut mem = MemoriaVirtual::new();

    // Asignar temporal
    let dir1 = mem.asignar_variable(TipoDato::Entero, TipoSegmento::Temporal).unwrap();
    assert_eq!(dir1, 13000);

    // Liberar temporal
    mem.liberar_temporal(dir1);

    // Reasignar (debe reusar)
    let dir2 = mem.asignar_variable(TipoDato::Entero, TipoSegmento::Temporal).unwrap();
    assert_eq!(dir2, 13000); // ← Mismo que dir1
}

#[test]
fn test_constantes_deduplicadas() {
    let mut mem = MemoriaVirtual::new();

    let dir1 = mem.asignar_constante_entera(42).unwrap();
    let dir2 = mem.asignar_constante_entera(42).unwrap();

    assert_eq!(dir1, dir2); // ← Misma dirección
}

#[test]
fn test_desbordamiento() {
    let mut mem = MemoriaVirtual::new();

    // Llenar memoria global de enteros (1000 variables)
    for _ in 0..1000 {
        mem.asignar_variable(TipoDato::Entero, TipoSegmento::Global).unwrap();
    }

    // Una más debe fallar
    let resultado = mem.asignar_variable(TipoDato::Entero, TipoSegmento::Global);
    assert!(resultado.is_err());
    assert!(resultado.unwrap_err().contains("Desbordamiento"));
}
```

## 10.3 Tests de Integración

### Test Parser + Semántico

**Archivo**: `src/bin/test_parser_completo.rs`

```rust
use compilador_rust::sintactico::AnalizadorSintactico;
use std::fs;

#[test]
fn test_programa_simple() {
    let codigo = r#"
programa test;
vars x : entero;
inicio {
    x = 5 + 3;
}
fin
"#;

    let mut parser = AnalizadorSintactico::new(codigo);
    let resultado = parser.analizar();

    assert!(resultado.is_ok(), "Parser falló: {:?}", resultado);

    // Verificar que se generaron cuádruplos
    let cuadruplos = parser.obtener_cuadruplos();
    assert!(!cuadruplos.is_empty());
}

#[test]
fn test_error_semantico() {
    let codigo = r#"
programa test;
vars x : entero;
inicio {
    x = "hola";
}
fin
"#;

    let mut parser = AnalizadorSintactico::new(codigo);
    let resultado = parser.analizar();

    assert!(resultado.is_err());
    assert!(resultado.unwrap_err().contains("tipo"));
}
```

### Test VM Completa

**Archivo**: `src/bin/test_vm.rs`

```rust
use compilador_rust::vm::Ejecutor;
use compilador_rust::intermedio::{Cuadruplo, OperadorCuadruplo, Operando};

#[test]
fn test_suma_simple() {
    let cuadruplos = vec![
        Cuadruplo::new(
            OperadorCuadruplo::Suma,
            Operando::Direccion(19000), // constante 5
            Operando::Direccion(19001), // constante 3
            Operando::Direccion(13000)  // temporal resultado
        ),
        Cuadruplo::new(
            OperadorCuadruplo::Imprimir,
            Operando::Nulo,
            Operando::Nulo,
            Operando::Direccion(13000)
        ),
    ];

    let mut vm = Ejecutor::new(cuadruplos);

    // Inicializar constantes manualmente
    vm.escribir_memoria(19000, Valor::Entero(5)).unwrap();
    vm.escribir_memoria(19001, Valor::Entero(3)).unwrap();

    let resultado = vm.ejecutar();
    assert!(resultado.is_ok());
}
```

## 10.4 Tests End-to-End

### Estructura de Tests de Programas

```
tests/
  programas/
    00_variables.txt       ← Declaración y asignación
    01_operaciones.txt     ← Operaciones aritméticas
    02_condiciones.txt     ← If-else
    03_ciclos.txt          ← Mientras
    04_funciones.txt       ← Llamadas a funciones
    ...
```

### Crear Test Automatizado

**Archivo**: `src/bin/test_programas.rs`

```rust
use std::fs;
use std::process::Command;

fn ejecutar_programa(archivo: &str) -> Result<String, String> {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "compilador_rust", "--", archivo])
        .output()
        .map_err(|e| format!("Error ejecutando: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[test]
fn test_00_variables() {
    let resultado = ejecutar_programa("tests/programas/00_variables.txt");
    assert!(resultado.is_ok());

    let salida = resultado.unwrap();
    assert!(salida.contains("Ejecución completada"));
}

#[test]
fn test_01_operaciones() {
    let resultado = ejecutar_programa("tests/programas/01_operaciones.txt");
    assert!(resultado.is_ok());

    // Verificar salida esperada
    let salida = resultado.unwrap();
    assert!(salida.contains("10")); // Resultado esperado
}

#[test]
fn test_todos_los_programas() {
    let archivos = vec![
        "00_variables.txt",
        "01_operaciones.txt",
        "02_condiciones.txt",
        "03_ciclos.txt",
        "04_funciones.txt",
        "05_recursion.txt",
        "06_factorial.txt",
        "test_funcion_simple.txt",
        "09_funcion_parametros.txt",
        "10_fibonacci.txt",
    ];

    let mut fallos = Vec::new();

    for archivo in archivos {
        let path = format!("tests/programas/{}", archivo);
        let resultado = ejecutar_programa(&path);

        if resultado.is_err() {
            fallos.push(format!("{}: {}", archivo, resultado.unwrap_err()));
        }
    }

    if !fallos.is_empty() {
        panic!("Tests fallidos:\n{}", fallos.join("\n"));
    }
}
```

**Ejecutar todos**:

```bash
cargo test test_todos_los_programas -- --nocapture
```

## 10.5 Debugging Paso a Paso

### Nivel 1: Verbose Léxico

**Modificar**: `src/lexico/mod.rs`

```rust
pub fn tokenizar(&mut self) -> Result<Vec<Token>, String> {
    let verbose = std::env::var("VERBOSE").unwrap_or_default();

    while let Some(token) = self.siguiente_token()? {
        if verbose == "LEXICO" || verbose == "ALL" {
            println!("[LEXICO] Token: {:?}", token);
        }
        self.tokens.push(token);
    }

    Ok(self.tokens.clone())
}
```

**Ejecutar**:

```bash
VERBOSE=LEXICO cargo run -- programa.txt
```

### Nivel 2: Verbose Sintáctico

**Modificar**: `src/sintactico/mod.rs`

```rust
fn shift(&mut self, estado: usize) {
    let verbose = std::env::var("VERBOSE").unwrap_or_default();

    if verbose == "SINTACTICO" || verbose == "ALL" {
        println!("[SHIFT] Estado {} → Token {:?}", estado, self.token_actual);
    }

    self.pila.push(estado);
    self.avanzar();
}

fn reduce(&mut self, produccion: &Produccion) {
    let verbose = std::env::var("VERBOSE").unwrap_or_default();

    if verbose == "SINTACTICO" || verbose == "ALL" {
        println!("[REDUCE] {}", produccion);
    }

    // ...
}
```

**Ejecutar**:

```bash
VERBOSE=SINTACTICO cargo run -- programa.txt
```

### Nivel 3: Verbose Semántico

```rust
pub fn generar_operaciones(&mut self, operador: &TipoToken) -> Result<(), String> {
    let verbose = std::env::var("VERBOSE").unwrap_or_default();

    if verbose == "SEMANTICO" || verbose == "ALL" {
        println!("[SEM] Operación {:?}", operador);
        println!("  Pila Operandos: {:?}", self.pilao);
        println!("  Pila Tipos: {:?}", self.ptypes);
    }

    // ...
}
```

### Nivel 4: Verbose Cuádruplos

```rust
pub fn generar_cuadruplo(&mut self, cuad: Cuadruplo) {
    let verbose = std::env::var("VERBOSE").unwrap_or_default();

    if verbose == "CUADRUPLOS" || verbose == "ALL" {
        println!("[CUAD #{}] {} {} {} {}",
            self.cuadruplos.len(),
            cuad.operador,
            cuad.operando_izq,
            cuad.operando_der,
            cuad.resultado
        );
    }

    self.cuadruplos.push(cuad);
}
```

### Nivel 5: Verbose VM

```rust
pub fn ejecutar(&mut self) -> Result<(), String> {
    let verbose = std::env::var("VERBOSE").unwrap_or_default();

    while self.ip < self.cuadruplos.len() {
        let cuad = &self.cuadruplos[self.ip].clone();

        if verbose == "VM" || verbose == "ALL" {
            println!("[VM IP={}] {:?}", self.ip, cuad);
        }

        self.ejecutar_cuadruplo(cuad)?;
        self.ip += 1;
    }

    Ok(())
}
```

### Modo Debug Total

**Ejecutar**:

```bash
VERBOSE=ALL cargo run -- programa.txt 2>&1 | less
```

Verás:

```
[LEXICO] Token: programa
[LEXICO] Token: test
[LEXICO] Token: ;
...
[SHIFT] Estado 0 → Token programa
[REDUCE] <PROGRAMA> → programa id
...
[SEM] Operación Suma
  Pila Operandos: [Dir(19000), Dir(19001)]
  Pila Tipos: [Entero, Entero]
[CUAD #0] + 19000 19001 13000
...
[VM IP=0] Cuadruplo { op: Suma, izq: 19000, der: 19001, res: 13000 }
Memoria[13000] = 8
```

## 10.6 Debugging con GDB/LLDB

### Compilar en Modo Debug

```bash
cargo build
```

### Ejecutar con LLDB (macOS)

```bash
lldb ./target/debug/compilador_rust
```

**Comandos útiles**:

```
(lldb) b src/main.rs:42              # Breakpoint en línea 42
(lldb) b AnalizadorSintactico::analizar  # Breakpoint en función
(lldb) run programa.txt               # Ejecutar
(lldb) n                              # Siguiente línea
(lldb) s                              # Step into
(lldb) c                              # Continuar
(lldb) print variable                 # Ver valor
(lldb) bt                             # Backtrace (pila de llamadas)
```

## 10.7 Errores Comunes y Soluciones

### Error: "Pila de operandos vacía"

**Causa**: Generación de cuádruplo sin operandos en pila.

**Debug**:

```bash
VERBOSE=SEMANTICO cargo run -- programa.txt
```

Buscar dónde PilaO se vacía inesperadamente.

**Solución**: Verificar orden de llamadas en acciones semánticas.

---

### Error: "Dirección de memoria inválida"

**Causa**: VM intentando acceder a dirección fuera de rangos.

**Debug**:

```bash
VERBOSE=VM cargo run -- programa.txt
```

Buscar cuádruplo con dirección sospechosa.

**Solución**: Verificar asignación en MemoriaVirtual.

---

### Error: "Tabla de variables no encontrada"

**Causa**: Variable no declarada o fuera de scope.

**Debug**: Imprimir tabla de variables:

```rust
println!("Tabla actual: {:?}", self.contexto.tabla_variables_actual());
```

**Solución**: Verificar flujo de scopes (inicio/fin bloque).

---

### Error: "Tipo no coincide en cuádruplo"

**Causa**: Cubo semántico permite operación pero VM falla.

**Debug**:

```rust
// En VM antes de operación
println!("Tipo esperado: {:?}, Tipo recibido: {:?}", esperado, recibido);
```

**Solución**: Sincronizar cubo semántico con VM.

---

## 10.8 Profiling y Optimización

### Medir Tiempo de Ejecución

```rust
use std::time::Instant;

fn main() {
    let inicio = Instant::now();

    // Tu código
    let mut parser = AnalizadorSintactico::new(&codigo);
    parser.analizar().unwrap();

    let duracion = inicio.elapsed();
    println!("Tiempo de parsing: {:?}", duracion);
}
```

### Contar Cuádruplos Generados

```rust
pub fn estadisticas(&self) {
    println!("═══ Estadísticas ═══");
    println!("Cuádruplos generados: {}", self.cuadruplos.len());
    println!("Temporales usados: {}", self.contador_temporales);
    println!("Variables globales: {}", self.memoria_virtual.globales_usadas());
    println!("Constantes únicas: {}", self.memoria_virtual.constantes_unicas());
}
```

### Memory Profiling

```bash
# Instalar valgrind (Linux) o instruments (macOS)
cargo build --release
valgrind --tool=massif ./target/release/compilador_rust programa.txt
```

## 10.9 Checklist de Testing Completo

Antes de considerar el compilador "terminado":

- [ ] **Léxico**

  - [ ] Test de números (enteros, flotantes)
  - [ ] Test de identificadores
  - [ ] Test de strings
  - [ ] Test de operadores
  - [ ] Test de errores

- [ ] **Sintáctico**

  - [ ] Test de gramática básica
  - [ ] Test de declaraciones
  - [ ] Test de expresiones
  - [ ] Test de estructuras de control
  - [ ] Test de funciones
  - [ ] Test de errores sintácticos

- [ ] **Semántico**

  - [ ] Test de cubo semántico (todas las operaciones)
  - [ ] Test de tabla de variables
  - [ ] Test de directorio de funciones
  - [ ] Test de asignaciones válidas/inválidas
  - [ ] Test de errores semánticos

- [ ] **Intermedio**

  - [ ] Test de generación de cuádruplos simples
  - [ ] Test de expresiones complejas
  - [ ] Test de condiciones
  - [ ] Test de ciclos
  - [ ] Test de funciones
  - [ ] Test de memoria virtual

- [ ] **VM**

  - [ ] Test de operaciones aritméticas
  - [ ] Test de operaciones relacionales
  - [ ] Test de saltos (Goto, GotoF)
  - [ ] Test de llamadas a función
  - [ ] Test de impresión/lectura
  - [ ] Test de errores en runtime

- [ ] **End-to-End**

  - [ ] Todos los programas de prueba (00-10)
  - [ ] Programas con funciones
  - [ ] Programas con recursión
  - [ ] Programas de estrés (muchas variables/cuádruplos)

- [ ] **Documentación**
  - [ ] README actualizado
  - [ ] Guía técnica completa
  - [ ] Comentarios en código crítico
  - [ ] Ejemplos de uso

---

## Resumen

En esta parte final cubrimos:

- Estrategia de testing (unitarios, integración, E2E)
- Tests para cada componente
- Debugging con verbose levels
- Debugging con GDB/LLDB
- Errores comunes y soluciones
- Profiling y optimización
- Checklist completo

**¡Guía Técnica Completa!**

[← Parte 9](PARTE_09_CASOS_USO.md) | [Volver al índice](../../GUIA_TECNICA.md)
