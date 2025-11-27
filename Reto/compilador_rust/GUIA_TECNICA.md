# GUÍA TÉCNICA COMPLETA DEL COMPILADOR PATITO

## Índice General

Esta guía técnica exhaustiva te permitirá entender y modificar cualquier aspecto del compilador desde cero.

### Documentos de la Guía

1. **[PARTE 1: Arquitectura y Flujo General](docs/guia/PARTE_01_ARQUITECTURA.md)**

   - Visión general del compilador
   - Flujo de datos entre módulos
   - Estructuras de datos principales
   - Convenciones del proyecto

2. **[PARTE 2: Análisis Léxico](docs/guia/PARTE_02_LEXICO.md)**

   - Cómo funciona el tokenizador
   - Agregar nuevos tokens
   - Modificar palabras reservadas
   - Manejo de errores léxicos

3. **[PARTE 3: Análisis Sintáctico](docs/guia/PARTE_03_SINTACTICO.md)**

   - Parser SLR(1) explicado
   - Modificar la gramática
   - Regenerar tablas SLR
   - Acciones semánticas durante el parsing

4. **[PARTE 4: Análisis Semántico](docs/guia/PARTE_04_SEMANTICO.md)**

   - Cubo semántico: agregar/modificar operadores
   - Tabla de variables y alcances
   - Directorio de funciones
   - Contexto semántico y validaciones

5. **[PARTE 5: Generación de Código Intermedio](docs/guia/PARTE_05_INTERMEDIO.md)**

   - Sistema de cuádruplos
   - Puntos neurálgicos completos
   - Pilas de traducción (POper, PilaO, PTypes)
   - Agregar nuevos tipos de cuádruplos

6. **[PARTE 6: Memoria Virtual](docs/guia/PARTE_06_MEMORIA.md)**

   - Arquitectura de memoria segmentada
   - Pool de temporales (AVAIL)
   - Modificar rangos de memoria
   - Deduplicación de constantes

7. **[PARTE 7: Máquina Virtual](docs/guia/PARTE_07_VM.md)**

   - Ejecutor de cuádruplos
   - Segmentos de memoria en ejecución
   - Stack de marcos para funciones
   - Agregar nuevas operaciones

8. **[PARTE 8: Funciones](docs/guia/PARTE_08_FUNCIONES.md)**

   - Protocolo completo de llamadas
   - ERA, PARAM, GOSUB, RETURN, ENDFUNC
   - Variables locales y parámetros
   - Recursión

9. **[PARTE 9: Casos de Uso Prácticos](docs/guia/PARTE_09_CASOS_USO.md)**

   - Cómo agregar un nuevo operador
   - Cómo agregar un nuevo tipo de dato
   - Cómo agregar una nueva estructura de control
   - Cómo modificar el cubo semántico
   - Cómo cambiar rangos de memoria

10. **[PARTE 10: Testing y Debugging](docs/guia/PARTE_10_TESTING.md)**
    - Estrategias de testing
    - Cómo escribir tests
    - Debugging con verbose
    - Validación de cuádruplos

## Cómo Usar Esta Guía

1. **Lectura Secuencial**: Si eres nuevo, lee las partes en orden del 1 al 10
2. **Consulta Rápida**: Si necesitas modificar algo específico, ve directo a la parte correspondiente
3. **Casos Prácticos**: La Parte 9 tiene ejemplos paso a paso para cambios comunes

## Conceptos Clave

Antes de empezar, familiarízate con estos términos:

- **Token**: Unidad léxica (palabra reservada, identificador, número, etc.)
- **Cuádruplo**: Instrucción de código intermedio (operador, op1, op2, resultado)
- **Punto Neurálgico**: Acción semántica ejecutada en un punto específico del parsing
- **Direccion Virtual**: Número que representa una ubicación de memoria (1000-24999)
- **AVAIL**: Pool de direcciones temporales reutilizables
- **Marco**: Estructura para mantener variables locales de una función

## Estructura del Proyecto

```
src/
├── lexico/           # Análisis léxico (tokenización)
│   ├── mod.rs
│   └── token.rs
├── gramatica/        # Análisis de gramáticas
│   ├── mod.rs
│   ├── parser.rs
│   ├── first_follow.rs
│   └── lr0.rs
├── sintactico/       # Parser SLR(1)
│   ├── mod.rs
│   ├── tabla_slr.rs
│   └── acciones_semanticas.rs
├── semantico/        # Análisis semántico
│   ├── mod.rs
│   ├── cubo_semantico.rs
│   ├── tabla_variables.rs
│   ├── directorio_funciones.rs
│   └── contexto_semantico.rs
├── intermedio/       # Código intermedio
│   ├── mod.rs
│   ├── cuadruplo.rs
│   ├── generador.rs
│   ├── memoria_virtual.rs
│   └── programa.rs
├── vm/              # Máquina virtual
│   ├── mod.rs
│   ├── ejecutor.rs
│   └── memoria.rs
├── bin/             # Ejecutables de prueba
└── lib.rs           # Módulo raíz
```

## Quick Start para Modificaciones

### Agregar un Operador Nuevo

→ Ver **Parte 9, Sección 1**: Paso a paso completo

### Cambiar Rangos de Memoria

→ Ver **Parte 6, Sección 3**: Modificar constantes de memoria

### Agregar un Tipo de Dato

→ Ver **Parte 9, Sección 2**: Proceso completo

### Modificar Gramática

→ Ver **Parte 3, Sección 2**: Editar gramática y regenerar tablas

---

**Comienza con [PARTE 1: Arquitectura y Flujo General](docs/guia/PARTE_01_ARQUITECTURA.md)**
