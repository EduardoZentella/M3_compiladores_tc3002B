# Documentación de Estructuras de Datos - Tarea 1

## Descripción General

Este proyecto implementa tres estructuras de datos fundamentales en Rust, diseñadas para soportar las principales operaciones de acceso y manipulación de datos. Cada estructura está optimizada para casos de uso específicos y proporciona un manejo robusto de errores.

## Estructuras Implementadas

### 1. Stack (Pila) - LIFO (Last In, First Out)

#### Descripción

Una pila es una estructura de datos lineal que sigue el principio LIFO, donde el último elemento insertado es el primero en ser removido. Es como una pila de platos donde solo puedes agregar o quitar platos desde la parte superior.

#### Características Técnicas

- **Estructura interna**: Basada en `Vec<T>` de Rust
- **Capacidad**: Configurable (limitada o ilimitada)
- **Tipo genérico**: Soporta cualquier tipo `T`

#### Operaciones Principales

| Operación        | Complejidad     | Descripción                          |
| ---------------- | --------------- | ------------------------------------ |
| `push(elemento)` | O(1) amortizado | Insertar elemento en el tope         |
| `pop()`          | O(1)            | Remover y retornar elemento del tope |
| `peek()`         | O(1)            | Ver elemento del tope sin remover    |
| `esta_vacio()`   | O(1)            | Verificar si está vacía              |
| `tamanio()`      | O(1)            | Obtener número de elementos          |

#### Operaciones Adicionales

- `limpiar()`: Vaciar toda la pila
- `push_multiples()`: Insertar múltiples elementos
- `pop_multiples()`: Remover múltiples elementos
- `iter()`: Iterador desde el fondo al tope
- `iter_reverso()`: Iterador desde el tope al fondo

#### Casos de Uso

- **Evaluación de expresiones**: Manejo de paréntesis, operadores
- **Call Stack**: Manejo de llamadas a funciones
- **Undo/Redo**: Historial de acciones
- **Backtracking**: Algoritmos de búsqueda con retroceso
- **Parsing**: Análisis sintáctico y compiladores

#### Ejemplo de Uso

```rust
use estructuras::Stack;

let mut pila = Stack::nuevo();
pila.push(1).unwrap();
pila.push(2).unwrap();
pila.push(3).unwrap();

assert_eq!(pila.pop().unwrap(), 3); // LIFO: último en entrar, primero en salir
assert_eq!(pila.peek().unwrap(), &2); // Ver sin remover
```

### 2. Queue (Cola) - FIFO (First In, First Out)

#### Descripción

Una cola es una estructura de datos lineal que sigue el principio FIFO, donde el primer elemento insertado es el primero en ser removido. Es como una fila de personas donde el primero en llegar es el primero en ser atendido.

#### Características Técnicas

- **Estructura interna**: Basada en `VecDeque<T>` de Rust
- **Capacidad**: Configurable (limitada o ilimitada)
- **Eficiencia**: Operaciones O(1) en ambos extremos

#### Operaciones Principales

| Operación           | Complejidad     | Descripción                        |
| ------------------- | --------------- | ---------------------------------- |
| `encolar(elemento)` | O(1) amortizado | Insertar elemento al final         |
| `desencolar()`      | O(1)            | Remover y retornar primer elemento |
| `frente()`          | O(1)            | Ver primer elemento sin remover    |
| `ultimo()`          | O(1)            | Ver último elemento sin remover    |
| `esta_vacia()`      | O(1)            | Verificar si está vacía            |
| `tamanio()`         | O(1)            | Obtener número de elementos        |

#### Operaciones Adicionales

- `limpiar()`: Vaciar toda la cola
- `encolar_multiples()`: Insertar múltiples elementos
- `desencolar_multiples()`: Remover múltiples elementos
- `obtener(indice)`: Acceso por índice
- `iter()`: Iterador sobre elementos

#### Casos de Uso

- **Sistemas de colas**: Task queues, job scheduling
- **Buffering**: Streaming de datos, I/O buffers
- **BFS**: Algoritmos de búsqueda en anchura
- **Simulaciones**: Modelado de procesos y eventos
- **Sistemas operativos**: Scheduling de procesos

#### Ejemplo de Uso

```rust
use estructuras::Cola;

let mut cola = Cola::nueva();
cola.encolar(1).unwrap();
cola.encolar(2).unwrap();
cola.encolar(3).unwrap();

assert_eq!(cola.desencolar().unwrap(), 1); // FIFO: primero en entrar, primero en salir
assert_eq!(cola.frente().unwrap(), &2); // Ver sin remover
```

### 3. HashMap (Tabla Hash/Diccionario)

#### Descripción

Un HashMap es una estructura de datos que implementa un mapeo asociativo entre claves únicas y valores. Utiliza una función hash para calcular un índice donde almacenar cada par clave-valor, permitiendo acceso extremadamente rápido.

#### Características Técnicas

- **Estructura interna**: Basada en `HashMap<K, V>` de Rust
- **Claves**: Deben implementar `Hash + Eq`
- **Capacidad**: Configurable (limitada o ilimitada)
- **Factor de carga**: Manejo automático de redimensionamiento

#### Operaciones Principales

| Operación                | Complejidad Promedio | Complejidad Peor Caso | Descripción                   |
| ------------------------ | -------------------- | --------------------- | ----------------------------- |
| `insertar(clave, valor)` | O(1)                 | O(n)                  | Insertar par clave-valor      |
| `obtener(clave)`         | O(1)                 | O(n)                  | Obtener valor por clave       |
| `remover(clave)`         | O(1)                 | O(n)                  | Remover par por clave         |
| `contiene_clave(clave)`  | O(1)                 | O(n)                  | Verificar existencia de clave |
| `esta_vacio()`           | O(1)                 | O(1)                  | Verificar si está vacío       |
| `tamanio()`              | O(1)                 | O(1)                  | Obtener número de elementos   |

#### Operaciones Adicionales

- `actualizar()`: Modificar valor existente
- `insertar_o_actualizar()`: Upsert operation
- `claves()`: Obtener todas las claves
- `valores()`: Obtener todos los valores
- `entradas()`: Obtener todos los pares clave-valor
- `insertar_multiples()`: Insertar múltiples pares
- `fusionar()`: Combinar con otro HashMap

#### Casos de Uso

- **Cachés**: Almacenamiento temporal de datos
- **Índices**: Bases de datos en memoria
- **Contadores**: Frecuencias y estadísticas
- **Configuraciones**: Mapeo de parámetros
- **Tablas de símbolos**: Compiladores e intérpretes
- **Memoización**: Optimización de algoritmos recursivos

#### Ejemplo de Uso

```rust
use estructuras::MiHashMap;

let mut mapa = MiHashMap::nuevo();
mapa.insertar("nombre".to_string(), "Juan".to_string()).unwrap();
mapa.insertar("edad".to_string(), "25".to_string()).unwrap();

assert_eq!(mapa.obtener(&"nombre".to_string()).unwrap(), &"Juan".to_string());
assert!(mapa.contiene_clave(&"edad".to_string()));
```

## Manejo de Errores

Todas las estructuras implementan un sistema robusto de manejo de errores usando enums específicos:

### ErrorStack

- `StackVacio`: Intento de acceso a pila vacía
- `CapacidadExcedida`: Exceso de capacidad máxima
- `IndiceInvalido`: Índice fuera de rango

### ErrorCola

- `ColaVacia`: Intento de acceso a cola vacía
- `CapacidadExcedida`: Exceso de capacidad máxima
- `IndiceInvalido`: Índice fuera de rango

### ErrorHashMap

- `HashMapVacio`: Intento de acceso a HashMap vacío
- `CapacidadExcedida`: Exceso de capacidad máxima
- `ClaveNoEncontrada`: Clave no existe en el mapa

## Comparación de Complejidades

| Operación           | Stack  | Queue  | HashMap  |
| ------------------- | ------ | ------ | -------- |
| Inserción           | O(1)\* | O(1)\* | O(1)\*\* |
| Eliminación         | O(1)   | O(1)   | O(1)\*\* |
| Búsqueda            | O(n)   | O(n)   | O(1)\*\* |
| Acceso por posición | O(1)   | O(1)   | N/A      |

\*Amortizado
\*\*Promedio, O(n) en el peor caso

## Consideraciones de Memoria

### Stack

- **Ventajas**: Locality of reference excelente, overhead mínimo
- **Desventajas**: Realocaciones ocasionales al crecer

### Queue

- **Ventajas**: Eficiente en ambos extremos, sin realocaciones frecuentes
- **Desventajas**: Overhead ligeramente mayor que Vec

### HashMap

- **Ventajas**: Acceso extremadamente rápido, escalabilidad excelente
- **Desventajas**: Overhead de memoria por factor de carga, no mantiene orden

## Tests y Validación

Cada estructura incluye tests exhaustivos que cubren:

- **Funcionalidad básica**: Operaciones CRUD
- **Casos límite**: Estructuras vacías, capacidades máximas
- **Manejo de errores**: Validación de todos los tipos de error
- **Rendimiento**: Tests de operaciones múltiples
- **Concurrencia**: Behavior en escenarios multi-hilo (donde aplique)

## Conclusiones

Este conjunto de estructuras de datos proporciona una base sólida para el desarrollo de aplicaciones que requieren:

1. **Eficiencia**: Complejidades optimales para cada tipo de operación
2. **Robustez**: Manejo completo de errores y casos límite
3. **Flexibilidad**: Configuración de capacidades y comportamientos
4. **Usabilidad**: APIs consistentes e intuitivas

Cada estructura está diseñada para casos de uso específicos, y la elección entre ellas debe basarse en los patrones de acceso a datos requeridos por la aplicación.
