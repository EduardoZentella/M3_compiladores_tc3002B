// Declarar el módulo estructuras
mod estructuras;

// Importar elementos desde el módulo estructuras
#[allow(unused_imports)]
use estructuras::{Cola, ErrorCola, ResultadoCola, Stack, ErrorStack, ResultadoStack, MiHashMap, ErrorHashMap, ResultadoHashMap};

fn main() {
    println!("=== Demostración de Estructuras de Datos con Manejo de Errores ===\n");

    demostrar_cola();
    println!("\n{}\n", "=".repeat(70));
    demostrar_stack();
    println!("\n{}\n", "=".repeat(70));
    demostrar_hashmap();

    println!("\n✨ ¡Demostración completada!");
}

fn demostrar_cola() {
    println!("📋 === COLA (FIFO - First In, First Out) ===");

    // Crear una cola con capacidad limitada
    let mut cola = Cola::nueva_con_capacidad(3);

    println!("\n1. Encolando elementos...");
    // Encolar algunos elementos
    for i in 1..=3 {
        match cola.encolar(i * 10) {
            Ok(()) => println!("   ✅ Elemento {} encolado", i * 10),
            Err(e) => println!("   ❌ Error: {}", e),
        }
    }

    // Ver estado de la cola
    println!("\n2. Estado de la cola:");
    println!("   • Tamaño: {}", cola.tamanio());
    println!("   • ¿Está vacía? {}", cola.esta_vacia());
    println!("   • ¿Está llena? {}", cola.esta_llena());

    // Intentar ver el frente
    match cola.frente() {
        Ok(elemento) => println!("   • Elemento al frente: {}", elemento),
        Err(e) => println!("   • Error al ver frente: {}", e),
    }

    match cola.ultimo() {
        Ok(elemento) => println!("   • Elemento al final: {}", elemento),
        Err(e) => println!("   • Error al ver último: {}", e),
    }

    println!("\n3. Intentando agregar un elemento más (debería fallar):");
    match cola.encolar(99) {
        Ok(()) => println!("   ✅ Elemento 99 encolado"),
        Err(e) => println!("   ❌ Como esperado: {}", e),
    }

    println!("\n4. Desencolando elementos (orden FIFO)...");
    while !cola.esta_vacia() {
        match cola.desencolar() {
            Ok(elemento) => println!("   ✅ Desencolado: {}", elemento),
            Err(e) => println!("   ❌ Error: {}", e),
        }
    }

    println!("\n5. Intentando desencolar de cola vacía:");
    match cola.desencolar() {
        Ok(elemento) => println!("   ✅ Desencolado: {}", elemento),
        Err(e) => println!("   ❌ Como esperado: {}", e),
    }
}

fn demostrar_stack() {
    println!("📚 === STACK (LIFO - Last In, First Out) ===");

    // Crear un Stack con capacidad limitada
    let mut stack = Stack::nuevo_con_capacidad(3);

    println!("\n1. Haciendo push de elementos...");
    for i in 1..=3 {
        match stack.push(i * 10) {
            Ok(()) => println!("   ✅ Elemento {} agregado al stack", i * 10),
            Err(e) => println!("   ❌ Error: {}", e),
        }
    }

    // Ver estado del stack
    println!("\n2. Estado del stack:");
    println!("   • Tamaño: {}", stack.tamanio());
    println!("   • ¿Está vacío? {}", stack.esta_vacio());
    println!("   • ¿Está lleno? {}", stack.esta_lleno());

    match stack.peek() {
        Ok(elemento) => println!("   • Elemento en el tope: {}", elemento),
        Err(e) => println!("   • Error al obtener tope: {}", e),
    }

    println!("\n3. Intentando agregar un elemento más (debería fallar):");
    match stack.push(99) {
        Ok(()) => println!("   ✅ Elemento 99 agregado"),
        Err(e) => println!("   ❌ Como esperado: {}", e),
    }

    println!("\n4. Haciendo pop de elementos (orden LIFO)...");
    while !stack.esta_vacio() {
        match stack.pop() {
            Ok(elemento) => println!("   ✅ Pop: {}", elemento),
            Err(e) => println!("   ❌ Error: {}", e),
        }
    }

    println!("\n5. Intentando pop de stack vacío:");
    match stack.pop() {
        Ok(elemento) => println!("   ✅ Pop: {}", elemento),
        Err(e) => println!("   ❌ Como esperado: {}", e),
    }
}

fn demostrar_hashmap() {
    println!("🗂️ === HASHMAP (Diccionario Clave-Valor) ===");

    // Crear un HashMap con capacidad limitada
    let mut mapa = MiHashMap::nuevo_con_capacidad(5);

    println!("\n1. Insertando pares clave-valor...");
    let datos = vec![
        ("nombre", "Juan"),
        ("apellido", "Pérez"),
        ("edad", "25"),
        ("ciudad", "Madrid"),
    ];

    for (clave, valor) in &datos {
        match mapa.insertar(*clave, *valor) {
            Ok(_) => println!("   ✅ Insertado: {} = {}", clave, valor),
            Err(e) => println!("   ❌ Error: {}", e),
        }
    }

    // 2. Mostrar estado del HashMap
    println!("\n2. Estado del HashMap:");
    println!("   • Tamaño: {}", mapa.tamanio());
    println!("   • ¿Está vacío? {}", mapa.esta_vacio());
    println!("   • ¿Está lleno? {}", mapa.esta_lleno());
    println!("   • Capacidad máxima: {:?}", mapa.capacidad_maxima());

    // 3. Buscar valores por clave
    println!("\n3. Buscando valores por clave:");
    let claves_a_buscar = ["nombre", "edad", "profesion"];
    for clave in &claves_a_buscar {
        match mapa.obtener(clave) {
            Ok(valor) => println!("   ✅ {}: {}", clave, valor),
            Err(e) => println!("   ❌ {}: {}", clave, e),
        }
    }

    // 4. Intentar exceder capacidad
    println!("\n4. Intentando insertar más elementos (debería fallar):");
    match mapa.insertar("pais", "España") {
        Ok(_) => println!("   ✅ País insertado"),
        Err(e) => println!("   ❌ Como esperado: {}", e),
    }

    // 5. Actualizar valor existente
    println!("\n5. Actualizando valor existente:");
    match mapa.actualizar("edad", "26") {
        Ok(valor_anterior) => println!("   ✅ Edad actualizada. Valor anterior: {:?}", valor_anterior),
        Err(e) => println!("   ❌ Error: {}", e),
    }

    // 6. Demostrar funcionalidades avanzadas
    println!("\n6. 🔍 Funcionalidades avanzadas:");

    // Verificar existencia de claves
    println!("   • ¿Contiene 'nombre'? {}", mapa.contiene_clave(&"nombre"));
    println!("   • ¿Contiene 'telefono'? {}", mapa.contiene_clave(&"telefono"));

    // Obtener todas las claves
    let claves = mapa.claves();
    println!("   • Claves disponibles: {:?}", claves);

    // 7. Remover elementos
    println!("\n7. Removiendo elementos:");
    match mapa.remover(&"ciudad") {
        Ok(valor_removido) => println!("   ✅ Removido ciudad: {}", valor_removido),
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("   • Tamaño después de remover: {}", mapa.tamanio());

    // 8. Demostrar insertar o actualizar
    println!("\n8. Demonstrando insertar_o_actualizar:");

    // Insertar nueva clave
    match mapa.insertar_o_actualizar("telefono", "123-456-7890") {
        Ok(None) => println!("   ✅ Nueva clave 'telefono' insertada"),
        Ok(Some(anterior)) => println!("   ✅ Clave 'telefono' actualizada. Anterior: {}", anterior),
        Err(e) => println!("   ❌ Error: {}", e),
    }

    // Actualizar clave existente
    match mapa.insertar_o_actualizar("nombre", "Juan Carlos") {
        Ok(None) => println!("   ✅ Nueva clave 'nombre' insertada"),
        Ok(Some(anterior)) => println!("   ✅ Clave 'nombre' actualizada. Anterior: {}", anterior),
        Err(e) => println!("   ❌ Error: {}", e),
    }

    // 9. Estado final
    println!("\n9. Estado final del HashMap:");
    for (clave, valor) in mapa.iter() {
        println!("   • {} = {}", clave, valor);
    }
}