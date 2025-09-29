// tests_stack.rs - Tests unitarios para la estructura de datos Stack
use super::stack::*;

// ==================== TESTS BÁSICOS ====================

#[test]
fn test_stack_nuevo_vacio() {
    let stack: Stack<i32> = Stack::nuevo();
    assert!(stack.esta_vacio());
    assert_eq!(stack.tamanio(), 0);
    assert!(!stack.esta_lleno());
    assert_eq!(stack.capacidad_maxima(), None);
}

#[test]
fn test_stack_con_capacidad() {
    let stack: Stack<i32> = Stack::nuevo_con_capacidad(5);
    assert!(stack.esta_vacio());
    assert_eq!(stack.tamanio(), 0);
    assert!(!stack.esta_lleno());
    assert_eq!(stack.capacidad_maxima(), Some(5));
}

#[test]
fn test_push_y_pop_basico() {
    let mut stack = Stack::nuevo();

    // Push elementos
    assert_eq!(stack.push(1), Ok(()));
    assert_eq!(stack.push(2), Ok(()));
    assert_eq!(stack.push(3), Ok(()));

    assert!(!stack.esta_vacio());
    assert_eq!(stack.tamanio(), 3);

    // Pop elementos en orden LIFO
    assert_eq!(stack.pop(), Ok(3));
    assert_eq!(stack.pop(), Ok(2));
    assert_eq!(stack.pop(), Ok(1));

    assert!(stack.esta_vacio());
    assert_eq!(stack.tamanio(), 0);
}

// ==================== TESTS DE CAPACIDAD LIMITADA ====================

#[test]
fn test_capacidad_maxima_respetada() {
    let mut stack = Stack::nuevo_con_capacidad(2);

    // Llenar hasta capacidad máxima
    assert_eq!(stack.push("A"), Ok(()));
    assert_eq!(stack.push("B"), Ok(()));
    assert!(stack.esta_lleno());

    // Intentar exceder capacidad
    assert_eq!(stack.push("C"), Err(ErrorStack::CapacidadExcedida));
    assert_eq!(stack.tamanio(), 2);
}

#[test]
fn test_capacidad_liberada_al_pop() {
    let mut stack = Stack::nuevo_con_capacidad(2);

    // Llenar stack
    stack.push(1).unwrap();
    stack.push(2).unwrap();
    assert!(stack.esta_lleno());

    // Pop y verificar que se puede agregar nuevamente
    assert_eq!(stack.pop(), Ok(2));
    assert!(!stack.esta_lleno());
    assert_eq!(stack.push(3), Ok(()));
}

// ==================== TESTS DE ERRORES ====================

#[test]
fn test_pop_stack_vacio() {
    let mut stack: Stack<i32> = Stack::nuevo();
    assert_eq!(stack.pop(), Err(ErrorStack::StackVacio));
}

#[test]
fn test_peek_stack_vacio() {
    let stack: Stack<i32> = Stack::nuevo();
    assert_eq!(stack.peek(), Err(ErrorStack::StackVacio));
}

#[test]
fn test_obtener_indice_invalido() {
    let stack: Stack<i32> = Stack::nuevo();
    assert_eq!(stack.obtener(0), Err(ErrorStack::IndiceInvalido));

    let mut stack_con_elementos = Stack::nuevo();
    stack_con_elementos.push(1).unwrap();
    stack_con_elementos.push(2).unwrap();

    assert_eq!(stack_con_elementos.obtener(0), Ok(&1)); // fondo del stack
    assert_eq!(stack_con_elementos.obtener(1), Ok(&2)); // tope del stack
    assert_eq!(stack_con_elementos.obtener(2), Err(ErrorStack::IndiceInvalido));
}

// ==================== TESTS DE VISUALIZACIÓN ====================

#[test]
fn test_peek() {
    let mut stack = Stack::nuevo();
    stack.push("primero").unwrap();
    stack.push("segundo").unwrap();
    stack.push("tercero").unwrap();

    assert_eq!(stack.peek(), Ok(&"tercero")); // el último agregado está en el tope

    // Verificar que no se modificó el stack
    assert_eq!(stack.tamanio(), 3);
    assert_eq!(stack.pop(), Ok("tercero"));
}

#[test]
fn test_obtener_por_indice() {
    let mut stack = Stack::nuevo();
    stack.push(10).unwrap();
    stack.push(20).unwrap();
    stack.push(30).unwrap();

    assert_eq!(stack.obtener(0), Ok(&10)); // fondo
    assert_eq!(stack.obtener(1), Ok(&20)); // medio
    assert_eq!(stack.obtener(2), Ok(&30)); // tope
}

// ==================== TESTS DE OPERACIONES AVANZADAS ====================

#[test]
fn test_push_multiples() {
    let mut stack = Stack::nuevo_con_capacidad(5);
    let elementos = vec![1, 2, 3, 4];

    assert_eq!(stack.push_multiples(elementos), Ok(()));
    assert_eq!(stack.tamanio(), 4);
    assert_eq!(stack.peek(), Ok(&4)); // el último agregado

    // Verificar orden LIFO
    assert_eq!(stack.pop(), Ok(4));
    assert_eq!(stack.pop(), Ok(3));
    assert_eq!(stack.pop(), Ok(2));
    assert_eq!(stack.pop(), Ok(1));
}

#[test]
fn test_push_multiples_excede_capacidad() {
    let mut stack = Stack::nuevo_con_capacidad(3);
    let elementos = vec![1, 2, 3, 4, 5];

    assert_eq!(stack.push_multiples(elementos), Err(ErrorStack::CapacidadExcedida));
    assert_eq!(stack.tamanio(), 0); // No se agregó nada
}

#[test]
fn test_pop_multiples() {
    let mut stack = Stack::nuevo();
    for i in 1..=5 {
        stack.push(i).unwrap();
    }

    let resultado = stack.pop_multiples(3);
    assert_eq!(resultado, Ok(vec![5, 4, 3])); // orden LIFO
    assert_eq!(stack.tamanio(), 2);
    assert_eq!(stack.peek(), Ok(&2));
}

#[test]
fn test_pop_multiples_insuficientes() {
    let mut stack = Stack::nuevo();
    stack.push(1).unwrap();
    stack.push(2).unwrap();

    assert_eq!(stack.pop_multiples(5), Err(ErrorStack::StackVacio));
    assert_eq!(stack.tamanio(), 2); // No se modificó
}

// ==================== TESTS DE GESTIÓN DE MEMORIA ====================

#[test]
fn test_limpiar_stack() {
    let mut stack = Stack::nuevo();
    stack.push(1).unwrap();
    stack.push(2).unwrap();
    stack.push(3).unwrap();

    assert_eq!(stack.tamanio(), 3);
    stack.limpiar();
    assert_eq!(stack.tamanio(), 0);
    assert!(stack.esta_vacio());
}

#[test]
fn test_reservar_capacidad() {
    let mut stack: Stack<i32> = Stack::nuevo_con_capacidad(10);
    assert_eq!(stack.reservar(5), Ok(()));

    // Intentar reservar más de la capacidad máxima
    assert_eq!(stack.reservar(50), Err(ErrorStack::CapacidadExcedida));
}

#[test]
fn test_reservar_sin_limite() {
    let mut stack: Stack<i32> = Stack::nuevo();
    assert_eq!(stack.reservar(100), Ok(()));
    assert_eq!(stack.reservar(1000), Ok(()));
}

#[test]
fn test_a_vector() {
    let mut stack = Stack::nuevo();
    stack.push("a").unwrap();
    stack.push("b").unwrap();
    stack.push("c").unwrap();

    let vector = stack.a_vector();
    assert_eq!(vector, vec!["a", "b", "c"]); // orden de inserción en el vector
}

#[test]
fn test_capacidad_vec() {
    let mut stack: Stack<i32> = Stack::nuevo();

    // Capacidad inicial debería ser 0
    assert_eq!(stack.capacidad(), 0);

    // Agregar elementos para que Vec asigne capacidad
    for i in 1..=10 {
        stack.push(i).unwrap();
    }

    // Ahora debería tener capacidad > 0
    assert!(stack.capacidad() > 0);
}

#[test]
fn test_reducir_capacidad() {
    let mut stack = Stack::nuevo();

    // Llenar el stack para que Vec asigne mucha capacidad
    for i in 1..=100 {
        stack.push(i).unwrap();
    }

    let capacidad_inicial = stack.capacidad();
    assert!(capacidad_inicial >= 100);

    // Limpiar el stack pero mantener capacidad
    stack.limpiar();
    assert_eq!(stack.capacidad(), capacidad_inicial);

    // Reducir capacidad
    stack.reducir_capacidad();
    assert!(stack.capacidad() < capacidad_inicial);
}

// ==================== TESTS DE ITERACIÓN ====================

#[test]
fn test_iterador() {
    let mut stack = Stack::nuevo();
    stack.push(1).unwrap();
    stack.push(2).unwrap();
    stack.push(3).unwrap();

    let elementos: Vec<&i32> = stack.iter().collect();
    assert_eq!(elementos, vec![&1, &2, &3]); // orden desde el fondo
}

#[test]
fn test_iterador_reverso() {
    let mut stack = Stack::nuevo();
    stack.push(1).unwrap();
    stack.push(2).unwrap();
    stack.push(3).unwrap();

    let elementos: Vec<&i32> = stack.iter_reverso().collect();
    assert_eq!(elementos, vec![&3, &2, &1]); // orden LIFO (desde el tope)
}

// ==================== TESTS DE TIPOS DIFERENTES ====================

#[test]
fn test_stack_strings() {
    let mut stack = Stack::nuevo();
    stack.push("Hola".to_string()).unwrap();
    stack.push("Mundo".to_string()).unwrap();

    assert_eq!(stack.pop(), Ok("Mundo".to_string())); // último en entrar, primero en salir
    assert_eq!(stack.pop(), Ok("Hola".to_string()));
}

#[test]
fn test_stack_estructuras_complejas() {
    #[derive(Debug, PartialEq, Clone)]
    struct Persona {
        nombre: String,
        edad: u32,
    }

    let mut stack = Stack::nuevo();
    let persona1 = Persona { nombre: "Ana".to_string(), edad: 25 };
    let persona2 = Persona { nombre: "Bob".to_string(), edad: 30 };

    stack.push(persona1.clone()).unwrap();
    stack.push(persona2.clone()).unwrap();

    assert_eq!(stack.pop(), Ok(persona2)); // LIFO
    assert_eq!(stack.pop(), Ok(persona1));
}

// ==================== TESTS DE ESCENARIOS EXTREMOS ====================

#[test]
fn test_operaciones_repetidas() {
    let mut stack = Stack::nuevo_con_capacidad(3);

    // Ciclo de llenar y vaciar múltiples veces
    for ciclo in 0..5 {
        // Llenar
        for i in 1..=3 {
            assert_eq!(stack.push(ciclo * 10 + i), Ok(()));
        }
        assert!(stack.esta_lleno());

        // Vaciar en orden LIFO
        for i in (1..=3).rev() {
            assert_eq!(stack.pop(), Ok(ciclo * 10 + i));
        }
        assert!(stack.esta_vacio());
    }
}

#[test]
fn test_stack_capacidad_uno() {
    let mut stack = Stack::nuevo_con_capacidad(1);

    assert_eq!(stack.push(42), Ok(()));
    assert!(stack.esta_lleno());
    assert_eq!(stack.push(99), Err(ErrorStack::CapacidadExcedida));

    assert_eq!(stack.pop(), Ok(42));
    assert!(stack.esta_vacio());
}

#[test]
fn test_comportamiento_lifo() {
    let mut stack = Stack::nuevo();

    // Agregar secuencia conocida
    let secuencia = vec![10, 20, 30, 40, 50];
    for &elemento in &secuencia {
        stack.push(elemento).unwrap();
    }

    // Verificar que sale en orden LIFO
    let mut secuencia_lifo = secuencia.clone();
    secuencia_lifo.reverse();

    for &esperado in &secuencia_lifo {
        assert_eq!(stack.pop(), Ok(esperado));
    }
}

// ==================== TESTS DE ERROR DISPLAY ====================

#[test]
fn test_error_display() {
    let error_vacio = ErrorStack::StackVacio;
    let error_capacidad = ErrorStack::CapacidadExcedida;
    let error_indice = ErrorStack::IndiceInvalido;

    assert_eq!(format!("{}", error_vacio), "Error: El stack está vacío");
    assert_eq!(format!("{}", error_capacidad), "Error: Se ha excedido la capacidad máxima del stack");
    assert_eq!(format!("{}", error_indice), "Error: Índice fuera de rango");
}

#[test]
fn test_error_debug() {
    let error = ErrorStack::StackVacio;
    assert_eq!(format!("{:?}", error), "StackVacio");
}

#[test]
fn test_error_equality() {
    assert_eq!(ErrorStack::StackVacio, ErrorStack::StackVacio);
    assert_eq!(ErrorStack::CapacidadExcedida, ErrorStack::CapacidadExcedida);
    assert_eq!(ErrorStack::IndiceInvalido, ErrorStack::IndiceInvalido);

    assert_ne!(ErrorStack::StackVacio, ErrorStack::CapacidadExcedida);
}

// ==================== TESTS DE COMPARACIÓN CON COLA ====================

#[test]
fn test_diferencia_con_cola_orden() {
    // Demostrar que Stack es LIFO vs Cola que es FIFO
    let mut stack = Stack::nuevo();

    // Agregar misma secuencia que usaríamos en una cola
    stack.push(1).unwrap();
    stack.push(2).unwrap();
    stack.push(3).unwrap();

    // Stack: último en entrar, primero en salir
    assert_eq!(stack.pop(), Ok(3));
    assert_eq!(stack.pop(), Ok(2));
    assert_eq!(stack.pop(), Ok(1));
}

#[test]
fn test_peek_vs_frente() {
    let mut stack = Stack::nuevo();
    stack.push("primero").unwrap();
    stack.push("ultimo").unwrap();

    // En Stack, peek devuelve el último elemento agregado (tope)
    assert_eq!(stack.peek(), Ok(&"ultimo"));

    // Verificar que peek no modifica el stack
    assert_eq!(stack.tamanio(), 2);
    assert_eq!(stack.peek(), Ok(&"ultimo"));
}