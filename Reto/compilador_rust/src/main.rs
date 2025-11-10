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

/// Obtiene la ruta del archivo a compilar y determina el modo verbose.
///
/// # Comportamiento
/// - Si hay argumentos en la línea de comandos, los usa
/// - Si no, solicita interactivamente la ruta al usuario
/// - Detecta flags `-v` o `--verbose` para salida detallada
/// - Convierte rutas relativas a absolutas usando `canonicalize`
///
/// # Retorna
/// Una tupla `(ruta_absoluta, es_verbose)`:
/// - `ruta_absoluta`: Path completo del archivo a compilar
/// - `es_verbose`: `true` si se debe mostrar salida detallada
///
/// # Panics
/// Termina el programa (exit 1) si no se puede resolver la ruta del archivo
fn obtain_file_path() -> (String, bool) {
    // Se obtiene el path del archivo por argumentos o se solicita al usuario
    let mut path = String::new();
    let args: Vec<String> = std::env::args().collect();

    // Verificar si hay argumentos adicionales
    if args.len() > 1 {
        // Concatenar todos los argumentos (excepto el nombre del programa)
        path = args[1..].join(" ");
    } else {
        println!("Ingrese el path del archivo a analizar (agregue -v o --verbose para modo detallado):");
        std::io::stdin().read_line(&mut path).expect("Error al leer la entrada");
    }

    // Se verifica que el path tenga -v o --verbose y se limpia
    let is_verbose = path.contains("-v") || path.contains("--verbose");
    path = path.replace("-v", "").replace("--verbose", "").trim().to_string();

    // Se transforma el path a un full path
    let full_path = match std::fs::canonicalize(&path) {
        Ok(p) => p.to_string_lossy().to_string(),
        Err(e) => {
            eprintln!("Error al obtener el path completo de '{}': {}", path, e);
            std::process::exit(1);
        }
    };

    (full_path, is_verbose)
}

/// Imprime un mensaje solo si el modo verbose está activado.
///
/// # Parámetros
/// - `message`: El mensaje a imprimir
/// - `is_verbose`: Si es `true`, imprime el mensaje; si no, no hace nada
///
/// # Ejemplo
/// ```
/// verbose_log("Procesando tokens...", &true);  // Imprime
/// verbose_log("Procesando tokens...", &false); // No imprime nada
/// ```
fn verbose_log(message: &str, is_verbose: &bool) {
    if *is_verbose {
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
    // PASO 1: Obtener configuración (ruta y modo verbose)
    let (full_path, is_verbose) = obtain_file_path();

    // PASO 2: Leer archivo fuente
    // El compilador trabaja sobre el contenido completo en memoria
    let contenido = match std::fs::read_to_string(&full_path) {
        Ok(cont) => cont,
        Err(e) => {
            eprintln!("Error al leer el archivo '{}': {}", full_path, e);
            return;
        }
    };

    verbose_log(&format!("Analizando archivo: {}", full_path), &is_verbose);
    verbose_log("\n=== Iniciando análisis léxico ===\n", &is_verbose);

    // PASO 3: ANÁLISIS LÉXICO
    // Convierte el código fuente (String) en una secuencia de tokens
    // Cada token representa un elemento léxico: palabra reservada, identificador, operador, etc.
    let tokens = match lexico::analyze(&contenido, &is_verbose) {
        Ok(toks) => {
            verbose_log(&format!("\n✓ Análisis léxico completado. Tokens generados: {}\n", toks.len()), &is_verbose);
            toks
        },
        Err(e) => {
            eprintln!("✗ Error en el análisis léxico: {}", e);
            return;
        }
    };

    verbose_log("=== Iniciando análisis sintáctico ===\n", &is_verbose);

    // PASO 4: ANÁLISIS SINTÁCTICO
    // Valida que la secuencia de tokens cumple con la gramática del lenguaje
    // Usa un parser SLR(1) basado en tablas ACTION y GOTO
    match sintactico::analyze(&tokens, &is_verbose) {
        Ok(_) => {
            // ✓ El programa es sintácticamente correcto
            println!("\n✓✓✓ Compilación exitosa ✓✓✓");
        },
        Err(e) => {
            // ✗ Hay un error de sintaxis
            eprintln!("\n✗✗✗ Error en el análisis sintáctico ✗✗✗");
            eprintln!("{}", e);
            return;
        }
    };

    // TODO: Próximas fases
    // - Análisis semántico (tabla de símbolos, tipos)
    // - Generación de código intermedio
    // - Generación de código para la máquina virtual
}
