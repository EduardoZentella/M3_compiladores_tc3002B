use crate::gramatica::{Gramatica, Simbolo, Produccion};

/// Parsea el contenido de un archivo de gramática y retorna la gramática aumentada
pub fn parsear_gramatica(contenido: &str) -> Result<Gramatica, String> {
    let mut gramatica = Gramatica::default();
    let mut primera_cabeza: Option<String> = None;
    let mut producciones = Vec::new();

    // Procesar cada línea del archivo
    for linea in contenido.lines() {
        let linea = linea.trim();

        // Ignorar líneas vacías o comentarios
        if linea.is_empty() || linea.starts_with('#') {
            continue;
        }

        // Dividir en cabeza → cuerpo
        let partes: Vec<&str> = linea.split("→").collect();
        if partes.len() != 2 {
            return Err(format!("Línea sin '→': {}", linea));
        }

        let cabeza_str = partes[0].trim();
        let cuerpo_str = partes[1].trim();

        // Validar que la cabeza sea un no-terminal
        if !cabeza_str.starts_with('<') || !cabeza_str.ends_with('>') {
            return Err(format!("La cabeza debe estar en <>: {}", cabeza_str));
        }

        // Crear símbolo de cabeza y registrarlo
        let cabeza = Simbolo::NoTerminal(cabeza_str.to_string());
        gramatica.simbolos_no_terminales.insert(cabeza.clone());

        // Guardar la primera cabeza para la regla aumentada
        if primera_cabeza.is_none() {
            primera_cabeza = Some(cabeza_str.to_string());
        }

        // Procesar el cuerpo de la producción
        let cuerpo = parsear_cuerpo(cuerpo_str, &mut gramatica)?;

        // Guardar la producción para agregar después
        producciones.push((cabeza, cuerpo));
    }

    // Verificar que haya al menos una producción
    let simbolo_inicial = primera_cabeza.ok_or("Gramática vacía")?;

    // Crear la regla aumentada (Regla 0)
    agregar_regla_aumentada(&mut gramatica, &simbolo_inicial);

    // Agregar el resto de las producciones
    for (i, (cabeza, cuerpo)) in producciones.into_iter().enumerate() {
        gramatica.producciones.push(Produccion {
            numero: i + 1, // +1 porque la regla 0 ya está agregada
            cabeza,
            cuerpo,
        });
    }

    Ok(gramatica)
}

/// Parsea el cuerpo de una producción y retorna un vector de símbolos
fn parsear_cuerpo(cuerpo_str: &str, gramatica: &mut Gramatica) -> Result<Vec<Simbolo>, String> {
    // Producción vacía (epsilon)
    if cuerpo_str == "ε" {
        return Ok(Vec::new());
    }

    let mut simbolos = Vec::new();

    for token in cuerpo_str.split_whitespace() {
        let simbolo = if token.starts_with('<') && token.ends_with('>') {
            // Es un no-terminal
            let nt = Simbolo::NoTerminal(token.to_string());
            gramatica.simbolos_no_terminales.insert(nt.clone());
            nt
        } else {
            // Es un terminal
            let t = Simbolo::Terminal(token.to_string());
            gramatica.simbolos_terminales.insert(t.clone());
            t
        };

        simbolos.push(simbolo);
    }

    Ok(simbolos)
}

/// Crea y agrega la regla aumentada (Regla 0) a la gramática
fn agregar_regla_aumentada(gramatica: &mut Gramatica, simbolo_inicial: &str) {
    // Crear el nuevo símbolo inicial: <Programa> -> <ProgramaPrime>
    let nombre_aumentado = format!("{}Prime", simbolo_inicial.trim_end_matches('>'));
    gramatica.regla0 = format!("{}>", nombre_aumentado);

    let cabeza_aumentada = Simbolo::NoTerminal(gramatica.regla0.clone());
    let cuerpo_aumentado = vec![Simbolo::NoTerminal(simbolo_inicial.to_string())];

    // Agregar la producción 0
    gramatica.producciones.push(Produccion {
        numero: 0,
        cabeza: cabeza_aumentada.clone(),
        cuerpo: cuerpo_aumentado,
    });

    // Registrar el nuevo no-terminal
    gramatica.simbolos_no_terminales.insert(cabeza_aumentada);
}
