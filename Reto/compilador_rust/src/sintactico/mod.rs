pub mod tabla_slr;
mod acciones_semanticas;

use crate::lexico::token::Token;
use crate::sintactico::tabla_slr::*;
use crate::semantico::ContextoSemantico;
use crate::intermedio::GeneradorCuadruplos;
use acciones_semanticas::ejecutar_accion_semantica;

/// Analiza una secuencia de tokens usando el analizador sintáctico SLR bottom-up
///
/// Este parser utiliza tablas ACTION y GOTO generadas por el algoritmo SLR(1).
/// Las acciones semánticas se ejecutan al REDUCIR cada producción, compatible
/// con la naturaleza bottom-up del parser.
///
/// # Parámetros
/// - `tokens`: Secuencia de tokens del análisis léxico
/// - `nivel_verbose`: Nivel de debug (0=ninguno, 1=básico, 2=detallado, 3=completo)
///
/// Retorna el generador de cuádruplos para permitir la ejecución en la VM.
pub fn analyze(tokens: &[Token], nivel_verbose: usize) -> Result<GeneradorCuadruplos, String> {
    let mut pila_estados: Vec<usize> = vec![0]; // Pila de estados, inicia en estado 0
    let mut cursor = 0; // Posición actual en el vector de tokens
    let mut contexto = ContextoSemantico::new();
    let mut generador = GeneradorCuadruplos::new();

    // Inicializar programa con nombre temporal
    contexto.inicializar_programa("__programa_temp__")?;

    // Establecer contexto en el generador
    generador.establecer_contexto(&contexto);

    // Generar GOTO al inicio del programa principal (saltar funciones)
    // Este GOTO se hará FILL cuando se encuentre el token 'inicio'
    generador.generar_goto_inicio()?;

    // Pila semántica para tracking de atributos
    let mut pila_semantica: Vec<String> = Vec::new();

    if nivel_verbose >= 1 {
        println!("\n=== Iniciando análisis sintáctico SLR ===");
        println!("Total de tokens: {}\n", tokens.len());
    }

    loop {
        let estado_actual = *pila_estados.last().unwrap();

        // Obtener el token actual usando su tipo
        let token_str = if cursor < tokens.len() {
            tokens[cursor].tipo.as_grammar().to_string()
        } else {
            "$".to_string()
        };

        if nivel_verbose >= 2 {
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
                if nivel_verbose >= 2 {
                    println!("  → Acción: Shift({})", nuevo_estado);
                }

                // Guardar atributo semántico del token
                if cursor < tokens.len() {
                    pila_semantica.push(tokens[cursor].valor.clone());

                    // Detectar token 'inicio': hacer FILL del GOTO al main
                    let token_actual = &tokens[cursor].tipo;
                    if matches!(token_actual, crate::lexico::token::TipoToken::Inicio) {
                        if nivel_verbose >= 2 {
                            println!("[PARSER] Detectado 'inicio' - haciendo FILL de GOTO al main");
                        }
                        generador.fill_goto_inicio()?;
                    }

                    // Detectar inicio de condición: si ( o mientras (
                    if matches!(token_actual, crate::lexico::token::TipoToken::Si) {
                        if cursor + 1 < tokens.len() &&
                           matches!(tokens[cursor + 1].tipo, crate::lexico::token::TipoToken::ParenAbre) {
                            if nivel_verbose >= 2 {
                                println!("[PARSER] Detectado 'si (' - marcando inicio de condición");
                            }
                            generador.iniciar_condicion();
                        }
                    } else if matches!(token_actual, crate::lexico::token::TipoToken::Mientras) {
                        if cursor + 1 < tokens.len() &&
                           matches!(tokens[cursor + 1].tipo, crate::lexico::token::TipoToken::ParenAbre) {
                            if nivel_verbose >= 2 {
                                println!("[PARSER] Detectado 'mientras (' - marcando inicio de ciclo");
                            }
                            generador.marcar_inicio_ciclo();
                            generador.iniciar_condicion();
                        }
                    } else if matches!(token_actual, crate::lexico::token::TipoToken::Sino) {
                        if nivel_verbose >= 2 {
                            println!("[PARSER] Detectado 'sino' - generando GOTO para saltar el else");
                        }
                        if let Err(e) = generador.iniciar_else() {
                            return Err(format!("Error al iniciar else: {}", e));
                        }
                    }
                }

                pila_estados.push(nuevo_estado);
                cursor += 1;
            }

            Some(Accion::Reduce(num_regla)) => {
                let regla = &PRODUCCIONES[num_regla];

                if nivel_verbose >= 3 {
                    println!("[DEBUG] REDUCE: Rule {} - {} → {} símbolos",
                             num_regla, regla.cabeza, regla.longitud_cuerpo);
                }

                if nivel_verbose >= 2 {
                    println!("  → Acción: Reduce({}) - {} → {} símbolos",
                             num_regla, regla.cabeza, regla.longitud_cuerpo);
                }

                // Extraer atributos semánticos de la pila
                let atributos = extraer_atributos(&mut pila_semantica, regla.longitud_cuerpo);

                if nivel_verbose >= 3 {
                    println!("[DEBUG] REDUCE: atributos extraídos = {:?}", atributos);
                }

                // Ejecutar acción semántica
                let atributo_sintetizado = ejecutar_accion_semantica(
                    &regla.cabeza,
                    regla.longitud_cuerpo,
                    &atributos,
                    &tokens,
                    &mut contexto,
                    &mut generador,
                    nivel_verbose,
                )?;

                if nivel_verbose >= 3 {
                    println!("[DEBUG] REDUCE: atributo_sintetizado = '{}'", atributo_sintetizado);
                }

                // Pushear atributo sintetizado de vuelta
                pila_semantica.push(atributo_sintetizado);

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
            }

            Some(Accion::Accept) => {
                if nivel_verbose >= 1 {
                    println!("  → Acción: Accept");
                }

                // Mostrar resultados
                println!("\n✓✓✓ Compilación exitosa ✓✓✓");

                // Mostrar cuádruplos generados
                let cuadruplos = generador.obtener_cuadruplos();
                if !cuadruplos.is_empty() {
                    println!("\n=== Código Intermedio Generado ===");
                    for (i, cuadruplo) in cuadruplos.iter().enumerate() {
                        println!("{}: {:?}", i, cuadruplo);
                    }
                }

                // Retornar el generador para que main.rs pueda exportar el programa
                return Ok(generador);
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

                if nivel_verbose >= 2 {
                    println!("  → Error detectado");
                }

                return Err(mensaje);
            }
        }
    }
}

/// Extrae N atributos semánticos de la pila
fn extraer_atributos(pila: &mut Vec<String>, n: usize) -> Vec<String> {
    let mut atributos = Vec::new();
    for _ in 0..n {
        if let Some(val) = pila.pop() {
            atributos.insert(0, val);
        } else {
            atributos.insert(0, String::new());
        }
    }
    atributos
}
