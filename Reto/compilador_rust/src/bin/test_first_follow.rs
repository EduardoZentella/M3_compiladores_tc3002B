// Test para verificar calcular_first_sets y calcular_follow_sets
use compilador_rust::gramatica;

fn main() {
    let contenido = r#"
<E> → <T> <E'>
<E'> → + <T> <E'>
<E'> → ε
<T> → <F> <T'>
<T'> → * <F> <T'>
<T'> → ε
<F> → ( <E> )
<F> → id
"#;

    match gramatica::parser::parsear_gramatica(contenido) {
        Ok(g) => {
            println!("✓ Gramática parseada correctamente!");
            println!();

            // Calcular conjuntos FIRST
            let first_sets = gramatica::first_follow::calcular_first_sets(&g);
            println!("=== Conjuntos FIRST ===");
            for (no_terminal, first_set) in &first_sets {
                println!("FIRST({}): {:?}", no_terminal, first_set);
            }
            println!();

            // Calcular conjuntos FOLLOW
            let follow_sets = gramatica::first_follow::calcular_follow_sets(&g, &first_sets);
            println!("=== Conjuntos FOLLOW ===");
            for (no_terminal, follow_set) in &follow_sets {
                println!("FOLLOW({}): {:?}", no_terminal, follow_set);
            }
        }
        Err(e) => {
            eprintln!("✗ Error: {}", e);
        }
    }
}
