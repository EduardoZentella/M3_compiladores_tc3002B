use crate::gramatica::{Gramatica, Simbolo};
use std::collections::{HashMap, HashSet};

/// Extrae el String de un Simbolo::Terminal o Simbolo::NoTerminal
fn extraer_string(simbolo: &Simbolo) -> Option<String> {
    match simbolo {
        Simbolo::Terminal(s) | Simbolo::NoTerminal(s) => Some(s.clone()),
    }
}

/// Calcula los conjuntos FIRST para cada símbolo de la gramática
pub fn calcular_first_sets(gramatica: &Gramatica) -> HashMap<String, HashSet<String>> {
    let mut first_sets: HashMap<String, HashSet<String>> = HashMap::new();

    // FIRST(terminal) = {terminal}
    for simbolo in &gramatica.simbolos_terminales {
        if let Simbolo::Terminal(t) = simbolo {
            first_sets.insert(t.clone(), HashSet::from([t.clone()]));
        }
    }

    // Inicializar FIRST(no-terminal) = {}
    for simbolo in &gramatica.simbolos_no_terminales {
        if let Simbolo::NoTerminal(nt) = simbolo {
            first_sets.insert(nt.clone(), HashSet::new());
        }
    }

    // Iterar hasta que no haya cambios
    let mut cambio = true;
    while cambio {
        cambio = false;

        for prod in &gramatica.producciones {
            let cabeza = extraer_string(&prod.cabeza).unwrap();
            let mut nuevos = Vec::new();
            let mut todo_epsilon = true;

            // Para cada símbolo en el cuerpo de la producción
            for simbolo in &prod.cuerpo {
                if !todo_epsilon { break; }

                let sym = extraer_string(simbolo).unwrap();
                let first_sym = first_sets.get(&sym).unwrap();

                // Agregar FIRST(símbolo) - {ε}
                nuevos.extend(first_sym.iter().filter(|s| *s != "ε").cloned());

                // Si no contiene ε, detenerse
                todo_epsilon = first_sym.contains("ε");
            }

            // Si todos contienen ε, agregar ε al FIRST
            if todo_epsilon {
                nuevos.push("ε".to_string());
            }

            // Insertar nuevos símbolos en FIRST(cabeza)
            for s in nuevos {
                if first_sets.get_mut(&cabeza).unwrap().insert(s) {
                    cambio = true;
                }
            }
        }
    }

    first_sets
}

/// Calcula los conjuntos FOLLOW para cada no-terminal de la gramática
pub fn calcular_follow_sets(
    gramatica: &Gramatica,
    first_sets: &HashMap<String, HashSet<String>>,
) -> HashMap<String, HashSet<String>> {
    let mut follow_sets: HashMap<String, HashSet<String>> = HashMap::new();

    // Inicializar FOLLOW(no-terminal) = {}
    for simbolo in &gramatica.simbolos_no_terminales {
        if let Simbolo::NoTerminal(nt) = simbolo {
            follow_sets.insert(nt.clone(), HashSet::new());
        }
    }

    // FOLLOW(símbolo inicial) contiene $
    if let Some(primera_prod) = gramatica.producciones.first() {
        let inicial = extraer_string(&primera_prod.cabeza).unwrap();
        follow_sets.get_mut(&inicial).unwrap().insert("$".to_string());
    }

    // Iterar hasta que no haya cambios
    let mut cambio = true;
    while cambio {
        cambio = false;

        for prod in &gramatica.producciones {
            let cabeza = extraer_string(&prod.cabeza).unwrap();

            for i in 0..prod.cuerpo.len() {
                if let Simbolo::NoTerminal(nt) = &prod.cuerpo[i] {
                    let mut nuevos = HashSet::new();
                    let mut todo_epsilon = true;

                    // Para cada símbolo después de nt
                    for simbolo in &prod.cuerpo[i + 1..] {
                        if !todo_epsilon { break; }

                        let sym = extraer_string(simbolo).unwrap();
                        let first_sym = first_sets.get(&sym).unwrap();

                        // Agregar FIRST(símbolo) - {ε} al FOLLOW
                        nuevos.extend(first_sym.iter().filter(|s| *s != "ε").cloned());

                        todo_epsilon = first_sym.contains("ε");
                    }

                    // Si todos los símbolos después pueden ser ε, agregar FOLLOW(cabeza)
                    if todo_epsilon {
                        nuevos.extend(follow_sets.get(&cabeza).unwrap().clone());
                    }

                    // Insertar en FOLLOW(nt)
                    for s in nuevos {
                        if follow_sets.get_mut(nt).unwrap().insert(s) {
                            cambio = true;
                        }
                    }
                }
            }
        }
    }

    follow_sets
}
