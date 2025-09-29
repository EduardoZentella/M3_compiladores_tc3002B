// ./src/estructuras/queue.rs
//! # Cola (Queue) - Estructura de Datos FIFO (First In, First Out)
//!
//! Este módulo implementa una estructura de datos Cola que sigue el principio FIFO.
//! Los elementos se insertan por un extremo (final) y se remueven por el otro (frente).
//!
//! ## Características principales:
//! - **FIFO**: El primer elemento insertado es el primero en ser removido
//! - **Operaciones principales**: encolar (insertar al final), desencolar (remover del frente)
//! - **Capacidad configurable**: Puede tener límite máximo de elementos o ser ilimitada
//! - **Manejo de errores**: Sistema robusto de manejo de errores con tipos específicos
//! - **Eficiencia**: Basado en VecDeque<T> para operaciones O(1) en ambos extremos
//!
//! ## Casos de uso comunes:
//! - Sistemas de colas de tareas (task queues)
//! - Buffering de datos en streaming
//! - Algoritmos BFS (Breadth-First Search)
//! - Simulación de procesos (scheduling)
//! - Manejo de eventos en orden de llegada

// Módulos a usar
use std::collections::VecDeque;
use std::fmt;

/// Definir un enum para errores específicos de la Cola
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum ErrorCola {
    /// Error cuando se intenta acceder a una cola vacía
    ColaVacia,
    /// Error cuando se excede la capacidad máxima (si aplica)
    CapacidadExcedida,
    /// Error de índice fuera de rango
    IndiceInvalido,
}

/// Implementar Display para mostrar errores de manera elegante
impl fmt::Display for ErrorCola {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCola::ColaVacia => write!(f, "Error: La cola está vacía"),
            ErrorCola::CapacidadExcedida => write!(f, "Error: Se ha excedido la capacidad máxima de la cola"),
            ErrorCola::IndiceInvalido => write!(f, "Error: Índice fuera de rango"),
        }
    }
}

/// Implementar std::error::Error para el tipo de error
impl std::error::Error for ErrorCola {}

/// Tipo Result personalizado para operaciones de la cola
pub type ResultadoCola<T> = Result<T, ErrorCola>;

/// Define una estructura Cola que usa VecDeque para operaciones FIFO eficientes
///
/// ## Estructura interna:
/// - `elementos`: VecDeque que almacena los elementos de la cola
/// - `capacidad_maxima`: Límite opcional de elementos (None = ilimitado)
///
/// ## Complejidad temporal:
/// - encolar(): O(1) amortizado
/// - desencolar(): O(1)
/// - frente(): O(1)
/// - ultimo(): O(1)
/// - esta_vacia(): O(1)
/// - tamanio(): O(1)
///
/// ## Complejidad espacial: O(n) donde n es el número de elementos
///
/// ## Ventajas de VecDeque sobre Vec:
/// - Inserción y eliminación eficiente en ambos extremos
/// - Mejor rendimiento para operaciones de cola que un Vec simple
/// - Manejo automático de la reorganización interna
pub struct Cola<T> {
    elementos: VecDeque<T>,
    capacidad_maxima: Option<usize>,
}

/// Implementar métodos para la estructura Cola
#[allow(dead_code)]
impl<T> Cola<T> {
    /// Crear una nueva Cola vacía sin límite de capacidad
    pub fn nueva() -> Self {
        Cola {
            elementos: VecDeque::new(),
            capacidad_maxima: None,
        }
    }

    /// Crear una nueva Cola vacía con capacidad máxima especificada
    pub fn nueva_con_capacidad(capacidad: usize) -> Self {
        Cola {
            elementos: VecDeque::with_capacity(capacidad),
            capacidad_maxima: Some(capacidad),
        }
    }

    /// Agregar un elemento al final de la Cola
    /// Retorna un error si se excede la capacidad máxima
    pub fn encolar(&mut self, elemento: T) -> ResultadoCola<()> {
        if let Some(cap) = self.capacidad_maxima {
            if self.elementos.len() >= cap {
                return Err(ErrorCola::CapacidadExcedida);
            }
        }
        self.elementos.push_back(elemento);
        Ok(())
    }

    /// Remover y retornar el primer elemento de la Cola
    /// Retorna un error si la cola está vacía
    pub fn desencolar(&mut self) -> ResultadoCola<T> {
        self.elementos.pop_front().ok_or(ErrorCola::ColaVacia)
    }

    /// Verificar si la Cola está vacía
    pub fn esta_vacia(&self) -> bool {
        self.elementos.is_empty()
    }

    /// Obtener el número de elementos en la Cola
    pub fn tamanio(&self) -> usize {
        self.elementos.len()
    }

    /// Ver el primer elemento sin removerlo de la Cola
    /// Retorna un error si la cola está vacía
    pub fn frente(&self) -> ResultadoCola<&T> {
        self.elementos.front().ok_or(ErrorCola::ColaVacia)
    }

    /// Ver el último elemento sin removerlo de la Cola
    /// Retorna un error si la cola está vacía
    pub fn ultimo(&self) -> ResultadoCola<&T> {
        self.elementos.back().ok_or(ErrorCola::ColaVacia)
    }

    /// Obtener una referencia a un elemento por índice
    /// Retorna un error si el índice es inválido
    pub fn obtener(&self, indice: usize) -> ResultadoCola<&T> {
        self.elementos.get(indice).ok_or(ErrorCola::IndiceInvalido)
    }

    /// Limpiar todos los elementos de la Cola
    /// Esta operación nunca falla
    pub fn limpiar(&mut self) {
        self.elementos.clear();
    }

    /// Obtener la capacidad actual de la Cola
    pub fn capacidad(&self) -> usize {
        self.elementos.capacity()
    }

    /// Obtener la capacidad máxima configurada (si existe)
    pub fn capacidad_maxima(&self) -> Option<usize> {
        self.capacidad_maxima
    }

    /// Verificar si la Cola ha alcanzado su capacidad máxima
    pub fn esta_llena(&self) -> bool {
        if let Some(cap_max) = self.capacidad_maxima {
            self.elementos.len() >= cap_max
        } else {
            false
        }
    }

    /// Reservar capacidad adicional para evitar realocaciones
    /// Retorna un error si excede la capacidad máxima configurada
    pub fn reservar(&mut self, capacidad_adicional: usize) -> ResultadoCola<()> {
        if let Some(cap_max) = self.capacidad_maxima {
            if self.elementos.len() + capacidad_adicional > cap_max {
                return Err(ErrorCola::CapacidadExcedida);
            }
        }
        self.elementos.reserve(capacidad_adicional);
        Ok(())
    }

    /// Reducir la capacidad de la Cola al mínimo necesario
    pub fn reducir_capacidad(&mut self) {
        self.elementos.shrink_to_fit();
    }

    /// Convertir la Cola en un vector, consumiendo la Cola
    pub fn a_vector(self) -> Vec<T> {
        self.elementos.into()
    }

    /// Crear un iterador sobre los elementos de la Cola
    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, T> {
        self.elementos.iter()
    }

    /// Intentar desencolar múltiples elementos de una vez
    /// Retorna un error si no hay suficientes elementos
    pub fn desencolar_multiples(&mut self, cantidad: usize) -> ResultadoCola<Vec<T>> {
        if self.elementos.len() < cantidad {
            return Err(ErrorCola::ColaVacia);
        }

        let mut resultado = Vec::with_capacity(cantidad);
        for _ in 0..cantidad {
            if let Some(elemento) = self.elementos.pop_front() {
                resultado.push(elemento);
            }
        }
        Ok(resultado)
    }

    /// Encolar múltiples elementos de una vez
    /// Retorna un error si excede la capacidad máxima
    pub fn encolar_multiples(&mut self, elementos: Vec<T>) -> ResultadoCola<()> {
        if let Some(cap_max) = self.capacidad_maxima {
            if self.elementos.len() + elementos.len() > cap_max {
                return Err(ErrorCola::CapacidadExcedida);
            }
        }

        for elemento in elementos {
            self.elementos.push_back(elemento);
        }
        Ok(())
    }
}