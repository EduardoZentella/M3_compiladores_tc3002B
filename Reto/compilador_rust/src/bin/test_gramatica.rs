// Test temporal para verificar el modulo de gramatica
use compilador_rust::gramatica;

fn main() {
    let contenido = r#"
    <Programa> → program <VARS> <STATEMENTS>
    <VARS> → vars <VAR_LIST>
    <VAR_LIST> → <VAR> <VAR_LIST>
    <VAR_LIST> → ε
    "#;

    match gramatica::parser::parsear_gramatica(contenido) {
        Ok(g) => {
            println!("Gramática parseada correctamente!");
            println!("  Regla 0: {}", g.regla0);
            println!("  Total reglas: {}", g.producciones.len());
            println!("  No-Terminales: {}", g.simbolos_no_terminales.len());
            println!("  Terminales: {}", g.simbolos_terminales.len());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
