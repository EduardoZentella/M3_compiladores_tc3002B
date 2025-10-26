// Usamos el modulo de gramatica desde la libreria
use compilador_rust::gramatica::{self, Simbolo, Produccion};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;

/// Representa una acción en la tabla ACTION
#[derive(Debug, Clone, PartialEq)]
enum Accion {
    Shift(usize),      // Desplazar al estado N
    Reduce(usize),     // Reducir por la producción N
    Accept,            // Aceptar
}

/// Tabla ACTION: (estado, terminal) -> Acción
type TablaAction = HashMap<(usize, String), Accion>;

/// Tabla GOTO: (estado, no-terminal) -> estado_destino
type TablaGoto = HashMap<(usize, String), usize>;

fn main() {
    println!("=== Generador de Analizador Sintáctico SLR ===\n");

    // Leemos el archivo de la gramatica
    let contenido = std::fs::read_to_string("gramatica.txt")
        .expect("No se pudo leer el archivo de la gramatica");

    // Parseamos la gramatica
    let gramatica = match gramatica::parser::parsear_gramatica(&contenido) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error al parsear la gramatica: {}", e);
            return;
        }
    };
    println!("✓ Gramática parseada correctamente!");
    println!("  - Total de producciones: {}", gramatica.producciones.len());
    println!("  - Símbolo inicial: {}", gramatica.regla0);
    println!();

    // Calculamos los conjuntos FIRST y FOLLOW
    println!("Calculando conjuntos FIRST y FOLLOW...");
    let first_sets = gramatica::first_follow::calcular_first_sets(&gramatica);
    let follow_sets = gramatica::first_follow::calcular_follow_sets(&gramatica, &first_sets);
    println!("✓ Conjuntos FIRST y FOLLOW calculados correctamente!");
    println!("  - Total de FIRST calculados: {}", first_sets.len());
    println!("  - Total de FOLLOW calculados: {}", follow_sets.len());
    println!();

    // Construimos el autómata LR(0)
    println!("Construyendo autómata LR(0)...");
    let (estados, transiciones) = gramatica::lr0::construir_automata(&gramatica);
    println!("✓ Autómata LR(0) construido correctamente!");
    println!("  - Total de estados: {}", estados.len());
    println!("  - Total de transiciones: {}", transiciones.len());
    println!();

    // Generar las tablas SLR (ACTION/GOTO)
    println!("Generando tablas SLR...");
    let (tabla_action, tabla_goto) = generar_tablas_slr(&estados, &transiciones, &gramatica, &follow_sets);
    println!("✓ Tablas SLR generadas correctamente!");
    println!("  - Entradas en ACTION: {}", tabla_action.len());
    println!("  - Entradas en GOTO: {}", tabla_goto.len());
    println!();

    // Escribir las tablas a archivo
    println!("Escribiendo archivo tabla_slr.rs...");
    match escribir_archivo_tablas(&tabla_action, &tabla_goto, &gramatica.producciones) {
        Ok(_) => {
            println!("✓ Archivo generado exitosamente en src/sintactico/tabla_slr.rs");
        }
        Err(e) => {
            eprintln!("Error al escribir el archivo: {}", e);
            return;
        }
    }

    println!("\n=== Generación completada exitosamente ===");
}

/// Extrae el string de un símbolo (terminal o no-terminal)
fn extraer_string(simbolo: &Simbolo) -> String {
    match simbolo {
        Simbolo::Terminal(s) | Simbolo::NoTerminal(s) => s.clone(),
    }
}

/// Genera las tablas ACTION y GOTO para el analizador SLR
fn generar_tablas_slr(
    estados: &[gramatica::lr0::EstadoLR0],
    transiciones: &HashMap<(usize, Simbolo), usize>,
    gramatica: &gramatica::Gramatica,
    follow_sets: &HashMap<String, HashSet<String>>,
) -> (TablaAction, TablaGoto) {
    let mut tabla_action = TablaAction::new();
    let mut tabla_goto = TablaGoto::new();

    // Para cada estado
    for (idx_estado, estado) in estados.iter().enumerate() {
        // Para cada ítem en el estado
        for item in estado {
            let produccion = &gramatica.producciones[item.regla_id];

            // Verificar si el punto está al final (ítem completo)
            if item.punto >= produccion.cuerpo.len() {
                // Ítem [A → α•]

                // Caso especial: Regla aumentada [S' → S•]
                if item.regla_id == 0 {
                    tabla_action.insert((idx_estado, "$".to_string()), Accion::Accept);
                } else {
                    // Para cada terminal en FOLLOW(A), ACTION[i, t] = Reduce(regla)
                    let cabeza = extraer_string(&produccion.cabeza);
                    if let Some(follow) = follow_sets.get(&cabeza) {
                        for terminal in follow {
                            tabla_action.insert(
                                (idx_estado, terminal.clone()),
                                Accion::Reduce(item.regla_id),
                            );
                        }
                    }
                }
            } else {
                // El punto no está al final, hay un símbolo después
                let simbolo_siguiente = &produccion.cuerpo[item.punto];

                match simbolo_siguiente {
                    Simbolo::Terminal(t) => {
                        // Si hay una transición con este terminal
                        if let Some(&estado_destino) = transiciones.get(&(idx_estado, simbolo_siguiente.clone())) {
                            tabla_action.insert(
                                (idx_estado, t.clone()),
                                Accion::Shift(estado_destino),
                            );
                        }
                    }
                    Simbolo::NoTerminal(nt) => {
                        // Si hay una transición con este no-terminal
                        if let Some(&estado_destino) = transiciones.get(&(idx_estado, simbolo_siguiente.clone())) {
                            tabla_goto.insert(
                                (idx_estado, nt.clone()),
                                estado_destino,
                            );
                        }
                    }
                }
            }
        }
    }

    (tabla_action, tabla_goto)
}

/// Escribe las tablas SLR en un archivo Rust
fn escribir_archivo_tablas(
    action: &TablaAction,
    goto: &TablaGoto,
    producciones: &[Produccion],
) -> std::io::Result<()> {
    let mut archivo = File::create("src/sintactico/tabla_slr.rs")?;

    // Encabezado
    writeln!(archivo, "// ==========================================")?;
    writeln!(archivo, "// ARCHIVO GENERADO AUTOMÁTICAMENTE")?;
    writeln!(archivo, "// Creado por 'generador_slr'")?;
    writeln!(archivo, "// NO EDITAR MANUALMENTE")?;
    writeln!(archivo, "// ==========================================\n")?;

    writeln!(archivo, "use lazy_static::lazy_static;")?;
    writeln!(archivo, "use std::collections::HashMap;\n")?;

    // Definir el enum Accion
    writeln!(archivo, "/// Acción en la tabla ACTION")?;
    writeln!(archivo, "#[derive(Debug, Clone, PartialEq)]")?;
    writeln!(archivo, "pub enum Accion {{")?;
    writeln!(archivo, "    Shift(usize),   // Desplazar al estado N")?;
    writeln!(archivo, "    Reduce(usize),  // Reducir por la producción N")?;
    writeln!(archivo, "    Accept,         // Aceptar")?;
    writeln!(archivo, "}}\n")?;

    // Definir la estructura Regla
    writeln!(archivo, "/// Representa una regla de producción")?;
    writeln!(archivo, "#[derive(Debug, Clone)]")?;
    writeln!(archivo, "pub struct Regla {{")?;
    writeln!(archivo, "    #[allow(dead_code)]")?;
    writeln!(archivo, "    pub id: usize,")?;
    writeln!(archivo, "    pub cabeza: String,")?;
    writeln!(archivo, "    pub longitud_cuerpo: usize,")?;
    writeln!(archivo, "}}\n")?;

    // Inicializar las tablas con lazy_static
    writeln!(archivo, "lazy_static! {{")?;

    // Escribir TABLA_ACTION
    writeln!(archivo, "    /// Tabla ACTION: (estado, terminal) -> Acción")?;
    writeln!(archivo, "    pub static ref TABLA_ACTION: HashMap<(usize, String), Accion> = {{")?;
    writeln!(archivo, "        let mut m = HashMap::new();")?;
    for ((estado, terminal), accion) in action {
        let accion_str = match accion {
            Accion::Shift(n) => format!("Accion::Shift({})", n),
            Accion::Reduce(n) => format!("Accion::Reduce({})", n),
            Accion::Accept => "Accion::Accept".to_string(),
        };
        writeln!(
            archivo,
            "        m.insert(({}, \"{}\".to_string()), {});",
            estado, terminal, accion_str
        )?;
    }
    writeln!(archivo, "        m")?;
    writeln!(archivo, "    }};\n")?;

    // Escribir TABLA_GOTO
    writeln!(archivo, "    /// Tabla GOTO: (estado, no-terminal) -> estado_destino")?;
    writeln!(archivo, "    pub static ref TABLA_GOTO: HashMap<(usize, String), usize> = {{")?;
    writeln!(archivo, "        let mut m = HashMap::new();")?;
    for ((estado, no_terminal), destino) in goto {
        writeln!(
            archivo,
            "        m.insert(({}, \"{}\".to_string()), {});",
            estado, no_terminal, destino
        )?;
    }
    writeln!(archivo, "        m")?;
    writeln!(archivo, "    }};\n")?;

    // Escribir PRODUCCIONES
    writeln!(archivo, "    /// Lista de producciones de la gramática")?;
    writeln!(archivo, "    pub static ref PRODUCCIONES: Vec<Regla> = vec![")?;
    for prod in producciones {
        let cabeza = extraer_string(&prod.cabeza);
        let longitud = prod.cuerpo.len();
        writeln!(
            archivo,
            "        Regla {{ id: {}, cabeza: \"{}\".to_string(), longitud_cuerpo: {} }},",
            prod.numero, cabeza, longitud
        )?;
    }
    writeln!(archivo, "    ];")?;

    writeln!(archivo, "}}")?;

    Ok(())
}