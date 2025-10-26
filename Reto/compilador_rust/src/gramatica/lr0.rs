use crate::gramatica::{Gramatica, Produccion, Simbolo};
use std::collections::{HashMap, HashSet};

/// Representa un ítem LR(0): una producción con un punto en alguna posición
/// Ejemplo: <E> → <T> • <E'> es el ítem con regla_id=1 y punto=1
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemLR0 {
    pub regla_id: usize,  // ID de la producción
    pub punto: usize,      // Posición del punto (0 = inicio, len = final)
}

impl ItemLR0 {
    /// Obtiene el símbolo que está después del punto (si existe)
    pub fn simbolo_siguiente<'a>(&self, produccion: &'a Produccion) -> Option<&'a Simbolo> {
        produccion.cuerpo.get(self.punto)
    }

    /// Verifica si el punto está al final (ítem completo)
    pub fn es_completo(&self, produccion: &Produccion) -> bool {
        self.punto >= produccion.cuerpo.len()
    }
}

/// Un estado LR(0) es un conjunto de ítems
pub type EstadoLR0 = HashSet<ItemLR0>;

/// Calcula la clausura de un conjunto de ítems LR(0)
///
/// Algoritmo:
/// 1. Comenzar con el conjunto inicial de ítems
/// 2. Para cada ítem [A → α•Bβ] donde B es no-terminal:
///    - Agregar todos los ítems [B → •γ] para cada producción B → γ
/// 3. Repetir hasta que no se agreguen más ítems
pub fn clausura(items: &EstadoLR0, gramatica: &Gramatica) -> EstadoLR0 {
    let mut clausura = items.clone();
    let mut agregados = true;

    while agregados {
        agregados = false;
        let items_actuales: Vec<ItemLR0> = clausura.iter().cloned().collect();

        for item in items_actuales {
            let produccion = &gramatica.producciones[item.regla_id];

            // Si el punto no está al final, ver qué símbolo sigue
            if let Some(simbolo) = item.simbolo_siguiente(produccion) {
                // Si es un no-terminal, agregar todas sus producciones con punto al inicio
                if let Simbolo::NoTerminal(nt) = simbolo {
                    for prod in &gramatica.producciones {
                        if let Simbolo::NoTerminal(cabeza_nt) = &prod.cabeza {
                            if cabeza_nt == nt {
                                let nuevo_item = ItemLR0 {
                                    regla_id: prod.numero,
                                    punto: 0,
                                };
                                if clausura.insert(nuevo_item) {
                                    agregados = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    clausura
}

/// Calcula el estado resultante de hacer la transición desde un estado con un símbolo
///
/// Algoritmo:
/// 1. Para cada ítem [A → α•Xβ] en el estado donde X es el símbolo dado:
///    - Agregar [A → αX•β] al nuevo conjunto
/// 2. Calcular la clausura del nuevo conjunto
pub fn ir_a(estado: &EstadoLR0, simbolo: &Simbolo, gramatica: &Gramatica) -> EstadoLR0 {
    let mut nuevo_estado = HashSet::new();

    for item in estado {
        let produccion = &gramatica.producciones[item.regla_id];

        // Verificar si el símbolo después del punto coincide con el dado
        if let Some(sig_simbolo) = item.simbolo_siguiente(produccion) {
            if sig_simbolo == simbolo {
                // Mover el punto una posición adelante
                nuevo_estado.insert(ItemLR0 {
                    regla_id: item.regla_id,
                    punto: item.punto + 1,
                });
            }
        }
    }

    // Si el nuevo estado no está vacío, calcular su clausura
    if !nuevo_estado.is_empty() {
        clausura(&nuevo_estado, gramatica)
    } else {
        nuevo_estado
    }
}

/// Construye el autómata LR(0) completo
///
/// Retorna:
/// - Vec<EstadoLR0>: Lista de todos los estados
/// - HashMap<(usize, Simbolo), usize>: Tabla de transiciones (estado, símbolo) -> estado_destino
pub fn construir_automata(gramatica: &Gramatica) -> (Vec<EstadoLR0>, HashMap<(usize, Simbolo), usize>) {
    let mut estados = Vec::new();
    let mut transiciones = HashMap::new();
    let mut estados_pendientes = Vec::new();

    // Crear el estado inicial (I0) con la regla aumentada
    let item_inicial = ItemLR0 {
        regla_id: 0,  // La regla 0 es la regla aumentada
        punto: 0,
    };
    let estado_inicial = clausura(&HashSet::from([item_inicial]), gramatica);

    estados.push(estado_inicial.clone());
    estados_pendientes.push(0); // Índice del estado inicial

    // Recolectar todos los símbolos de la gramática
    let simbolos = obtener_todos_simbolos(gramatica);

    // Procesar estados hasta que no haya más pendientes
    while let Some(idx_estado) = estados_pendientes.pop() {
        let estado_actual = &estados[idx_estado].clone();

        // Para cada símbolo, calcular ir_a
        for simbolo in &simbolos {
            let nuevo_estado = ir_a(estado_actual, simbolo, gramatica);

            // Si el estado no está vacío
            if !nuevo_estado.is_empty() {
                // Buscar si este estado ya existe
                if let Some(idx_existente) = encontrar_estado(&estados, &nuevo_estado) {
                    // Ya existe, solo agregar la transición
                    transiciones.insert((idx_estado, simbolo.clone()), idx_existente);
                } else {
                    // Es un estado nuevo
                    let idx_nuevo = estados.len();
                    estados.push(nuevo_estado);
                    estados_pendientes.push(idx_nuevo);
                    transiciones.insert((idx_estado, simbolo.clone()), idx_nuevo);
                }
            }
        }
    }

    (estados, transiciones)
}

/// Obtiene todos los símbolos (terminales y no-terminales) de la gramática
fn obtener_todos_simbolos(gramatica: &Gramatica) -> Vec<Simbolo> {
    let mut simbolos = Vec::new();

    simbolos.extend(gramatica.simbolos_terminales.iter().cloned());
    simbolos.extend(gramatica.simbolos_no_terminales.iter().cloned());

    simbolos
}

/// Busca un estado en la lista de estados y retorna su índice si existe
fn encontrar_estado(estados: &[EstadoLR0], estado_buscado: &EstadoLR0) -> Option<usize> {
    estados.iter().position(|e| e == estado_buscado)
}

/// Función auxiliar para imprimir un ítem de forma legible
pub fn item_to_string(item: &ItemLR0, gramatica: &Gramatica) -> String {
    let prod = &gramatica.producciones[item.regla_id];
    let cabeza = match &prod.cabeza {
        Simbolo::Terminal(s) | Simbolo::NoTerminal(s) => s.clone(),
    };

    let mut cuerpo_str = String::new();
    for (i, simbolo) in prod.cuerpo.iter().enumerate() {
        if i == item.punto {
            cuerpo_str.push_str("• ");
        }
        let sym_str = match simbolo {
            Simbolo::Terminal(s) | Simbolo::NoTerminal(s) => s.clone(),
        };
        cuerpo_str.push_str(&sym_str);
        cuerpo_str.push(' ');
    }

    // Si el punto está al final
    if item.punto >= prod.cuerpo.len() {
        cuerpo_str.push_str("•");
    }

    format!("{} → {}", cabeza, cuerpo_str.trim())
}
