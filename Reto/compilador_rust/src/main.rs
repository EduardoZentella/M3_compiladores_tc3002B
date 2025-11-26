//! # Compilador Principal
//!
//! Este es el punto de entrada principal del compilador. Coordina las diferentes
//! fases de compilación: análisis léxico y análisis sintáctico.
//!
//! ## Uso
//! ```bash
//! cargo run -- archivo.txt          # Compilar sin detalles
//! cargo run -- archivo.txt -v       # Compilar con salida detallada
//! cargo run -- archivo.txt --verbose # Compilar con salida detallada
//! ```
//!
//! ## Fases de Compilación
//! 1. **Análisis Léxico**: Convierte el código fuente en tokens
//! 2. **Análisis Sintáctico**: Valida la estructura gramatical usando SLR(1)
//!
//! ## Ejemplo de Salida (Modo Verbose)
//! ```
//! Analizando archivo: /path/to/programa.txt
//!
//! === Iniciando análisis léxico ===
//! Token encontrado: Programa ('programa') en linea 1
//! Token encontrado: Id ('test') en linea 1
//! ...
//! ✓ Análisis léxico completado. Tokens generados: 15
//!
//! === Iniciando análisis sintáctico ===
//! Estado: 0 | Token: 'programa' | Pila: [0]
//! ...
//! ✓✓✓ Compilación exitosa ✓✓✓
//! ```

// Usar módulos de la biblioteca
use compilador_rust::lexico;
use compilador_rust::sintactico;
use compilador_rust::intermedio::GeneradorCuadruplos;
use compilador_rust::vm::{MaquinaVirtual, ConsolaIO};

/// Obtiene la ruta del archivo a compilar y determina el nivel de verbose.
///
/// # Comportamiento
/// - Si hay argumentos en la línea de comandos, los usa
/// - Si no, solicita interactivamente la ruta al usuario
/// - Detecta flags `-v`, `-vv`, `-vvv` para niveles de verbose (1, 2, 3)
/// - Convierte rutas relativas a absolutas usando `canonicalize`
///
/// # Retorna
/// Una tupla `(ruta_absoluta, nivel_verbose)`:
/// - `ruta_absoluta`: Path completo del archivo a compilar
/// - `nivel_verbose`: 0 (sin debug), 1 (-v), 2 (-vv), 3 (-vvv)
///
/// # Panics
/// Termina el programa (exit 1) si no se puede resolver la ruta del archivo
fn obtain_file_path() -> (String, usize) {
    // Se obtiene el path del archivo por argumentos o se solicita al usuario
    let mut path = String::new();
    let args: Vec<String> = std::env::args().collect();

    // Verificar si hay argumentos adicionales
    if args.len() > 1 {
        // Concatenar todos los argumentos (excepto el nombre del programa)
        path = args[1..].join(" ");
    } else {
        println!("Ingrese el path del archivo a analizar (agregue -v/-vv/-vvv para debug):");
        std::io::stdin().read_line(&mut path).expect("Error al leer la entrada");
    }

    // Determinar nivel de verbose
    let nivel_verbose = if path.contains("-vvv") {
        3
    } else if path.contains("-vv") {
        2
    } else if path.contains("-v") {
        1
    } else {
        0
    };

    // Limpiar flags de verbose del path
    path = path.replace("-vvv", "").replace("-vv", "").replace("-v", "").trim().to_string();

    // Se transforma el path a un full path
    let full_path = match std::fs::canonicalize(&path) {
        Ok(p) => p.to_string_lossy().to_string(),
        Err(e) => {
            eprintln!("Error al obtener el path completo de '{}': {}", path, e);
            std::process::exit(1);
        }
    };

    (full_path, nivel_verbose)
}

/// Imprime un mensaje según el nivel de verbose configurado.
///
/// # Parámetros
/// - `message`: El mensaje a imprimir
/// - `nivel_requerido`: Nivel mínimo necesario para mostrar este mensaje (1-3)
/// - `nivel_actual`: Nivel de verbose actual (0-3)
///
/// # Niveles de Verbose
/// - Nivel 0: Solo output final (sin debug)
/// - Nivel 1 (-v): Fases principales del compilador
/// - Nivel 2 (-vv): + Acciones semánticas importantes
/// - Nivel 3 (-vvv): + Debug completo (reduce, atributos, tokens)
///
/// # Ejemplo
/// ```
/// verbose_log("Iniciando parser...", 1, 2);  // Imprime (nivel_actual >= 1)
/// verbose_log("Token: Id('x')", 3, 1);       // NO imprime (nivel_actual < 3)
/// ```
fn verbose_log(message: &str, nivel_requerido: usize, nivel_actual: usize) {
    if nivel_actual >= nivel_requerido {
        println!("{}", message);
    }
}

/// Función principal del compilador.
///
/// # Flujo de Ejecución
/// 1. Obtener ruta del archivo y modo verbose
/// 2. Leer contenido del archivo fuente
/// 3. **Fase 1 - Análisis Léxico**: Convertir código en tokens
/// 4. **Fase 2 - Análisis Sintáctico**: Validar estructura con parser SLR
/// 5. Reportar éxito o error
///
/// # Manejo de Errores
/// - Errores de lectura de archivo → Termina con mensaje de error
/// - Errores léxicos → Termina mostrando línea del error
/// - Errores sintácticos → Termina mostrando token y línea del error
///
/// # Ejemplo de Uso
/// ```bash
/// # Compilar programa simple
/// cargo run -- ejemplos/test.txt
///
/// # Compilar con detalles del proceso
/// cargo run -- ejemplos/test.txt -v
/// ```
fn main() {
    // PASO 1: Obtener configuración (ruta y nivel de verbose)
    let (full_path, nivel_verbose) = obtain_file_path();

    // PASO 2: Leer archivo fuente
    // El compilador trabaja sobre el contenido completo en memoria
    let contenido = match std::fs::read_to_string(&full_path) {
        Ok(cont) => cont,
        Err(e) => {
            eprintln!("Error al leer el archivo '{}': {}", full_path, e);
            return;
        }
    };

    verbose_log(&format!("Analizando archivo: {}", full_path), 1, nivel_verbose);
    verbose_log("\n=== Iniciando análisis léxico ===\n", 1, nivel_verbose);

    // PASO 3: ANÁLISIS LÉXICO
    // Convierte el código fuente (String) en una secuencia de tokens
    // Cada token representa un elemento léxico: palabra reservada, identificador, operador, etc.
    let tokens = match lexico::analyze(&contenido, nivel_verbose) {
        Ok(toks) => {
            verbose_log(&format!("\n✓ Análisis léxico completado. Tokens generados: {}\n", toks.len()), 1, nivel_verbose);
            toks
        },
        Err(e) => {
            eprintln!("✗ Error en el análisis léxico: {}", e);
            return;
        }
    };

    verbose_log("=== Iniciando análisis sintáctico ===\n", 1, nivel_verbose);

    // PASO 4: ANÁLISIS SINTÁCTICO
    // Valida que la secuencia de tokens cumple con la gramática del lenguaje
    // Usa un parser SLR(1) basado en tablas ACTION y GOTO
    // Retorna el generador de cuádruplos para la siguiente fase
    let generador = match sintactico::analyze(&tokens, nivel_verbose) {
        Ok(generador_cuadruplos) => {
            // El programa es sintácticamente correcto
            verbose_log("\n✓ Análisis sintáctico completado\n", 1, nivel_verbose);
            generador_cuadruplos
        },
        Err(e) => {
            // Hay un error de sintaxis
            eprintln!("\n✗✗✗ Error en el análisis sintáctico ✗✗✗");
            eprintln!("{}", e);
            return;
        }
    };

    // PASO 5: GENERACIÓN DE CÓDIGO INTERMEDIO Y EJECUCIÓN
    verbose_log("=== Preparando ejecución ===\n", 1, nivel_verbose);

    // Extraer nombre del programa desde la ruta del archivo
    let nombre_programa = std::path::Path::new(&full_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("programa")
        .to_string();

    // Exportar el "programa objeto" con función tabla y cuádruplos
    let programa = match generador.exportar_programa(nombre_programa) {
        Ok(prog) => {
            verbose_log(&format!("✓ Código intermedio preparado: {} cuádruplos\n", prog.cuadruplos.len()), 1, nivel_verbose);
            prog
        },
        Err(e) => {
            eprintln!("✗ Error al exportar programa: {}", e);
            return;
        }
    };

    verbose_log("=== Ejecutando programa en máquina virtual ===\n", 1, nivel_verbose);

    // Crear VM con sistema de IO (usa consola real para stdin/stdout)
    let mut vm = MaquinaVirtual::new(Box::new(ConsolaIO::new()));

    // Cargar el programa objeto (inicializa tabla de funciones, constantes, etc.)
    if let Err(e) = vm.cargar_programa(programa) {
        eprintln!("✗ Error al cargar programa en VM: {}", e);
        return;
    }

    // Ejecutar el programa
    match vm.ejecutar() {
        Ok(_) => {
            verbose_log("\n✓✓✓ Ejecución completada exitosamente ✓✓✓", 1, nivel_verbose);
        },
        Err(e) => {
            eprintln!("\n✗✗✗ Error durante la ejecución ✗✗✗");
            eprintln!("{}", e);
            return;
        }
    }
}
