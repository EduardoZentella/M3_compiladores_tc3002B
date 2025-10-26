use std::collections::HashSet;

// Modulos
pub mod parser;
pub mod first_follow;
pub mod lr0;

// Simbolo en la gramatica
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Simbolo {
    Terminal(String),
    NoTerminal(String),
}

// Produccion en la gramatica
#[derive(Debug, Clone)]
pub struct Produccion {
    pub numero: usize,
    pub cabeza: Simbolo,
    pub cuerpo: Vec<Simbolo>,
}

// Gramatica
#[derive(Debug, Clone, Default)]
pub struct Gramatica {
    pub producciones: Vec<Produccion>,
    pub simbolos_terminales: HashSet<Simbolo>,
    pub simbolos_no_terminales: HashSet<Simbolo>,
    pub regla0: String,
}