// ./src/estructuras/hashmap.rs
//! # HashMap/Tabla Hash/Diccionario - Estructura de Datos de Mapeo Clave-Valor
//!
//! Este módulo implementa una estructura de datos HashMap que proporciona mapeo
//! eficiente entre claves únicas y valores asociados.
//!
//! ## Características principales:
//! - **Acceso rápido**: Búsqueda, inserción y eliminación en tiempo O(1) promedio
//! - **Claves únicas**: Cada clave puede aparecer solo una vez en el mapa
//! - **Capacidad configurable**: Puede tener límite máximo de elementos o ser ilimitado
//! - **Manejo de errores**: Sistema robusto de manejo de errores con tipos específicos
//! - **Flexibilidad**: Soporta cualquier tipo que implemente Hash + Eq para claves
//!
//! ## Casos de uso comunes:
//! - Cachés y memoización
//! - Índices y bases de datos en memoria
//! - Contadores y estadísticas
//! - Configuraciones y metadatos
//! - Implementación de conjuntos (sets)
//! - Tablas de símbolos en compiladores

// Módulos a usar
use std::collections::HashMap;
use std::fmt;

/// Definir un enum para errores específicos del HashMap
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum ErrorHashMap {
    /// Error cuando se intenta acceder a un HashMap vacío
    HashMapVacio,
    /// Error cuando se excede la capacidad máxima (si aplica)
    CapacidadExcedida,
    /// Error de clave no encontrada
    ClaveNoEncontrada,
}

/// Implementar Display para mostrar errores de manera elegante
impl fmt::Display for ErrorHashMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorHashMap::HashMapVacio => write!(f, "Error: El HashMap está vacío"),
            ErrorHashMap::CapacidadExcedida => write!(f, "Error: Se ha excedido la capacidad máxima del HashMap"),
            ErrorHashMap::ClaveNoEncontrada => write!(f, "Error: Clave no encontrada en el HashMap"),
        }
    }
}

/// Implementar std::error::Error para el tipo de error
impl std::error::Error for ErrorHashMap {}

/// Tipo Result personalizado para operaciones del HashMap
pub type ResultadoHashMap<T> = Result<T, ErrorHashMap>;

/// Define una estructura HashMap personalizada
///
/// ## Estructura interna:
/// - `mapa`: HashMap estándar de Rust que almacena los pares clave-valor
/// - `capacidad_maxima`: Límite opcional de elementos (None = ilimitado)
///
/// ## Complejidad temporal (promedio):
/// - insertar(): O(1)
/// - obtener(): O(1)
/// - remover(): O(1)
/// - contiene_clave(): O(1)
///
/// ## Complejidad temporal (peor caso):
/// - Todas las operaciones: O(n) en caso de muchas colisiones
///
/// ## Complejidad espacial: O(n) donde n es el número de elementos
///
/// ## Requisitos para las claves (K):
/// - Debe implementar Hash: para calcular el hash de la clave
/// - Debe implementar Eq: para comparar claves en caso de colisión
/// - Debe implementar Clone para algunas operaciones (opcional)
pub struct MiHashMap<K, V> {
    mapa: HashMap<K, V>,
    capacidad_maxima: Option<usize>,
}

/// Implementar métodos para la estructura MiHashMap
#[allow(dead_code)]
impl<K: std::hash::Hash + Eq, V> MiHashMap<K, V> {
    /// Crear un nuevo HashMap vacío sin límite de capacidad
    pub fn nuevo() -> Self {
        MiHashMap {
            mapa: HashMap::new(),
            capacidad_maxima: None,
        }
    }

    /// Crear un nuevo HashMap vacío con capacidad máxima especificada
    pub fn nuevo_con_capacidad(capacidad: usize) -> Self {
        MiHashMap {
            mapa: HashMap::with_capacity(capacidad),
            capacidad_maxima: Some(capacidad),
        }
    }

    /// Insertar un par clave-valor en el HashMap
    pub fn insertar(&mut self, clave: K, valor: V) -> ResultadoHashMap<()> {
        if let Some(cap) = self.capacidad_maxima {
            if self.mapa.len() >= cap {
                return Err(ErrorHashMap::CapacidadExcedida);
            }
        }
        self.mapa.insert(clave, valor);
        Ok(())
    }

    /// Obtener un valor por su clave
    pub fn obtener(&self, clave: &K) -> ResultadoHashMap<&V> {
        self.mapa.get(clave).ok_or(ErrorHashMap::ClaveNoEncontrada)
    }

    /// Remover un par clave-valor por su clave
    pub fn remover(&mut self, clave: &K) -> ResultadoHashMap<V> {
        self.mapa.remove(clave).ok_or(ErrorHashMap::ClaveNoEncontrada)
    }

    /// Verificar si el HashMap está vacío
    pub fn esta_vacio(&self) -> bool {
        self.mapa.is_empty()
    }

    /// Obtener el número de elementos en el HashMap
    pub fn tamanio(&self) -> usize {
        self.mapa.len()
    }

    /// Obtener la capacidad máxima del HashMap (si aplica)
    pub fn capacidad_maxima(&self) -> Option<usize> {
        self.capacidad_maxima
    }

    /// Verificar si una clave existe en el HashMap
    pub fn contiene_clave(&self, clave: &K) -> bool {
        self.mapa.contains_key(clave)
    }

    /// Limpiar todos los elementos del HashMap
    pub fn limpiar(&mut self) {
        self.mapa.clear();
    }

    /// Obtener todas las claves del HashMap
    pub fn claves(&self) -> Vec<&K> {
        self.mapa.keys().collect()
    }

    /// Obtener todos los valores del HashMap
    pub fn valores(&self) -> Vec<&V> {
        self.mapa.values().collect()
    }

    /// Obtener todas las entradas (pares clave-valor) del HashMap
    pub fn entradas(&self) -> Vec<(&K, &V)> {
        self.mapa.iter().collect()
    }

    /// Verificar si el HashMap ha alcanzado su capacidad máxima
    pub fn esta_lleno(&self) -> bool {
        if let Some(capacidad) = self.capacidad_maxima {
            self.mapa.len() >= capacidad
        } else {
            false
        }
    }

    /// Obtener una referencia mutable a un valor por su clave
    pub fn obtener_mutable(&mut self, clave: &K) -> ResultadoHashMap<&mut V> {
        self.mapa.get_mut(clave).ok_or(ErrorHashMap::ClaveNoEncontrada)
    }

    /// Actualizar un valor existente, retorna el valor anterior
    pub fn actualizar(&mut self, clave: K, valor: V) -> ResultadoHashMap<Option<V>> {
        if !self.mapa.contains_key(&clave) {
            return Err(ErrorHashMap::ClaveNoEncontrada);
        }
        Ok(self.mapa.insert(clave, valor))
    }

    /// Insertar o actualizar un par clave-valor (upsert)
    pub fn insertar_o_actualizar(&mut self, clave: K, valor: V) -> ResultadoHashMap<Option<V>> {
        if let Some(cap) = self.capacidad_maxima {
            if !self.mapa.contains_key(&clave) && self.mapa.len() >= cap {
                return Err(ErrorHashMap::CapacidadExcedida);
            }
        }
        Ok(self.mapa.insert(clave, valor))
    }

    /// Obtener la capacidad actual del HashMap
    pub fn capacidad(&self) -> usize {
        self.mapa.capacity()
    }

    /// Reservar capacidad adicional para evitar realocaciones
    pub fn reservar(&mut self, capacidad_adicional: usize) -> ResultadoHashMap<()> {
        if let Some(cap_max) = self.capacidad_maxima {
            if self.mapa.len() + capacidad_adicional > cap_max {
                return Err(ErrorHashMap::CapacidadExcedida);
            }
        }
        self.mapa.reserve(capacidad_adicional);
        Ok(())
    }

    /// Reducir la capacidad del HashMap al mínimo necesario
    pub fn reducir_capacidad(&mut self) {
        self.mapa.shrink_to_fit();
    }

    /// Crear un iterador sobre las claves del HashMap
    pub fn iter_claves(&self) -> std::collections::hash_map::Keys<'_, K, V> {
        self.mapa.keys()
    }

    /// Crear un iterador sobre los valores del HashMap
    pub fn iter_valores(&self) -> std::collections::hash_map::Values<'_, K, V> {
        self.mapa.values()
    }

    /// Crear un iterador mutable sobre los valores del HashMap
    pub fn iter_valores_mut(&mut self) -> std::collections::hash_map::ValuesMut<'_, K, V> {
        self.mapa.values_mut()
    }

    /// Crear un iterador sobre las entradas (pares clave-valor) del HashMap
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, K, V> {
        self.mapa.iter()
    }

    /// Crear un iterador mutable sobre las entradas del HashMap
    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<'_, K, V> {
        self.mapa.iter_mut()
    }

    /// Insertar múltiples pares clave-valor de una vez
    pub fn insertar_multiples(&mut self, entradas: Vec<(K, V)>) -> ResultadoHashMap<()> {
        // Verificar capacidad antes de insertar
        if let Some(cap_max) = self.capacidad_maxima {
            let nuevas_claves = entradas.iter()
                .filter(|(k, _)| !self.mapa.contains_key(k))
                .count();
            if self.mapa.len() + nuevas_claves > cap_max {
                return Err(ErrorHashMap::CapacidadExcedida);
            }
        }

        for (clave, valor) in entradas {
            self.mapa.insert(clave, valor);
        }
        Ok(())
    }

    /// Remover múltiples claves de una vez
    pub fn remover_multiples(&mut self, claves: Vec<&K>) -> ResultadoHashMap<Vec<V>> {
        // Verificar que todas las claves existen antes de remover
        for clave in &claves {
            if !self.mapa.contains_key(clave) {
                return Err(ErrorHashMap::ClaveNoEncontrada);
            }
        }

        // Si todas las claves existen, proceder a remover
        let mut valores_removidos = Vec::new();
        for clave in claves {
            if let Some(valor) = self.mapa.remove(clave) {
                valores_removidos.push(valor);
            }
        }

        Ok(valores_removidos)
    }    /// Retener solo los elementos que cumplan con el predicado
    pub fn retener<F>(&mut self, mut predicado: F)
    where
        F: FnMut(&K, &V) -> bool,
    {
        self.mapa.retain(|k, v| predicado(k, v));
    }

    /// Obtener valor con valor por defecto si la clave no existe
    pub fn obtener_o_defecto<'a>(&'a self, clave: &K, defecto: &'a V) -> &'a V
    where
        V: Clone,
    {
        self.mapa.get(clave).unwrap_or(defecto)
    }    /// Fusionar otro HashMap con este
    pub fn fusionar(&mut self, otro: MiHashMap<K, V>) -> ResultadoHashMap<()> {
        if let Some(cap_max) = self.capacidad_maxima {
            let nuevas_claves = otro.mapa.iter()
                .filter(|(k, _)| !self.mapa.contains_key(k))
                .count();
            if self.mapa.len() + nuevas_claves > cap_max {
                return Err(ErrorHashMap::CapacidadExcedida);
            }
        }

        for (clave, valor) in otro.mapa {
            self.mapa.insert(clave, valor);
        }
        Ok(())
    }

    /// Convertir el HashMap en un vector de pares clave-valor
    pub fn a_vector(self) -> Vec<(K, V)> {
        self.mapa.into_iter().collect()
    }
}

// Implementar Clone para MiHashMap si K y V implementan Clone
impl<K: std::hash::Hash + Eq + Clone, V: Clone> Clone for MiHashMap<K, V> {
    fn clone(&self) -> Self {
        MiHashMap {
            mapa: self.mapa.clone(),
            capacidad_maxima: self.capacidad_maxima,
        }
    }
}