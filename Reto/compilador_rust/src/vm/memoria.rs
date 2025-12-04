//! # Memoria
//!
//! Gestión de memoria segmentada para la Máquina Virtual.
//! Maneja segmentos GLOBAL, LOCAL, TEMPORAL y CONSTANTE.

use std::collections::HashMap;

/// Rangos de direcciones virtuales
/// Deben coincidir con los rangos en memoria_virtual.rs del generador
pub const GLOBAL_INICIO: usize = 1000;
pub const GLOBAL_FIN: usize = 6999;
pub const LOCAL_INICIO: usize = 7000;
pub const LOCAL_FIN: usize = 12999;
pub const TEMPORAL_INICIO: usize = 13000;
pub const TEMPORAL_FIN: usize = 18999;
pub const CONSTANTE_INICIO: usize = 19000;
pub const CONSTANTE_FIN: usize = 24999;

/// Valor en la VM (entero o flotante)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Valor {
    Entero(i32),
    Flotante(f64),
    Letrero(usize),
}

impl Valor {
    /// Convierte a entero (con cast si es flotante)
    pub fn a_entero(&self) -> i32 {
        match self {
            Valor::Entero(v) => *v,
            Valor::Flotante(v) => *v as i32,
            Valor::Letrero(_) => 0, // Placeholder
        }
    }

    /// Convierte a flotante (con cast si es entero)
    pub fn a_flotante(&self) -> f64 {
        match self {
            Valor::Entero(v) => *v as f64,
            Valor::Flotante(v) => *v,
            Valor::Letrero(_) => 0.0, // Placeholder
        }
    }

    /// Intenta convertir desde un string (para constantes)
    pub fn desde_string(s: &str) -> Result<Self, String> {
        // Intentar parsear como entero
        if let Ok(entero) = s.parse::<i32>() {
            return Ok(Valor::Entero(entero));
        }

        // Intentar parsear como flotante
        if let Ok(flotante) = s.parse::<f64>() {
            return Ok(Valor::Flotante(flotante));
        }

        Err(format!("No se puede convertir '{}' a valor numérico", s))
    }

    /// Aplica operación aritmética entre dos valores
    pub fn operar_aritmetica<F>(&self, otro: &Valor, op: F) -> Valor
    where
        F: Fn(f64, f64) -> f64,
    {
        let resultado = op(self.a_flotante(), otro.a_flotante());

        // Si ambos son enteros, devolver entero
        if matches!(self, Valor::Entero(_)) && matches!(otro, Valor::Entero(_)) {
            Valor::Entero(resultado as i32)
        } else {
            Valor::Flotante(resultado)
        }
    }

    /// Aplica operación relacional entre dos valores
    pub fn operar_relacional<F>(&self, otro: &Valor, op: F) -> Valor
    where
        F: Fn(f64, f64) -> bool,
    {
        let resultado = op(self.a_flotante(), otro.a_flotante());
        Valor::Entero(if resultado { 1 } else { 0 })
    }
}

/// Segmento de memoria: almacena valores separados por tipo
#[derive(Debug, Clone)]
pub struct SegmentoMemoria {
    enteros: HashMap<usize, i32>,
    flotantes: HashMap<usize, f64>,
}

impl SegmentoMemoria {
    pub fn new() -> Self {
        SegmentoMemoria {
            enteros: HashMap::new(),
            flotantes: HashMap::new(),
        }
    }

    pub fn leer_entero(&self, offset: usize) -> Result<i32, String> {
        self.enteros
            .get(&offset)
            .copied()
            .ok_or_else(|| format!("Error: Dirección {} no inicializada (entero)", offset))
    }

    pub fn leer_flotante(&self, offset: usize) -> Result<f64, String> {
        self.flotantes
            .get(&offset)
            .copied()
            .ok_or_else(|| format!("Error: Dirección {} no inicializada (flotante)", offset))
    }

    pub fn escribir_entero(&mut self, offset: usize, valor: i32) {
        self.enteros.insert(offset, valor);
    }

    pub fn escribir_flotante(&mut self, offset: usize, valor: f64) {
        self.flotantes.insert(offset, valor);
    }

    pub fn leer_valor(&self, offset: usize, es_flotante: bool) -> Result<Valor, String> {
        if es_flotante {
            Ok(Valor::Flotante(self.leer_flotante(offset)?))
        } else {
            Ok(Valor::Entero(self.leer_entero(offset)?))
        }
    }

    pub fn escribir_valor(&mut self, offset: usize, valor: Valor) {
        match valor {
            Valor::Entero(v) => self.escribir_entero(offset, v),
            Valor::Flotante(v) => self.escribir_flotante(offset, v),
            Valor::Letrero(_) => {}, // Los letreros no se escriben en memoria, solo se imprimen
        }
    }
}

/// Marco de Memoria (Stack Frame) - contexto de ejecución de una función
#[derive(Debug, Clone)]
pub struct MarcoMemoria {
    /// Nombre de la función (para debug)
    pub nombre_funcion: String,

    /// Segmento de memoria local de esta función
    pub memoria_local: SegmentoMemoria,

    /// Segmento de memoria temporal de esta función
    pub memoria_temporal: SegmentoMemoria,

    /// Instruction Pointer de retorno
    pub ip_retorno: usize,

    /// Valor de retorno de la función (si tiene)
    pub valor_retorno: Option<Valor>,

    /// Destino donde copiar el valor de retorno (dirección de memoria)
    pub destino_retorno: Option<usize>,

    /// Parámetros recibidos (para debug)
    pub parametros: Vec<Valor>,
}

impl MarcoMemoria {
    /// Crea un nuevo marco de memoria para una función
    pub fn new(nombre_funcion: String, ip_retorno: usize) -> Self {
        MarcoMemoria {
            nombre_funcion,
            memoria_local: SegmentoMemoria::new(),
            memoria_temporal: SegmentoMemoria::new(),
            ip_retorno,
            valor_retorno: None,
            destino_retorno: None,
            parametros: Vec::new(),
        }
    }

    /// Agrega un parámetro al marco
    pub fn agregar_parametro(&mut self, valor: Valor) {
        self.parametros.push(valor);
    }
}

/// Traduce una dirección virtual a un segmento y offset
pub fn traducir_direccion(dir: usize) -> Result<(TipoSegmento, usize), String> {
    if dir >= GLOBAL_INICIO && dir <= GLOBAL_FIN {
        Ok((TipoSegmento::Global, dir - GLOBAL_INICIO))
    } else if dir >= LOCAL_INICIO && dir <= LOCAL_FIN {
        Ok((TipoSegmento::Local, dir - LOCAL_INICIO))
    } else if dir >= TEMPORAL_INICIO && dir <= TEMPORAL_FIN {
        Ok((TipoSegmento::Temporal, dir - TEMPORAL_INICIO))
    } else if dir >= CONSTANTE_INICIO && dir <= CONSTANTE_FIN {
        Ok((TipoSegmento::Constante, dir - CONSTANTE_INICIO))
    } else {
        Err(format!("Dirección virtual {} fuera de rango", dir))
    }
}

/// Tipo de segmento de memoria
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TipoSegmento {
    Global,
    Local,
    Temporal,
    Constante,
}
