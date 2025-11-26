// ==================== ACCIONES SEMÁNTICAS ====================
// Este módulo contiene la lógica de acciones semánticas para el parser SLR bottom-up
// Cada producción tiene una acción específica basada en (cabeza, longitud_cuerpo)

use crate::lexico::token::Token;
use crate::semantico::{ContextoSemantico, TipoDato};
use crate::intermedio::GeneradorCuadruplos;

/// Ejecuta la acción semántica asociada a una reducción en el parser SLR
///
/// Este sistema usa matching directo sobre las producciones gramaticales,
/// compatible con parsers bottom-up que ejecutan acciones al REDUCIR (no mid-rule).
///
/// # Argumentos
/// * `cabeza` - Cabeza de la regla de producción (lado izquierdo)
/// * `cuerpo_len` - Longitud del cuerpo de la producción (lado derecho)
/// * `atributos` - Atributos semánticos de los símbolos del lado derecho
/// * `_tokens` - Vector de tokens completo (no usado actualmente)
/// * `contexto` - Contexto semántico para tracking de variables/funciones
/// * `generador` - Generador de cuádruplos para código intermedio
/// * `nivel_verbose` - Nivel de debug (0=ninguno, 1=básico, 2=semántica, 3=completo)
///
/// # Retorna
/// * `Ok(String)` con el atributo sintetizado
/// * `Err(String)` si ocurre un error semántico
pub fn ejecutar_accion_semantica(
    cabeza: &str,
    cuerpo_len: usize,
    atributos: &[String],
    _tokens: &[Token],
    contexto: &mut ContextoSemantico,
    generador: &mut GeneradorCuadruplos,
    nivel_verbose: usize,
) -> Result<String, String> {

    if nivel_verbose >= 3 {
        println!("[DEBUG] Acción semántica: cabeza='{}', len={}, atributos={:?}", cabeza, cuerpo_len, atributos);
    }

    // Match directo sobre (cabeza, longitud del cuerpo)
    // Cada patrón corresponde a una producción específica de la gramática
    match (cabeza, cuerpo_len) {

        // ==================== PROGRAMA ====================

        ("<Programa>", 8) => {
            // programa id ; <VARS_OPT> <FUNCS_LIST> inicio <CUERPO> fin
            if nivel_verbose >= 2 {
                println!("[SEMANTICA] Programa completo parseado");
            }
            Ok(String::new())
        }

        // ==================== VARIABLES ====================

        // <TIPO> → entero | flotante
        ("<TIPO>", 1) => {
            if let Some(tipo_str) = atributos.first() {
                let tipo = match tipo_str.as_str() {
                    "entero" => TipoDato::Entero,
                    "flotante" => TipoDato::Flotante,
                    _ => return Err(format!("Tipo desconocido: {}", tipo_str)),
                };
                if nivel_verbose >= 2 {
                    println!("[SEMANTICA] Estableciendo tipo actual: {:?}", tipo);
                }
                contexto.establecer_tipo_actual(tipo);
                Ok(tipo_str.clone())
            } else {
                Ok(String::new())
            }
        }

        // <VAR_LIST> → id <VAR_LIST_PRIMA>
        ("<VAR_LIST>", 2) => {
            let mut nombres = atributos[0].clone();
            if !atributos[1].is_empty() {
                nombres.push(',');
                nombres.push_str(&atributos[1]);
            }
            if nivel_verbose >= 3 {
                println!("[SEMANTICA] VAR_LIST acumulado: '{}'", nombres);
            }
            Ok(nombres)
        }

        // <VAR_LIST_PRIMA> → , id <VAR_LIST_PRIMA>
        ("<VAR_LIST_PRIMA>", 3) => {
            let mut nombres = atributos[1].clone();
            if !atributos[2].is_empty() {
                nombres.push(',');
                nombres.push_str(&atributos[2]);
            }
            if nivel_verbose >= 3 {
                println!("[SEMANTICA] VAR_LIST_PRIMA acumulado: '{}'", nombres);
            }
            Ok(nombres)
        }

        // <VAR_LIST'> → ε
        ("<VAR_LIST'>", 0) => Ok(String::new()),

        // <VARS> → vars <VAR_LIST> : <TIPO> ;
        ("<VARS>", 5) => {
            if nivel_verbose >= 2 {
                println!("[SEMANTICA] Agregando variables: atributos = {:?}", atributos);
            }

            if atributos.len() >= 4 {
                let var_list = &atributos[1];
                let nombres: Vec<&str> = var_list
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();

                if nivel_verbose >= 2 {
                    println!("[SEMANTICA] Variables a agregar: {:?}", nombres);
                }

                for nombre_var in nombres {
                    if contexto.buscar_variable(nombre_var).is_some() {
                        return Err(format!("Error: Declaración múltiple de variable '{}'", nombre_var));
                    }
                    contexto.agregar_variable(nombre_var)?;
                    if nivel_verbose >= 3 {
                        println!("[SEMANTICA] Variable '{}' agregada exitosamente", nombre_var);
                    }
                }
            }
            Ok(String::new())
        }

        // ==================== FUNCIONES (con reglas intermedias) ====================

        // <TIPO_OPT> → nula
        ("<TIPO_OPT>", 1) if atributos.get(0) == Some(&"nula".to_string()) => {
            if nivel_verbose >= 2 {
                println!("[SEMANTICA] Tipo de retorno: nula");
            }
            Ok("nula".to_string())
        }

        // <TIPO_OPT> → <TIPO>
        ("<TIPO_OPT>", 1) => {
            if nivel_verbose >= 2 {
                println!("[SEMANTICA] Tipo de retorno: {}", atributos[0]);
            }
            Ok(atributos[0].clone())
        }

        // <FUNC_HEADER> → <TIPO_OPT> id
        ("<FUNC_HEADER>", 2) => {
            let tipo_retorno_str = &atributos[0];
            let nombre_func = &atributos[1];

            if nivel_verbose >= 2 {
                println!("[SEMANTICA] FUNC_HEADER: nombre='{}', tipo='{}'", nombre_func, tipo_retorno_str);
            }

            // Crear la función en el contexto semántico
            let tipo_retorno = if tipo_retorno_str == "nula" {
                crate::semantico::TipoRetorno::Nula
            } else {
                let tipo = TipoDato::from_str(tipo_retorno_str)
                    .ok_or_else(|| format!("Tipo desconocido: {}", tipo_retorno_str))?;
                crate::semantico::TipoRetorno::Tipo(tipo)
            };

            contexto.iniciar_funcion(nombre_func, tipo_retorno)?;

            if nivel_verbose >= 2 {
                println!("[SEMANTICA] Función '{}' creada", nombre_func);
            }

            Ok(nombre_func.clone())
        }

        // <ARG_LIST> → id : <TIPO> <ARG_LIST_PRIMA>
        ("<ARG_LIST>", 4) => {
            let nombre_param = &atributos[0];
            let tipo_str = &atributos[2];

            if nivel_verbose >= 2 {
                println!("[SEMANTICA] ARG_LIST: parámetro '{}' tipo '{}'", nombre_param, tipo_str);
            }

            // Agregar el parámetro a la función actual
            let tipo = TipoDato::from_str(tipo_str)
                .ok_or_else(|| format!("Tipo desconocido: {}", tipo_str))?;
            contexto.agregar_parametro(nombre_param, tipo)?;

            if nivel_verbose >= 3 {
                println!("[SEMANTICA] Parámetro '{}' agregado", nombre_param);
            }

            Ok(String::new())
        }        // <FUNC_ARGS> → <FUNC_HEADER> ( <ARG_OPT> )
        ("<FUNC_ARGS>", 4) => {
            let nombre_func = &atributos[0];

            if nivel_verbose >= 2 {
                println!("[SEMANTICA] FUNC_ARGS: finalizando declaración de '{}'", nombre_func);
            }

            // Finalizar la declaración de la función (asignar direcciones virtuales)
            contexto.finalizar_declaracion_funcion()?;

            // Iniciar ámbito local para el cuerpo de la función
            contexto.entrar_ambito_funcion(nombre_func)?;

            // Inicializar función en el generador de cuádruplos
            generador.iniciar_funcion(nombre_func)?;

            if nivel_verbose >= 2 {
                println!("[SEMANTICA] Función '{}' lista, entrando a su cuerpo", nombre_func);
            }

            Ok(nombre_func.clone())
        }

        // <FUNCS> → <FUNC_ARGS> { <VARS_OPT> <CUERPO> } ;
        ("<FUNCS>", 6) => {
            // Al reducir FUNCS completa, generamos ENDFUNC y salimos del ámbito
            if nivel_verbose >= 2 {
                println!("[SEMANTICA] FUNCS completa: generando ENDFUNC y saliendo de ámbito");
            }

            generador.finalizar_funcion()?;
            contexto.salir_ambito_funcion()?;

            Ok(String::new())
        }

        // ==================== RETURN ====================

        // <RETURN> → regresa <EXPRESIÓN> ;
        ("<RETURN>", 3) => {
            if nivel_verbose >= 2 {
                println!("[SEMANTICA] Procesando RETURN");
            }

            generador.generar_return()?;

            Ok(String::new())
        }

        // ==================== LLAMADAS (con reglas intermedias) ====================        // <LLAMADA_HEADER> → id
        ("<LLAMADA_HEADER>", 1) => {
            // Verificar que la función existe e iniciar llamada
            let nombre_func = &atributos[0];
            if nivel_verbose >= 2 {
                println!("[SEMANTICA] LLAMADA_HEADER: verificando función '{}'", nombre_func);
            }
            generador.iniciar_llamada(nombre_func)?;
            Ok(nombre_func.clone())
        }

        // <LLAMADA_ARGS> → <LLAMADA_HEADER> ( <EXPRESIÓN_OPT> )
        ("<LLAMADA_ARGS>", 4) => {
            // Generar GOSUB
            let nombre_func = &atributos[0];
            if nivel_verbose >= 2 {
                println!("[SEMANTICA] LLAMADA_ARGS: generando GOSUB para '{}'", nombre_func);
            }
            generador.generar_gosub(nombre_func)?;
            Ok(String::new())
        }

        // <LLAMADA> → <LLAMADA_ARGS>
        ("<LLAMADA>", 1) => {
            // Propagación
            Ok(atributos.first().unwrap_or(&String::new()).clone())
        }

        // ==================== EXPRESIONES ====================

        // <CTE_OPT> → id | <CTE>
        ("<CTE_OPT>", 1) => {
            if let Some(operando) = atributos.first() {
                if !operando.is_empty() {
                    if nivel_verbose >= 3 {
                        println!("[SEMANTICA] Procesando operando: '{}'", operando);
                    }
                    generador.procesar_operando(operando)?;
                    Ok(operando.clone())
                } else {
                    Ok(String::new())
                }
            } else {
                Ok(String::new())
            }
        }

        // <CTE> → cte_ent | cte_flot
        ("<CTE>", 1) => {
            Ok(atributos.first().unwrap_or(&String::new()).clone())
        }

        // <*/> → * | /
        ("<*/>", 1) => {
            if let Some(op) = atributos.first() {
                if nivel_verbose >= 3 {
                    println!("[SEMANTICA] Procesando operador mult/div: '{}'", op);
                }
                generador.procesar_mult_div(op)?;
                Ok(op.clone())
            } else {
                Ok(String::new())
            }
        }

        // <+-> → + | -
        ("<+->", 1) => {
            if let Some(op) = atributos.first() {
                if nivel_verbose >= 3 {
                    println!("[SEMANTICA] Procesando operador suma/resta: '{}'", op);
                }
                generador.procesar_suma_resta(op)?;
                Ok(op.clone())
            } else {
                Ok(String::new())
            }
        }

        // <OPERADOR> → > | < | == | !=
        ("<OPERADOR>", 1) => {
            if let Some(op) = atributos.first() {
                if nivel_verbose >= 3 {
                    println!("[SEMANTICA] Procesando operador relacional: '{}'", op);
                }
                generador.procesar_relacional(op)?;
                Ok(op.clone())
            } else {
                Ok(String::new())
            }
        }

        // <EXP_PRIMA> → <+-> <TÉRMINO> <EXP_PRIMA>
        ("<EXP_PRIMA>", 3) => {
            if nivel_verbose >= 3 {
                println!("[SEMANTICA] Generando cuádruplo suma/resta");
            }
            generador.generar_suma_resta()?;
            Ok(String::new())
        }

        // <EXP_PRIMA> → ε
        ("<EXP_PRIMA>", 0) => Ok(String::new()),

        // <EXPRESION_PRIMA> → <OPERADOR> <EXP>
        ("<EXPRESION_PRIMA>", 2) => {
            if nivel_verbose >= 3 {
                println!("[SEMANTICA] Generando cuádruplo relacional");
            }
            generador.generar_relacional()?;
            Ok(String::new())
        }

        // <EXPRESION_PRIMA> → ε
        ("<EXPRESION_PRIMA>", 0) => Ok(String::new()),

        // <TERMINO_PRIMA> → <*/> <FACTOR> <TERMINO_PRIMA>
        ("<TERMINO_PRIMA>", 3) => {
            if nivel_verbose >= 3 {
                println!("[SEMANTICA] Generando cuádruplo mult/div");
            }
            generador.generar_mult_div()?;
            Ok(String::new())
        }

        // <TERMINO_PRIMA> → ε
        ("<TERMINO_PRIMA>", 0) => Ok(String::new()),

        // <FACTOR> → ( <EXPRESIÓN> )
        ("<FACTOR>", 3) if atributos.get(0).map(|s| s.as_str()) == Some("(") => {
            if nivel_verbose >= 3 {
                println!("[SEMANTICA] Expresión entre paréntesis - precedencia manejada por gramática");
            }
            // No necesitamos abrir_parentesis/cerrar_parentesis porque el parser SLR
            // ya maneja la precedencia correctamente a través de la estructura de la gramática
            Ok(String::new())
        }

        // <FACTOR> → id
        ("<FACTOR>", 1) if atributos.get(0).map(|s| !s.is_empty() && !s.starts_with('+') && !s.starts_with('-') && s.chars().next().unwrap().is_alphabetic()) == Some(true) => {
            let operando = &atributos[0];
            if nivel_verbose >= 3 {
                println!("[SEMANTICA] Procesando operando id: '{}'", operando);
            }
            generador.procesar_operando(operando)?;
            Ok(operando.clone())
        }

        // <FACTOR> → <CTE>
        ("<FACTOR>", 1) if atributos.get(0).map(|s| s.chars().next().unwrap().is_numeric()) == Some(true) => {
            let operando = &atributos[0];
            if nivel_verbose >= 3 {
                println!("[SEMANTICA] Procesando operando <CTE>: '{}'", operando);
            }
            generador.procesar_operando(operando)?;
            Ok(operando.clone())
        }

        // <FACTOR> → + <CTE_OPT>
        ("<FACTOR>", 2) if atributos.get(0).map(|s| s.as_str()) == Some("+") => {
            let operando = &atributos[1];
            if nivel_verbose >= 3 {
                println!("[SEMANTICA] Procesando operando positivo: '+{}'", operando);
            }
            generador.procesar_operando(operando)?;
            Ok(operando.clone())
        }

        // <FACTOR> → - <CTE_OPT>
        ("<FACTOR>", 2) if atributos.get(0).map(|s| s.as_str()) == Some("-") => {
            let operando = &atributos[1];
            if nivel_verbose >= 3 {
                println!("[SEMANTICA] Procesando operando negativo: '-{}'", operando);
            }
            // Para negativos, podríamos generar una operación de negación
            // Por ahora, solo procesamos el operando
            generador.procesar_operando(operando)?;
            Ok(format!("-{}", operando))
        }

        // <FACTOR> → <LLAMADA>
        ("<FACTOR>", 1) => {
            // Propagación - catch-all para llamadas a funciones
            Ok(atributos.first().unwrap_or(&String::new()).clone())
        }

        // ==================== ASIGNACIÓN ====================

        // <ASIGNA> → id = <EXPRESIÓN> ;
        ("<ASIGNA>", 4) => {
            if nivel_verbose >= 2 {
                println!("[SEMANTICA] Generando asignación: atributos = {:?}", atributos);
            }

            if let Some(nombre_var) = atributos.first() {
                if contexto.buscar_variable(nombre_var).is_none() {
                    return Err(format!("Error: Variable '{}' no declarada", nombre_var));
                }
                if nivel_verbose >= 2 {
                    println!("[SEMANTICA] Asignación a variable: '{}'", nombre_var);
                }
                generador.generar_asignacion(nombre_var)?;
            }
            Ok(String::new())
        }

        // ==================== IMPRESIÓN ====================

        // <IMPRIME> → escribe ( <OBJ_IMPRIME> <IMPRIME_LIST> ) ;
        ("<IMPRIME>", 6) => {
            if nivel_verbose >= 2 {
                println!("[SEMANTICA] Generando escritura");
            }
            generador.generar_escritura()?;
            Ok(String::new())
        }

        // ==================== CONTROL ====================

        // <CONDICIÓN> → si ( <EXPRESIÓN> ) entonces <CUERPO> <SINO_OPT> ;
        ("<CONDICIÓN>", 7) => {
            if nivel_verbose >= 3 {
                println!("[SEMANTICA] <CONDICIÓN> completa parseada");
            }
            Ok(String::new())
        }

        // <SINO_OPT> → sino <CUERPO>
        ("<SINO_OPT>", 2) => {
            if nivel_verbose >= 2 {
                println!("[SEMANTICA] Condición con else: generando GOTO y FILL");
            }
            generador.generar_else()?;
            Ok(String::new())
        }

        // <SINO_OPT> → ε
        ("<SINO_OPT>", 0) => {
            if nivel_verbose >= 2 {
                println!("[SEMANTICA] Condición sin else: haciendo FILL");
            }
            generador.fill_salto_condicional()?;
            Ok(String::new())
        }

        // <CICLO> → mientras ( <EXPRESIÓN> ) haz <CUERPO>
        ("<CICLO>", 6) => {
            if nivel_verbose >= 2 {
                println!("[SEMANTICA] Ciclo: generando GOTO y rellenando salto");
            }
            generador.generar_fin_ciclo()?;
            Ok(String::new())
        }

        // ==================== PROPAGACIÓN ESTRUCTURAL ====================

        ("<EXPRESIÓN>", 2) |
        ("<EXP>", 2) |
        ("<TÉRMINO>", 2) |
        ("<ESTATUTO>", 1) |
        ("<VARS_OPT>", _) |
        ("<FUNCS_LIST>", _) |
        ("<CUERPO>", _) |
        ("<ESTATUTO_LIST>", _) |
        ("<ARG_OPT>", _) |
        ("<ARG_LIST>", _) |
        ("<ARG_LIST_PRIMA>", _) |
        ("<TIPO_OPT>", _) |
        ("<EXPRESIÓN_OPT>", _) |
        ("<EXPRESIÓN_LIST>", _) |
        ("<EXPRESIÓN_LIST_PRIMA>", _) |
        ("<IMPRIME_LIST>", _) => {
            // Propagación: pasar primer atributo no vacío
            for attr in atributos {
                if !attr.is_empty() {
                    return Ok(attr.clone());
                }
            }
            Ok(String::new())
        }

        // <OBJ_IMPRIME> → letrero
        ("<OBJ_IMPRIME>", 1) if atributos.get(0).map(|s| s.starts_with('"')) == Some(true) => {
            let letrero = &atributos[0];
            if nivel_verbose >= 3 {
                println!("[SEMANTICA] Procesando letrero para impresión: {}", letrero);
            }
            generador.procesar_operando(letrero)?;
            Ok(letrero.clone())
        }

        // <OBJ_IMPRIME> → <EXPRESIÓN>
        ("<OBJ_IMPRIME>", 1) => {
            // Propagación para expresión
            Ok(atributos.first().unwrap_or(&String::new()).clone())
        }

        // ==================== DEFAULT ====================
        _ => {
            // Cualquier otra regla: simplemente propagar
            if nivel_verbose >= 3 {
                println!("[DEBUG] Regla sin acción específica: cabeza='{}', len={} (pattern: ({:?}, {}))", cabeza, cuerpo_len, cabeza, cuerpo_len);
            }
            Ok(String::new())
        }
    }
}
