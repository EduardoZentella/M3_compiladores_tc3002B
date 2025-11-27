# PARTE 8: Funciones

[← Parte 7: Máquina Virtual](PARTE_07_VM.md) | [Volver al índice](../../GUIA_TECNICA.md) | [Siguiente: Casos de Uso →](PARTE_09_CASOS_USO.md)

## 8.1 Protocolo Completo de Llamadas

### Secuencia de Cuádruplos

Para llamar a una función, se genera la siguiente secuencia:

```
ERA nombre_funcion        // Activación de registro (crear marco temporal)
PARAM arg1, -, param1     // Copiar argumento 1 a parámetro 1
PARAM arg2, -, param2     // Copiar argumento 2 a parámetro 2
...
GOSUB nombre_funcion      // Saltar a la función
```

**Dentro de la función**:

```
... cuerpo de la función ...
RETURN valor_retorno      // Guardar valor de retorno (si aplica)
ENDFUNC                   // Finalizar función y retornar
```

### Ejemplo Completo

**Código Patito**:

```paradox
programa ejemplo;

vars resultado : entero;

entero suma(a : entero, b : entero) {
    regresa a + b;
}

inicio {
    resultado = suma(5, 3);
    escribe(resultado);
}
fin
```

**Cuádruplos generados**:

```
  0: Goto - - 5          // Saltar funciones al inicio

  // Función suma
  1: = @7000 - @7000     // Parámetro a (ya en su dirección)
  2: = @7001 - @7001     // Parámetro b (ya en su dirección)
  3: + @7000 @7001 @13000  // temp = a + b
  4: Return @13000 - -   // Retornar temp
  5: EndFunc - - -       // Fin de función

  // Programa principal
  6: Era - - suma        // Crear marco para suma
  7: Param @19000 - 0    // Pasar 5 (constante) como param 0
  8: Param @19001 - 1    // Pasar 3 (constante) como param 1
  9: GoSub - - 1         // Llamar a suma (dir 1)
 10: = @13001 - @1000    // resultado = valor de retorno
 11: Escritura - - @1000 // escribe(resultado)
 12: EndFunc - - -       // Fin del programa
```

## 8.2 ERA: Activation Record

### Qué hace ERA

**ERA** (Extended Register Activation) crea un **marco temporal** para la función que se va a llamar.

**Archivo**: `src/vm/ejecutor.rs`

```rust
fn ejecutar_era(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
    // Obtener nombre de la función desde operando
    let nombre_funcion = self.extraer_nombre_funcion(&cuad.resultado)?;

    // Crear marco temporal (no se empuja aún a la pila)
    let marco = MarcoMemoria::new(nombre_funcion, self.ip + 1);

    self.marco_temporal = Some(marco);

    Ok(())
}
```

**Nota**: El marco temporal NO se empuja a la pila todavía. Se empujará al ejecutar `GOSUB`.

## 8.3 PARAM: Paso de Parámetros

### Qué hace PARAM

**PARAM** copia el valor de un argumento a la dirección del parámetro en el marco temporal.

```rust
fn ejecutar_parametro(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
    // Leer valor del argumento
    let valor = self.leer_operando(&cuad.operando_izq)?;

    // Obtener posición del parámetro (en operando_der)
    let posicion = self.extraer_direccion(&cuad.operando_der)?;

    // Obtener dirección del parámetro en la función
    // Los parámetros están en segmento local de la función
    let dir_parametro = LOCAL_ENTERO_INICIO + posicion;

    // Escribir en el marco temporal
    let marco = self.marco_temporal.as_mut()
        .ok_or("No hay marco temporal (falta ERA)")?;

    marco.escribir_local(dir_parametro, valor)?;

    Ok(())
}
```

### Ejemplo

```
ERA suma                  // Crea marco temporal
PARAM @19000, -, 0        // Parámetro 0: copiar valor de @19000 a local[0]
PARAM @19001, -, 1        // Parámetro 1: copiar valor de @19001 a local[1]
```

## 8.4 GOSUB: Llamada a Subrutina

### Qué hace GOSUB

**GOSUB** empuja el marco temporal a la pila y salta al inicio de la función.

```rust
fn ejecutar_gosub(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
    // Obtener dirección de inicio de la función
    let dir_funcion = self.extraer_direccion(&cuad.resultado)?;

    // Tomar el marco temporal y empujarlo a la pila
    let marco = self.marco_temporal.take()
        .ok_or("No hay marco temporal (falta ERA)")?;

    self.pila_marcos.push(marco);

    // Saltar a la función
    self.ip = dir_funcion - 1; // -1 porque el loop hace IP++

    Ok(())
}
```

### Estado de la Pila

**Antes de GOSUB**:

```
┌──────────────────────────┐
│  Marco: main             │  ← Tope
│  └─ IP retorno: 10       │
└──────────────────────────┘

marco_temporal: Some(Marco suma)
```

**Después de GOSUB**:

```
┌──────────────────────────┐
│  Marco: suma             │  ← Tope (nuevo)
│  ├─ Parámetros: [a, b]   │
│  └─ IP retorno: 10       │
├──────────────────────────┤
│  Marco: main             │
│  └─ IP retorno: 0        │
└──────────────────────────┘

marco_temporal: None
IP: 1 (inicio de suma)
```

## 8.5 RETURN: Retorno de Función

### Qué hace RETURN

**RETURN** guarda el valor de retorno en un temporal para que la función llamadora pueda accederlo.

```rust
fn ejecutar_return(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
    // Leer el valor a retornar
    let valor = self.leer_operando(&cuad.operando_izq)?;

    // Obtener marco actual
    let marco = self.pila_marcos.last_mut()
        .ok_or("No hay marco en la pila")?;

    // Guardar valor de retorno en un temporal del marco LLAMADOR
    // Necesitamos un temporal en el marco anterior

    // Estrategia: Guardar en dirección conocida o en el marco
    // Aquí lo guardamos en el campo direccion_resultado del marco

    // Asignar temporal para el resultado
    let dir_resultado = self.asignar_temporal_resultado()?;

    // Escribir el valor en memoria global (temporalmente)
    self.escribir_memoria(dir_resultado, valor)?;

    // Guardar la dirección en el marco para que EndFunc la use
    marco.direccion_resultado = Some(dir_resultado);

    Ok(())
}
```

### Simplificación Común

Muchas implementaciones usan un **registro global de retorno**:

```rust
struct MaquinaVirtual {
    // ...
    registro_retorno: Option<Valor>,
}

fn ejecutar_return(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
    let valor = self.leer_operando(&cuad.operando_izq)?;
    self.registro_retorno = Some(valor);
    Ok(())
}
```

Luego, después de `EndFunc`, el llamador copia el valor:

```rust
// Después de GOSUB, el siguiente cuádruplo puede ser:
// Asignacion registro_retorno, -, resultado
```

## 8.6 ENDFUNC: Fin de Función

### Qué hace ENDFUNC

**ENDFUNC** desempila el marco actual y retorna al IP guardado.

```rust
fn ejecutar_endfunc(&mut self, _cuad: &Cuadruplo) -> Result<(), String> {
    // Desempilar marco
    let marco = self.pila_marcos.pop()
        .ok_or("No hay marco para retornar")?;

    // Si hay valor de retorno, copiarlo a un temporal accesible
    if let Some(dir_resultado) = marco.direccion_resultado {
        // El valor ya está en memoria, accesible desde el marco anterior
        // No se necesita acción adicional
    }

    // Retornar al IP guardado
    self.ip = marco.direccion_retorno - 1; // -1 porque el loop hace IP++

    // Si la pila está vacía, terminamos
    if self.pila_marcos.is_empty() {
        self.ejecutando = false;
    }

    Ok(())
}
```

### Estado de la Pila

**Antes de ENDFUNC**:

```
┌──────────────────────────┐
│  Marco: suma             │  ← Tope
│  ├─ Resultado: @13000=8  │
│  └─ IP retorno: 10       │
├──────────────────────────┤
│  Marco: main             │
└──────────────────────────┘

IP: 5 (cuádruplo EndFunc)
```

**Después de ENDFUNC**:

```
┌──────────────────────────┐
│  Marco: main             │  ← Tope
└──────────────────────────┘

IP: 10 (dirección de retorno)
Memoria[@13000]: 8 (valor de retorno accesible)
```

## 8.7 Variables Locales vs Parámetros

### Variables Locales

Son variables declaradas dentro de la función:

```paradox
entero calcular(a : entero) {
    vars resultado : entero;  // ← Variable local
    resultado = a * 2;
    regresa resultado;
}
```

**Asignación de direcciones**:

- `a` (parámetro): `7000` (LOCAL_ENTERO_INICIO)
- `resultado` (local): `7001` (siguiente dirección local)

### Parámetros

Son variables que reciben valores del llamador:

```paradox
entero suma(a : entero, b : entero) {
    regresa a + b;
}
```

**Durante generación**:

1. `a` se asigna a `7000`
2. `b` se asigna a `7001`
3. Se registran como parámetros en el directorio de funciones

**Durante llamada**:

1. `ERA suma` crea marco temporal
2. `PARAM arg1, -, 0` copia valor a `7000` del marco temporal
3. `PARAM arg2, -, 1` copia valor a `7001` del marco temporal

## 8.8 Recursión

### Cómo Funciona

La recursión funciona naturalmente gracias a la pila de marcos:

```paradox
entero factorial(n : entero) {
    vars resultado : entero;
    si (n <= 1) entonces {
        regresa 1;
    } sino {
        resultado = factorial(n - 1);
        regresa n * resultado;
    }
}
```

**Estado de la pila durante `factorial(3)`**:

```
Llamada factorial(3)
┌──────────────────────────┐
│  Marco: factorial        │  ← Tope
│  ├─ n: 3                 │
│  └─ IP retorno: X        │
├──────────────────────────┤
│  Marco: main             │
└──────────────────────────┘

  ↓ Llamada factorial(2)

┌──────────────────────────┐
│  Marco: factorial        │  ← Tope
│  ├─ n: 2                 │
│  └─ IP retorno: Y        │
├──────────────────────────┤
│  Marco: factorial        │
│  ├─ n: 3                 │
│  └─ IP retorno: X        │
├──────────────────────────┤
│  Marco: main             │
└──────────────────────────┘

  ↓ Llamada factorial(1)

┌──────────────────────────┐
│  Marco: factorial        │  ← Tope
│  ├─ n: 1                 │
│  └─ IP retorno: Z        │
├──────────────────────────┤
│  Marco: factorial        │
│  ├─ n: 2                 │
│  └─ IP retorno: Y        │
├──────────────────────────┤
│  Marco: factorial        │
│  ├─ n: 3                 │
│  └─ IP retorno: X        │
├──────────────────────────┤
│  Marco: main             │
└──────────────────────────┘

  ↓ n <= 1, retorna 1

┌──────────────────────────┐
│  Marco: factorial        │  ← Tope
│  ├─ n: 2                 │
│  ├─ resultado: 1         │
│  └─ IP retorno: Y        │
├──────────────────────────┤
│  ...                     │
```

Cada llamada recursiva tiene su propio marco con su propia copia de `n`.

### Límite de Recursión

Para evitar stack overflow, puedes limitar la profundidad:

```rust
const MAX_PROFUNDIDAD_RECURSION: usize = 1000;

fn ejecutar_gosub(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
    if self.pila_marcos.len() >= MAX_PROFUNDIDAD_RECURSION {
        return Err("Error: Profundidad de recursión excedida".to_string());
    }

    // ... resto del código ...
}
```

## 8.9 Debugging de Funciones

### Trazar Llamadas

```rust
fn ejecutar_gosub(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
    let verbose = std::env::var("VERBOSE").unwrap_or_default();

    if verbose == "FUNCIONES" || verbose == "ALL" {
        let marco = self.marco_temporal.as_ref().unwrap();
        println!("[FUNC] Llamando a: {}", marco.nombre_funcion);
        println!("[FUNC] IP retorno: {}", marco.direccion_retorno);
        println!("[FUNC] Profundidad de pila: {}", self.pila_marcos.len() + 1);
    }

    // ... código de GOSUB ...

    Ok(())
}

fn ejecutar_endfunc(&mut self, _cuad: &Cuadruplo) -> Result<(), String> {
    let verbose = std::env::var("VERBOSE").unwrap_or_default();

    let marco = self.pila_marcos.last().unwrap();

    if verbose == "FUNCIONES" || verbose == "ALL" {
        println!("[FUNC] Retornando de: {}", marco.nombre_funcion);
        println!("[FUNC] IP destino: {}", marco.direccion_retorno);
        if let Some(dir) = marco.direccion_resultado {
            println!("[FUNC] Valor retorno en: @{}", dir);
        }
    }

    // ... código de EndFunc ...

    Ok(())
}
```

**Ejecutar**:

```bash
VERBOSE=FUNCIONES cargo run -- programa.txt
```

**Salida**:

```
[FUNC] Llamando a: suma
[FUNC] IP retorno: 10
[FUNC] Profundidad de pila: 2
...
[FUNC] Retornando de: suma
[FUNC] IP destino: 10
[FUNC] Valor retorno en: @13000
```

---

## Resumen

En esta parte cubrimos:

- Protocolo completo de llamadas (ERA → PARAM → GOSUB → RETURN → ENDFUNC)
- ERA: Crear marco temporal
- PARAM: Paso de parámetros
- GOSUB: Llamada a función
- RETURN: Retorno de valor
- ENDFUNC: Finalizar función
- Variables locales vs parámetros
- Recursión (stack de marcos)
- Debugging de funciones

**Siguiente**: [PARTE 9: Casos de Uso Prácticos →](PARTE_09_CASOS_USO.md)

[← Parte 7](PARTE_07_VM.md) | [Volver al índice](../../GUIA_TECNICA.md)
