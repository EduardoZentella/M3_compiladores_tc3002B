//! Test del Directorio de Funciones

use compilador_rust::semantico::{DirectorioFunciones, TipoDato, TipoRetorno};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║   TEST DEL DIRECTORIO DE FUNCIONES - LENGUAJE PATITO         ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let mut dir = DirectorioFunciones::new();

    // Test 1: Crear alcance global
    println!("TEST 1: Crear Alcance Global");
    println!("─────────────────────────────");
    println!("  Creando programa 'mi_programa'...");
    match dir.agregar_funcion("mi_programa", TipoRetorno::Nula) {
        Ok(_) => println!("    Alcance global creado exitosamente"),
        Err(e) => println!("    Error: {}", e),
    }

    // Test 2: Agregar funciones
    println!("\nTEST 2: Agregar Funciones");
    println!("──────────────────────────");

    println!("  Agregando función 'suma' (retorna entero)...");
    match dir.agregar_funcion("suma", TipoRetorno::Tipo(TipoDato::Entero)) {
        Ok(_) => println!("    Función 'suma' agregada exitosamente"),
        Err(e) => println!("    Error: {}", e),
    }

    println!("  Agregando función 'promedio' (retorna flotante)...");
    match dir.agregar_funcion("promedio", TipoRetorno::Tipo(TipoDato::Flotante)) {
        Ok(_) => println!("    Función 'promedio' agregada exitosamente"),
        Err(e) => println!("    Error: {}", e),
    }

    println!("  Agregando función 'imprimir' (nula/void)...");
    match dir.agregar_funcion("imprimir", TipoRetorno::Nula) {
        Ok(_) => println!("    Función 'imprimir' agregada exitosamente"),
        Err(e) => println!("    Error: {}", e),
    }

    // Test 3: Validar función duplicada
    println!("\nTEST 3: Validar Función Duplicada");
    println!("───────────────────────────────────");
    println!("  Intentando agregar 'suma' de nuevo...");
    match dir.agregar_funcion("suma", TipoRetorno::Tipo(TipoDato::Entero)) {
        Ok(_) => println!("    ERROR: No debería permitir funciones duplicadas"),
        Err(e) => println!("    Correctamente rechazado: {}", e),
    }

    // Test 4: Agregar variables a funciones
    println!("\nTEST 4: Agregar Variables a Funciones");
    println!("───────────────────────────────────────");

    println!("  Agregando variables globales...");
    match dir.agregar_variable("mi_programa", "global_x", TipoDato::Entero) {
        Ok(_) => println!("    Variable 'global_x' agregada al alcance global"),
        Err(e) => println!("    Error: {}", e),
    }

    match dir.agregar_variable("mi_programa", "global_total", TipoDato::Flotante) {
        Ok(_) => println!("    Variable 'global_total' agregada al alcance global"),
        Err(e) => println!("    Error: {}", e),
    }

    println!("\n  Agregando variables a función 'suma'...");
    match dir.agregar_variable("suma", "a", TipoDato::Entero) {
        Ok(_) => println!("    Variable 'a' agregada a 'suma'"),
        Err(e) => println!("    Error: {}", e),
    }

    match dir.agregar_variable("suma", "b", TipoDato::Entero) {
        Ok(_) => println!("    Variable 'b' agregada a 'suma'"),
        Err(e) => println!("    Error: {}", e),
    }

    match dir.agregar_variable("suma", "resultado", TipoDato::Entero) {
        Ok(_) => println!("    Variable 'resultado' agregada a 'suma'"),
        Err(e) => println!("    Error: {}", e),
    }

    println!("\n  Agregando variables a función 'promedio'...");
    match dir.agregar_variable("promedio", "n1", TipoDato::Flotante) {
        Ok(_) => println!("    Variable 'n1' agregada a 'promedio'"),
        Err(e) => println!("    Error: {}", e),
    }

    match dir.agregar_variable("promedio", "n2", TipoDato::Flotante) {
        Ok(_) => println!("    Variable 'n2' agregada a 'promedio'"),
        Err(e) => println!("    Error: {}", e),
    }

    // Test 5: Validar variable duplicada
    println!("\nTEST 5: Validar Variable Duplicada");
    println!("────────────────────────────────────");
    println!("  Intentando agregar 'a' de nuevo a 'suma'...");
    match dir.agregar_variable("suma", "a", TipoDato::Flotante) {
        Ok(_) => println!("    ERROR: No debería permitir variables duplicadas"),
        Err(e) => println!("    Correctamente rechazado: {}", e),
    }

    // Test 6: Buscar funciones
    println!("\nTEST 6: Buscar Funciones");
    println!("─────────────────────────");

    println!("  Buscando función 'suma'...");
    match dir.buscar_funcion("suma") {
        Some(func) => println!("    Encontrada: tipo de retorno = {}", func.tipo_retorno),
        None => println!("    No encontrada"),
    }

    println!("  Buscando función 'inexistente'...");
    match dir.buscar_funcion("inexistente") {
        Some(_) => println!("    ERROR: No debería existir"),
        None => println!("    Correctamente reportada como inexistente"),
    }

    // Test 7: Buscar variables
    println!("\nTEST 7: Buscar Variables");
    println!("─────────────────────────");

    println!("  Buscando 'a' en función 'suma'...");
    match dir.buscar_variable("suma", "a") {
        Some(var) => println!("    Encontrada: tipo = {}", var.tipo),
        None => println!("    No encontrada"),
    }

    println!("  Buscando 'global_x' en alcance global...");
    match dir.buscar_variable("mi_programa", "global_x") {
        Some(var) => println!("    Encontrada: tipo = {}", var.tipo),
        None => println!("    No encontrada"),
    }

    // Test 8: Imprimir directorio completo
    println!("\nTEST 8: Visualización del Directorio Completo");
    println!("───────────────────────────────────────────────");
    dir.imprimir();

    println!("\n✓ Todas las pruebas completadas");
}
