//! # Gestor de Memoria Temporal
//!
//! Maneja la asignación y liberación de variables temporales.
//! Implementa el concepto de AVAIL (available temporaries).

use crate::semantico::TipoDato;
use std::collections::HashSet;

/// Gestor de memoria para variables temporales
pub struct GestorMemoria {
    /// Contador de temporales creados
    contador_temporales: usize,

    /// Temporales disponibles para reutilización (AVAIL)
    temporales_disponibles: HashSet<usize>,

    /// Tipos de los temporales (para validación semántica)
    tipos_temporales: Vec<TipoDato>,
}

impl GestorMemoria {
    /// Crea un nuevo gestor de memoria
    pub fn new() -> Self {
        GestorMemoria {
            contador_temporales: 0,
            temporales_disponibles: HashSet::new(),
            tipos_temporales: Vec::new(),
        }
    }

    /// Obtiene el siguiente temporal disponible (AVAIL.next())
    /// Si hay temporales en AVAIL, reutiliza uno; si no, crea uno nuevo
    pub fn siguiente_temporal(&mut self, tipo: TipoDato) -> usize {
        // Intentar reutilizar un temporal disponible
        if let Some(&temp) = self.temporales_disponibles.iter().next() {
            self.temporales_disponibles.remove(&temp);
            // Actualizar el tipo del temporal
            if temp < self.tipos_temporales.len() {
                self.tipos_temporales[temp] = tipo;
            }
            return temp;
        }

        // Si no hay disponibles, crear uno nuevo
        let nuevo_temporal = self.contador_temporales;
        self.contador_temporales += 1;
        self.tipos_temporales.push(tipo);
        nuevo_temporal
    }

    /// Libera un temporal para que pueda ser reutilizado
    /// Retorna el temporal a AVAIL
    pub fn liberar_temporal(&mut self, temporal: usize) {
        if temporal < self.contador_temporales {
            self.temporales_disponibles.insert(temporal);
        }
    }

    /// Obtiene el tipo de un temporal
    pub fn obtener_tipo_temporal(&self, temporal: usize) -> Option<TipoDato> {
        self.tipos_temporales.get(temporal).copied()
    }

    /// Reinicia el gestor de memoria
    pub fn reiniciar(&mut self) {
        self.contador_temporales = 0;
        self.temporales_disponibles.clear();
        self.tipos_temporales.clear();
    }

    /// Retorna el número total de temporales creados
    pub fn total_temporales(&self) -> usize {
        self.contador_temporales
    }

    /// Retorna el número de temporales disponibles
    pub fn temporales_libres(&self) -> usize {
        self.temporales_disponibles.len()
    }
}

impl Default for GestorMemoria {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_siguiente_temporal() {
        let mut gestor = GestorMemoria::new();

        let t1 = gestor.siguiente_temporal(TipoDato::Entero);
        assert_eq!(t1, 0);

        let t2 = gestor.siguiente_temporal(TipoDato::Flotante);
        assert_eq!(t2, 1);

        let t3 = gestor.siguiente_temporal(TipoDato::Entero);
        assert_eq!(t3, 2);

        assert_eq!(gestor.total_temporales(), 3);
    }

    #[test]
    fn test_liberar_temporal() {
        let mut gestor = GestorMemoria::new();

        let t1 = gestor.siguiente_temporal(TipoDato::Entero);
        let t2 = gestor.siguiente_temporal(TipoDato::Flotante);

        // Liberar t1
        gestor.liberar_temporal(t1);
        assert_eq!(gestor.temporales_libres(), 1);

        // El siguiente temporal debe reutilizar t1
        let t3 = gestor.siguiente_temporal(TipoDato::Entero);
        assert_eq!(t3, t1);
        assert_eq!(gestor.temporales_libres(), 0);
    }

    #[test]
    fn test_obtener_tipo_temporal() {
        let mut gestor = GestorMemoria::new();

        let t1 = gestor.siguiente_temporal(TipoDato::Entero);
        assert_eq!(gestor.obtener_tipo_temporal(t1), Some(TipoDato::Entero));

        let t2 = gestor.siguiente_temporal(TipoDato::Flotante);
        assert_eq!(gestor.obtener_tipo_temporal(t2), Some(TipoDato::Flotante));
    }

    #[test]
    fn test_reiniciar() {
        let mut gestor = GestorMemoria::new();

        gestor.siguiente_temporal(TipoDato::Entero);
        gestor.siguiente_temporal(TipoDato::Flotante);

        gestor.reiniciar();

        assert_eq!(gestor.total_temporales(), 0);
        assert_eq!(gestor.temporales_libres(), 0);
    }
}
