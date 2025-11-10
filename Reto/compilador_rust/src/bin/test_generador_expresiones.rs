//! Test básico de generación de cuádruplos
//! Prueba expresiones aritméticas simples

use compilador_rust::intermedio::GeneradorCuadruplos;
use compilador_rust::semantico::ContextoSemantico;
use compilador_rust::semantico::TipoDato;

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║         TEST: Generación de Cuádruplos - Expresiones           ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // Inicializar contexto semántico
    let mut ctx = ContextoSemantico::new();
    ctx.inicializar_programa("test_expresiones").unwrap();

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

    // ========== TEST 1: Expresión simple a + b ==========
    println!("TEST 1: Expresión simple (a + b)");
    println!("   Variables: a: entero, b: entero\n");

    generador.procesar_operando("a").unwrap();
    generador.procesar_suma_resta("+").unwrap();
    generador.procesar_operando("b").unwrap();
    generador.generar_suma_resta().unwrap();

    generador.imprimir_cuadruplos();
    generador.reiniciar();

    // ========== TEST 2: Expresión con precedencia a + b * c ==========
    println!("\nTEST 2: Expresión con precedencia (a + b * c)");
    println!("   Variables: a: entero, b: entero, c: entero\n");

    generador.procesar_operando("a").unwrap();
    generador.procesar_suma_resta("+").unwrap();
    generador.procesar_operando("b").unwrap();
    generador.procesar_mult_div("*").unwrap();
    generador.procesar_operando("c").unwrap();
    generador.generar_mult_div().unwrap();
    generador.generar_suma_resta().unwrap();

    generador.imprimir_cuadruplos();
    generador.reiniciar();

    // ========== TEST 3: Expresión con paréntesis (a + b) * c ==========
    println!("\nTEST 3: Expresión con paréntesis ((a + b) * c)");
    println!("   Variables: a: entero, b: entero, c: entero\n");

    generador.abrir_parentesis();
    generador.procesar_operando("a").unwrap();
    generador.procesar_suma_resta("+").unwrap();
    generador.procesar_operando("b").unwrap();
    generador.generar_suma_resta().unwrap();
    generador.cerrar_parentesis().unwrap();
    generador.procesar_mult_div("*").unwrap();
    generador.procesar_operando("c").unwrap();
    generador.generar_mult_div().unwrap();

    generador.imprimir_cuadruplos();
    generador.reiniciar();

    // ========== TEST 4: Expresión compleja (a + b) * (c - 5) ==========
    println!("\nTEST 4: Expresión compleja ((a + b) * (c - 5))");
    println!("   Variables: a: entero, b: entero, c: entero\n");

    generador.abrir_parentesis();
    generador.procesar_operando("a").unwrap();
    generador.procesar_suma_resta("+").unwrap();
    generador.procesar_operando("b").unwrap();
    generador.generar_suma_resta().unwrap();
    generador.cerrar_parentesis().unwrap();
    generador.procesar_mult_div("*").unwrap();
    generador.abrir_parentesis();
    generador.procesar_operando("c").unwrap();
    generador.procesar_suma_resta("-").unwrap();
    generador.procesar_operando("5").unwrap();
    generador.generar_suma_resta().unwrap();
    generador.cerrar_parentesis().unwrap();
    generador.generar_mult_div().unwrap();

    generador.imprimir_cuadruplos();
    generador.reiniciar();

    // ========== TEST 5: Expresión con flotantes x + y * 2.5 ==========
    println!("\nTEST 5: Expresión con flotantes (x + y * 2.5)");
    println!("   Variables: x: flotante, y: flotante\n");

    generador.procesar_operando("x").unwrap();
    generador.procesar_suma_resta("+").unwrap();
    generador.procesar_operando("y").unwrap();
    generador.procesar_mult_div("*").unwrap();
    generador.procesar_operando("2.5").unwrap();
    generador.generar_mult_div().unwrap();
    generador.generar_suma_resta().unwrap();

    generador.imprimir_cuadruplos();
    generador.reiniciar();

    // ========== TEST 6: Expresión mixta a + x ==========
    println!("\nTEST 6: Expresión con tipos mixtos (a + x)");
    println!("   Variables: a: entero, x: flotante");
    println!("   Resultado: flotante (promoción)\n");

    generador.procesar_operando("a").unwrap();
    generador.procesar_suma_resta("+").unwrap();
    generador.procesar_operando("x").unwrap();
    generador.generar_suma_resta().unwrap();

    generador.imprimir_cuadruplos();
    generador.reiniciar();

    println!("\nTodos los tests de expresiones completados exitosamente");
}
