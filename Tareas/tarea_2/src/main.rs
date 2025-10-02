// src/main.rs

// Importa el módulo lalrpop_util para manejar parseos y errores
use lalrpop_util::lalrpop_mod;

// Importa el módulo del parser generado por LALRPOP
lalrpop_mod!(pub calculator);

fn main() {
    let expr_parser = calculator::ExprParser::new();
    println!("Demostración de Calculadora con LALRPOP");
    println!("=====================================");

    // Operaciones de prueba para demostrar la funcionalidad
    let test_expressions = vec![
        "2 + 3",
        "5 * 4",
        "10 - 6",
        "8 / 2",
        "2 + 3 * 4",        // Precedencia: multiplicación antes que suma
        "2 * 3 + 4",        // Precedencia: multiplicación antes que suma
        "(2 + 3) * 4",      // Paréntesis cambian la precedencia
        "2 * (3 + 4)",      // Paréntesis cambian la precedencia
        "10 / 2 - 1",       // Operaciones de igual precedencia (izquierda a derecha)
        "20 - 5 + 3",       // Operaciones de igual precedencia (izquierda a derecha)
        "2 + 3 * 4 - 1",    // Múltiples operadores con diferente precedencia
        "(10 + 5) / (2 + 1)", // Paréntesis anidados
        "((2 + 3) * 4) / 2", // Paréntesis múltiples
    ];

    for expression in test_expressions {
        print!("Evaluando: {} ", expression);
        match expr_parser.parse(expression) {
            Ok(result) => println!("= {}", result),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    println!("\n¡Demostración completada!");
}