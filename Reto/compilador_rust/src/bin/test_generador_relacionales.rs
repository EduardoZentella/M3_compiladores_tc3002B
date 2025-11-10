//! Test de generación de cuádruplos para expresiones relacionales

use compilador_rust::intermedio::GeneradorCuadruplos;
use compilador_rust::semantico::ContextoSemantico;
use compilador_rust::semantico::TipoDato;

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║      TEST: Generación de Cuádruplos - Expresiones Relacionales ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // Inicializar contexto semántico
    let mut ctx = ContextoSemantico::new();
    ctx.inicializar_programa("test_relacionales").unwrap();

    // Declarar variables
    ctx.establecer_tipo_actual(TipoDato::Entero);
    ctx.agregar_variable("a").unwrap();
    ctx.agregar_variable("b").unwrap();
    ctx.agregar_variable("c").unwrap();

    ctx.establecer_tipo_actual(TipoDato::Flotante);
    ctx.agregar_variable("x").unwrap();
    ctx.agregar_variable("y").unwrap();

    // Crear generador de cuádruplos
    let mut generador = GeneradorCuadruplos::new();
    generador.establecer_contexto(&ctx);

    // ========== TEST 1: Comparación simple a > b ==========
    println!("TEST 1: Comparación simple (a > b)");
    println!("   Variables: a: entero, b: entero");
    println!("   Resultado: entero (booleano)\n");

    generador.procesar_operando("a").unwrap();
    generador.procesar_relacional(">").unwrap();
    generador.procesar_operando("b").unwrap();
    generador.generar_relacional().unwrap();

    generador.imprimir_cuadruplos();
    generador.reiniciar();

    // ========== TEST 2: Comparación con expresión a + b < c * 2 ==========
    println!("\nTEST 2: Comparación con expresiones (a + b < c * 2)");
    println!("   Variables: a: entero, b: entero, c: entero\n");

    generador.procesar_operando("a").unwrap();
    generador.procesar_suma_resta("+").unwrap();
    generador.procesar_operando("b").unwrap();
    generador.generar_suma_resta().unwrap();
    generador.procesar_relacional("<").unwrap();
    generador.procesar_operando("c").unwrap();
    generador.procesar_mult_div("*").unwrap();
    generador.procesar_operando("2").unwrap();
    generador.generar_mult_div().unwrap();
    generador.generar_relacional().unwrap();

    generador.imprimir_cuadruplos();
    generador.reiniciar();

    // ========== TEST 3: Igualdad x == y ==========
    println!("\nTEST 3: Igualdad (x == y)");
    println!("   Variables: x: flotante, y: flotante\n");

    generador.procesar_operando("x").unwrap();
    generador.procesar_relacional("==").unwrap();
    generador.procesar_operando("y").unwrap();
    generador.generar_relacional().unwrap();

    generador.imprimir_cuadruplos();
    generador.reiniciar();

    // ========== TEST 4: Desigualdad a != 10 ==========
    println!("\nTEST 4: Desigualdad (a != 10)");
    println!("   Variables: a: entero\n");

    generador.procesar_operando("a").unwrap();
    generador.procesar_relacional("!=").unwrap();
    generador.procesar_operando("10").unwrap();
    generador.generar_relacional().unwrap();

    generador.imprimir_cuadruplos();
    generador.reiniciar();

    // ========== TEST 5: Comparación compleja (a + b) > (c - 5) ==========
    println!("\nTEST 5: Comparación compleja ((a + b) > (c - 5))");
    println!("   Variables: a: entero, b: entero, c: entero\n");

    generador.abrir_parentesis();
    generador.procesar_operando("a").unwrap();
    generador.procesar_suma_resta("+").unwrap();
    generador.procesar_operando("b").unwrap();
    generador.generar_suma_resta().unwrap();
    generador.cerrar_parentesis().unwrap();
    generador.procesar_relacional(">").unwrap();
    generador.abrir_parentesis();
    generador.procesar_operando("c").unwrap();
    generador.procesar_suma_resta("-").unwrap();
    generador.procesar_operando("5").unwrap();
    generador.generar_suma_resta().unwrap();
    generador.cerrar_parentesis().unwrap();
    generador.generar_relacional().unwrap();

    generador.imprimir_cuadruplos();
    generador.reiniciar();

    // ========== TEST 6: Comparación con tipos mixtos a < x ==========
    println!("\nTEST 6: Comparación con tipos mixtos (a < x)");
    println!("   Variables: a: entero, x: flotante");
    println!("   Operandos promovidos a flotante, resultado: entero\n");

    generador.procesar_operando("a").unwrap();
    generador.procesar_relacional("<").unwrap();
    generador.procesar_operando("x").unwrap();
    generador.generar_relacional().unwrap();

    generador.imprimir_cuadruplos();
    generador.reiniciar();

    println!("\nTodos los tests de expresiones relacionales completados exitosamente");
}
