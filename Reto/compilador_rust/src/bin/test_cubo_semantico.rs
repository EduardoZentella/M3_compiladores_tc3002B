//! Test del Cubo Semántico para validar reglas de tipos

use compilador_rust::semantico::{CuboSemantico, TipoDato};
use compilador_rust::semantico::cubo_semantico::{Operador, ResultadoTipo};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║         TEST DEL CUBO SEMÁNTICO - LENGUAJE PATITO            ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let cubo = CuboSemantico::new();

    // Imprimir el cubo completo
    cubo.imprimir();

    // Pruebas específicas
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                    PRUEBAS ESPECÍFICAS                       ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Test 1: Operaciones aritméticas
    println!("TEST 1: Operaciones Aritméticas");
    println!("─────────────────────────────────");
    probar_operacion(&cubo, TipoDato::Entero, Operador::Suma, TipoDato::Entero, "entero + entero");
    probar_operacion(&cubo, TipoDato::Entero, Operador::Suma, TipoDato::Flotante, "entero + flotante");
    probar_operacion(&cubo, TipoDato::Flotante, Operador::Multiplicacion, TipoDato::Entero, "flotante * entero");
    probar_operacion(&cubo, TipoDato::Flotante, Operador::Division, TipoDato::Flotante, "flotante / flotante");

    // Test 2: Operaciones relacionales
    println!("\nTEST 2: Operaciones Relacionales");
    println!("──────────────────────────────────");
    probar_operacion(&cubo, TipoDato::Entero, Operador::MayorQue, TipoDato::Entero, "entero > entero");
    probar_operacion(&cubo, TipoDato::Flotante, Operador::MenorQue, TipoDato::Entero, "flotante < entero");
    probar_operacion(&cubo, TipoDato::Entero, Operador::Igual, TipoDato::Flotante, "entero == flotante");
    probar_operacion(&cubo, TipoDato::Flotante, Operador::Diferente, TipoDato::Flotante, "flotante != flotante");

    // Test 3: Asignaciones
    println!("\nTEST 3: Asignaciones");
    println!("─────────────────────");
    probar_operacion(&cubo, TipoDato::Entero, Operador::Asignacion, TipoDato::Entero, "entero = entero");
    probar_operacion(&cubo, TipoDato::Flotante, Operador::Asignacion, TipoDato::Flotante, "flotante = flotante");
    probar_operacion(&cubo, TipoDato::Flotante, Operador::Asignacion, TipoDato::Entero, "flotante = entero (promoción)");
    probar_operacion(&cubo, TipoDato::Entero, Operador::Asignacion, TipoDato::Flotante, "entero = flotante (NO permitido)");

    println!("\n✓ Todas las pruebas completadas");
}

fn probar_operacion(cubo: &CuboSemantico, tipo1: TipoDato, op: Operador, tipo2: TipoDato, descripcion: &str) {
    let resultado = cubo.validar(tipo1, op, tipo2);

    match resultado {
        ResultadoTipo::Ok(tipo_resultado) => {
            println!("  ✓ {} → {} ", descripcion, tipo_resultado);
        }
        ResultadoTipo::Error => {
            println!("  ✗ {} → ERROR (tipos incompatibles)", descripcion);
        }
    }
}
