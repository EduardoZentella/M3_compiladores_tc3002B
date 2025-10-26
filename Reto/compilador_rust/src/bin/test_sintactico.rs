// Test del analizador sintáctico SLR con tokens
use compilador_rust::sintactico;
use compilador_rust::lexico::token::{Token, TipoToken};

fn main() {
    println!("=== Test del Analizador Sintáctico SLR ===\n");

    // Test 1: Programa simple sin variables ni funciones
    println!("--- Test 1: Programa mínimo ---");
    let tokens1 = vec![
        Token { tipo: TipoToken::Programa, valor: "programa".to_string(), linea: 1 },
        Token { tipo: TipoToken::Id, valor: "test".to_string(), linea: 1 },
        Token { tipo: TipoToken::PuntoYComa, valor: ";".to_string(), linea: 1 },
        Token { tipo: TipoToken::Inicio, valor: "inicio".to_string(), linea: 2 },
        Token { tipo: TipoToken::LlaveAbre, valor: "{".to_string(), linea: 2 },
        Token { tipo: TipoToken::LlaveCierra, valor: "}".to_string(), linea: 3 },
        Token { tipo: TipoToken::Fin, valor: "fin".to_string(), linea: 3 },
    ];

    match sintactico::analyze(&tokens1, &true) {
        Ok(_) => println!("\n✓ Test 1 PASADO\n"),
        Err(e) => eprintln!("\n✗ Test 1 FALLIDO: {}\n", e),
    }

    // Test 2: Programa con una asignación simple
    println!("\n--- Test 2: Programa con asignación ---");
    let tokens2 = vec![
        Token { tipo: TipoToken::Programa, valor: "programa".to_string(), linea: 1 },
        Token { tipo: TipoToken::Id, valor: "test".to_string(), linea: 1 },
        Token { tipo: TipoToken::PuntoYComa, valor: ";".to_string(), linea: 1 },
        Token { tipo: TipoToken::Inicio, valor: "inicio".to_string(), linea: 2 },
        Token { tipo: TipoToken::LlaveAbre, valor: "{".to_string(), linea: 2 },
        Token { tipo: TipoToken::Id, valor: "x".to_string(), linea: 3 },
        Token { tipo: TipoToken::Asignacion, valor: "=".to_string(), linea: 3 },
        Token { tipo: TipoToken::CteEnt, valor: "5".to_string(), linea: 3 },
        Token { tipo: TipoToken::PuntoYComa, valor: ";".to_string(), linea: 3 },
        Token { tipo: TipoToken::LlaveCierra, valor: "}".to_string(), linea: 4 },
        Token { tipo: TipoToken::Fin, valor: "fin".to_string(), linea: 4 },
    ];

    match sintactico::analyze(&tokens2, &true) {
        Ok(_) => println!("✓ Test 2 PASADO\n"),
        Err(e) => eprintln!("✗ Test 2 FALLIDO: {}\n", e),
    }
}

