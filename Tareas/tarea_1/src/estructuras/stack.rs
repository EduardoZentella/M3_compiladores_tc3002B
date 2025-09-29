// ./src/estructuras/stack.rs
//! # Stack (Pila) - Estructura de Datos LIFO (Last In, First Out)
//!
//! Este módulo implementa una estructura de datos Stack (Pila) que sigue el principio LIFO.
//! Los elementos se insertan y remueven desde el mismo extremo (tope de la pila).
//!
//! ## Características principales:
//! - **LIFO**: El último elemento insertado es el primero en ser removido
//! - **Operaciones principales**: push (insertar), pop (remover), peek (ver el tope)
//! - **Capacidad configurable**: Puede tener límite máximo de elementos o ser ilimitada
//! - **Manejo de errores**: Sistema robusto de manejo de errores con tipos específicos
//! - **Eficiencia**: Basado en Vec<T> para operaciones O(1) amortizado
//!
//! ## Casos de uso comunes:
//! - Evaluación de expresiones matemáticas
//! - Manejo de llamadas a funciones (call stack)
//! - Navegación en historial (undo/redo)
//! - Algoritmos de backtracking
//! - Parsers y compiladores

// Módulos a usar
use std::fmt;

/// Definir un enum para errores específicos del Stack
///
/// Este enum encapsula todos los posibles errores que pueden ocurrir
/// durante las operaciones del Stack, proporcionando un manejo de errores
/// type-safe y descriptivo.
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum ErrorStack {
    /// Error cuando se intenta acceder a un stack vacío
    /// Se produce en operaciones como pop() o peek() cuando no hay elementos
    StackVacio,
    /// Error cuando se excede la capacidad máxima (si aplica)
    /// Ocurre al intentar insertar elementos cuando el stack está lleno
    CapacidadExcedida,
    /// Error de índice fuera de rango
    /// Se produce al acceder a posiciones que no existen en el stack
    IndiceInvalido,
}

/// Implementar Display para mostrar errores de manera elegante
impl fmt::Display for ErrorStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorStack::StackVacio => write!(f, "Error: El stack está vacío"),
            ErrorStack::CapacidadExcedida => write!(f, "Error: Se ha excedido la capacidad máxima del stack"),
            ErrorStack::IndiceInvalido => write!(f, "Error: Índice fuera de rango"),
        }
    }
}

/// Implementar std::error::Error para el tipo de error
impl std::error::Error for ErrorStack {}

/// Tipo Result personalizado para operaciones del stack
pub type ResultadoStack<T> = Result<T, ErrorStack>;

/// Define una estructura Stack que usa Vec para operaciones LIFO
///
/// ## Estructura interna:
/// - `elementos`: Vector que almacena los elementos del stack
/// - `capacidad_maxima`: Límite opcional de elementos (None = ilimitado)
///
/// ## Complejidad temporal:
/// - push(): O(1) amortizado
/// - pop(): O(1)
/// - peek(): O(1)
/// - esta_vacio(): O(1)
/// - tamanio(): O(1)
///
/// ## Complejidad espacial: O(n) donde n es el número de elementos
pub struct Stack<T> {
    elementos: Vec<T>,
    capacidad_maxima: Option<usize>,
}

/// Implementar métodos para la estructura Stack
#[allow(dead_code)]
impl<T> Stack<T> {
    /// Crear un nuevo Stack vacío sin límite de capacidad
    pub fn nuevo() -> Self {
        Stack {
            elementos: Vec::new(),
            capacidad_maxima: None,
        }
    }

    /// Crear un nuevo Stack vacío con capacidad máxima especificada
    pub fn nuevo_con_capacidad(capacidad: usize) -> Self {
        Stack {
            elementos: Vec::with_capacity(capacidad),
            capacidad_maxima: Some(capacidad),
        }
    }

    /// Agregar un elemento al tope del stack
    pub fn push(&mut self, elemento: T) -> ResultadoStack<()> {
        if let Some(capacidad) = self.capacidad_maxima {
            if self.elementos.len() >= capacidad {
                return Err(ErrorStack::CapacidadExcedida);
            }
        }
        self.elementos.push(elemento);
        Ok(())
    }

    /// Remover y retornar el elemento en el tope del stack
    pub fn pop(&mut self) -> ResultadoStack<T> {
        self.elementos.pop().ok_or(ErrorStack::StackVacio)
    }

    /// Ver el elemento en el tope del stack sin removerlo
    pub fn peek(&self) -> ResultadoStack<&T> {
        self.elementos.last().ok_or(ErrorStack::StackVacio)
    }

    /// Verificar si el stack está vacío
    pub fn esta_vacio(&self) -> bool {
        self.elementos.is_empty()
    }

    /// Obtener el número de elementos en el stack
    pub fn tamanio(&self) -> usize {
        self.elementos.len()
    }

    /// Obtener la capacidad máxima del stack (si aplica)
    pub fn capacidad_maxima(&self) -> Option<usize> {
        self.capacidad_maxima
    }

    /// Verificar si el stack está lleno (solo si tiene capacidad máxima)
    pub fn esta_lleno(&self) -> bool {
        if let Some(capacidad) = self.capacidad_maxima {
            self.elementos.len() >= capacidad
        } else {
            false
        }
    }

    /// Obtener una referencia a un elemento por índice (desde el fondo del stack)
    /// Retorna un error si el índice es inválido
    pub fn obtener(&self, indice: usize) -> ResultadoStack<&T> {
        self.elementos.get(indice).ok_or(ErrorStack::IndiceInvalido)
    }

    /// Limpiar todos los elementos del Stack
    /// Esta operación nunca falla
    pub fn limpiar(&mut self) {
        self.elementos.clear();
    }

    /// Obtener la capacidad actual del Stack
    pub fn capacidad(&self) -> usize {
        self.elementos.capacity()
    }

    /// Reservar capacidad adicional para evitar realocaciones
    /// Retorna un error si excede la capacidad máxima configurada
    pub fn reservar(&mut self, capacidad_adicional: usize) -> ResultadoStack<()> {
        if let Some(cap_max) = self.capacidad_maxima {
            if self.elementos.len() + capacidad_adicional > cap_max {
                return Err(ErrorStack::CapacidadExcedida);
            }
        }
        self.elementos.reserve(capacidad_adicional);
        Ok(())
    }

    /// Reducir la capacidad del Stack al mínimo necesario
    pub fn reducir_capacidad(&mut self) {
        self.elementos.shrink_to_fit();
    }

    /// Convertir el Stack en un vector, consumiendo el Stack
    pub fn a_vector(self) -> Vec<T> {
        self.elementos
    }

    /// Crear un iterador sobre los elementos del Stack (desde el fondo hacia el tope)
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.elementos.iter()
    }

    /// Crear un iterador que va desde el tope hacia el fondo (orden LIFO)
    pub fn iter_reverso(&self) -> std::iter::Rev<std::slice::Iter<'_, T>> {
        self.elementos.iter().rev()
    }

    /// Intentar hacer pop de múltiples elementos de una vez
    /// Retorna un error si no hay suficientes elementos
    pub fn pop_multiples(&mut self, cantidad: usize) -> ResultadoStack<Vec<T>> {
        if self.elementos.len() < cantidad {
            return Err(ErrorStack::StackVacio);
        }

        let mut resultado = Vec::with_capacity(cantidad);
        for _ in 0..cantidad {
            if let Some(elemento) = self.elementos.pop() {
                resultado.push(elemento);
            }
        }
        Ok(resultado)
    }

    /// Hacer push de múltiples elementos de una vez
    /// Retorna un error si excede la capacidad máxima
    pub fn push_multiples(&mut self, elementos: Vec<T>) -> ResultadoStack<()> {
        if let Some(cap_max) = self.capacidad_maxima {
            if self.elementos.len() + elementos.len() > cap_max {
                return Err(ErrorStack::CapacidadExcedida);
            }
        }

        for elemento in elementos {
            self.elementos.push(elemento);
        }
        Ok(())
    }
}