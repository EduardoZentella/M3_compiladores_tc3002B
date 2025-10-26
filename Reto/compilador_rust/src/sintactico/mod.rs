pub mod tabla_slr;

use crate::lexico::token::Token;
use crate::sintactico::tabla_slr::{TABLA_ACTION, TABLA_GOTO, PRODUCCIONES, Accion};

/// Analiza una secuencia de tokens usando el analizador sintáctico SLR
pub fn analyze(tokens: &[Token], is_verbose: &bool) -> Result<(), String> {
    let mut pila_estados: Vec<usize> = vec![0]; // Pila de estados, inicia en estado 0
    let mut cursor = 0; // Posición actual en el vector de tokens

    if *is_verbose {
        println!("\n=== Iniciando análisis sintáctico SLR ===");
        println!("Total de tokens: {}\n", tokens.len());
    }

    loop {
        let estado_actual = *pila_estados.last().unwrap();

        // Obtener el token actual usando su tipo
        // El método as_grammar() convierte el TipoToken a la representación
        // usada en las tablas ACTION y GOTO
        let token_str = if cursor < tokens.len() {
            tokens[cursor].tipo.as_grammar().to_string()
        } else {
            "$".to_string()
        };

        if *is_verbose {
            let token_valor = if cursor < tokens.len() {
                &tokens[cursor].valor
            } else {
                "$"
            };
            println!("Estado: {} | Token: '{}' (tipo: {}) | Pila: {:?}",
                     estado_actual, token_valor, token_str, pila_estados);
        }

        // Consultar la Tabla ACTION
        let accion = TABLA_ACTION
            .get(&(estado_actual, token_str.clone()))
            .cloned();

        match accion {
            Some(Accion::Shift(nuevo_estado)) => {
                if *is_verbose {
                    println!("  → Acción: Shift({})", nuevo_estado);
                }
                pila_estados.push(nuevo_estado);
                cursor += 1;
            }

            Some(Accion::Reduce(num_regla)) => {
                let regla = &PRODUCCIONES[num_regla];

                if *is_verbose {
                    println!("  → Acción: Reduce({}) - {} → {} símbolos",
                             num_regla, regla.cabeza, regla.longitud_cuerpo);
                }

                // Sacar (pop) 'longitud_cuerpo' estados de la pila
                for _ in 0..regla.longitud_cuerpo {
                    pila_estados.pop();
                }

                // Consultar la Tabla GOTO
                let estado_anterior = *pila_estados.last().unwrap();
                let nuevo_estado = TABLA_GOTO
                    .get(&(estado_anterior, regla.cabeza.clone()))
                    .ok_or_else(|| {
                        format!(
                            "Error fatal: GOTO no encontrado para estado {} y no-terminal {}",
                            estado_anterior, regla.cabeza
                        )
                    })?;

                pila_estados.push(*nuevo_estado);

                // TODO: Construir AST
            }

            Some(Accion::Accept) => {
                if *is_verbose {
                    println!("  → Acción: Accept");
                    println!("\n✓ Análisis sintáctico completado exitosamente");
                }
                return Ok(()); // ¡Éxito!
            }

            None => {
                // Error de sintaxis
                let mensaje = if cursor < tokens.len() {
                    let token = &tokens[cursor];
                    format!(
                        "Error de sintaxis en línea {}: token inesperado '{}' (esperado en estado {})",
                        token.linea, token.valor, estado_actual
                    )
                } else {
                    format!(
                        "Error de sintaxis: fin inesperado del archivo (estado {})",
                        estado_actual
                    )
                };

                if *is_verbose {
                    println!("  → Error detectado");
                }

                return Err(mensaje);
            }
        }
    }
}
