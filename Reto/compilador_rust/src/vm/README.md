# Máquina Virtual (VM)

Módulo para la ejecución de código intermedio (cuádruplos) generado por el compilador de Patito.

## Estructura Modular

```
src/vm/
├── mod.rs          # Módulo principal y re-exportaciones
├── memoria.rs      # Gestión de memoria segmentada y stack frames
└── ejecutor.rs     # Máquina virtual y ejecución de cuádruplos
```

## Arquitectura

### Memoria Segmentada

La VM utiliza un modelo de memoria segmentada con direcciones virtuales:

| Segmento  | Rango       | Propósito                                  |
| --------- | ----------- | ------------------------------------------ |
| GLOBAL    | 1000-4999   | Variables globales del programa            |
| LOCAL     | 5000-9999   | Variables locales de funciones             |
| TEMPORAL  | 10000-14999 | Valores temporales durante expresiones     |
| CONSTANTE | 15000-24999 | Constantes literales (enteros y flotantes) |

### Stack Frames (Marcos de Memoria)

Cada llamada a función crea un `MarcoMemoria` con:

- Memoria local (parámetros y variables locales)
- Memoria temporal (resultados intermedios)
- IP de retorno (dirección donde continuar después de retornar)
- Valor de retorno (si la función retorna un valor)
- Lista de parámetros recibidos

## Módulos

### `memoria.rs`

Contiene:

- **`Valor`**: Enum para valores enteros y flotantes

  - `Valor::Entero(i32)`
  - `Valor::Flotante(f64)`
  - Métodos: `a_entero()`, `a_flotante()`, `operar_aritmetica()`, `operar_relacional()`

- **`SegmentoMemoria`**: Almacén de valores separado por tipo

  - `leer_entero(offset)`, `escribir_entero(offset, valor)`
  - `leer_flotante(offset)`, `escribir_flotante(offset, valor)`
  - `leer_valor(offset, es_flotante)`, `escribir_valor(offset, valor)`

- **`MarcoMemoria`**: Contexto de ejecución de una función

  - `nombre_funcion`: Nombre para debugging
  - `memoria_local`: Variables y parámetros de la función
  - `memoria_temporal`: Temporales de la función
  - `ip_retorno`: Dirección de retorno
  - `valor_retorno`: Valor que retorna la función
  - `parametros`: Lista de parámetros recibidos

- **`TipoSegmento`**: Enum para tipos de segmento de memoria
- **`traducir_direccion(dir)`**: Traduce dirección virtual a (segmento, offset)

### `ejecutor.rs`

Contiene:

- **`MaquinaVirtual`**: Ejecutor principal de cuádruplos

  - `cuadruplos`: Lista de instrucciones a ejecutar
  - `ip`: Instruction Pointer (contador de programa)
  - `memoria_global`: Segmento de memoria global
  - `memoria_constantes`: Segmento de constantes
  - `pila_marcos`: Stack de marcos de memoria
  - `tabla_funciones`: Mapeo nombre -> dirección de funciones
  - `marco_temporal`: Marco temporal durante llamadas a función
  - `ejecutando`: Flag de ejecución

- **`InfoFuncion`**: Información de una función en la tabla
  - `nombre`: Nombre de la función
  - `direccion_inicio`: Dirección del primer cuádruplo
  - `tiene_retorno`: Si retorna un valor

### Operadores Implementados

#### Aritméticos

- **Suma** (`+`): `resultado = op1 + op2`
- **Resta** (`-`): `resultado = op1 - op2`
- **Multiplicación** (`*`): `resultado = op1 * op2`
- **División** (`/`): `resultado = op1 / op2` (con verificación de división por cero)

#### Relacionales

- **Mayor que** (`>`): `resultado = op1 > op2`
- **Menor que** (`<`): `resultado = op1 < op2`
- **Igual** (`==`): `resultado = op1 == op2`
- **Diferente** (`!=`): `resultado = op1 != op2`

Los operadores relacionales retornan `Valor::Entero(1)` para verdadero y `Valor::Entero(0)` para falso.

#### Control de Flujo

- **Asignación** (`=`): `destino = fuente`
- **Goto**: Salto incondicional a etiqueta
- **GotoF**: Salto si condición es falsa (0)
- **GotoV**: Salto si condición es verdadera (≠0)

#### E/S

- **Escritura**: Imprime valor en consola
- **Lectura**: (Pendiente de implementar)

#### Funciones

- **Era**: Crea registro de activación temporal para llamada
- **Parametro**: Copia parámetro al marco temporal
- **GoSub**: Invoca función (push marco y salta)
- **EndFunc**: Retorna de función (pop marco)
- **Return**: Guarda valor de retorno

## Flujo de Ejecución de Funciones

1. **Era** → Crea `MarcoMemoria` temporal con nombre de función
2. **Parametro(s)** → Copia valores de parámetros al marco temporal
3. **GoSub** → Push marco a pila, salta a dirección de función
4. _Ejecución de la función..._
5. **Return** (opcional) → Guarda valor de retorno en marco
6. **EndFunc** → Pop marco, restaura IP de retorno

### Resolución de Operandos

El método `leer_operando(&operando)` maneja diferentes tipos de operandos:

- **Constantes literales**: Devuelve `Valor` directamente
- **Direcciones**: Traduce y lee de memoria
- **Temporales**: Calcula dirección en segmento TEMPORAL
- **Etiquetas**: Usa dirección de salto

### Gestión de la Pila

La pila de marcos (`pila_marcos`) funciona como una call stack tradicional:

- Cada llamada a función hace **push** de un nuevo marco
- Cada retorno hace **pop** del marco actual
- El marco en el tope (`last()`) es el contexto actual de ejecución

## Pendientes

- [ ] Implementar operador **Lectura** (lee desde consola)
- [ ] Mejorar `construir_tabla_funciones()` para detectar funciones automáticamente
- [ ] Agregar soporte para strings en memoria
- [ ] Implementar verificación de tipos en tiempo de ejecución
- [ ] Agregar modo de debugging con breakpoints

## 📚 Referencias

- [Diseño de Compiladores - Aho, Sethi, Ullman](https://en.wikipedia.org/wiki/Compilers:_Principles,_Techniques,_and_Tools)
- [Virtual Machine Design - Stack-based vs Register-based](https://en.wikipedia.org/wiki/Virtual_machine)
- [Call Stack - Wikipedia](https://en.wikipedia.org/wiki/Call_stack)
