//! # Contexto Semántico
//! - Rastrear el alcance actual (función en la que estamos)
//! - Rastrear el tipo actual (durante declaraciones de variables)
//! - Proporcionar acceso al directorio de funciones y cubo semántico
//! - Facilitar la búsqueda de variables en el alcance correcto

use crate::semantico::{
    DirectorioFunciones,
    CuboSemantico,
    TipoDato,
    TipoRetorno,
};
use crate::semantico::tabla_variables::EntradaVariable;

/// Contexto Semántico
/// Mantiene el estado del análisis semántico durante el parsing.
pub struct ContextoSemantico {
    /// Directorio de funciones (estructura principal)
    pub dir_funciones: DirectorioFunciones,

    /// Cubo semántico para validación de tipos
    pub cubo_semantico: CuboSemantico,

    /// Nombre del alcance actual (función en la que estamos)
    /// Inicialmente es el nombre del programa (alcance global)
    alcance_actual: String,

    /// Tipo actual durante declaración de variables
    /// Se usa para recordar el tipo mientras procesamos una lista de variables
    tipo_actual: Option<TipoDato>,

    /// Nombre del programa (alcance global)
    nombre_programa: String,
}

impl ContextoSemantico {
    /// Crea un nuevo contexto semántico
    pub fn new() -> Self {
        ContextoSemantico {
            dir_funciones: DirectorioFunciones::new(),
            cubo_semantico: CuboSemantico::new(),
            alcance_actual: String::new(),
            tipo_actual: None,
            nombre_programa: String::new(),
        }
    }

    /// GESTIÓN DE ALCANCES
    /// Establece el nombre del programa (alcance global)
    /// Esta función se llama en el punto neurálgico PN1 (después de `programa id;`)
    pub fn inicializar_programa(&mut self, nombre: &str) -> Result<(), String> {
        self.nombre_programa = nombre.to_string();
        self.alcance_actual = nombre.to_string();

        // Crear la entrada para el alcance global
        self.dir_funciones.agregar_funcion(nombre, TipoRetorno::Nula)?;

        Ok(())
    }

    /// Inicia una nueva función
    /// Esta función se llama en el punto neurálgico PN4 (al iniciar `<FUNCS>`)
    pub fn iniciar_funcion(&mut self, nombre: &str, tipo_retorno: TipoRetorno) -> Result<(), String> {
        // Validar que no sea el nombre del programa
        if nombre == self.nombre_programa {
            return Err(format!(
                "Error semántico: El nombre de función '{}' no puede ser igual al nombre del programa",
                nombre
            ));
        }

        // Agregar la función al directorio
        self.dir_funciones.agregar_funcion(nombre, tipo_retorno)?;

        // Cambiar el alcance actual a la nueva función
        self.alcance_actual = nombre.to_string();

        Ok(())
    }

    /// Finaliza la función actual y regresa al alcance global
    /// Esta función se llama en el punto neurálgico PN6 (al finalizar `<FUNCS>`)
    pub fn finalizar_funcion(&mut self) {
        self.alcance_actual = self.nombre_programa.clone();
    }

    /// Retorna el nombre del alcance actual
    pub fn alcance_actual(&self) -> &str {
        &self.alcance_actual
    }

    /// Retorna el nombre del programa (alcance global)
    pub fn nombre_programa(&self) -> &str {
        &self.nombre_programa
    }

    // GESTIÓN DE VARIABLES
    /// Establece el tipo actual para declaración de variables
    /// Esta función se llama en el punto neurálgico PN2 (después de reconocer `<TIPO>`)
    pub fn establecer_tipo_actual(&mut self, tipo: TipoDato) {
        self.tipo_actual = Some(tipo);
    }

    /// Obtiene el tipo actual
    pub fn obtener_tipo_actual(&self) -> Option<TipoDato> {
        self.tipo_actual
    }

    /// Agrega una variable al alcance actual
    /// Esta función se llama en el punto neurálgico PN3 (al reconocer un `id` en `<VAR_LIST>`)
    pub fn agregar_variable(&mut self, nombre: &str) -> Result<(), String> {
        // Obtener el tipo actual
        let tipo = self.tipo_actual.ok_or_else(||
            "Error interno: No hay tipo actual establecido para declaración de variable".to_string()
        )?;

        // Agregar la variable al alcance actual
        self.dir_funciones.agregar_variable(&self.alcance_actual, nombre, tipo)
    }

    /// Busca una variable (primero en alcance local, luego en global)
    /// Esta función se llama en el punto neurálgico PN7 (al usar un `id`)
    pub fn buscar_variable(&self, nombre: &str) -> Option<&EntradaVariable> {
        // Buscar en alcance local (actual)
        if let Some(var) = self.dir_funciones.buscar_variable(&self.alcance_actual, nombre) {
            return Some(var);
        }

        // Si no está en local y no estamos en el global, buscar en global
        if self.alcance_actual != self.nombre_programa {
            if let Some(var) = self.dir_funciones.buscar_variable(&self.nombre_programa, nombre) {
                return Some(var);
            }
        }

        None
    }

    /// Valida que una variable exista y retorna su tipo
    pub fn obtener_tipo_variable(&self, nombre: &str) -> Result<TipoDato, String> {
        self.buscar_variable(nombre)
            .map(|entrada| entrada.tipo)
            .ok_or_else(|| format!(
                "Error semántico: Variable '{}' no declarada",
                nombre
            ))
    }

    /// Verifica que una función exista en el directorio
    pub fn verificar_funcion_existe(&self, nombre: &str) -> Result<(), String> {
        if self.dir_funciones.buscar_funcion(nombre).is_some() {
            Ok(())
        } else {
            Err(format!("Error semántico: Función '{}' no declarada", nombre))
        }
    }

    // UTILIDADES
    /// Imprime el estado completo del contexto semántico
    pub fn imprimir(&self) {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║              CONTEXTO SEMÁNTICO                             ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!("Programa: {}", self.nombre_programa);
        println!("Alcance actual: {}", self.alcance_actual);
        println!("Tipo actual: {:?}", self.tipo_actual);

        self.dir_funciones.imprimir();
    }
}

impl Default for ContextoSemantico {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inicializar_programa() {
        let mut ctx = ContextoSemantico::new();

        assert!(ctx.inicializar_programa("mi_programa").is_ok());
        assert_eq!(ctx.nombre_programa(), "mi_programa");
        assert_eq!(ctx.alcance_actual(), "mi_programa");
    }

    #[test]
    fn test_funciones() {
        let mut ctx = ContextoSemantico::new();
        ctx.inicializar_programa("test").unwrap();

        // Iniciar función
        assert!(ctx.iniciar_funcion("suma", TipoRetorno::Tipo(TipoDato::Entero)).is_ok());
        assert_eq!(ctx.alcance_actual(), "suma");

        // Finalizar función
        ctx.finalizar_funcion();
        assert_eq!(ctx.alcance_actual(), "test");
    }

    #[test]
    fn test_variables() {
        let mut ctx = ContextoSemantico::new();
        ctx.inicializar_programa("test").unwrap();

        // Establecer tipo y agregar variable
        ctx.establecer_tipo_actual(TipoDato::Entero);
        assert!(ctx.agregar_variable("x").is_ok());

        // Buscar variable
        assert!(ctx.buscar_variable("x").is_some());
        assert_eq!(ctx.obtener_tipo_variable("x").unwrap(), TipoDato::Entero);

        // Variable inexistente
        assert!(ctx.buscar_variable("y").is_none());
    }

    #[test]
    fn test_alcance_variables() {
        let mut ctx = ContextoSemantico::new();
        ctx.inicializar_programa("test").unwrap();

        // Variable global
        ctx.establecer_tipo_actual(TipoDato::Entero);
        ctx.agregar_variable("global_var").unwrap();

        // Iniciar función y agregar variable local
        ctx.iniciar_funcion("mi_func", TipoRetorno::Nula).unwrap();
        ctx.establecer_tipo_actual(TipoDato::Flotante);
        ctx.agregar_variable("local_var").unwrap();

        // Desde la función, debe ver ambas variables
        assert!(ctx.buscar_variable("local_var").is_some());
        assert!(ctx.buscar_variable("global_var").is_some());

        // Finalizar función y regresar a global
        ctx.finalizar_funcion();

        // Desde global, no debe ver la variable local
        assert!(ctx.buscar_variable("global_var").is_some());
        assert!(ctx.buscar_variable("local_var").is_none());
    }
}
