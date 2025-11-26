//! Test de Tablas de Variables para el lenguaje Patito

use compilador_rust::semantico::{TablaVariables, TipoDato};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║      TEST DE TABLAS DE VARIABLES - LENGUAJE PATITO           ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Test 1: Crear tabla y agregar variables
    println!("TEST 1: Agregar Variables");
    println!("─────────────────────────");
    let mut tabla = TablaVariables::new();

    println!("  Agregando variable 'x' (entero) en dirección 1000...");
    match tabla.agregar("x", TipoDato::Entero, 1000) {
        Ok(_) => println!("    Variable 'x' agregada exitosamente"),
        Err(e) => println!("    Error: {}", e),
    }

    println!("  Agregando variable 'total' (flotante) en dirección 3000...");
    match tabla.agregar("total", TipoDato::Flotante, 3000) {
        Ok(_) => println!("    Variable 'total' agregada exitosamente"),
        Err(e) => println!("    Error: {}", e),
    }

    println!("  Agregando variable 'contador' (entero) en dirección 1001...");
    match tabla.agregar("contador", TipoDato::Entero, 1001) {
        Ok(_) => println!("    Variable 'contador' agregada exitosamente"),
        Err(e) => println!("    Error: {}", e),
    }

    // Test 2: Intentar agregar variable duplicada
    println!("\nTEST 2: Validar Declaración Duplicada");
    println!("───────────────────────────────────────");
    println!("  Intentando agregar 'x' de nuevo...");
    match tabla.agregar("x", TipoDato::Flotante, 3001) {
        Ok(_) => println!("    ERROR: No debería permitir duplicados"),
        Err(e) => println!("    Correctamente rechazado: {}", e),
    }

    // Test 3: Buscar variables
    println!("\nTEST 3: Buscar Variables");
    println!("─────────────────────────");

    println!("  Buscando 'x'...");
    match tabla.buscar("x") {
        Some(entrada) => println!("    Encontrada: tipo = {}", entrada.tipo),
        None => println!("    No encontrada"),
    }

    println!("  Buscando 'total'...");
    match tabla.buscar("total") {
        Some(entrada) => println!("    Encontrada: tipo = {}", entrada.tipo),
        None => println!("    No encontrada"),
    }

    println!("  Buscando 'inexistente'...");
    match tabla.buscar("inexistente") {
        Some(_) => println!("    ERROR: No debería existir"),
        None => println!("    Correctamente reportada como inexistente"),
    }

    // Test 4: Listar variables
    println!("\nTEST 4: Listar Variables");
    println!("─────────────────────────");
    let vars = tabla.listar();
    println!("  Total de variables: {}", vars.len());
    for (nombre, tipo) in vars {
        println!("    - {}: {}", nombre, tipo);
    }

    // Test 5: Imprimir tabla
    println!("\nTEST 5: Visualización de Tabla");
    println!("────────────────────────────────");
    tabla.imprimir("mi_funcion");

    println!("\n✓ Todas las pruebas completadas");
}
