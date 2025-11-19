# Programas de Prueba

Este directorio contiene programas de ejemplo en lenguaje Patito para probar el compilador.

## Organización

### Programas Básicos

- `test_minimal.txt` - Programa mínimo válido
- `test_simple.txt` - Programa simple con variables
- `test_var_assign.txt` - Asignación de variables
- `test_just_var.txt` - Solo declaración de variables
- `test_two_assigns.txt` - Múltiples asignaciones
- `test_solo_asign.txt` - Asignación simple
- `test_two_consts.txt` - Uso de constantes
- `test_with_spaces.txt` - Programa con espacios adicionales
- `test_fresh.txt` - Programa de prueba general

### Expresiones

- `test_expr.txt` - Expresiones aritméticas

### Entrada/Salida

- `test_escribe_var.txt` - Escritura de variables
- `test_escribe_letrero.txt` - Escritura de letreros/strings

### Control de Flujo (con Constantes)

Estos tests funcionan correctamente:

- `test_if_const.txt` - IF simple con constantes
- `test_if_simple.txt` - IF básico
- `test_if_else_const.txt` - IF-ELSE con constantes
- `test_while_const.txt` - WHILE con constantes
- `test_complejo_const.txt` - Estructuras anidadas (WHILE + IF-ELSE)
- `test_control_flow.txt` - Control de flujo general

### Control de Flujo (con Variables)

Estos tests tienen problemas conocidos debido a un conflicto en la tabla SLR:

- `test_if_var.txt` - IF con variable en condición (Error estado 32)
- `test_if_else.txt` - IF-ELSE con variable (Error estado 32)
- `test_while.txt` - WHILE con variable (Error estado 32)

**Problema**: La tabla SLR tiene un conflicto en estado 32 cuando procesa variables en expresiones relacionales.
**Workaround**: Usar solo constantes en condiciones por ahora.

## Ejecutar Tests

### Compilar un Programa

```bash
# Desde la raíz del proyecto
cargo run --bin compilador_rust tests/programas/NOMBRE_ARCHIVO.txt

# Ejemplos:
cargo run --bin compilador_rust tests/programas/test_if_const.txt
cargo run --bin compilador_rust tests/programas/test_while_const.txt
cargo run --bin compilador_rust tests/programas/test_complejo_const.txt
```

### Tests Recomendados

```bash
# Tests básicos
cargo run --bin compilador_rust tests/programas/test_simple.txt
cargo run --bin compilador_rust tests/programas/test_expr.txt

# Tests de control de flujo (todos deben pasar)
cargo run --bin compilador_rust tests/programas/test_if_const.txt
cargo run --bin compilador_rust tests/programas/test_if_else_const.txt
cargo run --bin compilador_rust tests/programas/test_while_const.txt
cargo run --bin compilador_rust tests/programas/test_complejo_const.txt

# Tests de E/S
cargo run --bin compilador_rust tests/programas/test_escribe_var.txt
```

## Cuádruplos Esperados

### test_if_const.txt

```
0: a = 10
1: t0 = 1 > 2
2: GOTOF t0, goto 4
3: a = 5
```

### test_if_else_const.txt

```
0: a = 10
1: t0 = 1 > 2
2: GOTOF t0, goto 5    ← Salta a else
3: a = 100             ← Then
4: GOTO 6              ← Salta después de else
5: a = 200             ← Else
```

### test_while_const.txt

```
0: i = 0
1: t0 = 1 < 10         ← Inicio ciclo
2: GOTOF t0, goto 5    ← Salir si falso
3: i = 5               ← Cuerpo
4: GOTO 1              ← Volver a inicio
```

### test_complejo_const.txt

```
0: suma = 0
1: i = 1
2: t0 = 1 < 10         ← WHILE condición
3: GOTOF t0, goto 11   ← WHILE salida
4: t0 = 2 > 1          ← IF condición
5: GOTOF t0, goto 8    ← IF saltar a else
6: suma = 10           ← IF then
7: GOTO 9              ← IF saltar else
8: suma = 20           ← IF else
9: i = 2               ← Continuar después de IF
10: GOTO 2             ← WHILE volver
```

## Agregar Nuevos Tests

Para agregar un nuevo programa de prueba:

1. Crear archivo `.txt` en este directorio
2. Seguir la sintaxis de Patito (ver gramática en `gramatica.txt`)
3. Usar constantes en condiciones (por limitación actual)
4. Ejecutar con el comando mostrado arriba
5. Verificar los cuádruplos generados

## Convenciones

- Usar nombres descriptivos: `test_FUNCIONALIDAD.txt`
- Incluir comentarios si el test es complejo
- Mantener programas pequeños y enfocados
- Para estructuras de control, preferir sufijo `_const` si usa constantes
