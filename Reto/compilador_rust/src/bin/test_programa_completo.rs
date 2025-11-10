//! Test completo de generación de cuádruplos
//! Simula un programa completo de Patito con todas las construcciones

use compilador_rust::intermedio::GeneradorCuadruplos;
use compilador_rust::semantico::ContextoSemantico;
use compilador_rust::semantico::TipoDato;

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║         TEST: Generación de Cuádruplos - Programa Completo     ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // Programa de ejemplo:
    // programa ejemplo;
    // var
    //     entero: a, b, c, resultado;
    //     flotante: x, y;
    // inicio
    //     lee(a);
    //     lee(b);
    //     c = (a + b) * 2;
    //     resultado = c - 10;
    //     escribe(resultado);
    //     x = 3.14;
    //     y = x * 2.0;
    //     si (a > b) entonces
    //         escribe(a);
    //     sino
    //         escribe(b);
    //     fin
    // fin

    println!("Programa de ejemplo:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("programa ejemplo;");
    println!("var");
    println!("    entero: a, b, c, resultado;");
    println!("    flotante: x, y;");
    println!("inicio");
    println!("    lee(a);");
    println!("    lee(b);");
    println!("    c = (a + b) * 2;");
    println!("    resultado = c - 10;");
    println!("    escribe(resultado);");
    println!("    x = 3.14;");
    println!("    y = x * 2.0;");
    println!("    si (a > b) entonces");
    println!("        escribe(a);");
    println!("    sino");
    println!("        escribe(b);");
    println!("    fin");
    println!("fin");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Inicializar contexto semántico
    let mut ctx = ContextoSemantico::new();
    ctx.inicializar_programa("ejemplo").unwrap();

    // Declarar variables
    ctx.establecer_tipo_actual(TipoDato::Entero);
    ctx.agregar_variable("a").unwrap();
    ctx.agregar_variable("b").unwrap();
    ctx.agregar_variable("c").unwrap();
    ctx.agregar_variable("resultado").unwrap();

    ctx.establecer_tipo_actual(TipoDato::Flotante);
    ctx.agregar_variable("x").unwrap();
    ctx.agregar_variable("y").unwrap();

    // Crear generador de cuádruplos
    let mut generador = GeneradorCuadruplos::new();
    generador.establecer_contexto(&ctx);

    println!("Generando cuádruplos...\n");

    // lee(a);
    generador.generar_lectura("a").unwrap();

    // lee(b);
    generador.generar_lectura("b").unwrap();

    // c = (a + b) * 2;
    generador.abrir_parentesis();
    generador.procesar_operando("a").unwrap();
    generador.procesar_suma_resta("+").unwrap();
    generador.procesar_operando("b").unwrap();
    generador.generar_suma_resta().unwrap();
    generador.cerrar_parentesis().unwrap();
    generador.procesar_mult_div("*").unwrap();
    generador.procesar_operando("2").unwrap();
    generador.generar_mult_div().unwrap();
    generador.generar_asignacion("c").unwrap();

    // resultado = c - 10;
    generador.procesar_operando("c").unwrap();
    generador.procesar_suma_resta("-").unwrap();
    generador.procesar_operando("10").unwrap();
    generador.generar_suma_resta().unwrap();
    generador.generar_asignacion("resultado").unwrap();

    // escribe(resultado);
    generador.procesar_operando("resultado").unwrap();
    generador.generar_escritura().unwrap();

    // x = 3.14;
    generador.procesar_operando("3.14").unwrap();
    generador.generar_asignacion("x").unwrap();

    // y = x * 2.0;
    generador.procesar_operando("x").unwrap();
    generador.procesar_mult_div("*").unwrap();
    generador.procesar_operando("2.0").unwrap();
    generador.generar_mult_div().unwrap();
    generador.generar_asignacion("y").unwrap();

    // si (a > b) entonces
    //     escribe(a);
    // sino
    //     escribe(b);
    // fin
    // Nota: Solo generamos la condición por ahora (en entregas futuras se agregarán GOTO)
    generador.procesar_operando("a").unwrap();
    generador.procesar_relacional(">").unwrap();
    generador.procesar_operando("b").unwrap();
    generador.generar_relacional().unwrap();

    // escribe(a); - rama entonces
    generador.procesar_operando("a").unwrap();
    generador.generar_escritura().unwrap();

    // escribe(b); - rama sino
    generador.procesar_operando("b").unwrap();
    generador.generar_escritura().unwrap();

    // Imprimir todos los cuádruplos generados
    generador.imprimir_cuadruplos();

    println!("\nEstadísticas:");
    println!("   • Total de cuádruplos: {}", generador.obtener_cuadruplos().len());
    println!("   • Variables declaradas: 6 (4 enteros, 2 flotantes)");
    println!("   • Operaciones aritméticas: 4");
    println!("   • Operaciones relacionales: 1");
    println!("   • Asignaciones: 4");
    println!("   • Lecturas: 2");
    println!("   • Escrituras: 3");

    println!("\nGeneración de cuádruplos completada exitosamente");
    println!("Nota: Los cuádruplos para control de flujo (GOTO, GOTOF) se");
    println!("   implementarán en entregas futuras para ciclos y condicionales.");
}
