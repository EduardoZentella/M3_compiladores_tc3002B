//! # Ejecutor
//!
//! Máquina Virtual que ejecuta cuádruplos.

use std::collections::HashMap;
use crate::intermedio::cuadruplo::{Cuadruplo, OperadorCuadruplo, Operando};
use crate::intermedio::programa::ProgramaObjeto;
use super::memoria::{
    SegmentoMemoria, MarcoMemoria, Valor, TipoSegmento,
    traducir_direccion
};
use super::io::SistemaIO;

/// Información de una función en la tabla de funciones
#[derive(Debug, Clone)]
pub struct InfoFuncion {
    pub nombre: String,
    pub direccion_inicio: usize,
    pub tiene_retorno: bool,
}

/// Máquina Virtual - Ejecuta código intermedio
pub struct MaquinaVirtual {
    /// Cuádruplos a ejecutar
    cuadruplos: Vec<Cuadruplo>,

    /// Instruction Pointer
    ip: usize,

    /// Memoria global (permanente durante toda la ejecución)
    memoria_global: SegmentoMemoria,

    /// Memoria de constantes (permanente)
    memoria_constantes: SegmentoMemoria,

    /// Pila de marcos de memoria (stack frames)
    pila_marcos: Vec<MarcoMemoria>,

    /// Tabla de funciones (nombre -> dirección de inicio)
    tabla_funciones: HashMap<String, InfoFuncion>,

    /// Marco temporal para parámetros (antes de GoSub)
    marco_temporal: Option<MarcoMemoria>,

    /// Flag de ejecución
    ejecutando: bool,

    /// Sistema de E/S (inyectado)
    io: Box<dyn SistemaIO>,

    /// Tabla de strings literales (letreros)
    tabla_strings: Vec<String>,
}

impl MaquinaVirtual {
    /// Crea una nueva máquina virtual con sistema de E/S inyectado
    pub fn new(io: Box<dyn SistemaIO>) -> Self {
        MaquinaVirtual {
            cuadruplos: Vec::new(),
            ip: 0,
            memoria_global: SegmentoMemoria::new(),
            memoria_constantes: SegmentoMemoria::new(),
            pila_marcos: Vec::new(),
            tabla_funciones: HashMap::new(),
            marco_temporal: None,
            ejecutando: true,
            io,
            tabla_strings: Vec::new(),
        }
    }

    /// Carga un programa objeto en la VM
    pub fn cargar_programa(&mut self, programa: ProgramaObjeto) -> Result<(), String> {
        // Cargar cuádruplos
        self.cuadruplos = programa.cuadruplos;

        // Cargar tabla de funciones
        self.tabla_funciones.clear();
        for (nombre, info) in programa.mapa_funciones {
            self.tabla_funciones.insert(nombre, InfoFuncion {
                nombre: info.nombre,
                direccion_inicio: info.direccion_inicio,
                tiene_retorno: info.tiene_retorno,
            });
        }

        // Cargar constantes en memoria
        for (direccion, valor) in programa.mapa_constantes {
            self.cargar_constante(direccion, valor)?;
        }

        // Cargar tabla de strings
        self.tabla_strings = programa.tabla_strings;

        Ok(())
    }

    /// Ejecuta todos los cuádruplos
    pub fn ejecutar(&mut self) -> Result<(), String> {
        // Verificar que hay un programa cargado
        if self.cuadruplos.is_empty() {
            return Err("No hay programa cargado".to_string());
        }

        // Crear marco inicial para el programa principal
        self.pila_marcos.push(MarcoMemoria::new("main".to_string(), 0));

        while self.ejecutando && self.ip < self.cuadruplos.len() {
            let cuadruplo = &self.cuadruplos[self.ip].clone();
            self.ejecutar_cuadruplo(cuadruplo)?;
            self.ip += 1;
        }

        Ok(())
    }    /// Ejecuta un solo cuádruplo
    fn ejecutar_cuadruplo(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
        match cuad.operador {
            OperadorCuadruplo::Suma | OperadorCuadruplo::Resta |
            OperadorCuadruplo::Multiplicacion | OperadorCuadruplo::Division => {
                self.ejecutar_aritmetica(cuad)?;
            },
            OperadorCuadruplo::MayorQue | OperadorCuadruplo::MenorQue |
            OperadorCuadruplo::Diferente | OperadorCuadruplo::Igual => {
                self.ejecutar_relacional(cuad)?;
            },
            OperadorCuadruplo::Asignacion => {
                self.ejecutar_asignacion(cuad)?;
            },
            OperadorCuadruplo::Goto => {
                self.ejecutar_goto(cuad)?;
            },
            OperadorCuadruplo::GotoF => {
                self.ejecutar_gotof(cuad)?;
            },
            OperadorCuadruplo::GotoV => {
                self.ejecutar_gotov(cuad)?;
            },
            OperadorCuadruplo::Escritura => {
                self.ejecutar_escribe(cuad)?;
            },
            OperadorCuadruplo::Lectura => {
                self.ejecutar_lectura(cuad)?;
            },
            OperadorCuadruplo::Era => {
                self.ejecutar_era(cuad)?;
            },
            OperadorCuadruplo::Parametro => {
                self.ejecutar_parametro(cuad)?;
            },
            OperadorCuadruplo::GoSub => {
                self.ejecutar_gosub(cuad)?;
            },
            OperadorCuadruplo::EndFunc => {
                self.ejecutar_endfunc(cuad)?;
            },
            OperadorCuadruplo::Return => {
                self.ejecutar_return(cuad)?;
            },
        }

        Ok(())
    }    /// Extrae dirección de memoria desde un Operando
    fn extraer_direccion(&self, operando: &Operando) -> Result<usize, String> {
        match operando {
            Operando::Direccion(dir) => Ok(*dir),
            Operando::Temporal(num) => {
                // Los temporales están en el rango TEMPORAL_INICIO + num
                Ok(10000 + num)
            },
            Operando::ConstanteEntera(_) | Operando::ConstanteFlotante(_) => {
                Err("Se esperaba dirección, se encontró constante literal".to_string())
            },
            Operando::Variable(nombre) => {
                Err(format!("No se puede resolver variable '{}' a dirección", nombre))
            },
            Operando::Etiqueta(dir) => Ok(*dir),
            Operando::Vacio | Operando::Pendiente => {
                Err("Operando vacío o pendiente no tiene dirección".to_string())
            },
            Operando::Letrero(_) => {
                Err("Los letreros no tienen dirección de memoria".to_string())
            },
        }
    }

    /// Lee valor desde Operando (puede ser dirección o constante literal)
    fn leer_operando(&self, operando: &Operando) -> Result<Valor, String> {
        match operando {
            Operando::ConstanteEntera(v) => Ok(Valor::Entero(*v)),
            Operando::ConstanteFlotante(v) => Ok(Valor::Flotante(*v)),
            Operando::Letrero(idx) => Ok(Valor::Letrero(*idx)),
            _ => {
                let dir = self.extraer_direccion(operando)?;
                self.leer_memoria(dir)
            }
        }
    }

    /// Ejecuta operación aritmética: resultado = op1 OPERADOR op2
    fn ejecutar_aritmetica(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
        let valor1 = self.leer_operando(&cuad.operando_izq)?;
        let valor2 = self.leer_operando(&cuad.operando_der)?;
        let resultado_dir = self.extraer_direccion(&cuad.resultado)?;

        // Aplicar operación
        let resultado = match cuad.operador {
            OperadorCuadruplo::Suma => valor1.operar_aritmetica(&valor2, |a, b| a + b),
            OperadorCuadruplo::Resta => valor1.operar_aritmetica(&valor2, |a, b| a - b),
            OperadorCuadruplo::Multiplicacion => valor1.operar_aritmetica(&valor2, |a, b| a * b),
            OperadorCuadruplo::Division => {
                if valor2.a_flotante() == 0.0 {
                    return Err("Error: División por cero".to_string());
                }
                valor1.operar_aritmetica(&valor2, |a, b| a / b)
            },
            _ => return Err(format!("Operador aritmético no reconocido: {:?}", cuad.operador)),
        };

        // Escribir resultado
        self.escribir_memoria(resultado_dir, resultado)?;

        Ok(())
    }

    /// Ejecuta operación relacional: resultado = op1 COMPARADOR op2
    fn ejecutar_relacional(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
        let valor1 = self.leer_operando(&cuad.operando_izq)?;
        let valor2 = self.leer_operando(&cuad.operando_der)?;
        let resultado_dir = self.extraer_direccion(&cuad.resultado)?;

        let resultado = match cuad.operador {
            OperadorCuadruplo::MayorQue => valor1.operar_relacional(&valor2, |a, b| a > b),
            OperadorCuadruplo::MenorQue => valor1.operar_relacional(&valor2, |a, b| a < b),
            OperadorCuadruplo::Diferente => valor1.operar_relacional(&valor2, |a, b| (a - b).abs() > f64::EPSILON),
            OperadorCuadruplo::Igual => valor1.operar_relacional(&valor2, |a, b| (a - b).abs() <= f64::EPSILON),
            _ => return Err(format!("Operador relacional no reconocido: {:?}", cuad.operador)),
        };

        self.escribir_memoria(resultado_dir, resultado)?;

        Ok(())
    }

    /// Ejecuta asignación: destino = fuente
    fn ejecutar_asignacion(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
        let valor = self.leer_operando(&cuad.operando_izq)?;
        let destino_dir = self.extraer_direccion(&cuad.resultado)?;

        self.escribir_memoria(destino_dir, valor)?;

        Ok(())
    }

    /// Ejecuta salto incondicional
    fn ejecutar_goto(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
        let destino = self.extraer_direccion(&cuad.resultado)?;

        self.ip = destino;
        self.ip -= 1; // Compensar el incremento automático del loop

        Ok(())
    }

    /// Ejecuta salto condicional si es falso
    fn ejecutar_gotof(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
        let condicion = self.leer_operando(&cuad.operando_izq)?;
        let destino = self.extraer_direccion(&cuad.resultado)?;

        // Si la condición es 0 (falso), saltar
        if condicion.a_entero() == 0 {
            self.ip = destino;
            self.ip -= 1; // Compensar incremento automático
        }

        Ok(())
    }

    /// Ejecuta salto condicional si es verdadero
    fn ejecutar_gotov(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
        let condicion = self.leer_operando(&cuad.operando_izq)?;
        let destino = self.extraer_direccion(&cuad.resultado)?;

        // Si la condición es distinta de 0 (verdadero), saltar
        if condicion.a_entero() != 0 {
            self.ip = destino;
            self.ip -= 1; // Compensar incremento automático
        }

        Ok(())
    }

    /// Ejecuta escritura a consola
    fn ejecutar_escribe(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
        let valor = self.leer_operando(&cuad.resultado)?;

        let msg = match valor {
            Valor::Entero(v) => v.to_string(),
            Valor::Flotante(v) => v.to_string(),
            Valor::Letrero(idx) => {
                // Buscar el string en la tabla
                self.tabla_strings.get(idx)
                    .ok_or(format!("Error: Índice de string inválido: {}", idx))?
                    .trim_matches('"')
                    .to_string()
            },
        };

        self.io.escribir_linea(&msg);

        Ok(())
    }

    /// Ejecuta lectura desde entrada
    fn ejecutar_lectura(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
        let destino_dir = self.extraer_direccion(&cuad.resultado)?;

        // Leer línea de entrada
        let linea = self.io.leer_linea()?;

        // Intentar parsear como entero primero
        if let Ok(valor_entero) = linea.parse::<i32>() {
            self.escribir_memoria(destino_dir, Valor::Entero(valor_entero))?;
            return Ok(());
        }

        // Intentar parsear como flotante
        if let Ok(valor_flotante) = linea.parse::<f64>() {
            self.escribir_memoria(destino_dir, Valor::Flotante(valor_flotante))?;
            return Ok(());
        }

        Err(format!("No se pudo convertir '{}' a número", linea))
    }    /// Ejecuta ERA: crea registro de activación temporal
    fn ejecutar_era(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
        let nombre_funcion = match &cuad.operando_izq {
            Operando::Variable(nombre) => nombre.clone(),
            _ => return Err("Era requiere nombre de función como operando".to_string()),
        };

        // Crear marco temporal para la función que será llamada
        self.marco_temporal = Some(MarcoMemoria::new(nombre_funcion, self.ip + 1));

        Ok(())
    }

    /// Ejecuta PARAMETRO: copia parámetro al marco temporal
    fn ejecutar_parametro(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
        let valor = self.leer_operando(&cuad.operando_izq)?;

        let param_num = match &cuad.resultado {
            Operando::ConstanteEntera(n) => *n as usize,
            _ => return Err("Parametro requiere número de parámetro en resultado".to_string()),
        };

        // Agregarlo al marco temporal
        if let Some(ref mut marco) = self.marco_temporal {
            // Escribir en la dirección local correspondiente del marco temporal
            let offset = param_num;
            marco.memoria_local.escribir_valor(offset, valor);
        } else {
            return Err("No hay marco temporal (falta Era antes de Parametro)".to_string());
        }

        Ok(())
    }

    /// Ejecuta GOSUB: invoca función (push marco a pila y salta)
    fn ejecutar_gosub(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
        let nombre_funcion = match &cuad.operando_izq {
            Operando::Variable(nombre) => nombre.clone(),
            _ => return Err("GoSub requiere nombre de función".to_string()),
        };

        // Obtener información de la función
        let info = self.tabla_funciones.get(&nombre_funcion)
            .ok_or_else(|| format!("Función '{}' no encontrada", nombre_funcion))?
            .clone();

        // Guardar si hay destino para valor de retorno
        let destino_retorno = if !matches!(cuad.resultado, Operando::Vacio) {
            Some(cuad.resultado.clone())
        } else {
            None
        };

        // Tomar el marco temporal y pushearlo a la pila
        let marco = self.marco_temporal.take()
            .ok_or("No hay marco temporal (falta Era antes de GoSub)")?;

        self.pila_marcos.push(marco);

        // Saltar a la dirección de inicio de la función
        let ip_retorno = self.ip + 1;
        self.ip = info.direccion_inicio;

        // Ejecutar hasta encontrar ENDFUNC o RETURN
        while self.ip < self.cuadruplos.len() {
            let cuad_actual = &self.cuadruplos[self.ip].clone();

            // Si es ENDFUNC, salir del loop
            if matches!(cuad_actual.operador, OperadorCuadruplo::EndFunc) {
                break;
            }

            self.ejecutar_cuadruplo(cuad_actual)?;
            self.ip += 1;
        }

        // Pop el marco de la función
        let marco_funcion = self.pila_marcos.pop()
            .ok_or("No hay marco para hacer pop después de GOSUB")?;

        // Si hay valor de retorno y destino, copiarlo
        if let (Some(destino), Some(valor)) = (destino_retorno, marco_funcion.valor_retorno) {
            let destino_dir = self.extraer_direccion(&destino)?;
            self.escribir_memoria(destino_dir, valor)?;
        }

        // Restaurar IP
        self.ip = ip_retorno;
        self.ip -= 1; // Compensar incremento automático

        Ok(())
    }

    /// Ejecuta ENDFUNC: retorna de función (pop marco de pila)
    fn ejecutar_endfunc(&mut self, _cuad: &Cuadruplo) -> Result<(), String> {
        // Pop el marco actual
        let marco_actual = self.pila_marcos.pop()
            .ok_or("No hay marco para hacer pop (pila vacía)")?;

        // Si es el marco principal, terminar ejecución
        if self.pila_marcos.is_empty() {
            self.ejecutando = false;
            return Ok(());
        }

        // Restaurar IP al punto de retorno
        self.ip = marco_actual.ip_retorno;

        Ok(())
    }

    /// Ejecuta RETURN: guarda valor de retorno para que el llamador lo use
    fn ejecutar_return(&mut self, cuad: &Cuadruplo) -> Result<(), String> {
        // Si hay un operando, es el temporal con el valor de retorno
        if !matches!(cuad.operando_izq, Operando::Vacio) {
            let valor = self.leer_operando(&cuad.operando_izq)?;

            // Guardar en el marco actual para que GOSUB lo recupere
            if let Some(marco) = self.pila_marcos.last_mut() {
                marco.valor_retorno = Some(valor);
            }
        }

        // El RETURN no cambia el IP, solo marca que hay un valor
        // El pop del marco lo hace ENDFUNC

        Ok(())
    }

    /// Lee un valor de memoria (resuelve segmento y offset)
    fn leer_memoria(&self, dir: usize) -> Result<Valor, String> {
        let (tipo_segmento, offset) = traducir_direccion(dir)?;

        match tipo_segmento {
            TipoSegmento::Global => {
                // Asumimos que son enteros por defecto, ajustar según necesidad
                self.memoria_global.leer_valor(offset, false)
            },
            TipoSegmento::Local => {
                let marco = self.pila_marcos.last()
                    .ok_or("No hay marco activo para leer memoria local")?;
                marco.memoria_local.leer_valor(offset, false)
            },
            TipoSegmento::Temporal => {
                let marco = self.pila_marcos.last()
                    .ok_or("No hay marco activo para leer memoria temporal")?;
                marco.memoria_temporal.leer_valor(offset, false)
            },
            TipoSegmento::Constante => {
                self.memoria_constantes.leer_valor(offset, false)
            },
        }
    }

    /// Escribe un valor en memoria (resuelve segmento y offset)
    fn escribir_memoria(&mut self, dir: usize, valor: Valor) -> Result<(), String> {
        let (tipo_segmento, offset) = traducir_direccion(dir)?;

        match tipo_segmento {
            TipoSegmento::Global => {
                self.memoria_global.escribir_valor(offset, valor);
            },
            TipoSegmento::Local => {
                let marco = self.pila_marcos.last_mut()
                    .ok_or("No hay marco activo para escribir memoria local")?;
                marco.memoria_local.escribir_valor(offset, valor);
            },
            TipoSegmento::Temporal => {
                let marco = self.pila_marcos.last_mut()
                    .ok_or("No hay marco activo para escribir memoria temporal")?;
                marco.memoria_temporal.escribir_valor(offset, valor);
            },
            TipoSegmento::Constante => {
                // No se debería escribir en constantes, pero lo permitimos por flexibilidad
                self.memoria_constantes.escribir_valor(offset, valor);
            },
        }

        Ok(())
    }

    /// Carga una constante en memoria (llamar antes de ejecutar)
    pub fn cargar_constante(&mut self, direccion: usize, valor: Valor) -> Result<(), String> {
        let (tipo_segmento, offset) = traducir_direccion(direccion)?;

        if tipo_segmento != TipoSegmento::Constante {
            return Err("Solo se pueden cargar constantes en el segmento de constantes".to_string());
        }

        self.memoria_constantes.escribir_valor(offset, valor);
        Ok(())
    }
}
