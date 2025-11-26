//! # Memoria Virtual
//!
//! Sistema de direcciones virtuales para la máquina virtual.
//! Organiza la memoria en segmentos (global, local, temporal, constante)
//! y asigna direcciones únicas a cada variable según su tipo y alcance.
//!
//! ## Esquema de Direcciones Virtuales
//!
//! ```text
//! SEGMENTO GLOBAL (1000-6999)
//!   ├── Enteros:   1000-2999 (2000 espacios)
//!   ├── Flotantes: 3000-4999 (2000 espacios)
//!   └── Chars:     5000-6999 (2000 espacios)
//!
//! SEGMENTO LOCAL (7000-12999)
//!   ├── Enteros:   7000-8999 (2000 espacios)
//!   ├── Flotantes: 9000-10999 (2000 espacios)
//!   └── Chars:     11000-12999 (2000 espacios)
//!
//! SEGMENTO TEMPORAL (13000-18999)
//!   ├── Enteros:   13000-14999 (2000 espacios)
//!   ├── Flotantes: 15000-16999 (2000 espacios)
//!   └── Chars:     17000-18999 (2000 espacios)
//!
//! SEGMENTO CONSTANTE (19000-24999)
//!   ├── Enteros:   19000-20999 (2000 espacios)
//!   ├── Flotantes: 21000-22999 (2000 espacios)
//!   └── Chars:     23000-24999 (2000 espacios)
//! ```

use crate::semantico::TipoDato;
use std::collections::HashMap;

// Límites de direcciones para cada segmento y tipo
const GLOBAL_ENTERO_INICIO: usize = 1000;
const GLOBAL_ENTERO_FIN: usize = 2999;
const GLOBAL_FLOTANTE_INICIO: usize = 3000;
const GLOBAL_FLOTANTE_FIN: usize = 4999;
const GLOBAL_CHAR_INICIO: usize = 5000;
const GLOBAL_CHAR_FIN: usize = 6999;

const LOCAL_ENTERO_INICIO: usize = 7000;
const LOCAL_ENTERO_FIN: usize = 8999;
const LOCAL_FLOTANTE_INICIO: usize = 9000;
const LOCAL_FLOTANTE_FIN: usize = 10999;
const LOCAL_CHAR_INICIO: usize = 11000;
const LOCAL_CHAR_FIN: usize = 12999;

const TEMPORAL_ENTERO_INICIO: usize = 13000;
const TEMPORAL_ENTERO_FIN: usize = 14999;
const TEMPORAL_FLOTANTE_INICIO: usize = 15000;
const TEMPORAL_FLOTANTE_FIN: usize = 16999;
const TEMPORAL_CHAR_INICIO: usize = 17000;
const TEMPORAL_CHAR_FIN: usize = 18999;

const CONSTANTE_ENTERO_INICIO: usize = 19000;
const CONSTANTE_ENTERO_FIN: usize = 20999;
const CONSTANTE_FLOTANTE_INICIO: usize = 21000;
const CONSTANTE_FLOTANTE_FIN: usize = 22999;
const CONSTANTE_CHAR_INICIO: usize = 23000;
const CONSTANTE_CHAR_FIN: usize = 24999;

/// Tipo de segmento de memoria
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TipoSegmento {
    Global,
    Local,
    Temporal,
    Constante,
}

/// Gestor de memoria virtual
pub struct MemoriaVirtual {
    // Contadores de direcciones por segmento y tipo
    global_entero: usize,
    global_flotante: usize,
    global_char: usize,

    local_entero: usize,
    local_flotante: usize,
    local_char: usize,

    temporal_entero: usize,
    temporal_flotante: usize,
    temporal_char: usize,

    constante_entero: usize,
    constante_flotante: usize,
    constante_char: usize,

    // Tablas de constantes (valor -> dirección)
    tabla_constantes_entero: HashMap<i32, usize>,
    tabla_constantes_flotante: HashMap<String, usize>, // String para evitar problemas de precisión
    tabla_constantes_char: HashMap<char, usize>,
}

impl MemoriaVirtual {
    /// Crea un nuevo gestor de memoria virtual
    pub fn new() -> Self {
        MemoriaVirtual {
            global_entero: GLOBAL_ENTERO_INICIO,
            global_flotante: GLOBAL_FLOTANTE_INICIO,
            global_char: GLOBAL_CHAR_INICIO,

            local_entero: LOCAL_ENTERO_INICIO,
            local_flotante: LOCAL_FLOTANTE_INICIO,
            local_char: LOCAL_CHAR_INICIO,

            temporal_entero: TEMPORAL_ENTERO_INICIO,
            temporal_flotante: TEMPORAL_FLOTANTE_INICIO,
            temporal_char: TEMPORAL_CHAR_INICIO,

            constante_entero: CONSTANTE_ENTERO_INICIO,
            constante_flotante: CONSTANTE_FLOTANTE_INICIO,
            constante_char: CONSTANTE_CHAR_INICIO,

            tabla_constantes_entero: HashMap::new(),
            tabla_constantes_flotante: HashMap::new(),
            tabla_constantes_char: HashMap::new(),
        }
    }

    /// Asigna una dirección virtual para una variable
    pub fn asignar_variable(&mut self, tipo: TipoDato, segmento: TipoSegmento) -> Result<usize, String> {
        match (segmento, tipo) {
            // Segmento Global
            (TipoSegmento::Global, TipoDato::Entero) => {
                if self.global_entero > GLOBAL_ENTERO_FIN {
                    return Err("Error: Desbordamiento de memoria global para enteros".to_string());
                }
                let dir = self.global_entero;
                self.global_entero += 1;
                Ok(dir)
            }
            (TipoSegmento::Global, TipoDato::Flotante) => {
                if self.global_flotante > GLOBAL_FLOTANTE_FIN {
                    return Err("Error: Desbordamiento de memoria global para flotantes".to_string());
                }
                let dir = self.global_flotante;
                self.global_flotante += 1;
                Ok(dir)
            }
            (TipoSegmento::Global, TipoDato::Char) => {
                if self.global_char > GLOBAL_CHAR_FIN {
                    return Err("Error: Desbordamiento de memoria global para chars".to_string());
                }
                let dir = self.global_char;
                self.global_char += 1;
                Ok(dir)
            }
            (TipoSegmento::Global, TipoDato::Letrero) => {
                if self.global_char > GLOBAL_CHAR_FIN {
                    return Err("Error: Desbordamiento de memoria global para letreros".to_string());
                }
                let dir = self.global_char;
                self.global_char += 1;
                Ok(dir)
            }

            // Segmento Local
            (TipoSegmento::Local, TipoDato::Entero) => {
                if self.local_entero > LOCAL_ENTERO_FIN {
                    return Err("Error: Desbordamiento de memoria local para enteros".to_string());
                }
                let dir = self.local_entero;
                self.local_entero += 1;
                Ok(dir)
            }
            (TipoSegmento::Local, TipoDato::Flotante) => {
                if self.local_flotante > LOCAL_FLOTANTE_FIN {
                    return Err("Error: Desbordamiento de memoria local para flotantes".to_string());
                }
                let dir = self.local_flotante;
                self.local_flotante += 1;
                Ok(dir)
            }
            (TipoSegmento::Local, TipoDato::Char) => {
                if self.local_char > LOCAL_CHAR_FIN {
                    return Err("Error: Desbordamiento de memoria local para chars".to_string());
                }
                let dir = self.local_char;
                self.local_char += 1;
                Ok(dir)
            }
            (TipoSegmento::Local, TipoDato::Letrero) => {
                if self.local_char > LOCAL_CHAR_FIN {
                    return Err("Error: Desbordamiento de memoria local para letreros".to_string());
                }
                let dir = self.local_char;
                self.local_char += 1;
                Ok(dir)
            }

            // Segmento Temporal
            (TipoSegmento::Temporal, TipoDato::Entero) => {
                if self.temporal_entero > TEMPORAL_ENTERO_FIN {
                    return Err("Error: Desbordamiento de memoria temporal para enteros".to_string());
                }
                let dir = self.temporal_entero;
                self.temporal_entero += 1;
                Ok(dir)
            }
            (TipoSegmento::Temporal, TipoDato::Flotante) => {
                if self.temporal_flotante > TEMPORAL_FLOTANTE_FIN {
                    return Err("Error: Desbordamiento de memoria temporal para flotantes".to_string());
                }
                let dir = self.temporal_flotante;
                self.temporal_flotante += 1;
                Ok(dir)
            }
            (TipoSegmento::Temporal, TipoDato::Char) => {
                if self.temporal_char > TEMPORAL_CHAR_FIN {
                    return Err("Error: Desbordamiento de memoria temporal para chars".to_string());
                }
                let dir = self.temporal_char;
                self.temporal_char += 1;
                Ok(dir)
            }
            (TipoSegmento::Temporal, TipoDato::Letrero) => {
                if self.temporal_char > TEMPORAL_CHAR_FIN {
                    return Err("Error: Desbordamiento de memoria temporal para letreros".to_string());
                }
                let dir = self.temporal_char;
                self.temporal_char += 1;
                Ok(dir)
            }

            // Constantes - Estas no se asignan aquí, ver asignar_constante_*
            (TipoSegmento::Constante, _) => {
                Err("Error: Use asignar_constante_* para constantes".to_string())
            }
        }
    }

    /// Asigna una dirección para una constante entera (reutiliza si ya existe)
    pub fn asignar_constante_entera(&mut self, valor: i32) -> Result<usize, String> {
        // Si la constante ya existe, retornar su dirección
        if let Some(&dir) = self.tabla_constantes_entero.get(&valor) {
            return Ok(dir);
        }

        // Si no existe, asignar nueva dirección
        if self.constante_entero > CONSTANTE_ENTERO_FIN {
            return Err("Error: Desbordamiento de memoria para constantes enteras".to_string());
        }

        let dir = self.constante_entero;
        self.constante_entero += 1;
        self.tabla_constantes_entero.insert(valor, dir);
        Ok(dir)
    }

    /// Asigna una dirección para una constante flotante (reutiliza si ya existe)
    pub fn asignar_constante_flotante(&mut self, valor: f64) -> Result<usize, String> {
        let key = valor.to_string(); // Usar string como clave para evitar problemas de precisión

        if let Some(&dir) = self.tabla_constantes_flotante.get(&key) {
            return Ok(dir);
        }

        if self.constante_flotante > CONSTANTE_FLOTANTE_FIN {
            return Err("Error: Desbordamiento de memoria para constantes flotantes".to_string());
        }

        let dir = self.constante_flotante;
        self.constante_flotante += 1;
        self.tabla_constantes_flotante.insert(key, dir);
        Ok(dir)
    }

    /// Asigna una dirección para una constante char (reutiliza si ya existe)
    pub fn asignar_constante_char(&mut self, valor: char) -> Result<usize, String> {
        if let Some(&dir) = self.tabla_constantes_char.get(&valor) {
            return Ok(dir);
        }

        if self.constante_char > CONSTANTE_CHAR_FIN {
            return Err("Error: Desbordamiento de memoria para constantes char".to_string());
        }

        let dir = self.constante_char;
        self.constante_char += 1;
        self.tabla_constantes_char.insert(valor, dir);
        Ok(dir)
    }

    /// Reinicia los contadores del segmento local (al cambiar de función)
    pub fn reiniciar_local(&mut self) {
        self.local_entero = LOCAL_ENTERO_INICIO;
        self.local_flotante = LOCAL_FLOTANTE_INICIO;
        self.local_char = LOCAL_CHAR_INICIO;
    }

    /// Reinicia los contadores del segmento temporal
    pub fn reiniciar_temporal(&mut self) {
        self.temporal_entero = TEMPORAL_ENTERO_INICIO;
        self.temporal_flotante = TEMPORAL_FLOTANTE_INICIO;
        self.temporal_char = TEMPORAL_CHAR_INICIO;
    }

    /// Obtiene las tablas de constantes (para serializar al archivo objeto)
    pub fn obtener_tablas_constantes(&self) -> (HashMap<i32, usize>, HashMap<String, usize>, HashMap<char, usize>) {
        (
            self.tabla_constantes_entero.clone(),
            self.tabla_constantes_flotante.clone(),
            self.tabla_constantes_char.clone(),
        )
    }

    /// Determina el tipo de dato a partir de una dirección virtual
    pub fn obtener_tipo_desde_direccion(dir: usize) -> Result<TipoDato, String> {
        match dir {
            GLOBAL_ENTERO_INICIO..=GLOBAL_ENTERO_FIN => Ok(TipoDato::Entero),
            GLOBAL_FLOTANTE_INICIO..=GLOBAL_FLOTANTE_FIN => Ok(TipoDato::Flotante),
            GLOBAL_CHAR_INICIO..=GLOBAL_CHAR_FIN => Ok(TipoDato::Char),

            LOCAL_ENTERO_INICIO..=LOCAL_ENTERO_FIN => Ok(TipoDato::Entero),
            LOCAL_FLOTANTE_INICIO..=LOCAL_FLOTANTE_FIN => Ok(TipoDato::Flotante),
            LOCAL_CHAR_INICIO..=LOCAL_CHAR_FIN => Ok(TipoDato::Char),

            TEMPORAL_ENTERO_INICIO..=TEMPORAL_ENTERO_FIN => Ok(TipoDato::Entero),
            TEMPORAL_FLOTANTE_INICIO..=TEMPORAL_FLOTANTE_FIN => Ok(TipoDato::Flotante),
            TEMPORAL_CHAR_INICIO..=TEMPORAL_CHAR_FIN => Ok(TipoDato::Char),

            CONSTANTE_ENTERO_INICIO..=CONSTANTE_ENTERO_FIN => Ok(TipoDato::Entero),
            CONSTANTE_FLOTANTE_INICIO..=CONSTANTE_FLOTANTE_FIN => Ok(TipoDato::Flotante),
            CONSTANTE_CHAR_INICIO..=CONSTANTE_CHAR_FIN => Ok(TipoDato::Char),

            _ => Err(format!("Error: Dirección virtual {} fuera de rango", dir)),
        }
    }

    /// Determina el segmento a partir de una dirección virtual
    pub fn obtener_segmento_desde_direccion(dir: usize) -> Result<TipoSegmento, String> {
        match dir {
            GLOBAL_ENTERO_INICIO..=GLOBAL_CHAR_FIN => Ok(TipoSegmento::Global),
            LOCAL_ENTERO_INICIO..=LOCAL_CHAR_FIN => Ok(TipoSegmento::Local),
            TEMPORAL_ENTERO_INICIO..=TEMPORAL_CHAR_FIN => Ok(TipoSegmento::Temporal),
            CONSTANTE_ENTERO_INICIO..=CONSTANTE_CHAR_FIN => Ok(TipoSegmento::Constante),
            _ => Err(format!("Error: Dirección virtual {} fuera de rango", dir)),
        }
    }
}

impl Default for MemoriaVirtual {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asignar_variables_globales() {
        let mut mem = MemoriaVirtual::new();

        let dir1 = mem.asignar_variable(TipoDato::Entero, TipoSegmento::Global).unwrap();
        assert_eq!(dir1, 1000);

        let dir2 = mem.asignar_variable(TipoDato::Entero, TipoSegmento::Global).unwrap();
        assert_eq!(dir2, 1001);

        let dir3 = mem.asignar_variable(TipoDato::Flotante, TipoSegmento::Global).unwrap();
        assert_eq!(dir3, 3000);
    }

    #[test]
    fn test_asignar_constantes_reutiliza() {
        let mut mem = MemoriaVirtual::new();

        let dir1 = mem.asignar_constante_entera(42).unwrap();
        let dir2 = mem.asignar_constante_entera(42).unwrap();

        assert_eq!(dir1, dir2); // Debe reutilizar la misma dirección
    }

    #[test]
    fn test_reiniciar_local() {
        let mut mem = MemoriaVirtual::new();

        mem.asignar_variable(TipoDato::Entero, TipoSegmento::Local).unwrap();
        mem.reiniciar_local();

        let dir = mem.asignar_variable(TipoDato::Entero, TipoSegmento::Local).unwrap();
        assert_eq!(dir, LOCAL_ENTERO_INICIO);
    }

    #[test]
    fn test_obtener_tipo_desde_direccion() {
        assert_eq!(MemoriaVirtual::obtener_tipo_desde_direccion(1000).unwrap(), TipoDato::Entero);
        assert_eq!(MemoriaVirtual::obtener_tipo_desde_direccion(3000).unwrap(), TipoDato::Flotante);
        assert_eq!(MemoriaVirtual::obtener_tipo_desde_direccion(13500).unwrap(), TipoDato::Entero);
    }

    #[test]
    fn test_obtener_segmento_desde_direccion() {
        assert_eq!(MemoriaVirtual::obtener_segmento_desde_direccion(1500).unwrap(), TipoSegmento::Global);
        assert_eq!(MemoriaVirtual::obtener_segmento_desde_direccion(7500).unwrap(), TipoSegmento::Local);
        assert_eq!(MemoriaVirtual::obtener_segmento_desde_direccion(13500).unwrap(), TipoSegmento::Temporal);
        assert_eq!(MemoriaVirtual::obtener_segmento_desde_direccion(19500).unwrap(), TipoSegmento::Constante);
    }
}
