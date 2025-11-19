pub mod tabla_slr;

use crate::lexico::token::Token;
use crate::sintactico::tabla_slr::*;
use crate::semantico::{ContextoSemantico, TipoDato};
use crate::intermedio::GeneradorCuadruplos;

// ==================== SISTEMA DE EVENTOS SEMÁNTICOS ====================
// Detectamos producciones por su estructura

#[derive(Debug, Clone, PartialEq)]
enum EventoSemantico {
    // Contexto y Tablas (Paso 1-12)
    ProgramaInicio,           // Paso 1: programa id ;
    ProgramaFin,              // Paso 2: fin
    VarsInicio,               // Paso 3: vars
    VarListRecolectar,        // Paso 4: colectar nombres
    TipoEstablecer,           // Paso 5: establecer tipo
    VarsAgregar,              // Paso 6: agregar variables
    FuncionDeclaracion,       // Paso 7-12: función completa
    ParametroAgregar,         // Paso 11: parámetro

    // Expresiones (Paso 1-5)
    OperandoProcesar,         // Paso 1: id o constante
    OperadorMult,             // Paso 2: * o /
    OperadorSuma,             // Paso 3: + o -
    OperadorRelacional,       // Operador relacional (>, <, ==, !=)
    ExpresionSumaGenerar,     // Paso 4: generar +/-
    ExpresionRelacionalGenerar, // Generar expresión relacional (>, <, ==, !=)
    TerminoMultGenerar,       // Paso 5: generar *//
    ParentesisAbrir,          // Paréntesis (
    ParentesisCerrar,         // Paréntesis )

    // Asignación
    AsignacionGenerar,        // id = EXPR ;

    // Impresión
    ImpresionGenerar,         // escribe ( ... ) ;

    // Control (Paso 13-20)
    CondicionExpresion,       // Paso 13: si ( EXPR )
    CondicionSinElse,         // Paso 14: fin si sin else
    CondicionAntesElse,       // Paso 16: antes de else
    CondicionDespuesElse,     // Paso 17: después de else
    CicloInicio,              // Paso 18: mientras (
    CicloExpresion,           // Paso 19: mientras ( EXPR )
    CicloFin,                 // Paso 20: fin mientras

    // Llamadas (Paso 1-6)
    LlamadaVerificar,         // Paso 1: verificar función
    LlamadaERA,               // Paso 2: generar ERA
    LlamadaParametro,         // Paso 3-4: PARAMETER
    LlamadaVerifParam,        // Paso 5: verificar # params
    LlamadaGOSUB,             // Paso 6: GOSUB

    // Estructural (sin acción semántica)
    Epsilon,
    Propagacion,
}

/// Detecta qué evento semántico corresponde a una reducción
fn detectar_evento(cabeza: &str, cuerpo_len: usize, atributos: &[String]) -> EventoSemantico {
    // DEBUG: Imprimir cabeza para diagnóstico
    if false {
        eprintln!("[DETECT] cabeza='{}' (bytes: {:?}), len={}", cabeza, cabeza.as_bytes(), cuerpo_len);
    }

    match (cabeza, cuerpo_len) {
        // ==================== PROGRAMA ====================
        ("<Programa>", 8) => EventoSemantico::ProgramaInicio,

        // ==================== VARIABLES ====================
        ("<VARS>", 5) => EventoSemantico::VarsAgregar,
        ("<VAR_LIST>", 2) => EventoSemantico::VarListRecolectar,
        ("<VAR_LIST'>", 3) => EventoSemantico::VarListRecolectar,
        ("<VAR_LIST'>", 0) => EventoSemantico::Epsilon,

        ("<TIPO>", 1) => {
            if atributos.first().map(|s| s.as_str()) == Some("entero")
                || atributos.first().map(|s| s.as_str()) == Some("flotante")
                || atributos.first().map(|s| s.as_str()) == Some("char") {
                EventoSemantico::TipoEstablecer
            } else {
                EventoSemantico::Propagacion
            }
        }

        // ==================== EXPRESIONES ====================
        ("<CTE_OPT>", 1) => {
            // Detectar si es id o CTE
            if !atributos.is_empty() {
                EventoSemantico::OperandoProcesar
            } else {
                EventoSemantico::Propagacion
            }
        }

        ("<CTE>", 1) => EventoSemantico::Propagacion, // CTE propaga a CTE_OPT

        ("<*/>", 1) => EventoSemantico::OperadorMult,
        ("<+->", 1) => EventoSemantico::OperadorSuma,
        ("<OPERADOR>", 1) => EventoSemantico::OperadorRelacional,  // >, <, ==, !=
        ("<EXP'>", 3) => EventoSemantico::ExpresionSumaGenerar,  // +- TÉRMINO EXP'
        ("<EXP’>", 0) => EventoSemantico::Epsilon,

        ("<TÉRMINO'>", 3) => EventoSemantico::TerminoMultGenerar,  // */ FACTOR TÉRMINO'
        ("<TÉRMINO'>", 0) => EventoSemantico::Epsilon,

        ("<FACTOR>", 3) if atributos.get(0).map(|s| s.as_str()) == Some("(") => {
            EventoSemantico::ParentesisCerrar
        }
        ("<FACTOR>", 2) => EventoSemantico::Propagacion,  // +-_OPT CTE_OPT
        ("<FACTOR>", 1) => EventoSemantico::Propagacion,  // LLAMADA

        ("<+-_OPT>", 1) => EventoSemantico::Propagacion,
        ("<+-_OPT>", 0) => EventoSemantico::Epsilon,

        // ==================== ASIGNACIÓN ====================
        ("<ASIGNA>", 4) => EventoSemantico::AsignacionGenerar,

        // ==================== IMPRESIÓN ====================
        ("<IMPRIME>", 6) => EventoSemantico::ImpresionGenerar,

        // ==================== CONTROL ====================
        ("<CONDICIÓN>", 7) => EventoSemantico::CondicionExpresion,
        ("<SINO_OPT>", 2) => EventoSemantico::CondicionAntesElse,
        ("<SINO_OPT>", 0) => EventoSemantico::CondicionSinElse,

        // <CICLO> → mientras ( <EXPRESIÓN> ) haz <CUERPO> (6 símbolos)
        // Pattern (producción, longitud) se dispara cuando REDUCE esa producción
        // Necesitamos CicloExpresion cuando ya tiene mientras ( <EXPRESIÓN> )
        // Usaremos SHIFT para mientras ( y el REDUCE final para CicloFin
        ("<CICLO>", 6) => EventoSemantico::CicloFin,

        // ==================== FUNCIONES ====================
        ("<FUNCS>", 9) => EventoSemantico::FuncionDeclaracion,
        ("<ARG_LIST>", 4) => EventoSemantico::ParametroAgregar,

        // ==================== LLAMADAS ====================
        ("<LLAMADA>", 4) => EventoSemantico::LlamadaGOSUB,

        // ==================== ESTRUCTURAL ====================
        ("<EXPRESIÓN>", 2) => EventoSemantico::Propagacion,
        ("<EXPRESIÓN'>", 2) => EventoSemantico::ExpresionRelacionalGenerar,  // <OPERADOR> <EXP>
        ("<EXPRESIÓN'>", 0) => EventoSemantico::Epsilon,
        ("<EXP>", 2) => EventoSemantico::Propagacion,
        ("<TÉRMINO>", 2) => EventoSemantico::Propagacion,
        ("<ESTATUTO>", 1) => EventoSemantico::Propagacion,
        ("<VARS_OPT>", _) => EventoSemantico::Propagacion,
        ("<FUNCS_LIST>", _) => EventoSemantico::Propagacion,
        ("<CUERPO>", _) => EventoSemantico::Propagacion,
        ("<ESTATUTO_LIST>", _) => EventoSemantico::Propagacion,

        _ => EventoSemantico::Propagacion,
    }
}


/// Analiza una secuencia de tokens usando el analizador sintáctico SLR
pub fn analyze(tokens: &[Token], is_verbose: &bool) -> Result<(), String> {
    let mut pila_estados: Vec<usize> = vec![0]; // Pila de estados, inicia en estado 0
    let mut cursor = 0; // Posición actual en el vector de tokens
    let mut contexto = ContextoSemantico::new();
    let mut generador = GeneradorCuadruplos::new();

    // Inicializar programa con nombre temporal
    // Esto se actualizará cuando se procese el nombre real del programa
    contexto.inicializar_programa("__programa_temp__")?;

    // Establecer contexto en el generador
    generador.establecer_contexto(&contexto);

    // Pila semántica para tracking de atributos
    let mut pila_semantica: Vec<String> = Vec::new();

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

                // Guardar atributo semántico del token
                if cursor < tokens.len() {
                    pila_semantica.push(tokens[cursor].valor.clone());

                    // Detectar inicio de condición: si ( o mientras (
                    let token_actual = &tokens[cursor].tipo;
                    if matches!(token_actual, crate::lexico::token::TipoToken::Si) {
                        // Mirar adelante para confirmar que viene (
                        if cursor + 1 < tokens.len() &&
                           matches!(tokens[cursor + 1].tipo, crate::lexico::token::TipoToken::ParenAbre) {
                            eprintln!("[PARSER] Detectado 'si (' - marcando inicio de condición");
                            generador.iniciar_condicion();
                        }
                    } else if matches!(token_actual, crate::lexico::token::TipoToken::Mientras) {
                        if cursor + 1 < tokens.len() &&
                           matches!(tokens[cursor + 1].tipo, crate::lexico::token::TipoToken::ParenAbre) {
                            eprintln!("[PARSER] Detectado 'mientras (' - marcando inicio de ciclo");
                            generador.marcar_inicio_ciclo();
                            generador.iniciar_condicion(); // También marca que estamos en condición para GOTOF
                        }
                    } else if matches!(token_actual, crate::lexico::token::TipoToken::Sino) {
                        // Detectar inicio de else - generar GOTO antes de procesar el cuerpo
                        eprintln!("[PARSER] Detectado 'sino' - generando GOTO para saltar el else");
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

                eprintln!("[DEBUG] REDUCE: Rule {} - {} → {} símbolos",
                         num_regla, regla.cabeza, regla.longitud_cuerpo);

                if *is_verbose {
                    println!("  → Acción: Reduce({}) - {} → {} símbolos",
                             num_regla, regla.cabeza, regla.longitud_cuerpo);
                }

                // Extraer atributos semánticos de la pila
                let atributos = extraer_atributos(&mut pila_semantica, regla.longitud_cuerpo);

                eprintln!("[DEBUG] REDUCE: atributos extraídos = {:?}", atributos);

                // Ejecutar acción semántica basada en EVENTO
                let atributo_sintetizado = ejecutar_accion_semantica(
                    &regla.cabeza,
                    regla.longitud_cuerpo,
                    &atributos,
                    &tokens,
                    &mut contexto,
                    &mut generador,
                )?;

                eprintln!("[DEBUG] REDUCE: atributo_sintetizado = '{}'", atributo_sintetizado);

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
                if *is_verbose {
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

                return Ok(()); // ¡Éxito!
            }            None => {
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

/// Ejecuta la acción semántica asociada a una regla de producción
/// Implementa los Puntos Neurálgicos (PN) según la especificación
/// # Argumentos
/// * `num_regla` - Número de la regla de producción
/// * `atributos` - Atributos semánticos de los símbolos del lado derecho
/// * `tokens` - Vector de tokens completo
/// * `contexto` - Contexto semántico para tracking de variables/funciones
/// * `generador` - Generador de cuádruplos para código intermedio
/// # Retorna
/// * `Ok(String)` con el atributo sintetizado
/// * `Err(String)` si ocurre un error semántico
fn ejecutar_accion_semantica(
    cabeza: &str,
    cuerpo_len: usize,
    atributos: &[String],
    _tokens: &[Token],
    contexto: &mut ContextoSemantico,
    generador: &mut GeneradorCuadruplos,
) -> Result<String, String> {

    // Detectar qué evento semántico corresponde a esta reducción
    let evento = detectar_evento(cabeza, cuerpo_len, atributos);

    eprintln!("[DEBUG] Evento: {:?} (cabeza: {}, len: {})", evento, cabeza, cuerpo_len);

    match evento {
        // ==================== CONTEXTO Y TABLAS (Paso 1-12) ====================

        EventoSemantico::ProgramaInicio => {
            // Paso 1-2: Ya inicializado al inicio de analyze()
            Ok(String::new())
        }

        EventoSemantico::TipoEstablecer => {
            // Paso 5: Establecer tipo actual
            if let Some(tipo_str) = atributos.first() {
                let tipo = match tipo_str.as_str() {
                    "entero" => TipoDato::Entero,
                    "flotante" => TipoDato::Flotante,
                    "char" => TipoDato::Char,
                    _ => return Err(format!("Tipo desconocido: {}", tipo_str)),
                };
                eprintln!("[SEMANTICA] Estableciendo tipo actual: {:?}", tipo);
                contexto.establecer_tipo_actual(tipo);
                Ok(tipo_str.clone())
            } else {
                Ok(String::new())
            }
        }

        EventoSemantico::VarListRecolectar => {
            // Paso 4: Recolectar nombres de variables
            // <VAR_LIST> → id <VAR_LIST'>: atributos[0]=id, atributos[1]=VAR_LIST'
            // <VAR_LIST'> → , id <VAR_LIST'>: atributos[0]=",", atributos[1]=id, atributos[2]=VAR_LIST'
            if cabeza == "<VAR_LIST>" && atributos.len() >= 2 {
                let mut nombres = atributos[0].clone();
                if !atributos[1].is_empty() {
                    nombres.push(',');
                    nombres.push_str(&atributos[1]);
                }
                Ok(nombres)
            } else if cabeza == "<VAR_LIST'>" && atributos.len() >= 3 {
                let mut nombres = atributos[1].clone();
                if !atributos[2].is_empty() {
                    nombres.push(',');
                    nombres.push_str(&atributos[2]);
                }
                Ok(nombres)
            } else {
                Ok(atributos.first().unwrap_or(&String::new()).clone())
            }
        }

        EventoSemantico::VarsAgregar => {
            // Paso 6: Agregar variables a la tabla
            // <VARS> → vars <VAR_LIST> : <TIPO> ;
            // atributos[0]="vars", atributos[1]=var_list, atributos[2]=":", atributos[3]=tipo, atributos[4]=";"
            eprintln!("[SEMANTICA] Agregando variables: atributos = {:?}", atributos);

            if atributos.len() >= 4 {
                let var_list = &atributos[1];
                let nombres: Vec<&str> = var_list
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();

                eprintln!("[SEMANTICA] Variables a agregar: {:?}", nombres);

                for nombre_var in nombres {
                    if contexto.buscar_variable(nombre_var).is_some() {
                        return Err(format!("Error: Declaración múltiple de variable '{}'", nombre_var));
                    }
                    contexto.agregar_variable(nombre_var)?;
                    eprintln!("[SEMANTICA] Variable '{}' agregada exitosamente", nombre_var);
                }
            }
            Ok(String::new())
        }

        // ==================== EXPRESIONES (Paso 1-5) ====================

        EventoSemantico::OperandoProcesar => {
            // Paso 1: Procesar operando (id o constante)
            if let Some(operando) = atributos.first() {
                eprintln!("[SEMANTICA] Procesando operando: '{}'", operando);
                generador.procesar_operando(operando)?;
                Ok(operando.clone())
            } else {
                Ok(String::new())
            }
        }

        EventoSemantico::OperadorMult => {
            // Paso 2: Procesar operador * o /
            if let Some(op) = atributos.first() {
                eprintln!("[SEMANTICA] Procesando operador mult/div: '{}'", op);
                generador.procesar_mult_div(op)?;
                Ok(op.clone())
            } else {
                Ok(String::new())
            }
        }

        EventoSemantico::OperadorSuma => {
            // Paso 3: Procesar operador + o -
            if let Some(op) = atributos.get(0) {
                eprintln!("[SEMANTICA] Procesando operador suma/resta: '{}'", op);
                generador.procesar_suma_resta(op)?;
                Ok(op.clone())
            } else {
                Ok(String::new())
            }
        }

        EventoSemantico::OperadorRelacional => {
            // Procesar operador relacional (>, <, ==, !=)
            if let Some(op) = atributos.get(0) {
                eprintln!("[SEMANTICA] Procesando operador relacional: '{}'", op);
                generador.procesar_relacional(op)?;
                Ok(op.clone())
            } else {
                Ok(String::new())
            }
        }

        EventoSemantico::ExpresionSumaGenerar => {
            // Paso 4: Generar cuádruplo de suma/resta
            eprintln!("[SEMANTICA] Generando cuádruplo suma/resta");
            generador.generar_suma_resta()?;
            Ok(String::new())
        }

        EventoSemantico::ExpresionRelacionalGenerar => {
            // Generar cuádruplo de operador relacional (>, <, ==, !=)
            // Este evento se dispara cuando se reduce <EXPRESIÓN'> → <OPERADOR> <EXP>
            eprintln!("[SEMANTICA] Generando cuádruplo relacional");
            generador.generar_relacional()?;
            Ok(String::new())
        }

        EventoSemantico::TerminoMultGenerar => {
            // Paso 5: Generar cuádruplo de multiplicación/división
            eprintln!("[SEMANTICA] Generando cuádruplo mult/div");
            generador.generar_mult_div()?;
            Ok(String::new())
        }

        EventoSemantico::ParentesisCerrar => {
            eprintln!("[SEMANTICA] Cerrando paréntesis");
            generador.cerrar_parentesis()?;
            Ok(String::new())
        }

        // ==================== ASIGNACIÓN ====================

        EventoSemantico::AsignacionGenerar => {
            // <ASIGNA> → id = <EXPRESIÓN> ;
            eprintln!("[SEMANTICA] Generando asignación: atributos = {:?}", atributos);

            if let Some(nombre_var) = atributos.first() {
                if contexto.buscar_variable(nombre_var).is_none() {
                    return Err(format!("Error: Variable '{}' no declarada", nombre_var));
                }
                eprintln!("[SEMANTICA] Asignación a variable: '{}'", nombre_var);
                generador.generar_asignacion(nombre_var)?;
            }
            Ok(String::new())
        }

        // ==================== IMPRESIÓN ====================

        EventoSemantico::ImpresionGenerar => {
            eprintln!("[SEMANTICA] Generando escritura");
            generador.generar_escritura()?;
            Ok(String::new())
        }

        // ==================== CONTROL (Paso 13-20) ====================

        EventoSemantico::CondicionExpresion => {
            // Este evento se dispara cuando se reduce <CONDICIÓN> completa
            // En este punto ya se procesó todo: expresión, cuerpo, sino_opt
            // Marcar que iniciamos una condición ANTES de parsear la expresión
            // (este evento se dispara al FINAL, pero marca el contexto para futuras reducciones)
            eprintln!("[SEMANTICA] <CONDICIÓN> completa parseada");
            Ok(String::new())
        }

        EventoSemantico::CondicionSinElse => {
            // Se dispara cuando <SINO_OPT> → ε (no hay else)
            // Aquí debemos hacer FILL del GOTOF que se generó al procesar la expresión
            eprintln!("[SEMANTICA] Condición sin else: haciendo FILL");
            generador.fill_salto_condicional()?;
            Ok(String::new())
        }

        EventoSemantico::CondicionAntesElse => {
            // Se dispara cuando <SINO_OPT> → sino <CUERPO> (hay else)
            // Generar GOTO para saltar al final del else, y FILL del GOTOF
            eprintln!("[SEMANTICA] Condición con else: generando GOTO y FILL");
            generador.generar_else()?;
            Ok(String::new())
        }

        EventoSemantico::CondicionDespuesElse => {
            // Se dispara después del else (si existiera un patrón para esto)
            eprintln!("[SEMANTICA] Después de else: haciendo FILL del GOTO");
            generador.fill_salto_condicional()?;
            Ok(String::new())
        }

        EventoSemantico::CicloInicio => {
            // Paso 18: mientras ( - Guardar posición de inicio
            eprintln!("[SEMANTICA] Ciclo: guardando inicio");
            generador.marcar_inicio_ciclo();
            Ok(String::new())
        }

        EventoSemantico::CicloExpresion => {
            // Paso 19: mientras ( EXPRESIÓN ) - Generar GOTOF
            eprintln!("[SEMANTICA] Ciclo: generando GOTOF");
            generador.generar_gotof()?;
            Ok(String::new())
        }

        EventoSemantico::CicloFin => {
            // Paso 20: Fin de mientras - GOTO inicio y FILL GOTOF
            eprintln!("[SEMANTICA] Ciclo: generando GOTO y rellenando salto");
            generador.generar_fin_ciclo()?;
            Ok(String::new())
        }

        // ==================== LLAMADAS (Paso 1-6) ====================

        EventoSemantico::LlamadaVerificar => {
            // Paso 1: Verificar función existe
            if let Some(nombre_func) = atributos.first() {
                eprintln!("[SEMANTICA] Llamada: verificando función '{}'", nombre_func);
                contexto.verificar_funcion_existe(nombre_func)?;
            }
            Ok(String::new())
        }

        EventoSemantico::LlamadaERA => {
            // Paso 2: Generar ERA
            eprintln!("[SEMANTICA] Llamada: generando ERA");
            generador.generar_era()?;
            Ok(String::new())
        }

        EventoSemantico::LlamadaParametro => {
            // Paso 3-4: Generar PARAMETER
            eprintln!("[SEMANTICA] Llamada: generando PARAMETER");
            generador.generar_parameter()?;
            Ok(String::new())
        }

        EventoSemantico::LlamadaVerifParam => {
            // Paso 5: Verificar número de parámetros
            eprintln!("[SEMANTICA] Llamada: verificando parámetros");
            generador.verificar_parametros()?;
            Ok(String::new())
        }

        EventoSemantico::LlamadaGOSUB => {
            // Paso 6: Generar GOSUB
            eprintln!("[SEMANTICA] Llamada: generando GOSUB");
            generador.generar_gosub()?;
            Ok(String::new())
        }

        // ==================== ESTRUCTURAL ====================

        EventoSemantico::Epsilon => {
            // Epsilon: no hay atributos
            Ok(String::new())
        }

        EventoSemantico::Propagacion => {
            // Propagación: pasar el primer atributo no vacío
            for attr in atributos {
                if !attr.is_empty() {
                    return Ok(attr.clone());
                }
            }
            Ok(String::new())
        }

        _ => Ok(String::new()),
    }
}