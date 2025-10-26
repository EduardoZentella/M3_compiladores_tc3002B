// Test para verificar la construcción del autómata LR(0)
use compilador_rust::gramatica;

fn main() {
    // Gramática simple para prueba:
    // E → T E'
    // E' → + T E' | ε
    // T → F T'
    // T' → * F T' | ε
    // F → ( E ) | id
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
            println!("  Total de producciones: {}", g.producciones.len());
            println!();

            // Construir el autómata LR(0)
            println!("Construyendo autómata LR(0)...");
            let (estados, transiciones) = gramatica::lr0::construir_automata(&g);

            println!("✓ Autómata construido!");
            println!("  Total de estados: {}", estados.len());
            println!("  Total de transiciones: {}", transiciones.len());
            println!();

            // Mostrar los primeros estados
            for (idx, estado) in estados.iter().take(5).enumerate() {
                println!("=== Estado I{} ===", idx);
                for item in estado {
                    println!("  {}", gramatica::lr0::item_to_string(item, &g));
                }
                println!();
            }

            // Mostrar algunas transiciones
            println!("=== Transiciones (primeras 10) ===");
            for ((desde, simbolo), hacia) in transiciones.iter().take(10) {
                let sim_str = match simbolo {
                    gramatica::Simbolo::Terminal(s) | gramatica::Simbolo::NoTerminal(s) => s,
                };
                println!("  I{} --[{}]--> I{}", desde, sim_str, hacia);
            }
        }
        Err(e) => {
            eprintln!("✗ Error: {}", e);
        }
    }
}
