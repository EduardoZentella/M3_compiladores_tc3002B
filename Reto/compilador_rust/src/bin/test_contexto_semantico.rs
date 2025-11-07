//! Test del Contexto Semántico para un flujo completo

use compilador_rust::semantico::{ContextoSemantico, TipoDato, TipoRetorno};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║    TEST DEL CONTEXTO SEMÁNTICO - LENGUAJE PATITO             ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let mut ctx = ContextoSemantico::new();

    println!("SIMULACIÓN DE ANÁLISIS SEMÁNTICO");
    println!("═════════════════════════════════\n");

    // Simular programa de ejemplo:
    // programa mi_programa;
    // vars x, y : entero;
    // vars total : flotante;
    //
    // entero suma(a : entero, b : entero) {
    //     vars resultado : entero;
    //     { resultado = a + b; }
    // };
    //
    // nula imprimir() {
    //     { escribe(total); }
    // };
    //
    // inicio {
    //     x = 5;
    //     y = 10;
    //     total = 3.14;
    // } fin

    // PN1: Después de "programa mi_programa;"
    println!("PN1: Inicializando programa 'mi_programa'");
    println!("─────────────────────────────────────────");
    match ctx.inicializar_programa("mi_programa") {
        Ok(_) => println!("  Programa inicializado"),
        Err(e) => println!("  Error: {}", e),
    }
    println!("  Alcance actual: {}\n", ctx.alcance_actual());

    // PN2 + PN3: Declaración de variables globales
    println!("PN2+PN3: Declarando variables globales");
    println!("────────────────────────────────────────");

    println!("  vars x, y : entero;");
    ctx.establecer_tipo_actual(TipoDato::Entero);
    match ctx.agregar_variable("x") {
        Ok(_) => println!("    Variable 'x' agregada"),
        Err(e) => println!("    Error: {}", e),
    }
    match ctx.agregar_variable("y") {
        Ok(_) => println!("    Variable 'y' agregada"),
        Err(e) => println!("    Error: {}", e),
    }

    println!("  vars total : flotante;");
    ctx.establecer_tipo_actual(TipoDato::Flotante);
    match ctx.agregar_variable("total") {
        Ok(_) => println!("    Variable 'total' agregada\n"),
        Err(e) => println!("    Error: {}\n", e),
    }

    // PN4: Declaración de función 'suma'
    println!("PN4: Declarando función 'suma'");
    println!("────────────────────────────────");
    println!("  entero suma(a : entero, b : entero) {{");
    match ctx.iniciar_funcion("suma", TipoRetorno::Tipo(TipoDato::Entero)) {
        Ok(_) => println!("    Función 'suma' creada"),
        Err(e) => println!("    Error: {}", e),
    }
    println!("    Alcance actual: {}", ctx.alcance_actual());

    // PN5: Parámetros de la función
    println!("\n  PN5: Agregando parámetros");
    ctx.establecer_tipo_actual(TipoDato::Entero);
    match ctx.agregar_variable("a") {
        Ok(_) => println!("    Parámetro 'a' agregado"),
        Err(e) => println!("    Error: {}", e),
    }
    match ctx.agregar_variable("b") {
        Ok(_) => println!("    Parámetro 'b' agregado"),
        Err(e) => println!("    Error: {}", e),
    }

    // Variables locales de 'suma'
    println!("\n  PN2+PN3: Variables locales");
    println!("    vars resultado : entero;");
    ctx.establecer_tipo_actual(TipoDato::Entero);
    match ctx.agregar_variable("resultado") {
        Ok(_) => println!("      Variable 'resultado' agregada"),
        Err(e) => println!("      Error: {}", e),
    }

    // PN7: Uso de variables en expresión
    println!("\n  PN7: Validando uso de variables en 'resultado = a + b;'");
    match ctx.obtener_tipo_variable("resultado") {
        Ok(tipo) => println!("    'resultado' existe, tipo: {}", tipo),
        Err(e) => println!("    Error: {}", e),
    }
    match ctx.obtener_tipo_variable("a") {
        Ok(tipo) => println!("    'a' existe, tipo: {}", tipo),
        Err(e) => println!("    Error: {}", e),
    }
    match ctx.obtener_tipo_variable("b") {
        Ok(tipo) => println!("    'b' existe, tipo: {}", tipo),
        Err(e) => println!("    Error: {}", e),
    }

    // Verificar acceso a variable global desde función
    println!("\n  Verificando acceso a variable global 'x' desde función:");
    match ctx.obtener_tipo_variable("x") {
        Ok(tipo) => println!("    'x' (global) accesible, tipo: {}", tipo),
        Err(e) => println!("    Error: {}", e),
    }

    println!("  }};");

    // PN6: Finalizar función
    println!("\nPN6: Finalizando función 'suma'");
    println!("─────────────────────────────────");
    ctx.finalizar_funcion();
    println!("  ✓ Función finalizada");
    println!("  Alcance actual: {}\n", ctx.alcance_actual());

    // PN4: Declaración de función 'imprimir'
    println!("PN4: Declarando función 'imprimir'");
    println!("────────────────────────────────────");
    println!("  nula imprimir() {{");
    match ctx.iniciar_funcion("imprimir", TipoRetorno::Nula) {
        Ok(_) => println!("    Función 'imprimir' creada"),
        Err(e) => println!("    Error: {}", e),
    }

    // PN7: Uso de variable global en función
    println!("\n  PN7: Validando uso de 'total' en 'escribe(total);'");
    match ctx.obtener_tipo_variable("total") {
        Ok(tipo) => println!("    'total' (global) accesible, tipo: {}", tipo),
        Err(e) => println!("    Error: {}", e),
    }

    println!("  }};");

    // PN6: Finalizar función
    ctx.finalizar_funcion();
    println!("\nPN6: Finalizando función 'imprimir'");
    println!("  Función finalizada");
    println!("  Alcance actual: {}\n", ctx.alcance_actual());

    // Cuerpo principal (inicio)
    println!("Cuerpo Principal (inicio)");
    println!("──────────────────────────");
    println!("  inicio {{");

    // PN7: Validar asignaciones
    println!("    PN7: Validando 'x = 5;'");
    match ctx.obtener_tipo_variable("x") {
        Ok(tipo) => println!("      'x' existe, tipo: {}", tipo),
        Err(e) => println!("      Error: {}", e),
    }

    println!("    PN7: Validando 'y = 10;'");
    match ctx.obtener_tipo_variable("y") {
        Ok(tipo) => println!("      'y' existe, tipo: {}", tipo),
        Err(e) => println!("      Error: {}", e),
    }

    println!("    PN7: Validando 'total = 3.14;'");
    match ctx.obtener_tipo_variable("total") {
        Ok(tipo) => println!("      'total' existe, tipo: {}", tipo),
        Err(e) => println!("      Error: {}", e),
    }

    println!("  }} fin\n");

    // Test de errores
    println!("PRUEBAS DE VALIDACIÓN DE ERRORES");
    println!("═════════════════════════════════\n");

    println!("Test 1: Variable no declarada");
    println!("───────────────────────────────");
    match ctx.obtener_tipo_variable("inexistente") {
        Ok(_) => println!("  ERROR: No debería existir"),
        Err(e) => println!("  {}", e),
    }

    println!("\nTest 2: Función duplicada");
    println!("──────────────────────────");
    match ctx.iniciar_funcion("suma", TipoRetorno::Nula) {
        Ok(_) => println!("  ERROR: No debería permitir duplicados"),
        Err(e) => println!("  {}", e),
    }

    println!("\nTest 3: Variable duplicada en alcance global");
    println!("──────────────────────────────────────────────");
    ctx.establecer_tipo_actual(TipoDato::Entero);
    match ctx.agregar_variable("x") {
        Ok(_) => println!("  ERROR: No debería permitir duplicados"),
        Err(e) => println!("  {}", e),
    }

    println!("\nTest 4: Variable local no visible desde global");
    println!("────────────────────────────────────────────────");
    println!("  (Alcance actual: {})", ctx.alcance_actual());
    println!("  Buscando 'resultado' (declarada en función 'suma')...");
    match ctx.obtener_tipo_variable("resultado") {
        Ok(_) => println!("  ERROR: No debería ser visible"),
        Err(e) => println!("  {}", e),
    }

    // Imprimir estado final
    println!("\nESTADO FINAL DEL CONTEXTO SEMÁNTICO");
    println!("════════════════════════════════════");
    ctx.imprimir();

    println!("\n✓ Todas las pruebas completadas");
}
