//! # Generador de Cuádruplos
//!
//! Implementa los algoritmos de traducción a código intermedio (cuádruplos)
//! para expresiones aritméticas, relacionales y estatutos lineales.
//!
//! Utiliza las siguientes estructuras:
//! - **POper**: Pila de operadores pendientes
//! - **PilaO**: Pila de operandos pendientes
//! - **PTypes**: Pila de tipos de operandos
//! - **Quad**: Cola de cuádruplos generados
//! - **Pjumps**: Pila de saltos pendientes

use crate::intermedio::{Cuadruplo, OperadorCuadruplo, memoria::GestorMemoria};
use crate::intermedio::memoria_virtual::MemoriaVirtual;
use crate::semantico::{CuboSemantico, TipoDato, ContextoSemantico};
use crate::intermedio::cuadruplo::Operando;
use std::collections::{VecDeque, HashMap};

/// Metadatos de una función recopilados durante la generación de código
#[derive(Debug, Clone)]
pub struct MetadatosFuncion {
    pub nombre: String,
    pub direccion_inicio: usize,
    pub tiene_retorno: bool,
    pub tipo_retorno: Option<TipoDato>,
    pub parametros: Vec<(String, TipoDato)>,
}


/// Generador de cuádruplos para código intermedio
pub struct GeneradorCuadruplos {
    // ==================== ESTRUCTURAS DE DATOS ====================
    /// Cuádruplos generados (lista de cuádruplos)
    quad: VecDeque<Cuadruplo>,

    /// Pila de operandos (valores, variables, temporales)
    pilao: Vec<Operando>,

    /// Pila de operadores (+, -, *, /, etc.)
    poper: Vec<OperadorCuadruplo>,

    /// Pila de tipos de datos (para verificación semántica)
    ptypes: Vec<TipoDato>,

    /// Pila de saltos pendientes (para if/while)
    pjumps: Vec<usize>,

    /// Contador de expresiones dentro de condiciones (para saber cuándo generar GOTOF)
    en_condicion: usize,

    // Gestión de memoria
    /// Gestor de variables temporales
    gestor_memoria: GestorMemoria,

    /// Sistema de memoria virtual (direcciones 1000-24999)
    memoria_virtual: MemoriaVirtual,

    // Validación semántica
    /// Cubo semántico para validación de tipos
    cubo_semantico: CuboSemantico,

    // Contexto semántico (para buscar variables)
    /// Referencia al contexto semántico del compilador
    contexto: Option<*const ContextoSemantico>,

    // Metadatos de funciones (construidos durante generación de código)
    /// Mapa de funciones con sus metadatos (direccion_inicio, parametros, etc.)
    metadatos_funciones: HashMap<String, MetadatosFuncion>,

    /// Nombre de la función actual siendo compilada
    funcion_actual: Option<String>,

    /// Tabla de strings literales (para letreros)
    tabla_strings: Vec<String>,
}

impl GeneradorCuadruplos {
    /// Crea un nuevo generador de cuádruplos
    pub fn new() -> Self {
        GeneradorCuadruplos {
            poper: Vec::new(),
            pilao: Vec::new(),
            ptypes: Vec::new(),
            quad: VecDeque::new(),
            pjumps: Vec::new(),
            en_condicion: 0,
            gestor_memoria: GestorMemoria::new(),
            memoria_virtual: MemoriaVirtual::new(),
            cubo_semantico: CuboSemantico::new(),
            contexto: None,
            metadatos_funciones: HashMap::new(),
            funcion_actual: None,
            tabla_strings: Vec::new(),
        }
    }

    /// Establece el contexto semántico
    pub fn establecer_contexto(&mut self, contexto: &ContextoSemantico) {
        self.contexto = Some(contexto as *const ContextoSemantico);
    }

    /// Obtiene una referencia al contexto semántico
    fn obtener_contexto(&self) -> Result<&ContextoSemantico, String> {
        unsafe {
            self.contexto
                .map(|ptr| &*ptr)
                .ok_or_else(|| "Error: Contexto semántico no establecido".to_string())
        }
    }

    // ==================== PUNTO NEURÁLGICO 1 ====================
    /// PN1: Procesar un identificador o constante
    /// PilaO.Push(id.name) y PTypes.Push(id.type)
    pub fn procesar_operando(&mut self, nombre: &str) -> Result<(), String> {
        // Si es un letrero (string literal entre comillas)
        if nombre.starts_with('"') && nombre.ends_with('"') {
            // Agregar el string a la tabla y obtener su índice
            let idx = self.tabla_strings.len();
            self.tabla_strings.push(nombre.to_string());
            
            self.pilao.push(Operando::Letrero(idx));
            self.ptypes.push(TipoDato::Letrero);
            return Ok(());
        }

        // Si es un número (constante)
        if let Ok(valor_entero) = nombre.parse::<i32>() {
            self.pilao.push(Operando::ConstanteEntera(valor_entero));
            self.ptypes.push(TipoDato::Entero);
            return Ok(());
        }

        if let Ok(valor_flotante) = nombre.parse::<f64>() {
            self.pilao.push(Operando::ConstanteFlotante(valor_flotante));
            self.ptypes.push(TipoDato::Flotante);
            return Ok(());
        }

        // Si es una variable, buscar su tipo y dirección
        let contexto = self.obtener_contexto()?;
        let tipo = contexto.obtener_tipo_variable(nombre)?;
        let direccion = contexto.obtener_direccion_variable(nombre)?;

        self.pilao.push(Operando::Direccion(direccion));
        self.ptypes.push(tipo);

        Ok(())
    }

    // ==================== PUNTO NEURÁLGICO 2 ====================
    /// PN2: Procesar operador de suma o resta
    /// POper.Push(+ o -)
    pub fn procesar_suma_resta(&mut self, operador: &str) -> Result<(), String> {
        let op = match operador {
            "+" => OperadorCuadruplo::Suma,
            "-" => OperadorCuadruplo::Resta,
            _ => return Err(format!("Operador inválido para suma/resta: {}", operador)),
        };

        self.poper.push(op);
        Ok(())
    }

    // ==================== PUNTO NEURÁLGICO 3 ====================
    /// PN3: Procesar operador de multiplicación o división
    /// POper.Push(* o /)
    pub fn procesar_mult_div(&mut self, operador: &str) -> Result<(), String> {
        let op = match operador {
            "*" => OperadorCuadruplo::Multiplicacion,
            "/" => OperadorCuadruplo::Division,
            _ => return Err(format!("Operador inválido para mult/div: {}", operador)),
        };

        self.poper.push(op);
        Ok(())
    }

    // ==================== PUNTO NEURÁLGICO 4 ====================
    /// PN4: Generar cuádruplo para suma o resta
    /// If POper.top() == '+' or '-' then ...
    pub fn generar_suma_resta(&mut self) -> Result<(), String> {
        if let Some(&op) = self.poper.last() {
            if op == OperadorCuadruplo::Suma || op == OperadorCuadruplo::Resta {
                self.generar_cuadruplo_aritmetico()?;
            }
        }
        Ok(())
    }

    // ==================== PUNTO NEURÁLGICO 5 ====================
    /// PN5: Generar cuádruplo para multiplicación o división
    /// If POper.top() == '*' or '/' then ...
    pub fn generar_mult_div(&mut self) -> Result<(), String> {
        if let Some(&op) = self.poper.last() {
            if op == OperadorCuadruplo::Multiplicacion || op == OperadorCuadruplo::Division {
                self.generar_cuadruplo_aritmetico()?;
            }
        }
        Ok(())
    }

    /// Algoritmo genérico para generar cuádruplo aritmético (usado por PN4 y PN5)
    fn generar_cuadruplo_aritmetico(&mut self) -> Result<(), String> {
        // right_operand = PilaO.Pop()
        let operando_der = self.pilao.pop()
            .ok_or("Error: Pila de operandos vacía (derecho)")?;

        // right_Type = PTypes.Pop()
        let tipo_der = self.ptypes.pop()
            .ok_or("Error: Pila de tipos vacía (derecho)")?;

        // left_operand = PilaO.Pop()
        let operando_izq = self.pilao.pop()
            .ok_or("Error: Pila de operandos vacía (izquierdo)")?;

        // left_Type = PTypes.Pop()
        let tipo_izq = self.ptypes.pop()
            .ok_or("Error: Pila de tipos vacía (izquierdo)")?;

        // operator = POper.Pop()
        let operador = self.poper.pop()
            .ok_or("Error: Pila de operadores vacía")?;

        // Convertir OperadorCuadruplo a Operador del cubo semántico
        let op_semantico = self.convertir_operador_semantico(operador)?;

        // result_Type = Semantics[left_Type, right_Type, operator]
        let tipo_resultado = match self.cubo_semantico.validar(tipo_izq, op_semantico, tipo_der) {
            crate::semantico::cubo_semantico::ResultadoTipo::Ok(tipo) => tipo,
            crate::semantico::cubo_semantico::ResultadoTipo::Error => {
                return Err(format!(
                    "Error semántico: tipos incompatibles {:?} {:?} {:?}",
                    tipo_izq, operador, tipo_der
                ));
            }
        };        // result = AVAIL.next()
        let num_temporal = self.gestor_memoria.siguiente_temporal(tipo_resultado);
        let resultado = Operando::Temporal(num_temporal);

        // generate quad = (operator, left_operand, right_operand, result)
        let cuadruplo = Cuadruplo::new(
            operador,
            operando_izq.clone(),
            operando_der.clone(),
            resultado.clone(),
        );

        // Quad.Push(quad)
        self.quad.push_back(cuadruplo);

        // PilaO.Push(result)
        self.pilao.push(resultado);

        // PTypes.Push(result_Type)
        self.ptypes.push(tipo_resultado);

        // Si los operandos son temporales, liberarlos
        self.liberar_si_temporal(&operando_izq);
        self.liberar_si_temporal(&operando_der);

        Ok(())
    }

    // ==================== PUNTO NEURÁLGICO 6 ====================
    /// PN6: Marca de fondo falso (abre paréntesis)
    /// POper.Push(False bottom mark)
    pub fn abrir_parentesis(&mut self) {
        // Usamos un operador especial como marca de fondo falso
        // En este caso, usamos Goto como marca (no se generará cuádruplo con él)
        self.poper.push(OperadorCuadruplo::Goto);
    }

    // ==================== PUNTO NEURÁLGICO 7 ====================
    /// PN7: Quitar marca de fondo falso (cierra paréntesis)
    /// POper.Pop(False bottom mark)
    pub fn cerrar_parentesis(&mut self) -> Result<(), String> {
        if let Some(op) = self.poper.last() {
            if *op == OperadorCuadruplo::Goto {
                self.poper.pop();
                return Ok(());
            }
        }
        Err("Error: No se encontró marca de fondo falso al cerrar paréntesis".to_string())
    }

    // ==================== PUNTO NEURÁLGICO 8 ====================
    /// PN8: Procesar operador relacional
    /// POper.Push(rel.op) (like >, <, ==, !=)
    pub fn procesar_relacional(&mut self, operador: &str) -> Result<(), String> {
        let op = match operador {
            ">" => OperadorCuadruplo::MayorQue,
            "<" => OperadorCuadruplo::MenorQue,
            "==" => OperadorCuadruplo::Igual,
            "!=" => OperadorCuadruplo::Diferente,
            _ => return Err(format!("Operador relacional inválido: {}", operador)),
        };

        self.poper.push(op);
        Ok(())
    }

    // ==================== PUNTO NEURÁLGICO 9 ====================
    /// PN9: Generar cuádruplo para operador relacional
    /// If POper.top() == rel.op then ...
    pub fn generar_relacional(&mut self) -> Result<(), String> {
        eprintln!("[GENERADOR] generar_relacional: poper.last() = {:?}, en_condicion = {}",
                  self.poper.last(), self.en_condicion);

        if let Some(&op) = self.poper.last() {
            if matches!(op,
                OperadorCuadruplo::MayorQue |
                OperadorCuadruplo::MenorQue |
                OperadorCuadruplo::Igual |
                OperadorCuadruplo::Diferente
            ) {
                self.generar_cuadruplo_aritmetico()?;

                // Si estamos en una condición, generar GOTOF automáticamente
                if self.esta_en_condicion() {
                    eprintln!("[GENERADOR] Generando GOTOF automático después de expresión relacional");
                    self.generar_gotof()?;
                    self.finalizar_condicion(); // Ya procesamos la expresión
                }
            }
        }
        Ok(())
    }

    // ==================== ESTATUTOS LINEALES ====================

    /// Generar cuádruplo de asignación
    /// variable = expresión
    pub fn generar_asignacion(&mut self, variable: &str) -> Result<(), String> {
        // Obtener tipo y dirección de la variable
        let contexto = self.obtener_contexto()?;
        let tipo_var = contexto.obtener_tipo_variable(variable)?;
        let dir_var = contexto.obtener_direccion_variable(variable)?;

        // Obtener operando de la expresión
        let operando_expr = self.pilao.pop()
            .ok_or("Error: No hay expresión para asignar")?;
        let tipo_expr = self.ptypes.pop()
            .ok_or("Error: No hay tipo de expresión para asignar")?;

        // Validar compatibilidad de tipos con el cubo semántico
        let op_asig = crate::semantico::cubo_semantico::Operador::Asignacion;
        match self.cubo_semantico.validar(tipo_var, op_asig, tipo_expr) {
            crate::semantico::cubo_semantico::ResultadoTipo::Ok(_) => {},
            crate::semantico::cubo_semantico::ResultadoTipo::Error => {
                return Err(format!(
                    "Error en asignación: tipos incompatibles {:?} = {:?}",
                    tipo_var, tipo_expr
                ));
            }
        }

        // Generar cuádruplo: (=, expresión, -, dirección_variable)
        let cuadruplo = Cuadruplo::new(
            OperadorCuadruplo::Asignacion,
            operando_expr.clone(),
            Operando::Vacio,
            Operando::Direccion(dir_var),
        );

        self.quad.push_back(cuadruplo);

        // Liberar temporal si fue usado
        self.liberar_si_temporal(&operando_expr);

        Ok(())
    }

    /// Generar cuádruplo de escritura (escribe)
    pub fn generar_escritura(&mut self) -> Result<(), String> {
        let operando = self.pilao.pop()
            .ok_or("Error: No hay operando para escribir")?;
        let _tipo = self.ptypes.pop()
            .ok_or("Error: No hay tipo para escribir")?;

        // Generar cuádruplo: (escribe, -, -, operando)
        let cuadruplo = Cuadruplo::new(
            OperadorCuadruplo::Escritura,
            Operando::Vacio,
            Operando::Vacio,
            operando.clone(),
        );

        self.quad.push_back(cuadruplo);

        // Liberar temporal si fue usado
        self.liberar_si_temporal(&operando);

        Ok(())
    }

    /// Generar cuádruplo de lectura (lee)
    pub fn generar_lectura(&mut self, variable: &str) -> Result<(), String> {
        // Validar que la variable exista y obtener su dirección
        let contexto = self.obtener_contexto()?;
        contexto.obtener_tipo_variable(variable)?;
        let direccion = contexto.obtener_direccion_variable(variable)?;

        // Generar cuádruplo: (lee, -, -, dirección_variable)
        let cuadruplo = Cuadruplo::new(
            OperadorCuadruplo::Lectura,
            Operando::Vacio,
            Operando::Vacio,
            Operando::Direccion(direccion),
        );

        self.quad.push_back(cuadruplo);

        Ok(())
    }

    // ==================== ESTRUCTURAS DE CONTROL ====================

    /// Marcar que estamos entrando en una expresión condicional (si, mientras)
    pub fn iniciar_condicion(&mut self) {
        self.en_condicion += 1;
        eprintln!("[GENERADOR] Iniciando condición (nivel {})", self.en_condicion);
    }

    /// Marcar que salimos de una expresión condicional
    pub fn finalizar_condicion(&mut self) {
        if self.en_condicion > 0 {
            self.en_condicion -= 1;
        }
        eprintln!("[GENERADOR] Finalizando condición (nivel {})", self.en_condicion);
    }

    /// Verificar si estamos dentro de una expresión condicional
    pub fn esta_en_condicion(&self) -> bool {
        self.en_condicion > 0
    }

    /// Paso 13-19: Generar GOTOF para condicionales y ciclos
    /// Verifica que el resultado de la expresión sea booleano y genera salto condicional
    pub fn generar_gotof(&mut self) -> Result<(), String> {
        // Obtener el resultado de la expresión (debe estar en la pila)
        let tipo_expr = self.ptypes.last()
            .ok_or("Error: No hay expresión para evaluar en condicional")?;

        // Validar que sea tipo entero (se usa como booleano: 0=false, !=0=true)
        if *tipo_expr != TipoDato::Entero {
            return Err(format!(
                "Error: La expresión condicional debe ser de tipo entero (booleano), se encontró {:?}",
                tipo_expr
            ));
        }

        let operando_cond = self.pilao.pop()
            .ok_or("Error: No hay operando condicional")?;
        self.ptypes.pop();

        // Generar cuádruplo GOTOF con salto pendiente
        let cuadruplo = Cuadruplo::new(
            OperadorCuadruplo::GotoF,
            operando_cond.clone(),
            Operando::Vacio,
            Operando::Pendiente, // Se llenará después con FILL
        );

        self.quad.push_back(cuadruplo);

        // Guardar la posición del cuádruplo para FILL posterior
        let pos_salto = self.quad.len() - 1;
        self.pjumps.push(pos_salto);

        // Liberar temporal si fue usado
        self.liberar_si_temporal(&operando_cond);

        Ok(())
    }

    /// Paso 14: Rellenar salto de condicional (FILL)
    pub fn fill_salto_condicional(&mut self) -> Result<(), String> {
        let pos_salto = self.pjumps.pop()
            .ok_or("Error: No hay salto pendiente para rellenar")?;

        // La dirección de salto es la posición actual (siguiente cuádruplo)
        let direccion_salto = self.quad.len();

        // Actualizar el cuádruplo con la dirección correcta
        if let Some(cuadruplo) = self.quad.get_mut(pos_salto) {
            cuadruplo.resultado = Operando::Etiqueta(direccion_salto);
        } else {
            return Err(format!("Error: Posición de salto {} inválida", pos_salto));
        }

        Ok(())
    }

    /// Paso 16: Generar else (FILL del GOTOF y generar GOTO)
    /// Iniciar else: genera GOTO al final del then (antes de procesar el cuerpo del else)
    pub fn iniciar_else(&mut self) -> Result<(), String> {
        eprintln!("[INTERMEDIO] Iniciando else - generando GOTO");

        // Generar GOTO incondicional (saltará al final del else)
        let cuadruplo_goto = Cuadruplo::new(
            OperadorCuadruplo::Goto,
            Operando::Vacio,
            Operando::Vacio,
            Operando::Pendiente,
        );

        self.quad.push_back(cuadruplo_goto);

        // Guardar posición del GOTO para FILL posterior
        let pos_goto = self.quad.len() - 1;
        self.pjumps.push(pos_goto);

        // La siguiente posición será el inicio del else
        let inicio_else = self.quad.len();
        self.pjumps.push(inicio_else);

        Ok(())
    }

    pub fn generar_else(&mut self) -> Result<(), String> {
        // En pjumps tenemos: [GOTOF, GOTO, INICIO_ELSE]
        // Necesitamos extraer estos tres valores

        let inicio_else = self.pjumps.pop()
            .ok_or("Error: No hay inicio_else pendiente")?;

        let pos_goto = self.pjumps.pop()
            .ok_or("Error: No hay GOTO pendiente para else")?;

        let pos_gotof = self.pjumps.pop()
            .ok_or("Error: No hay GOTOF pendiente para else")?;

        // Rellenar el GOTOF para que apunte al inicio del else
        if let Some(cuadruplo) = self.quad.get_mut(pos_gotof) {
            cuadruplo.resultado = Operando::Etiqueta(inicio_else);
            eprintln!("[INTERMEDIO] FILL GOTOF: posición {} apunta a {}", pos_gotof, inicio_else);
        }

        // Rellenar el GOTO para que apunte después del else (posición actual)
        let fin_else = self.quad.len();
        if let Some(cuadruplo) = self.quad.get_mut(pos_goto) {
            cuadruplo.resultado = Operando::Etiqueta(fin_else);
            eprintln!("[INTERMEDIO] FILL GOTO: posición {} apunta a {}", pos_goto, fin_else);
        }

        Ok(())
    }    /// Paso 18: Marcar inicio de ciclo
    pub fn marcar_inicio_ciclo(&mut self) {
        // Guardar la posición actual (inicio del ciclo)
        let pos_inicio = self.quad.len();
        self.pjumps.push(pos_inicio);
    }

    /// Paso 20: Generar fin de ciclo (GOTO inicio y FILL GOTOF)
    pub fn generar_fin_ciclo(&mut self) -> Result<(), String> {
        // Obtener posición del GOTOF (salida del ciclo)
        let pos_gotof = self.pjumps.pop()
            .ok_or("Error: No hay GOTOF de ciclo para rellenar")?;

        // Obtener posición de inicio del ciclo
        let pos_inicio = self.pjumps.pop()
            .ok_or("Error: No hay marca de inicio de ciclo")?;

        // Generar GOTO para regresar al inicio
        let cuadruplo_goto = Cuadruplo::new(
            OperadorCuadruplo::Goto,
            Operando::Vacio,
            Operando::Vacio,
            Operando::Etiqueta(pos_inicio),
        );

        self.quad.push_back(cuadruplo_goto);

        // Rellenar el GOTOF para que salte al final del ciclo
        let direccion_fin = self.quad.len();
        if let Some(cuadruplo) = self.quad.get_mut(pos_gotof) {
            cuadruplo.resultado = Operando::Etiqueta(direccion_fin);
        }

        Ok(())
    }

    // ==================== LLAMADAS A FUNCIONES ====================

    /// Marcar el inicio de una función durante la compilación
    /// Esto debe llamarse cuando se reduce la producción de declaración de función
    pub fn iniciar_funcion(&mut self, nombre: &str) -> Result<(), String> {
        let contexto = self.obtener_contexto()?;

        // Obtener información del contexto semántico
        let entrada_funcion = contexto.dir_funciones.buscar_funcion(nombre)
            .ok_or_else(|| format!("Función '{}' no existe en el directorio", nombre))?;

        // Extraer tipo de retorno
        let (tiene_retorno, tipo_retorno) = match &entrada_funcion.tipo_retorno {
            crate::semantico::tipos::TipoRetorno::Nula => (false, None),
            crate::semantico::tipos::TipoRetorno::Tipo(t) => (true, Some(t.clone())),
        };

        // Obtener parámetros de la tabla de variables
        let parametros = contexto.dir_funciones.obtener_parametros(nombre);

        // Crear metadatos
        let metadatos = MetadatosFuncion {
            nombre: nombre.to_string(),
            direccion_inicio: self.quad.len(), // La función empieza en el cuádruplo actual
            tiene_retorno,
            tipo_retorno,
            parametros,
        };

        // Guardar metadatos
        self.metadatos_funciones.insert(nombre.to_string(), metadatos);
        self.funcion_actual = Some(nombre.to_string());

        Ok(())
    }

    /// Marcar el fin de una función y generar ENDFUNC
    /// Se llama cuando se reduce la producción completa de función
    pub fn finalizar_funcion(&mut self) -> Result<(), String> {
        // Generar cuádruplo ENDFUNC
        self.generar_endfunc()?;

        // Limpiar función actual
        self.funcion_actual = None;

        Ok(())
    }

    /// Paso 1: Verificar que una función existe y guardar su nombre
    pub fn iniciar_llamada(&mut self, nombre_func: &str) -> Result<(), String> {
        let contexto = self.obtener_contexto()?;
        contexto.verificar_funcion_existe(nombre_func)?;

        // Guardar el nombre de la función para usarlo en generar_gosub
        self.pilao.push(Operando::Variable(nombre_func.to_string()));

        Ok(())
    }

    /// Paso 2: Generar ERA (Expand Activation Record)
    /// Se invoca cuando se reduce <LLAMADA_HEADER> (después de procesar id)
    pub fn generar_era(&mut self, nombre_func: &str) -> Result<(), String> {
        // Verificar que la función existe
        let contexto = self.obtener_contexto()?;
        contexto.verificar_funcion_existe(nombre_func)?;

        // Generar cuádruplo ERA con el nombre de la función
        let cuadruplo = Cuadruplo::new(
            OperadorCuadruplo::Era,
            Operando::Variable(nombre_func.to_string()),
            Operando::Vacio,
            Operando::Vacio,
        );

        self.quad.push_back(cuadruplo);
        Ok(())
    }

    /// Paso 3-4: Generar PARAM
    /// Se invoca cuando se procesa cada expresión en la lista de argumentos
    pub fn generar_param(&mut self, num_param: usize) -> Result<(), String> {
        let operando = self.pilao.pop()
            .ok_or("Error: No hay parámetro para pasar")?;
        self.ptypes.pop();

        // Generar cuádruplo: (param, argumento, -, num_param)
        let cuadruplo = Cuadruplo::new(
            OperadorCuadruplo::Parametro,
            operando.clone(),
            Operando::Vacio,
            Operando::ConstanteEntera(num_param as i32),
        );

        self.quad.push_back(cuadruplo);
        self.liberar_si_temporal(&operando);

        Ok(())
    }

    /// Paso 6: Generar GOSUB
    /// Se invoca cuando se reduce <LLAMADA> completa
    pub fn generar_gosub(&mut self, nombre_func: &str) -> Result<(), String> {
        // Primero verificamos si la función tiene retorno y obtenemos el tipo
        let tipo_retorno_opt = {
            let contexto = self.obtener_contexto()?;
            contexto.obtener_tipo_retorno_funcion(nombre_func).ok()
        };

        // Usar el nombre de la función como operando
        let cuadruplo = Cuadruplo::new(
            OperadorCuadruplo::GoSub,
            Operando::Variable(nombre_func.to_string()),
            Operando::Vacio,
            Operando::Vacio,
        );

        self.quad.push_back(cuadruplo);

        // Si la función tiene retorno, generar temporal para el resultado
        if let Some(tipo_retorno) = tipo_retorno_opt {
            // La función tiene retorno (no es Nula)
            let num_temporal = self.gestor_memoria.siguiente_temporal(tipo_retorno);
            let resultado = Operando::Temporal(num_temporal);

            // Generar cuádruplo: (=, RET, -, temp)
            let cuadruplo_ret = Cuadruplo::new(
                OperadorCuadruplo::Asignacion,
                Operando::Variable("RET".to_string()),
                Operando::Vacio,
                resultado.clone(),
            );

            self.quad.push_back(cuadruplo_ret);

            // Pushear el temporal a la pila de operandos
            self.pilao.push(resultado);
            self.ptypes.push(tipo_retorno);
        }
        // Si es None, la función es Nula (void) - no hacer nada

        Ok(())
    }

    /// Generar ENDFUNC (fin de función)
    /// Se invoca cuando se reduce <FUNCS> completa
    pub fn generar_endfunc(&mut self) -> Result<(), String> {
        let cuadruplo = Cuadruplo::new(
            OperadorCuadruplo::EndFunc,
            Operando::Vacio,
            Operando::Vacio,
            Operando::Vacio,
        );

        self.quad.push_back(cuadruplo);
        Ok(())
    }

    /// Generar RETURN (retorno de función con valor)
    /// Se invoca cuando se procesa un estatuto return
    pub fn generar_return(&mut self) -> Result<(), String> {
        let operando = self.pilao.pop()
            .ok_or("Error: No hay valor de retorno")?;
        let _tipo = self.ptypes.pop();

        // Generar cuádruplo: (=, expresión, -, RET)
        let cuadruplo = Cuadruplo::new(
            OperadorCuadruplo::Asignacion,
            operando.clone(),
            Operando::Vacio,
            Operando::Variable("RET".to_string()),
        );

        self.quad.push_back(cuadruplo);
        self.liberar_si_temporal(&operando);

        // Generar RETURN
        let cuadruplo_return = Cuadruplo::new(
            OperadorCuadruplo::Return,
            Operando::Vacio,
            Operando::Vacio,
            Operando::Vacio,
        );

        self.quad.push_back(cuadruplo_return);

        Ok(())
    }

    // ==================== UTILIDADES ====================

    /// Libera un temporal si el operando es temporal
    fn liberar_si_temporal(&mut self, operando: &Operando) {
        if let Operando::Temporal(num) = operando {
            self.gestor_memoria.liberar_temporal(*num);
        }
    }

    /// Convierte OperadorCuadruplo a Operador del cubo semántico
    fn convertir_operador_semantico(&self, op: OperadorCuadruplo)
        -> Result<crate::semantico::cubo_semantico::Operador, String> {
        use crate::semantico::cubo_semantico::Operador;

        match op {
            OperadorCuadruplo::Suma => Ok(Operador::Suma),
            OperadorCuadruplo::Resta => Ok(Operador::Resta),
            OperadorCuadruplo::Multiplicacion => Ok(Operador::Multiplicacion),
            OperadorCuadruplo::Division => Ok(Operador::Division),
            OperadorCuadruplo::MayorQue => Ok(Operador::MayorQue),
            OperadorCuadruplo::MenorQue => Ok(Operador::MenorQue),
            OperadorCuadruplo::Igual => Ok(Operador::Igual),
            OperadorCuadruplo::Diferente => Ok(Operador::Diferente),
            OperadorCuadruplo::Asignacion => Ok(Operador::Asignacion),
            _ => Err(format!("Operador no soportado para conversión semántica: {:?}", op)),
        }
    }

    /// Obtiene la cola de cuádruplos generados
    pub fn obtener_cuadruplos(&self) -> &VecDeque<Cuadruplo> {
        &self.quad
    }

    /// Obtiene la tabla de strings literales
    pub fn obtener_tabla_strings(&self) -> &Vec<String> {
        &self.tabla_strings
    }

    /// Imprime todos los cuádruplos generados
    pub fn imprimir_cuadruplos(&self) {
        println!("\n╔════════════════════════════════════════════════════════════════╗");
        println!("║                    CÓDIGO INTERMEDIO (CUÁDRUPLOS)              ║");
        println!("╚════════════════════════════════════════════════════════════════╝\n");

        if self.quad.is_empty() {
            println!("  (No se generaron cuádruplos)\n");
            return;
        }

        println!("  {:>4}  {}", "Num", "Cuádruplo");
        println!("  {}", "─".repeat(60));

        for (i, cuadruplo) in self.quad.iter().enumerate() {
            println!("  {:>4}: {}", i, cuadruplo);
        }

        println!("\n  Total de cuádruplos: {}", self.quad.len());
        println!("  Temporales usados: {}\n", self.gestor_memoria.total_temporales());
    }

    /// Reinicia el generador (limpia todas las estructuras)
    pub fn reiniciar(&mut self) {
        self.poper.clear();
        self.pilao.clear();
        self.ptypes.clear();
        self.quad.clear();
        self.pjumps.clear();
        self.gestor_memoria.reiniciar();
    }

    /// Obtiene el estado de las pilas (para debugging)
    pub fn estado_pilas(&self) -> String {
        format!(
            "POper: {:?}\nPilaO: {:?}\nPTypes: {:?}",
            self.poper, self.pilao, self.ptypes
        )
    }

    /// Exporta el programa compilado como ProgramaObjeto
    /// Este método crea el "binario" listo para la VM
    pub fn exportar_programa(&self, nombre_programa: String) -> Result<crate::intermedio::programa::ProgramaObjeto, String> {
        use crate::intermedio::programa::{ProgramaObjeto, InfoFuncionPrograma};
        use std::collections::HashMap;
        use crate::vm::memoria::Valor;

        // Construir mapa de funciones desde los metadatos recopilados
        let mut mapa_funciones = HashMap::new();

        // Si no hay funciones en metadatos (programa simple sin funciones declaradas),
        // crear entrada para "main" por defecto
        if self.metadatos_funciones.is_empty() {
            mapa_funciones.insert("main".to_string(), InfoFuncionPrograma {
                nombre: "main".to_string(),
                direccion_inicio: 0,
                tiene_retorno: false,
                tipo_retorno: None,
                num_parametros: 0,
            });
        } else {
            // Usar los metadatos recopilados durante la compilación
            for (nombre, metadatos) in &self.metadatos_funciones {
                let tipo_retorno_str = metadatos.tipo_retorno.as_ref()
                    .map(|t| format!("{:?}", t));

                mapa_funciones.insert(nombre.clone(), InfoFuncionPrograma {
                    nombre: nombre.clone(),
                    direccion_inicio: metadatos.direccion_inicio,
                    tiene_retorno: metadatos.tiene_retorno,
                    tipo_retorno: tipo_retorno_str,
                    num_parametros: metadatos.parametros.len(),
                });
            }

            // Asegurarse de que "main" existe (puede ser el programa principal)
            if !mapa_funciones.contains_key("main") {
                mapa_funciones.insert("main".to_string(), InfoFuncionPrograma {
                    nombre: "main".to_string(),
                    direccion_inicio: 0,
                    tiene_retorno: false,
                    tipo_retorno: None,
                    num_parametros: 0,
                });
            }
        }

        // Construir mapa de constantes escaneando cuádruplos
        let mut mapa_constantes = HashMap::new();
        let mut siguiente_dir_constante = 15000; // Segmento de constantes empieza en 15000

        for cuadruplo in &self.quad {
            // Función auxiliar para agregar constante si no existe
            let mut agregar_constante = |valor: Valor| {
                // Verificar si ya existe
                let existe = mapa_constantes.values().any(|v| match (v, &valor) {
                    (Valor::Entero(a), Valor::Entero(b)) => a == b,
                    (Valor::Flotante(a), Valor::Flotante(b)) => (a - b).abs() < f64::EPSILON,
                    _ => false,
                });

                if !existe {
                    mapa_constantes.insert(siguiente_dir_constante, valor);
                    siguiente_dir_constante += 1;
                }
            };

            // Revisar operando izquierdo
            match &cuadruplo.operando_izq {
                Operando::ConstanteEntera(v) => agregar_constante(Valor::Entero(*v)),
                Operando::ConstanteFlotante(v) => agregar_constante(Valor::Flotante(*v)),
                _ => {}
            }

            // Revisar operando derecho
            match &cuadruplo.operando_der {
                Operando::ConstanteEntera(v) => agregar_constante(Valor::Entero(*v)),
                Operando::ConstanteFlotante(v) => agregar_constante(Valor::Flotante(*v)),
                _ => {}
            }

            // Revisar resultado (puede tener constantes en casos especiales)
            match &cuadruplo.resultado {
                Operando::ConstanteEntera(v) => agregar_constante(Valor::Entero(*v)),
                Operando::ConstanteFlotante(v) => agregar_constante(Valor::Flotante(*v)),
                _ => {}
            }
        }

        // Crear programa objeto
        let programa = ProgramaObjeto::crear(
            nombre_programa,
            self.quad.iter().cloned().collect(),
            mapa_funciones,
            mapa_constantes,
            self.tabla_strings.clone(),
        );

        Ok(programa)
    }
}

impl Default for GeneradorCuadruplos {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_procesar_constantes() {
        let mut generador = GeneradorCuadruplos::new();

        // Constante entera
        assert!(generador.procesar_operando("42").is_ok());
        assert_eq!(generador.pilao.len(), 1);
        assert_eq!(generador.ptypes.len(), 1);

        // Constante flotante
        assert!(generador.procesar_operando("3.14").is_ok());
        assert_eq!(generador.pilao.len(), 2);
        assert_eq!(generador.ptypes.len(), 2);
    }

    #[test]
    fn test_operadores() {
        let mut generador = GeneradorCuadruplos::new();

        assert!(generador.procesar_suma_resta("+").is_ok());
        assert_eq!(generador.poper.len(), 1);

        assert!(generador.procesar_mult_div("*").is_ok());
        assert_eq!(generador.poper.len(), 2);

        assert!(generador.procesar_relacional(">").is_ok());
        assert_eq!(generador.poper.len(), 3);
    }

    #[test]
    fn test_parentesis() {
        let mut generador = GeneradorCuadruplos::new();

        generador.abrir_parentesis();
        assert_eq!(generador.poper.len(), 1);

        assert!(generador.cerrar_parentesis().is_ok());
        assert_eq!(generador.poper.len(), 0);
    }
}
