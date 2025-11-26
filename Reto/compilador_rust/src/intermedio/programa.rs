//! # Programa Objeto
//!
//! Representa el "binario" compilado listo para ejecutar en la VM.
//! Es el enlace entre el compilador y la máquina virtual.

use std::collections::HashMap;
use crate::intermedio::cuadruplo::Cuadruplo;
use crate::vm::memoria::Valor;

/// Información de una función en el programa compilado
#[derive(Debug, Clone)]
pub struct InfoFuncionPrograma {
    /// Nombre de la función
    pub nombre: String,

    /// Dirección (índice) del primer cuádruplo de la función
    pub direccion_inicio: usize,

    /// Si la función retorna un valor
    pub tiene_retorno: bool,

    /// Tipo de retorno (si tiene_retorno es true)
    pub tipo_retorno: Option<String>,

    /// Número de parámetros
    pub num_parametros: usize,
}

/// Programa objeto - el "ejecutable" generado por el compilador
#[derive(Debug, Clone)]
pub struct ProgramaObjeto {
    /// Lista de cuádruplos (instrucciones)
    pub cuadruplos: Vec<Cuadruplo>,

    /// Mapa de funciones: nombre -> información de función
    pub mapa_funciones: HashMap<String, InfoFuncionPrograma>,

    /// Mapa de constantes: dirección virtual -> valor
    pub mapa_constantes: HashMap<usize, Valor>,

    /// Nombre del programa
    pub nombre_programa: String,

    /// Tabla de strings literales
    pub tabla_strings: Vec<String>,
}

impl ProgramaObjeto {
    /// Crea un nuevo programa objeto vacío
    pub fn new(nombre_programa: String) -> Self {
        ProgramaObjeto {
            cuadruplos: Vec::new(),
            mapa_funciones: HashMap::new(),
            mapa_constantes: HashMap::new(),
            nombre_programa,
            tabla_strings: Vec::new(),
        }
    }

    /// Crea un programa objeto con los componentes especificados
    pub fn crear(
        nombre_programa: String,
        cuadruplos: Vec<Cuadruplo>,
        mapa_funciones: HashMap<String, InfoFuncionPrograma>,
        mapa_constantes: HashMap<usize, Valor>,
        tabla_strings: Vec<String>,
    ) -> Self {
        ProgramaObjeto {
            cuadruplos,
            mapa_funciones,
            mapa_constantes,
            nombre_programa,
            tabla_strings,
        }
    }

    /// Obtiene el número total de cuádruplos
    pub fn num_cuadruplos(&self) -> usize {
        self.cuadruplos.len()
    }

    /// Obtiene el número de funciones
    pub fn num_funciones(&self) -> usize {
        self.mapa_funciones.len()
    }

    /// Obtiene información de una función por nombre
    pub fn obtener_funcion(&self, nombre: &str) -> Option<&InfoFuncionPrograma> {
        self.mapa_funciones.get(nombre)
    }

    /// Verifica si una función existe
    pub fn tiene_funcion(&self, nombre: &str) -> bool {
        self.mapa_funciones.contains_key(nombre)
    }

    /// Imprime un resumen del programa para debugging
    pub fn resumen(&self) -> String {
        format!(
            "Programa: {}\n  Cuádruplos: {}\n  Funciones: {}\n  Constantes: {}",
            self.nombre_programa,
            self.num_cuadruplos(),
            self.num_funciones(),
            self.mapa_constantes.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intermedio::cuadruplo::{Cuadruplo, OperadorCuadruplo, Operando};

    #[test]
    fn test_programa_objeto_vacio() {
        let programa = ProgramaObjeto::new("test".to_string());
        assert_eq!(programa.nombre_programa, "test");
        assert_eq!(programa.num_cuadruplos(), 0);
        assert_eq!(programa.num_funciones(), 0);
    }

    #[test]
    fn test_programa_objeto_con_datos() {
        let mut mapa_funciones = HashMap::new();
        mapa_funciones.insert("main".to_string(), InfoFuncionPrograma {
            nombre: "main".to_string(),
            direccion_inicio: 0,
            tiene_retorno: false,
            tipo_retorno: None,
            num_parametros: 0,
        });

        let cuadruplos = vec![
            Cuadruplo::new(
                OperadorCuadruplo::Suma,
                Operando::ConstanteEntera(1),
                Operando::ConstanteEntera(2),
                Operando::Direccion(10000),
            ),
        ];

        let programa = ProgramaObjeto::crear(
            "test".to_string(),
            cuadruplos,
            mapa_funciones,
            HashMap::new(),
            Vec::new(),
        );

        assert_eq!(programa.num_cuadruplos(), 1);
        assert_eq!(programa.num_funciones(), 1);
        assert!(programa.tiene_funcion("main"));
    }
}
