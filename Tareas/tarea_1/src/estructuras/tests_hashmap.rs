// tests_hashmap.rs - Tests unitarios para la estructura de datos HashMap
use super::hashmap::*;

// ==================== TESTS BÁSICOS ====================

#[test]
fn test_hashmap_nuevo_vacio() {
    let mapa: MiHashMap<i32, String> = MiHashMap::nuevo();
    assert!(mapa.esta_vacio());
    assert_eq!(mapa.tamanio(), 0);
    assert!(!mapa.esta_lleno());
    assert_eq!(mapa.capacidad_maxima(), None);
}

#[test]
fn test_hashmap_con_capacidad() {
    let mapa: MiHashMap<String, i32> = MiHashMap::nuevo_con_capacidad(5);
    assert!(mapa.esta_vacio());
    assert_eq!(mapa.tamanio(), 0);
    assert!(!mapa.esta_lleno());
    assert_eq!(mapa.capacidad_maxima(), Some(5));
}

#[test]
fn test_insertar_y_obtener_basico() {
    let mut mapa = MiHashMap::nuevo();

    // Insertar elementos
    assert_eq!(mapa.insertar("clave1".to_string(), 100), Ok(()));
    assert_eq!(mapa.insertar("clave2".to_string(), 200), Ok(()));
    assert_eq!(mapa.insertar("clave3".to_string(), 300), Ok(()));

    assert!(!mapa.esta_vacio());
    assert_eq!(mapa.tamanio(), 3);

    // Obtener elementos
    assert_eq!(mapa.obtener(&"clave1".to_string()), Ok(&100));
    assert_eq!(mapa.obtener(&"clave2".to_string()), Ok(&200));
    assert_eq!(mapa.obtener(&"clave3".to_string()), Ok(&300));
}

// ==================== TESTS DE CAPACIDAD LIMITADA ====================

#[test]
fn test_capacidad_maxima_respetada() {
    let mut mapa = MiHashMap::nuevo_con_capacidad(2);

    // Llenar hasta capacidad máxima
    assert_eq!(mapa.insertar("A", 1), Ok(()));
    assert_eq!(mapa.insertar("B", 2), Ok(()));
    assert!(mapa.esta_lleno());

    // Intentar exceder capacidad
    assert_eq!(mapa.insertar("C", 3), Err(ErrorHashMap::CapacidadExcedida));
    assert_eq!(mapa.tamanio(), 2);
}

#[test]
fn test_capacidad_liberada_al_remover() {
    let mut mapa = MiHashMap::nuevo_con_capacidad(2);

    // Llenar mapa
    mapa.insertar("key1", 10).unwrap();
    mapa.insertar("key2", 20).unwrap();
    assert!(mapa.esta_lleno());

    // Remover y verificar que se puede agregar nuevamente
    assert_eq!(mapa.remover(&"key1"), Ok(10));
    assert!(!mapa.esta_lleno());
    assert_eq!(mapa.insertar("key3", 30), Ok(()));
}

// ==================== TESTS DE ERRORES ====================

#[test]
fn test_obtener_clave_no_encontrada() {
    let mapa: MiHashMap<String, i32> = MiHashMap::nuevo();
    assert_eq!(mapa.obtener(&"inexistente".to_string()), Err(ErrorHashMap::ClaveNoEncontrada));
}

#[test]
fn test_remover_clave_no_encontrada() {
    let mut mapa: MiHashMap<String, i32> = MiHashMap::nuevo();
    assert_eq!(mapa.remover(&"inexistente".to_string()), Err(ErrorHashMap::ClaveNoEncontrada));
}

#[test]
fn test_actualizar_clave_no_encontrada() {
    let mut mapa: MiHashMap<String, i32> = MiHashMap::nuevo();
    assert_eq!(mapa.actualizar("inexistente".to_string(), 100), Err(ErrorHashMap::ClaveNoEncontrada));
}

// ==================== TESTS DE FUNCIONALIDAD AVANZADA ====================

#[test]
fn test_contiene_clave() {
    let mut mapa = MiHashMap::nuevo();
    mapa.insertar("existe", 42).unwrap();

    assert!(mapa.contiene_clave(&"existe"));
    assert!(!mapa.contiene_clave(&"no_existe"));
}

#[test]
fn test_actualizar_valor_existente() {
    let mut mapa = MiHashMap::nuevo();
    mapa.insertar("key", "valor_original").unwrap();

    let valor_anterior = mapa.actualizar("key", "nuevo_valor").unwrap();
    assert_eq!(valor_anterior, Some("valor_original"));
    assert_eq!(mapa.obtener(&"key"), Ok(&"nuevo_valor"));
}

#[test]
fn test_insertar_o_actualizar() {
    let mut mapa = MiHashMap::nuevo_con_capacidad(2);

    // Insertar nuevo
    let resultado1 = mapa.insertar_o_actualizar("nueva", 100).unwrap();
    assert_eq!(resultado1, None);
    assert_eq!(mapa.obtener(&"nueva"), Ok(&100));

    // Actualizar existente
    let resultado2 = mapa.insertar_o_actualizar("nueva", 200).unwrap();
    assert_eq!(resultado2, Some(100));
    assert_eq!(mapa.obtener(&"nueva"), Ok(&200));
}

#[test]
fn test_obtener_mutable() {
    let mut mapa = MiHashMap::nuevo();
    mapa.insertar("key", 100).unwrap();

    match mapa.obtener_mutable(&"key") {
        Ok(valor_mut) => *valor_mut = 200,
        Err(e) => panic!("Error inesperado: {}", e),
    }

    assert_eq!(mapa.obtener(&"key"), Ok(&200));
}

// ==================== TESTS DE OPERACIONES MÚLTIPLES ====================

#[test]
fn test_insertar_multiples() {
    let mut mapa = MiHashMap::nuevo_con_capacidad(5);
    let entradas = vec![
        ("a", 1),
        ("b", 2),
        ("c", 3),
        ("d", 4),
    ];

    assert_eq!(mapa.insertar_multiples(entradas), Ok(()));
    assert_eq!(mapa.tamanio(), 4);
    assert_eq!(mapa.obtener(&"a"), Ok(&1));
    assert_eq!(mapa.obtener(&"d"), Ok(&4));
}

#[test]
fn test_insertar_multiples_excede_capacidad() {
    let mut mapa = MiHashMap::nuevo_con_capacidad(2);
    let entradas = vec![
        ("a", 1),
        ("b", 2),
        ("c", 3),
    ];

    assert_eq!(mapa.insertar_multiples(entradas), Err(ErrorHashMap::CapacidadExcedida));
    assert_eq!(mapa.tamanio(), 0); // No se insertó nada
}

#[test]
fn test_remover_multiples() {
    let mut mapa = MiHashMap::nuevo();
    mapa.insertar("a", 1).unwrap();
    mapa.insertar("b", 2).unwrap();
    mapa.insertar("c", 3).unwrap();
    mapa.insertar("d", 4).unwrap();

    let claves_a_remover = vec![&"a", &"c"];
    let valores_removidos = mapa.remover_multiples(claves_a_remover).unwrap();

    assert_eq!(valores_removidos.len(), 2);
    assert!(valores_removidos.contains(&1));
    assert!(valores_removidos.contains(&3));
    assert_eq!(mapa.tamanio(), 2);
    assert!(!mapa.contiene_clave(&"a"));
    assert!(!mapa.contiene_clave(&"c"));
}

#[test]
fn test_remover_multiples_clave_no_encontrada() {
    let mut mapa = MiHashMap::nuevo();
    mapa.insertar("a", 1).unwrap();

    let claves_a_remover = vec![&"a", &"inexistente"];
    assert_eq!(mapa.remover_multiples(claves_a_remover), Err(ErrorHashMap::ClaveNoEncontrada));

    // Verificar que el mapa no se modificó
    assert_eq!(mapa.tamanio(), 1);
    assert!(mapa.contiene_clave(&"a"));
}

// ==================== TESTS DE GESTIÓN DE MEMORIA ====================

#[test]
fn test_limpiar_hashmap() {
    let mut mapa = MiHashMap::nuevo();
    mapa.insertar("a", 1).unwrap();
    mapa.insertar("b", 2).unwrap();
    mapa.insertar("c", 3).unwrap();

    assert_eq!(mapa.tamanio(), 3);
    mapa.limpiar();
    assert_eq!(mapa.tamanio(), 0);
    assert!(mapa.esta_vacio());
}

#[test]
fn test_reservar_capacidad() {
    let mut mapa: MiHashMap<String, i32> = MiHashMap::nuevo_con_capacidad(10);
    assert_eq!(mapa.reservar(5), Ok(()));

    // Intentar reservar más de la capacidad máxima
    assert_eq!(mapa.reservar(50), Err(ErrorHashMap::CapacidadExcedida));
}

#[test]
fn test_reservar_sin_limite() {
    let mut mapa: MiHashMap<String, i32> = MiHashMap::nuevo();
    assert_eq!(mapa.reservar(100), Ok(()));
    assert_eq!(mapa.reservar(1000), Ok(()));
}

#[test]
fn test_reducir_capacidad() {
    let mut mapa = MiHashMap::nuevo();

    // Llenar el mapa para que HashMap asigne capacidad
    for i in 1..=50 {
        mapa.insertar(format!("key{}", i), i).unwrap();
    }

    let capacidad_inicial = mapa.capacidad();
    assert!(capacidad_inicial >= 50);

    // Limpiar el mapa pero mantener capacidad
    mapa.limpiar();
    assert_eq!(mapa.capacidad(), capacidad_inicial);

    // Reducir capacidad
    mapa.reducir_capacidad();
    assert!(mapa.capacidad() <= capacidad_inicial);
}

// ==================== TESTS DE ITERACIÓN ====================

#[test]
fn test_claves_valores_entradas() {
    let mut mapa = MiHashMap::nuevo();
    mapa.insertar("a", 1).unwrap();
    mapa.insertar("b", 2).unwrap();
    mapa.insertar("c", 3).unwrap();

    let claves = mapa.claves();
    let valores = mapa.valores();
    let entradas = mapa.entradas();

    assert_eq!(claves.len(), 3);
    assert_eq!(valores.len(), 3);
    assert_eq!(entradas.len(), 3);

    assert!(claves.contains(&&"a"));
    assert!(valores.contains(&&1));
    assert!(entradas.contains(&(&"a", &1)));
}

#[test]
fn test_iteradores() {
    let mut mapa = MiHashMap::nuevo();
    mapa.insertar("x".to_string(), 10).unwrap();
    mapa.insertar("y".to_string(), 20).unwrap();
    mapa.insertar("z".to_string(), 30).unwrap();

    // Test iterador de claves
    let claves_iter: Vec<&String> = mapa.iter_claves().collect();
    assert_eq!(claves_iter.len(), 3);

    // Test iterador de valores
    let valores_iter: Vec<&i32> = mapa.iter_valores().collect();
    assert_eq!(valores_iter.len(), 3);

    // Test iterador de entradas
    let entradas_iter: Vec<(&String, &i32)> = mapa.iter().collect();
    assert_eq!(entradas_iter.len(), 3);
}

#[test]
fn test_iterador_mutable() {
    let mut mapa = MiHashMap::nuevo();
    mapa.insertar("a", 1).unwrap();
    mapa.insertar("b", 2).unwrap();

    // Modificar valores usando iterador mutable
    for valor in mapa.iter_valores_mut() {
        *valor *= 10;
    }

    assert_eq!(mapa.obtener(&"a"), Ok(&10));
    assert_eq!(mapa.obtener(&"b"), Ok(&20));
}

// ==================== TESTS DE TIPOS DIFERENTES ====================

#[test]
fn test_hashmap_strings() {
    let mut mapa = MiHashMap::nuevo();
    mapa.insertar("nombre".to_string(), "Juan".to_string()).unwrap();
    mapa.insertar("apellido".to_string(), "Pérez".to_string()).unwrap();

    assert_eq!(mapa.obtener(&"nombre".to_string()), Ok(&"Juan".to_string()));
    assert_eq!(mapa.obtener(&"apellido".to_string()), Ok(&"Pérez".to_string()));
}

#[test]
fn test_hashmap_estructuras_complejas() {
    #[derive(Debug, PartialEq, Clone)]
    struct Persona {
        nombre: String,
        edad: u32,
    }

    let mut mapa = MiHashMap::nuevo();
    let persona1 = Persona { nombre: "Ana".to_string(), edad: 25 };
    let persona2 = Persona { nombre: "Bob".to_string(), edad: 30 };

    mapa.insertar(1, persona1.clone()).unwrap();
    mapa.insertar(2, persona2.clone()).unwrap();

    assert_eq!(mapa.obtener(&1), Ok(&persona1));
    assert_eq!(mapa.obtener(&2), Ok(&persona2));
}

// ==================== TESTS DE OPERACIONES AVANZADAS ====================

#[test]
fn test_retener() {
    let mut mapa = MiHashMap::nuevo();
    mapa.insertar("a", 1).unwrap();
    mapa.insertar("b", 2).unwrap();
    mapa.insertar("c", 3).unwrap();
    mapa.insertar("d", 4).unwrap();

    // Retener solo valores pares
    mapa.retener(|_k, &v| v % 2 == 0);

    assert_eq!(mapa.tamanio(), 2);
    assert!(mapa.contiene_clave(&"b"));
    assert!(mapa.contiene_clave(&"d"));
    assert!(!mapa.contiene_clave(&"a"));
    assert!(!mapa.contiene_clave(&"c"));
}

#[test]
fn test_obtener_o_defecto() {
    let mut mapa = MiHashMap::nuevo();
    mapa.insertar("existe", 42).unwrap();

    let valor_defecto = 0;
    assert_eq!(mapa.obtener_o_defecto(&"existe", &valor_defecto), &42);
    assert_eq!(mapa.obtener_o_defecto(&"no_existe", &valor_defecto), &0);
}

#[test]
fn test_fusionar() {
    let mut mapa1 = MiHashMap::nuevo_con_capacidad(5);
    mapa1.insertar("a", 1).unwrap();
    mapa1.insertar("b", 2).unwrap();

    let mut mapa2 = MiHashMap::nuevo();
    mapa2.insertar("c", 3).unwrap();
    mapa2.insertar("d", 4).unwrap();

    assert_eq!(mapa1.fusionar(mapa2), Ok(()));
    assert_eq!(mapa1.tamanio(), 4);
    assert_eq!(mapa1.obtener(&"c"), Ok(&3));
    assert_eq!(mapa1.obtener(&"d"), Ok(&4));
}

#[test]
fn test_fusionar_excede_capacidad() {
    let mut mapa1 = MiHashMap::nuevo_con_capacidad(3);
    mapa1.insertar("a", 1).unwrap();
    mapa1.insertar("b", 2).unwrap();

    let mut mapa2 = MiHashMap::nuevo();
    mapa2.insertar("c", 3).unwrap();
    mapa2.insertar("d", 4).unwrap();

    assert_eq!(mapa1.fusionar(mapa2), Err(ErrorHashMap::CapacidadExcedida));
    assert_eq!(mapa1.tamanio(), 2); // No se modificó
}

#[test]
fn test_a_vector() {
    let mut mapa = MiHashMap::nuevo();
    mapa.insertar("x", 10).unwrap();
    mapa.insertar("y", 20).unwrap();
    mapa.insertar("z", 30).unwrap();

    let vector = mapa.a_vector();
    assert_eq!(vector.len(), 3);

    // Verificar que contiene todos los pares (orden puede variar)
    assert!(vector.contains(&("x", 10)));
    assert!(vector.contains(&("y", 20)));
    assert!(vector.contains(&("z", 30)));
}

// ==================== TESTS DE CLONACIÓN ====================

#[test]
fn test_clone() {
    let mut mapa_original = MiHashMap::nuevo_con_capacidad(5);
    mapa_original.insertar("a", 1).unwrap();
    mapa_original.insertar("b", 2).unwrap();

    let mapa_clonado = mapa_original.clone();

    assert_eq!(mapa_clonado.tamanio(), 2);
    assert_eq!(mapa_clonado.capacidad_maxima(), Some(5));
    assert_eq!(mapa_clonado.obtener(&"a"), Ok(&1));
    assert_eq!(mapa_clonado.obtener(&"b"), Ok(&2));
}

// ==================== TESTS DE ERROR DISPLAY ====================

#[test]
fn test_error_display() {
    let error_vacio = ErrorHashMap::HashMapVacio;
    let error_capacidad = ErrorHashMap::CapacidadExcedida;
    let error_clave = ErrorHashMap::ClaveNoEncontrada;

    assert_eq!(format!("{}", error_vacio), "Error: El HashMap está vacío");
    assert_eq!(format!("{}", error_capacidad), "Error: Se ha excedido la capacidad máxima del HashMap");
    assert_eq!(format!("{}", error_clave), "Error: Clave no encontrada en el HashMap");
}

#[test]
fn test_error_debug() {
    let error = ErrorHashMap::ClaveNoEncontrada;
    assert_eq!(format!("{:?}", error), "ClaveNoEncontrada");
}

#[test]
fn test_error_equality() {
    assert_eq!(ErrorHashMap::HashMapVacio, ErrorHashMap::HashMapVacio);
    assert_eq!(ErrorHashMap::CapacidadExcedida, ErrorHashMap::CapacidadExcedida);
    assert_eq!(ErrorHashMap::ClaveNoEncontrada, ErrorHashMap::ClaveNoEncontrada);

    assert_ne!(ErrorHashMap::HashMapVacio, ErrorHashMap::CapacidadExcedida);
}

// ==================== TESTS DE ESCENARIOS EXTREMOS ====================

#[test]
fn test_operaciones_repetidas() {
    let mut mapa = MiHashMap::nuevo_con_capacidad(3);

    // Ciclo de llenar y vaciar múltiples veces
    for ciclo in 0..5 {
        // Llenar
        for i in 1..=3 {
            let clave = format!("key{}", i);
            let valor = ciclo * 10 + i;
            assert_eq!(mapa.insertar(clave, valor), Ok(()));
        }
        assert!(mapa.esta_lleno());

        // Vaciar
        for i in 1..=3 {
            let clave = format!("key{}", i);
            assert!(mapa.remover(&clave).is_ok());
        }
        assert!(mapa.esta_vacio());
    }
}

#[test]
fn test_hashmap_capacidad_uno() {
    let mut mapa = MiHashMap::nuevo_con_capacidad(1);

    assert_eq!(mapa.insertar("única", 42), Ok(()));
    assert!(mapa.esta_lleno());
    assert_eq!(mapa.insertar("otra", 99), Err(ErrorHashMap::CapacidadExcedida));

    assert_eq!(mapa.remover(&"única"), Ok(42));
    assert!(mapa.esta_vacio());
}