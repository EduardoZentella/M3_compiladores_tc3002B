# Tests del Compilador Patito - Validación de Rúbrica

Este directorio contiene programas de prueba diseñados para validar cada dimensión de la rúbrica del proyecto.

## Estructura de Tests

### Dimensión A: Expresiones Aritméticas

**Archivo:** `01_expresiones_aritmeticas.txt`

- **Prueba:** Operadores +, -, \*, / con precedencia correcta
- **Validación:** Cuádruplos generados en orden correcto
- **Ejemplo:** `c = a + b * 2` debe generar `MULT`, luego `SUMA`

### Dimensión B: Entrada/Salida

**Archivo:** `02_entrada_salida.txt`

- **Prueba:** Estatutos `escribe()` con variables y letreros
- **Validación:** Cuádruplos de tipo `Escritura` generados
- **Nota:** Lectura (`lee`) no implementada actualmente

### Dimensión C: Decisiones

**Archivo:** `03_decisiones.txt`

- **Prueba:**
  - `si (condicion) entonces { ... };`
  - `si (condicion) entonces { ... } sino { ... };`
  - Decisiones anidadas
- **Validación:**
  - Cuádruplos `GOTOF` (salto condicional falso)
  - Cuádruplos `GOTO` (salto incondicional para else)
  - Fill correcto de saltos pendientes

### Dimensión D: Ciclos

**Archivo:** `04_ciclos.txt`

- **Prueba:**
  - `mientras (condicion) haz { ... }`
  - Ciclos múltiples
  - Acumuladores y contadores
- **Validación:**
  - Marca de inicio de ciclo guardada
  - `GOTOF` para salir del ciclo
  - `GOTO` para regresar al inicio
  - Fill correcto de saltos

### Dimensión E: Funciones (Definición)

**Archivos:**

- `05_factorial_funcion.txt`
- `06_fibonacci_funcion.txt`
- `09_multiples_funciones.txt`

**Prueba:**

- Definición de funciones con tipo de retorno
- Parámetros formales con tipos
- Variables locales dentro de funciones
- Múltiples funciones en un programa

**Validación:**

- Directorio de funciones poblado correctamente
- Cuádruplo `ENDFUNC` al final de cada función
- Alcance de variables (local vs global)

### Dimensión F: Llamadas a Funciones

**Archivos:**

- `05_factorial_funcion.txt`
- `06_fibonacci_funcion.txt`
- `09_multiples_funciones.txt`
- `10_programa_completo.txt`

**Prueba:**

- Llamadas a funciones desde main
- Paso de parámetros
- Asignación del valor de retorno

**Validación:**

- Cuádruplo `ERA` (Activation Record)
- Cuádruplos `PARAM` para cada parámetro
- Cuádruplo `GOSUB` para saltar a la función
- Verificación de número y tipo de parámetros

## Tests Sin Funciones (Solo Main)

### `07_factorial_main.txt`

Calcula factorial usando solo el bloque `inicio` (sin funciones)

- Valida: Ciclos, expresiones, variables

### `08_fibonacci_main.txt`

Genera serie Fibonacci usando solo el bloque `inicio`

- Valida: Ciclos, asignaciones múltiples, escribe en ciclo

## Programa Completo - Integración Total

### `10_programa_completo.txt`

Combina todas las dimensiones:

- Variables globales y locales
- Funciones con parámetros
- Llamadas a funciones
- Ciclos dentro de funciones
- Decisiones con else
- Expresiones aritméticas y relacionales
- Entrada/salida

## Cómo Ejecutar los Tests

### Ejecución Individual

```bash
# Sin verbose (solo output)
cargo run -- tests/programas/01_expresiones_aritmeticas.txt

# Con verbose nivel 1 (fases principales)
cargo run -- tests/programas/02_entrada_salida.txt -v

# Con verbose nivel 2 (acciones semánticas)
cargo run -- tests/programas/03_decisiones.txt -vv

# Con verbose nivel 3 (debug completo)
cargo run -- tests/programas/04_ciclos.txt -vvv
```

### Ejecutar Todos los Tests

```bash
#!/bin/bash
for file in tests/programas/*.txt; do
    echo "========================================="
    echo "Testing: $file"
    echo "========================================="
    cargo run -- "$file" -v
    echo ""
done
```

## Mapeo a Rúbrica

| Dimensión | Descripción                                                                    | Tests que Validan |
| --------- | ------------------------------------------------------------------------------ | ----------------- |
| **A**     | Expresiones aritméticas correctamente parentizadas con jerarquía de operadores | 01, 07, 08, 10    |
| **B**     | Estatutos de I/O (lectura y escritura)                                         | 02, 07, 08, 10    |
| **C**     | Estatuto condicional (decisión if/else)                                        | 03, 10            |
| **D**     | Estatuto cíclico (while)                                                       | 04, 07, 08, 10    |
| **E**     | Definir funciones con parámetros                                               | 05, 06, 09, 10    |
| **F**     | Llamar funciones con parámetros                                                | 05, 06, 09, 10    |

## Cuádruplos Esperados por Dimensión

### A: Expresiones

- `Suma`, `Resta`, `Multiplicacion`, `Division`
- `Asignacion` de resultados a temporales y variables

### B: I/O

- `Escritura` con direcciones de variables o constantes

### C: Decisiones

- `Mayor`, `Menor`, `Igual`, `Diferente` (comparaciones)
- `GOTOF` (goto falso - salta si condición es falsa)
- `GOTO` (goto incondicional - para else)

### D: Ciclos

- `GOTOF` (salir del ciclo si condición falsa)
- `GOTO` (regresar al inicio del ciclo)

### E/F: Funciones

- `ERA` (establecer registro de activación)
- `PARAM` (pasar parámetros)
- `GOSUB` (llamar función)
- `ENDFUNC` (fin de función)
- `RETURN` (retornar valor - si aplica)

## Estado de Implementación

### Completamente Implementado

- [x] Expresiones aritméticas con precedencia
- [x] Generación de cuádruplos
- [x] Memoria virtual (segmentada 1000-24999)
- [x] Escritura (`escribe`)
- [x] Decisiones (if/else) con GOTOF/GOTO
- [x] Ciclos (while) con saltos
- [x] Definición de funciones
- [x] Parámetros en funciones
- [x] Directorio de funciones
- [x] Tabla de variables por alcance
- [x] Máquina Virtual ejecuta cuádruplos básicos

### Parcialmente Implementado

- [ ] Lectura (`lee`) - estructura creada pero no implementada
- [ ] Llamadas a funciones - ERA/GOSUB/ENDFUNC generados, falta VM
- [ ] Retorno de valores - estructura presente, falta integración completa

### Notas Importantes

1. **Ejecución Actual**: Los tests 01-04, 07-08 funcionan completamente (sin funciones)

2. **Direcciones Virtuales**: Sistema completamente funcional
   - GLOBAL: 1000-6999
   - LOCAL: 7000-12999
   - TEMPORAL: 13000-18999
   - CONSTANTE: 19000-24999

## Salida Esperada

Cada test debe:

1. Compilar sin errores sintácticos ni semánticos
2. Generar cuádruplos correctos
3. Mostrar el código intermedio generado
4. Ejecutar en la VM (tests sin funciones: 01-04, 07-08)
5. Ejecutar funciones (requiere completar VM: 05-06, 09-10)
