//! Test de generación de cuádruplos para estatutos lineales
//! Incluye: asignación, lectura (lee) y escritura (escribe)

use compilador_rust::intermedio::GeneradorCuadruplos;
use compilador_rust::semantico::ContextoSemantico;
use compilador_rust::semantico::TipoDato;

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║      TEST: Generación de Cuádruplos - Estatutos Lineales       ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // Inicializar contexto semántico
    let mut ctx = ContextoSemantico::new();
    ctx.inicializar_programa("test_estatutos").unwrap();

    // Declarar variables
    ctx.establecer_tipo_actual(TipoDato::Entero);
    ctx.agregar_variable("a").unwrap();
    ctx.agregar_variable("b").unwrap();
    ctx.agregar_variable("c").unwrap();
    ctx.agregar_variable("resultado").unwrap();

    ctx.establecer_tipo_actual(TipoDato::Flotante);
    ctx.agregar_variable("x").unwrap();
    ctx.agregar_variable("y").unwrap();
    ctx.agregar_variable("z").unwrap();

    // Crear generador de cuádruplos
    let mut generador = GeneradorCuadruplos::new();
    generador.establecer_contexto(&ctx);

    // ========== TEST 1: Asignación simple a = 10 ==========
    println!("TEST 1: Asignación simple (a = 10)");
    println!("   Variables: a: entero\n");

    generador.procesar_operando("10").unwrap();
    generador.generar_asignacion("a").unwrap();

    generador.imprimir_cuadruplos();
    generador.reiniciar();

    // ========== TEST 2: Asignación con expresión c = a + b ==========
    println!("\nTEST 2: Asignación con expresión (c = a + b)");
    println!("   Variables: a: entero, b: entero, c: entero\n");

    generador.procesar_operando("a").unwrap();
    generador.procesar_suma_resta("+").unwrap();
    generador.procesar_operando("b").unwrap();
    generador.generar_suma_resta().unwrap();
    generador.generar_asignacion("c").unwrap();

    generador.imprimir_cuadruplos();
    generador.reiniciar();

    // ========== TEST 3: Asignación compleja resultado = (a + b) * c ==========
    println!("\nTEST 3: Asignación compleja (resultado = (a + b) * c)");
    println!("   Variables: a: entero, b: entero, c: entero, resultado: entero\n");

    generador.abrir_parentesis();
    generador.procesar_operando("a").unwrap();
    generador.procesar_suma_resta("+").unwrap();
    generador.procesar_operando("b").unwrap();
    generador.generar_suma_resta().unwrap();
    generador.cerrar_parentesis().unwrap();
    generador.procesar_mult_div("*").unwrap();
    generador.procesar_operando("c").unwrap();
    generador.generar_mult_div().unwrap();
    generador.generar_asignacion("resultado").unwrap();

    generador.imprimir_cuadruplos();
    generador.reiniciar();

    // ========== TEST 4: Lectura lee(a) ==========
    println!("\nTEST 4: Lectura (lee a)");
    println!("   Variables: a: entero\n");

    generador.generar_lectura("a").unwrap();

    generador.imprimir_cuadruplos();
    generador.reiniciar();

    // ========== TEST 5: Escritura escribe(a) ==========
    println!("\nTEST 5: Escritura (escribe a)");
    println!("   Variables: a: entero\n");

    generador.procesar_operando("a").unwrap();
    generador.generar_escritura().unwrap();

    generador.imprimir_cuadruplos();
    generador.reiniciar();

    // ========== TEST 6: Escritura de expresión escribe(a + b) ==========
    println!("\nTEST 6: Escritura de expresión (escribe a + b)");
    println!("   Variables: a: entero, b: entero\n");

    generador.procesar_operando("a").unwrap();
    generador.procesar_suma_resta("+").unwrap();
    generador.procesar_operando("b").unwrap();
    generador.generar_suma_resta().unwrap();
    generador.generar_escritura().unwrap();

    generador.imprimir_cuadruplos();
    generador.reiniciar();

    // ========== TEST 7: Promoción de tipos x = a ==========
    println!("\nTEST 7: Asignación con promoción (x = a)");
    println!("   Variables: x: flotante, a: entero");
    println!("   Válido: entero se promueve a flotante\n");

    generador.procesar_operando("a").unwrap();
    generador.generar_asignacion("x").unwrap();

    generador.imprimir_cuadruplos();
    generador.reiniciar();

    // ========== TEST 8: Asignación con flotantes z = x + y * 2.5 ==========
    println!("\nTEST 8: Asignación con flotantes (z = x + y * 2.5)");
    println!("   Variables: x: flotante, y: flotante, z: flotante\n");

    generador.procesar_operando("x").unwrap();
    generador.procesar_suma_resta("+").unwrap();
    generador.procesar_operando("y").unwrap();
    generador.procesar_mult_div("*").unwrap();
    generador.procesar_operando("2.5").unwrap();
    generador.generar_mult_div().unwrap();
    generador.generar_suma_resta().unwrap();
    generador.generar_asignacion("z").unwrap();

    generador.imprimir_cuadruplos();
    generador.reiniciar();

    println!("\nTodos los tests de estatutos lineales completados exitosamente");
}
