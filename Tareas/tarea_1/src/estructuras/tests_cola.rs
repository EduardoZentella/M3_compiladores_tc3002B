// tests_cola.rs - Tests unitarios para la estructura de datos Cola
use super::queue::*;

// ==================== TESTS BÁSICOS ====================

#[test]
fn test_cola_nueva_vacia() {
    let cola: Cola<i32> = Cola::nueva();
    assert!(cola.esta_vacia());
    assert_eq!(cola.tamanio(), 0);
    assert!(!cola.esta_llena());
    assert_eq!(cola.capacidad_maxima(), None);
}

#[test]
fn test_cola_con_capacidad() {
    let cola: Cola<i32> = Cola::nueva_con_capacidad(5);
    assert!(cola.esta_vacia());
    assert_eq!(cola.tamanio(), 0);
    assert!(!cola.esta_llena());
    assert_eq!(cola.capacidad_maxima(), Some(5));
}

#[test]
fn test_encolar_y_desencolar_basico() {
    let mut cola = Cola::nueva();

    // Encolar elementos
    assert_eq!(cola.encolar(1), Ok(()));
    assert_eq!(cola.encolar(2), Ok(()));
    assert_eq!(cola.encolar(3), Ok(()));

    assert!(!cola.esta_vacia());
    assert_eq!(cola.tamanio(), 3);

    // Desencolar elementos en orden FIFO
    assert_eq!(cola.desencolar(), Ok(1));
    assert_eq!(cola.desencolar(), Ok(2));
    assert_eq!(cola.desencolar(), Ok(3));

    assert!(cola.esta_vacia());
    assert_eq!(cola.tamanio(), 0);
}

// ==================== TESTS DE CAPACIDAD LIMITADA ====================

#[test]
fn test_capacidad_maxima_respetada() {
    let mut cola = Cola::nueva_con_capacidad(2);

    // Llenar hasta capacidad máxima
    assert_eq!(cola.encolar("A"), Ok(()));
    assert_eq!(cola.encolar("B"), Ok(()));
    assert!(cola.esta_llena());

    // Intentar exceder capacidad
    assert_eq!(cola.encolar("C"), Err(ErrorCola::CapacidadExcedida));
    assert_eq!(cola.tamanio(), 2);
}

#[test]
fn test_capacidad_liberada_al_desencolar() {
    let mut cola = Cola::nueva_con_capacidad(2);

    // Llenar cola
    cola.encolar(1).unwrap();
    cola.encolar(2).unwrap();
    assert!(cola.esta_llena());

    // Desencolar y verificar que se puede agregar nuevamente
    assert_eq!(cola.desencolar(), Ok(1));
    assert!(!cola.esta_llena());
    assert_eq!(cola.encolar(3), Ok(()));
}

// ==================== TESTS DE ERRORES ====================

#[test]
fn test_desencolar_cola_vacia() {
    let mut cola: Cola<i32> = Cola::nueva();
    assert_eq!(cola.desencolar(), Err(ErrorCola::ColaVacia));
}

#[test]
fn test_frente_cola_vacia() {
    let cola: Cola<i32> = Cola::nueva();
    assert_eq!(cola.frente(), Err(ErrorCola::ColaVacia));
}

#[test]
fn test_ultimo_cola_vacia() {
    let cola: Cola<i32> = Cola::nueva();
    assert_eq!(cola.ultimo(), Err(ErrorCola::ColaVacia));
}

#[test]
fn test_obtener_indice_invalido() {
    let cola: Cola<i32> = Cola::nueva();
    assert_eq!(cola.obtener(0), Err(ErrorCola::IndiceInvalido));

    let mut cola_con_elementos = Cola::nueva();
    cola_con_elementos.encolar(1).unwrap();
    cola_con_elementos.encolar(2).unwrap();

    assert_eq!(cola_con_elementos.obtener(0), Ok(&1));
    assert_eq!(cola_con_elementos.obtener(1), Ok(&2));
    assert_eq!(cola_con_elementos.obtener(2), Err(ErrorCola::IndiceInvalido));
}

// ==================== TESTS DE VISUALIZACIÓN ====================

#[test]
fn test_frente_y_ultimo() {
    let mut cola = Cola::nueva();
    cola.encolar("primero").unwrap();
    cola.encolar("segundo").unwrap();
    cola.encolar("tercero").unwrap();

    assert_eq!(cola.frente(), Ok(&"primero"));
    assert_eq!(cola.ultimo(), Ok(&"tercero"));

    // Verificar que no se modificó la cola
    assert_eq!(cola.tamanio(), 3);
}

#[test]
fn test_obtener_por_indice() {
    let mut cola = Cola::nueva();
    cola.encolar(10).unwrap();
    cola.encolar(20).unwrap();
    cola.encolar(30).unwrap();

    assert_eq!(cola.obtener(0), Ok(&10));
    assert_eq!(cola.obtener(1), Ok(&20));
    assert_eq!(cola.obtener(2), Ok(&30));
}

// ==================== TESTS DE OPERACIONES AVANZADAS ====================

#[test]
fn test_encolar_multiples() {
    let mut cola = Cola::nueva_con_capacidad(5);
    let elementos = vec![1, 2, 3, 4];

    assert_eq!(cola.encolar_multiples(elementos), Ok(()));
    assert_eq!(cola.tamanio(), 4);
    assert_eq!(cola.frente(), Ok(&1));
    assert_eq!(cola.ultimo(), Ok(&4));
}

#[test]
fn test_encolar_multiples_excede_capacidad() {
    let mut cola = Cola::nueva_con_capacidad(3);
    let elementos = vec![1, 2, 3, 4, 5];

    assert_eq!(cola.encolar_multiples(elementos), Err(ErrorCola::CapacidadExcedida));
    assert_eq!(cola.tamanio(), 0); // No se agregó nada
}

#[test]
fn test_desencolar_multiples() {
    let mut cola = Cola::nueva();
    for i in 1..=5 {
        cola.encolar(i).unwrap();
    }

    let resultado = cola.desencolar_multiples(3);
    assert_eq!(resultado, Ok(vec![1, 2, 3]));
    assert_eq!(cola.tamanio(), 2);
    assert_eq!(cola.frente(), Ok(&4));
}

#[test]
fn test_desencolar_multiples_insuficientes() {
    let mut cola = Cola::nueva();
    cola.encolar(1).unwrap();
    cola.encolar(2).unwrap();

    assert_eq!(cola.desencolar_multiples(5), Err(ErrorCola::ColaVacia));
    assert_eq!(cola.tamanio(), 2); // No se modificó
}

// ==================== TESTS DE GESTIÓN DE MEMORIA ====================

#[test]
fn test_limpiar_cola() {
    let mut cola = Cola::nueva();
    cola.encolar(1).unwrap();
    cola.encolar(2).unwrap();
    cola.encolar(3).unwrap();

    assert_eq!(cola.tamanio(), 3);
    cola.limpiar();
    assert_eq!(cola.tamanio(), 0);
    assert!(cola.esta_vacia());
}

#[test]
fn test_reservar_capacidad() {
    let mut cola: Cola<i32> = Cola::nueva_con_capacidad(10);
    assert_eq!(cola.reservar(5), Ok(()));

    // Intentar reservar más de la capacidad máxima
    assert_eq!(cola.reservar(50), Err(ErrorCola::CapacidadExcedida));
}

#[test]
fn test_reservar_sin_limite() {
    let mut cola: Cola<i32> = Cola::nueva();
    assert_eq!(cola.reservar(100), Ok(()));
    assert_eq!(cola.reservar(1000), Ok(()));
}

#[test]
fn test_a_vector() {
    let mut cola = Cola::nueva();
    cola.encolar("a").unwrap();
    cola.encolar("b").unwrap();
    cola.encolar("c").unwrap();

    let vector = cola.a_vector();
    assert_eq!(vector, vec!["a", "b", "c"]);
}

#[test]
fn test_capacidad_vec_deque() {
    let mut cola: Cola<i32> = Cola::nueva();

    // Capacidad inicial debería ser 0
    assert_eq!(cola.capacidad(), 0);

    // Agregar elementos para que VecDeque asigne capacidad
    for i in 1..=10 {
        cola.encolar(i).unwrap();
    }

    // Ahora debería tener capacidad > 0
    assert!(cola.capacidad() > 0);
}

#[test]
fn test_reducir_capacidad() {
    let mut cola = Cola::nueva();

    // Llenar la cola para que VecDeque asigne mucha capacidad
    for i in 1..=100 {
        cola.encolar(i).unwrap();
    }

    let capacidad_inicial = cola.capacidad();
    assert!(capacidad_inicial >= 100);

    // Limpiar la cola pero mantener capacidad
    cola.limpiar();
    assert_eq!(cola.capacidad(), capacidad_inicial);

    // Reducir capacidad
    cola.reducir_capacidad();
    assert!(cola.capacidad() < capacidad_inicial);
}

// ==================== TESTS DE ITERACIÓN ====================

#[test]
fn test_iterador() {
    let mut cola = Cola::nueva();
    cola.encolar(1).unwrap();
    cola.encolar(2).unwrap();
    cola.encolar(3).unwrap();

    let elementos: Vec<&i32> = cola.iter().collect();
    assert_eq!(elementos, vec![&1, &2, &3]);
}

// ==================== TESTS DE TIPOS DIFERENTES ====================

#[test]
fn test_cola_strings() {
    let mut cola = Cola::nueva();
    cola.encolar("Hola".to_string()).unwrap();
    cola.encolar("Mundo".to_string()).unwrap();

    assert_eq!(cola.desencolar(), Ok("Hola".to_string()));
    assert_eq!(cola.desencolar(), Ok("Mundo".to_string()));
}

#[test]
fn test_cola_estructuras_complejas() {
    #[derive(Debug, PartialEq, Clone)]
    struct Persona {
        nombre: String,
        edad: u32,
    }

    let mut cola = Cola::nueva();
    let persona1 = Persona { nombre: "Ana".to_string(), edad: 25 };
    let persona2 = Persona { nombre: "Bob".to_string(), edad: 30 };

    cola.encolar(persona1.clone()).unwrap();
    cola.encolar(persona2.clone()).unwrap();

    assert_eq!(cola.desencolar(), Ok(persona1));
    assert_eq!(cola.desencolar(), Ok(persona2));
}

// ==================== TESTS DE ESCENARIOS EXTREMOS ====================

#[test]
fn test_operaciones_repetidas() {
    let mut cola = Cola::nueva_con_capacidad(3);

    // Ciclo de llenar y vaciar múltiples veces
    for ciclo in 0..5 {
        // Llenar
        for i in 1..=3 {
            assert_eq!(cola.encolar(ciclo * 10 + i), Ok(()));
        }
        assert!(cola.esta_llena());

        // Vaciar
        for i in 1..=3 {
            assert_eq!(cola.desencolar(), Ok(ciclo * 10 + i));
        }
        assert!(cola.esta_vacia());
    }
}

#[test]
fn test_cola_capacidad_uno() {
    let mut cola = Cola::nueva_con_capacidad(1);

    assert_eq!(cola.encolar(42), Ok(()));
    assert!(cola.esta_llena());
    assert_eq!(cola.encolar(99), Err(ErrorCola::CapacidadExcedida));

    assert_eq!(cola.desencolar(), Ok(42));
    assert!(cola.esta_vacia());
}

// ==================== TESTS DE ERROR DISPLAY ====================

#[test]
fn test_error_display() {
    let error_vacia = ErrorCola::ColaVacia;
    let error_capacidad = ErrorCola::CapacidadExcedida;
    let error_indice = ErrorCola::IndiceInvalido;

    assert_eq!(format!("{}", error_vacia), "Error: La cola está vacía");
    assert_eq!(format!("{}", error_capacidad), "Error: Se ha excedido la capacidad máxima de la cola");
    assert_eq!(format!("{}", error_indice), "Error: Índice fuera de rango");
}

#[test]
fn test_error_debug() {
    let error = ErrorCola::ColaVacia;
    assert_eq!(format!("{:?}", error), "ColaVacia");
}

#[test]
fn test_error_equality() {
    assert_eq!(ErrorCola::ColaVacia, ErrorCola::ColaVacia);
    assert_eq!(ErrorCola::CapacidadExcedida, ErrorCola::CapacidadExcedida);
    assert_eq!(ErrorCola::IndiceInvalido, ErrorCola::IndiceInvalido);

    assert_ne!(ErrorCola::ColaVacia, ErrorCola::CapacidadExcedida);
}