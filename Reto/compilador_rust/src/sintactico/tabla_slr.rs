// ==========================================
// ARCHIVO GENERADO AUTOMÁTICAMENTE
// Creado por 'generador_slr'
// NO EDITAR MANUALMENTE
// ==========================================

use lazy_static::lazy_static;
use std::collections::HashMap;

/// Acción en la tabla ACTION
#[derive(Debug, Clone, PartialEq)]
pub enum Accion {
    Shift(usize),   // Desplazar al estado N
    Reduce(usize),  // Reducir por la producción N
    Accept,         // Aceptar
}

/// Representa una regla de producción
#[derive(Debug, Clone)]
pub struct Regla {
    #[allow(dead_code)]
    pub id: usize,
    pub cabeza: String,
    pub longitud_cuerpo: usize,
}

lazy_static! {
    /// Tabla ACTION: (estado, terminal) -> Acción
    pub static ref TABLA_ACTION: HashMap<(usize, String), Accion> = {
        let mut m = HashMap::new();
        m.insert((32, "si".to_string()), Accion::Shift(37));
        m.insert((45, ";".to_string()), Accion::Reduce(73));
        m.insert((91, ";".to_string()), Accion::Reduce(56));
        m.insert((46, "regresa".to_string()), Accion::Reduce(21));
        m.insert((89, "/".to_string()), Accion::Reduce(59));
        m.insert((67, "-".to_string()), Accion::Reduce(39));
        m.insert((63, "-".to_string()), Accion::Reduce(50));
        m.insert((100, "letrero".to_string()), Accion::Shift(97));
        m.insert((91, ",".to_string()), Accion::Reduce(56));
        m.insert((61, "*".to_string()), Accion::Reduce(55));
        m.insert((49, "}".to_string()), Accion::Reduce(14));
        m.insert((100, "cte_flot".to_string()), Accion::Shift(52));
        m.insert((73, "-".to_string()), Accion::Reduce(51));
        m.insert((64, "<".to_string()), Accion::Shift(65));
        m.insert((53, "(".to_string()), Accion::Shift(53));
        m.insert((100, "-".to_string()), Accion::Shift(55));
        m.insert((10, "id".to_string()), Accion::Reduce(64));
        m.insert((58, "==".to_string()), Accion::Reduce(58));
        m.insert((63, "==".to_string()), Accion::Reduce(50));
        m.insert((77, "<".to_string()), Accion::Reduce(49));
        m.insert((38, "(".to_string()), Accion::Shift(96));
        m.insert((79, "+".to_string()), Accion::Reduce(46));
        m.insert((122, "-".to_string()), Accion::Shift(55));
        m.insert((68, "+".to_string()), Accion::Reduce(37));
        m.insert((91, "<".to_string()), Accion::Reduce(56));
        m.insert((95, "]".to_string()), Accion::Reduce(19));
        m.insert((98, ")".to_string()), Accion::Reduce(25));
        m.insert((6, "si".to_string()), Accion::Reduce(3));
        m.insert((131, "letrero".to_string()), Accion::Reduce(2));
        m.insert((136, "id".to_string()), Accion::Reduce(6));
        m.insert((128, "entero".to_string()), Accion::Reduce(63));
        m.insert((84, "cte_flot".to_string()), Accion::Shift(52));
        m.insert((12, "inicio".to_string()), Accion::Reduce(5));
        m.insert((66, "cte_flot".to_string()), Accion::Reduce(40));
        m.insert((50, "mientras".to_string()), Accion::Reduce(13));
        m.insert((56, ">".to_string()), Accion::Reduce(41));
        m.insert((77, "!=".to_string()), Accion::Reduce(49));
        m.insert((117, "}".to_string()), Accion::Reduce(29));
        m.insert((75, "(".to_string()), Accion::Shift(53));
        m.insert((126, "}".to_string()), Accion::Reduce(30));
        m.insert((29, "[".to_string()), Accion::Reduce(3));
        m.insert((58, "+".to_string()), Accion::Reduce(58));
        m.insert((45, "+".to_string()), Accion::Reduce(73));
        m.insert((51, "-".to_string()), Accion::Shift(55));
        m.insert((63, ">".to_string()), Accion::Reduce(50));
        m.insert((75, "-".to_string()), Accion::Shift(55));
        m.insert((83, ";".to_string()), Accion::Reduce(44));
        m.insert((52, ")".to_string()), Accion::Reduce(42));
        m.insert((113, "mientras".to_string()), Accion::Reduce(31));
        m.insert((35, "id".to_string()), Accion::Shift(36));
        m.insert((17, "(".to_string()), Accion::Shift(18));
        m.insert((8, "id".to_string()), Accion::Reduce(11));
        m.insert((14, "{".to_string()), Accion::Shift(29));
        m.insert((88, "*".to_string()), Accion::Reduce(72));
        m.insert((68, "cte_flot".to_string()), Accion::Reduce(37));
        m.insert((26, ")".to_string()), Accion::Reduce(68));
        m.insert((29, "id".to_string()), Accion::Reduce(3));
        m.insert((124, "haz".to_string()), Accion::Shift(125));
        m.insert((138, ":".to_string()), Accion::Reduce(7));
        m.insert((48, "regresa".to_string()), Accion::Reduce(18));
        m.insert((61, ">".to_string()), Accion::Reduce(55));
        m.insert((42, "}".to_string()), Accion::Shift(50));
        m.insert((113, "}".to_string()), Accion::Reduce(31));
        m.insert((80, "+".to_string()), Accion::Shift(57));
        m.insert((86, ")".to_string()), Accion::Reduce(77));
        m.insert((74, "==".to_string()), Accion::Reduce(48));
        m.insert((74, ")".to_string()), Accion::Reduce(48));
        m.insert((78, "id".to_string()), Accion::Reduce(47));
        m.insert((7, "letrero".to_string()), Accion::Shift(11));
        m.insert((54, "+".to_string()), Accion::Reduce(54));
        m.insert((34, "(".to_string()), Accion::Shift(53));
        m.insert((120, ";".to_string()), Accion::Shift(121));
        m.insert((44, "mientras".to_string()), Accion::Reduce(16));
        m.insert((56, ")".to_string()), Accion::Reduce(41));
        m.insert((13, "id".to_string()), Accion::Shift(129));
        m.insert((27, ")".to_string()), Accion::Reduce(69));
        m.insert((35, "]".to_string()), Accion::Reduce(15));
        m.insert((73, "+".to_string()), Accion::Reduce(51));
        m.insert((56, ";".to_string()), Accion::Reduce(41));
        m.insert((56, "-".to_string()), Accion::Reduce(41));
        m.insert((67, "(".to_string()), Accion::Reduce(39));
        m.insert((76, "==".to_string()), Accion::Reduce(50));
        m.insert((72, "-".to_string()), Accion::Reduce(52));
        m.insert((78, "-".to_string()), Accion::Reduce(47));
        m.insert((60, ",".to_string()), Accion::Shift(84));
        m.insert((4, "si".to_string()), Accion::Reduce(3));
        m.insert((69, ")".to_string()), Accion::Reduce(34));
        m.insert((80, "(".to_string()), Accion::Shift(53));
        m.insert((90, "+".to_string()), Accion::Reduce(60));
        m.insert((32, "}".to_string()), Accion::Reduce(15));
        m.insert((94, "-".to_string()), Accion::Reduce(53));
        m.insert((96, "id".to_string()), Accion::Shift(54));
        m.insert((101, ")".to_string()), Accion::Shift(102));
        m.insert((134, "entero".to_string()), Accion::Shift(9));
        m.insert((117, "escribe".to_string()), Accion::Reduce(29));
        m.insert((23, "entero".to_string()), Accion::Shift(9));
        m.insert((89, "*".to_string()), Accion::Reduce(59));
        m.insert((54, ")".to_string()), Accion::Reduce(54));
        m.insert((3, ";".to_string()), Accion::Shift(4));
        m.insert((44, "[".to_string()), Accion::Reduce(16));
        m.insert((68, "id".to_string()), Accion::Reduce(37));
        m.insert((65, "cte_ent".to_string()), Accion::Reduce(38));
        m.insert((112, ";".to_string()), Accion::Shift(113));
        m.insert((135, ";".to_string()), Accion::Shift(136));
        m.insert((71, ")".to_string()), Accion::Reduce(35));
        m.insert((4, "regresa".to_string()), Accion::Reduce(3));
        m.insert((128, "flotante".to_string()), Accion::Reduce(63));
        m.insert((94, "<".to_string()), Accion::Reduce(53));
        m.insert((6, "letrero".to_string()), Accion::Reduce(3));
        m.insert((76, ">".to_string()), Accion::Reduce(50));
        m.insert((81, ";".to_string()), Accion::Reduce(43));
        m.insert((88, ")".to_string()), Accion::Reduce(72));
        m.insert((62, "<".to_string()), Accion::Reduce(45));
        m.insert((117, "id".to_string()), Accion::Reduce(29));
        m.insert((90, ";".to_string()), Accion::Reduce(60));
        m.insert((136, "vars".to_string()), Accion::Reduce(6));
        m.insert((54, "/".to_string()), Accion::Reduce(54));
        m.insert((88, ",".to_string()), Accion::Reduce(72));
        m.insert((11, ";".to_string()), Accion::Reduce(12));
        m.insert((78, "(".to_string()), Accion::Reduce(47));
        m.insert((92, "*".to_string()), Accion::Reduce(57));
        m.insert((58, ">".to_string()), Accion::Reduce(58));
        m.insert((131, "si".to_string()), Accion::Reduce(2));
        m.insert((44, "id".to_string()), Accion::Reduce(16));
        m.insert((59, ")".to_string()), Accion::Shift(88));
        m.insert((47, "si".to_string()), Accion::Reduce(17));
        m.insert((75, "cte_ent".to_string()), Accion::Shift(56));
        m.insert((74, "!=".to_string()), Accion::Reduce(48));
        m.insert((109, "{".to_string()), Accion::Shift(29));
        m.insert((79, "cte_flot".to_string()), Accion::Reduce(46));
        m.insert((110, "sino".to_string()), Accion::Shift(111));
        m.insert((47, "escribe".to_string()), Accion::Reduce(17));
        m.insert((64, "==".to_string()), Accion::Shift(67));
        m.insert((82, ",".to_string()), Accion::Reduce(45));
        m.insert((46, "[".to_string()), Accion::Reduce(21));
        m.insert((103, "]".to_string()), Accion::Reduce(24));
        m.insert((119, "mientras".to_string()), Accion::Reduce(22));
        m.insert((90, "/".to_string()), Accion::Reduce(60));
        m.insert((126, "si".to_string()), Accion::Reduce(30));
        m.insert((131, "regresa".to_string()), Accion::Reduce(2));
        m.insert((54, ";".to_string()), Accion::Reduce(54));
        m.insert((51, "+".to_string()), Accion::Shift(57));
        m.insert((56, "+".to_string()), Accion::Reduce(41));
        m.insert((121, "id".to_string()), Accion::Reduce(23));
        m.insert((92, "<".to_string()), Accion::Reduce(57));
        m.insert((84, "+".to_string()), Accion::Shift(57));
        m.insert((53, "id".to_string()), Accion::Shift(54));
        m.insert((9, ";".to_string()), Accion::Reduce(10));
        m.insert((43, "si".to_string()), Accion::Shift(37));
        m.insert((37, "(".to_string()), Accion::Shift(106));
        m.insert((96, "letrero".to_string()), Accion::Shift(97));
        m.insert((67, "cte_ent".to_string()), Accion::Reduce(39));
        m.insert((52, ",".to_string()), Accion::Reduce(42));
        m.insert((139, ",".to_string()), Accion::Shift(137));
        m.insert((92, ")".to_string()), Accion::Reduce(57));
        m.insert((29, "letrero".to_string()), Accion::Reduce(3));
        m.insert((95, "regresa".to_string()), Accion::Reduce(19));
        m.insert((82, "<".to_string()), Accion::Reduce(45));
        m.insert((100, "(".to_string()), Accion::Shift(53));
        m.insert((121, "]".to_string()), Accion::Reduce(23));
        m.insert((91, "!=".to_string()), Accion::Reduce(56));
        m.insert((121, "regresa".to_string()), Accion::Reduce(23));
        m.insert((94, "!=".to_string()), Accion::Reduce(53));
        m.insert((97, ",".to_string()), Accion::Reduce(26));
        m.insert((23, "flotante".to_string()), Accion::Shift(8));
        m.insert((136, "mientras".to_string()), Accion::Reduce(6));
        m.insert((82, "==".to_string()), Accion::Reduce(45));
        m.insert((65, "+".to_string()), Accion::Reduce(38));
        m.insert((78, "+".to_string()), Accion::Reduce(47));
        m.insert((29, "si".to_string()), Accion::Reduce(3));
        m.insert((73, "cte_flot".to_string()), Accion::Reduce(51));
        m.insert((34, "-".to_string()), Accion::Shift(55));
        m.insert((46, "si".to_string()), Accion::Reduce(21));
        m.insert((84, "id".to_string()), Accion::Shift(54));
        m.insert((55, "id".to_string()), Accion::Shift(89));
        m.insert((90, ")".to_string()), Accion::Reduce(60));
        m.insert((44, "}".to_string()), Accion::Reduce(16));
        m.insert((73, "(".to_string()), Accion::Reduce(51));
        m.insert((58, "*".to_string()), Accion::Reduce(58));
        m.insert((74, "+".to_string()), Accion::Reduce(48));
        m.insert((78, "cte_flot".to_string()), Accion::Reduce(47));
        m.insert((32, "]".to_string()), Accion::Reduce(15));
        m.insert((90, ">".to_string()), Accion::Reduce(60));
        m.insert((50, ";".to_string()), Accion::Reduce(13));
        m.insert((113, "id".to_string()), Accion::Reduce(31));
        m.insert((126, "]".to_string()), Accion::Reduce(30));
        m.insert((46, "}".to_string()), Accion::Reduce(21));
        m.insert((92, "==".to_string()), Accion::Reduce(57));
        m.insert((12, "flotante".to_string()), Accion::Shift(8));
        m.insert((92, "+".to_string()), Accion::Reduce(57));
        m.insert((90, "!=".to_string()), Accion::Reduce(60));
        m.insert((47, "[".to_string()), Accion::Reduce(17));
        m.insert((7, "nula".to_string()), Accion::Shift(10));
        m.insert((68, "-".to_string()), Accion::Reduce(37));
        m.insert((136, "letrero".to_string()), Accion::Reduce(6));
        m.insert((106, "+".to_string()), Accion::Shift(57));
        m.insert((43, "id".to_string()), Accion::Shift(36));
        m.insert((136, "flotante".to_string()), Accion::Reduce(6));
        m.insert((6, "inicio".to_string()), Accion::Reduce(3));
        m.insert((61, ";".to_string()), Accion::Reduce(55));
        m.insert((35, "mientras".to_string()), Accion::Shift(33));
        m.insert((50, "]".to_string()), Accion::Reduce(13));
        m.insert((61, "<".to_string()), Accion::Reduce(55));
        m.insert((66, "-".to_string()), Accion::Reduce(40));
        m.insert((77, ")".to_string()), Accion::Reduce(49));
        m.insert((81, "!=".to_string()), Accion::Reduce(43));
        m.insert((89, "!=".to_string()), Accion::Reduce(59));
        m.insert((103, "si".to_string()), Accion::Reduce(24));
        m.insert((105, ")".to_string()), Accion::Reduce(27));
        m.insert((88, "-".to_string()), Accion::Reduce(72));
        m.insert((106, "cte_flot".to_string()), Accion::Shift(52));
        m.insert((108, "entonces".to_string()), Accion::Shift(109));
        m.insert((91, "-".to_string()), Accion::Reduce(56));
        m.insert((119, "id".to_string()), Accion::Reduce(22));
        m.insert((122, "id".to_string()), Accion::Shift(54));
        m.insert((77, "-".to_string()), Accion::Reduce(49));
        m.insert((47, "}".to_string()), Accion::Reduce(17));
        m.insert((41, "(".to_string()), Accion::Shift(51));
        m.insert((32, "escribe".to_string()), Accion::Shift(38));
        m.insert((65, "-".to_string()), Accion::Reduce(38));
        m.insert((92, ";".to_string()), Accion::Reduce(57));
        m.insert((91, ">".to_string()), Accion::Reduce(56));
        m.insert((104, ",".to_string()), Accion::Shift(100));
        m.insert((115, "id".to_string()), Accion::Shift(54));
        m.insert((63, ",".to_string()), Accion::Reduce(50));
        m.insert((50, "}".to_string()), Accion::Reduce(13));
        m.insert((81, ",".to_string()), Accion::Reduce(43));
        m.insert((95, "si".to_string()), Accion::Reduce(19));
        m.insert((128, "letrero".to_string()), Accion::Reduce(63));
        m.insert((128, "nula".to_string()), Accion::Reduce(63));
        m.insert((88, ";".to_string()), Accion::Reduce(72));
        m.insert((92, "-".to_string()), Accion::Reduce(57));
        m.insert((136, "escribe".to_string()), Accion::Reduce(6));
        m.insert((64, ",".to_string()), Accion::Reduce(36));
        m.insert((131, "[".to_string()), Accion::Reduce(2));
        m.insert((57, "cte_flot".to_string()), Accion::Shift(52));
        m.insert((65, "cte_flot".to_string()), Accion::Reduce(38));
        m.insert((80, "id".to_string()), Accion::Shift(54));
        m.insert((89, ";".to_string()), Accion::Reduce(59));
        m.insert((56, ",".to_string()), Accion::Reduce(41));
        m.insert((53, "-".to_string()), Accion::Shift(55));
        m.insert((74, ">".to_string()), Accion::Reduce(48));
        m.insert((99, ",".to_string()), Accion::Shift(100));
        m.insert((88, "/".to_string()), Accion::Reduce(72));
        m.insert((88, "<".to_string()), Accion::Reduce(72));
        m.insert((34, "id".to_string()), Accion::Shift(54));
        m.insert((117, "regresa".to_string()), Accion::Reduce(29));
        m.insert((100, "cte_ent".to_string()), Accion::Shift(56));
        m.insert((30, "fin".to_string()), Accion::Shift(31));
        m.insert((91, "==".to_string()), Accion::Reduce(56));
        m.insert((99, ")".to_string()), Accion::Reduce(28));
        m.insert((45, "==".to_string()), Accion::Reduce(73));
        m.insert((6, "flotante".to_string()), Accion::Reduce(3));
        m.insert((94, "==".to_string()), Accion::Reduce(53));
        m.insert((95, "mientras".to_string()), Accion::Reduce(19));
        m.insert((51, "id".to_string()), Accion::Shift(54));
        m.insert((56, "!=".to_string()), Accion::Reduce(41));
        m.insert((5, "id".to_string()), Accion::Shift(132));
        m.insert((52, ";".to_string()), Accion::Reduce(42));
        m.insert((19, ":".to_string()), Accion::Shift(23));
        m.insert((61, "-".to_string()), Accion::Reduce(55));
        m.insert((6, "nula".to_string()), Accion::Reduce(3));
        m.insert((117, "[".to_string()), Accion::Reduce(29));
        m.insert((119, "si".to_string()), Accion::Reduce(22));
        m.insert((126, "mientras".to_string()), Accion::Reduce(30));
        m.insert((131, "inicio".to_string()), Accion::Reduce(2));
        m.insert((137, "id".to_string()), Accion::Shift(139));
        m.insert((121, "[".to_string()), Accion::Reduce(23));
        m.insert((136, "si".to_string()), Accion::Reduce(6));
        m.insert((52, "+".to_string()), Accion::Reduce(42));
        m.insert((29, "mientras".to_string()), Accion::Reduce(3));
        m.insert((79, "id".to_string()), Accion::Reduce(46));
        m.insert((44, "]".to_string()), Accion::Reduce(16));
        m.insert((73, "id".to_string()), Accion::Reduce(51));
        m.insert((79, "cte_ent".to_string()), Accion::Reduce(46));
        m.insert((79, "-".to_string()), Accion::Reduce(46));
        m.insert((46, "id".to_string()), Accion::Reduce(21));
        m.insert((94, ",".to_string()), Accion::Reduce(53));
        m.insert((8, ",".to_string()), Accion::Reduce(11));
        m.insert((93, ")".to_string()), Accion::Shift(94));
        m.insert((71, ";".to_string()), Accion::Reduce(35));
        m.insert((45, "*".to_string()), Accion::Reduce(73));
        m.insert((63, "/".to_string()), Accion::Shift(72));
        m.insert((113, "escribe".to_string()), Accion::Reduce(31));
        m.insert((20, ")".to_string()), Accion::Shift(22));
        m.insert((82, ";".to_string()), Accion::Reduce(45));
        m.insert((6, "regresa".to_string()), Accion::Reduce(3));
        m.insert((96, "+".to_string()), Accion::Shift(57));
        m.insert((54, "==".to_string()), Accion::Reduce(54));
        m.insert((7, "inicio".to_string()), Accion::Reduce(5));
        m.insert((136, "entero".to_string()), Accion::Reduce(6));
        m.insert((80, "cte_ent".to_string()), Accion::Shift(56));
        m.insert((4, "}".to_string()), Accion::Reduce(3));
        m.insert((40, "escribe".to_string()), Accion::Reduce(20));
        m.insert((70, "id".to_string()), Accion::Shift(54));
        m.insert((136, "[".to_string()), Accion::Reduce(6));
        m.insert((50, "escribe".to_string()), Accion::Reduce(13));
        m.insert((6, "id".to_string()), Accion::Reduce(3));
        m.insert((49, "]".to_string()), Accion::Reduce(14));
        m.insert((94, ";".to_string()), Accion::Reduce(53));
        m.insert((74, "-".to_string()), Accion::Reduce(48));
        m.insert((64, "!=".to_string()), Accion::Shift(66));
        m.insert((52, "*".to_string()), Accion::Reduce(42));
        m.insert((82, ">".to_string()), Accion::Reduce(45));
        m.insert((62, "!=".to_string()), Accion::Reduce(45));
        m.insert((106, "cte_ent".to_string()), Accion::Shift(56));
        m.insert((40, "}".to_string()), Accion::Reduce(20));
        m.insert((45, ",".to_string()), Accion::Reduce(73));
        m.insert((94, "*".to_string()), Accion::Reduce(53));
        m.insert((25, "id".to_string()), Accion::Shift(19));
        m.insert((69, ";".to_string()), Accion::Reduce(34));
        m.insert((106, "id".to_string()), Accion::Shift(54));
        m.insert((58, "/".to_string()), Accion::Reduce(58));
        m.insert((4, "mientras".to_string()), Accion::Reduce(3));
        m.insert((54, "*".to_string()), Accion::Reduce(54));
        m.insert((89, "-".to_string()), Accion::Reduce(59));
        m.insert((97, ")".to_string()), Accion::Reduce(26));
        m.insert((66, "+".to_string()), Accion::Reduce(40));
        m.insert((134, "flotante".to_string()), Accion::Shift(8));
        m.insert((21, ")".to_string()), Accion::Reduce(66));
        m.insert((96, "-".to_string()), Accion::Shift(55));
        m.insert((114, ";".to_string()), Accion::Reduce(32));
        m.insert((9, ",".to_string()), Accion::Reduce(10));
        m.insert((28, "{".to_string()), Accion::Shift(29));
        m.insert((136, "regresa".to_string()), Accion::Reduce(6));
        m.insert((23, "letrero".to_string()), Accion::Shift(11));
        m.insert((29, "regresa".to_string()), Accion::Reduce(3));
        m.insert((71, ",".to_string()), Accion::Reduce(35));
        m.insert((82, "!=".to_string()), Accion::Reduce(45));
        m.insert((131, "id".to_string()), Accion::Reduce(2));
        m.insert((95, "escribe".to_string()), Accion::Reduce(19));
        m.insert((45, "-".to_string()), Accion::Reduce(73));
        m.insert((32, "regresa".to_string()), Accion::Shift(34));
        m.insert((130, "inicio".to_string()), Accion::Reduce(4));
        m.insert((136, "inicio".to_string()), Accion::Reduce(6));
        m.insert((67, "cte_flot".to_string()), Accion::Reduce(39));
        m.insert((2, "$".to_string()), Accion::Accept);
        m.insert((39, ";".to_string()), Accion::Shift(95));
        m.insert((121, "}".to_string()), Accion::Reduce(23));
        m.insert((121, "si".to_string()), Accion::Reduce(23));
        m.insert((55, "cte_ent".to_string()), Accion::Shift(56));
        m.insert((89, "+".to_string()), Accion::Reduce(59));
        m.insert((84, "cte_ent".to_string()), Accion::Shift(56));
        m.insert((44, "si".to_string()), Accion::Reduce(16));
        m.insert((6, "mientras".to_string()), Accion::Reduce(3));
        m.insert((54, "-".to_string()), Accion::Reduce(54));
        m.insert((40, "id".to_string()), Accion::Reduce(20));
        m.insert((50, "[".to_string()), Accion::Reduce(13));
        m.insert((34, "+".to_string()), Accion::Shift(57));
        m.insert((66, "id".to_string()), Accion::Reduce(40));
        m.insert((96, "cte_flot".to_string()), Accion::Shift(52));
        m.insert((72, "(".to_string()), Accion::Reduce(52));
        m.insert((134, "letrero".to_string()), Accion::Shift(11));
        m.insert((45, "!=".to_string()), Accion::Reduce(73));
        m.insert((76, "*".to_string()), Accion::Shift(73));
        m.insert((43, "escribe".to_string()), Accion::Shift(38));
        m.insert((6, "escribe".to_string()), Accion::Reduce(3));
        m.insert((47, "regresa".to_string()), Accion::Reduce(17));
        m.insert((11, "id".to_string()), Accion::Reduce(12));
        m.insert((68, "(".to_string()), Accion::Reduce(37));
        m.insert((94, ")".to_string()), Accion::Reduce(53));
        m.insert((122, "(".to_string()), Accion::Shift(53));
        m.insert((70, "-".to_string()), Accion::Shift(55));
        m.insert((103, "escribe".to_string()), Accion::Reduce(24));
        m.insert((56, "*".to_string()), Accion::Reduce(41));
        m.insert((91, ")".to_string()), Accion::Reduce(56));
        m.insert((24, ",".to_string()), Accion::Shift(25));
        m.insert((46, "mientras".to_string()), Accion::Reduce(21));
        m.insert((76, ")".to_string()), Accion::Reduce(50));
        m.insert((90, "==".to_string()), Accion::Reduce(60));
        m.insert((76, "/".to_string()), Accion::Shift(72));
        m.insert((92, "/".to_string()), Accion::Reduce(57));
        m.insert((40, "[".to_string()), Accion::Reduce(20));
        m.insert((121, "mientras".to_string()), Accion::Reduce(23));
        m.insert((8, ";".to_string()), Accion::Reduce(11));
        m.insert((52, "/".to_string()), Accion::Reduce(42));
        m.insert((29, "}".to_string()), Accion::Reduce(3));
        m.insert((32, "mientras".to_string()), Accion::Shift(33));
        m.insert((118, "]".to_string()), Accion::Shift(119));
        m.insert((63, "*".to_string()), Accion::Shift(73));
        m.insert((98, ",".to_string()), Accion::Reduce(25));
        m.insert((81, "==".to_string()), Accion::Reduce(43));
        m.insert((64, ">".to_string()), Accion::Shift(68));
        m.insert((57, "cte_ent".to_string()), Accion::Shift(56));
        m.insert((40, "si".to_string()), Accion::Reduce(20));
        m.insert((63, ")".to_string()), Accion::Reduce(50));
        m.insert((70, "cte_flot".to_string()), Accion::Shift(52));
        m.insert((43, "regresa".to_string()), Accion::Shift(34));
        m.insert((7, "flotante".to_string()), Accion::Shift(8));
        m.insert((126, "regresa".to_string()), Accion::Reduce(30));
        m.insert((47, "id".to_string()), Accion::Reduce(17));
        m.insert((45, "<".to_string()), Accion::Reduce(73));
        m.insert((113, "regresa".to_string()), Accion::Reduce(31));
        m.insert((136, "}".to_string()), Accion::Reduce(6));
        m.insert((15, "inicio".to_string()), Accion::Shift(28));
        m.insert((62, ")".to_string()), Accion::Reduce(45));
        m.insert((64, ";".to_string()), Accion::Reduce(36));
        m.insert((67, "id".to_string()), Accion::Reduce(39));
        m.insert((83, ">".to_string()), Accion::Reduce(44));
        m.insert((29, "nula".to_string()), Accion::Reduce(3));
        m.insert((57, "id".to_string()), Accion::Shift(89));
        m.insert((76, ",".to_string()), Accion::Reduce(50));
        m.insert((78, "cte_ent".to_string()), Accion::Reduce(47));
        m.insert((82, "-".to_string()), Accion::Shift(78));
        m.insert((52, ">".to_string()), Accion::Reduce(42));
        m.insert((89, "==".to_string()), Accion::Reduce(59));
        m.insert((94, "+".to_string()), Accion::Reduce(53));
        m.insert((139, ":".to_string()), Accion::Reduce(9));
        m.insert((88, "==".to_string()), Accion::Reduce(72));
        m.insert((4, "letrero".to_string()), Accion::Reduce(3));
        m.insert((94, ">".to_string()), Accion::Reduce(53));
        m.insert((94, "/".to_string()), Accion::Reduce(53));
        m.insert((117, "mientras".to_string()), Accion::Reduce(29));
        m.insert((131, "escribe".to_string()), Accion::Reduce(2));
        m.insert((103, "regresa".to_string()), Accion::Reduce(24));
        m.insert((40, "regresa".to_string()), Accion::Reduce(20));
        m.insert((91, "+".to_string()), Accion::Reduce(56));
        m.insert((66, "cte_ent".to_string()), Accion::Reduce(40));
        m.insert((32, "[".to_string()), Accion::Shift(35));
        m.insert((140, ":".to_string()), Accion::Reduce(8));
        m.insert((65, "(".to_string()), Accion::Reduce(38));
        m.insert((4, "flotante".to_string()), Accion::Reduce(3));
        m.insert((73, "cte_ent".to_string()), Accion::Reduce(51));
        m.insert((119, "regresa".to_string()), Accion::Reduce(22));
        m.insert((67, "+".to_string()), Accion::Reduce(39));
        m.insert((6, "}".to_string()), Accion::Reduce(3));
        m.insert((45, "/".to_string()), Accion::Reduce(73));
        m.insert((48, "si".to_string()), Accion::Reduce(18));
        m.insert((61, ",".to_string()), Accion::Reduce(55));
        m.insert((58, "<".to_string()), Accion::Reduce(58));
        m.insert((63, "+".to_string()), Accion::Reduce(50));
        m.insert((129, "(".to_string()), Accion::Reduce(61));
        m.insert((1, "id".to_string()), Accion::Shift(3));
        m.insert((100, "+".to_string()), Accion::Shift(57));
        m.insert((76, "-".to_string()), Accion::Reduce(50));
        m.insert((51, "cte_ent".to_string()), Accion::Shift(56));
        m.insert((83, "==".to_string()), Accion::Reduce(44));
        m.insert((132, ":".to_string()), Accion::Reduce(9));
        m.insert((123, ")".to_string()), Accion::Shift(124));
        m.insert((12, "letrero".to_string()), Accion::Shift(11));
        m.insert((18, "id".to_string()), Accion::Shift(19));
        m.insert((4, "[".to_string()), Accion::Reduce(3));
        m.insert((16, "id".to_string()), Accion::Reduce(65));
        m.insert((56, "<".to_string()), Accion::Reduce(41));
        m.insert((119, "escribe".to_string()), Accion::Reduce(22));
        m.insert((18, ")".to_string()), Accion::Reduce(67));
        m.insert((43, "[".to_string()), Accion::Shift(35));
        m.insert((86, ",".to_string()), Accion::Shift(84));
        m.insert((115, "(".to_string()), Accion::Shift(53));
        m.insert((11, ")".to_string()), Accion::Reduce(12));
        m.insert((46, "escribe".to_string()), Accion::Reduce(21));
        m.insert((63, "!=".to_string()), Accion::Reduce(50));
        m.insert((6, "[".to_string()), Accion::Reduce(3));
        m.insert((132, ",".to_string()), Accion::Shift(137));
        m.insert((48, "}".to_string()), Accion::Reduce(18));
        m.insert((81, ")".to_string()), Accion::Reduce(43));
        m.insert((31, "$".to_string()), Accion::Reduce(1));
        m.insert((74, ",".to_string()), Accion::Reduce(48));
        m.insert((90, "*".to_string()), Accion::Reduce(60));
        m.insert((77, ",".to_string()), Accion::Reduce(49));
        m.insert((4, "vars".to_string()), Accion::Shift(5));
        m.insert((90, "<".to_string()), Accion::Reduce(60));
        m.insert((88, ">".to_string()), Accion::Reduce(72));
        m.insert((51, "(".to_string()), Accion::Shift(53));
        m.insert((52, "==".to_string()), Accion::Reduce(42));
        m.insert((54, ",".to_string()), Accion::Reduce(54));
        m.insert((89, ",".to_string()), Accion::Reduce(59));
        m.insert((29, "flotante".to_string()), Accion::Reduce(3));
        m.insert((44, "escribe".to_string()), Accion::Reduce(16));
        m.insert((47, "]".to_string()), Accion::Reduce(17));
        m.insert((103, "}".to_string()), Accion::Reduce(24));
        m.insert((47, "mientras".to_string()), Accion::Reduce(17));
        m.insert((110, ";".to_string()), Accion::Reduce(33));
        m.insert((81, ">".to_string()), Accion::Reduce(43));
        m.insert((75, "+".to_string()), Accion::Shift(57));
        m.insert((7, "entero".to_string()), Accion::Shift(9));
        m.insert((75, "id".to_string()), Accion::Shift(54));
        m.insert((35, "si".to_string()), Accion::Shift(37));
        m.insert((52, "<".to_string()), Accion::Reduce(42));
        m.insert((84, "-".to_string()), Accion::Shift(55));
        m.insert((61, "!=".to_string()), Accion::Reduce(55));
        m.insert((52, "-".to_string()), Accion::Reduce(42));
        m.insert((6, "entero".to_string()), Accion::Reduce(3));
        m.insert((62, "+".to_string()), Accion::Shift(79));
        m.insert((82, "+".to_string()), Accion::Shift(79));
        m.insert((95, "[".to_string()), Accion::Reduce(19));
        m.insert((82, ")".to_string()), Accion::Reduce(45));
        m.insert((33, "(".to_string()), Accion::Shift(122));
        m.insert((53, "+".to_string()), Accion::Shift(57));
        m.insert((68, "cte_ent".to_string()), Accion::Reduce(37));
        m.insert((103, "[".to_string()), Accion::Reduce(24));
        m.insert((122, "cte_ent".to_string()), Accion::Shift(56));
        m.insert((131, "mientras".to_string()), Accion::Reduce(2));
        m.insert((77, "+".to_string()), Accion::Reduce(49));
        m.insert((88, "!=".to_string()), Accion::Reduce(72));
        m.insert((104, ")".to_string()), Accion::Reduce(28));
        m.insert((48, "mientras".to_string()), Accion::Reduce(18));
        m.insert((115, "+".to_string()), Accion::Shift(57));
        m.insert((62, "-".to_string()), Accion::Shift(78));
        m.insert((9, ")".to_string()), Accion::Reduce(10));
        m.insert((92, ">".to_string()), Accion::Reduce(57));
        m.insert((72, "+".to_string()), Accion::Reduce(52));
        m.insert((113, "[".to_string()), Accion::Reduce(31));
        m.insert((29, "vars".to_string()), Accion::Shift(5));
        m.insert((56, "/".to_string()), Accion::Reduce(41));
        m.insert((95, "}".to_string()), Accion::Reduce(19));
        m.insert((90, ",".to_string()), Accion::Reduce(60));
        m.insert((4, "id".to_string()), Accion::Reduce(3));
        m.insert((29, "inicio".to_string()), Accion::Reduce(3));
        m.insert((58, ";".to_string()), Accion::Reduce(58));
        m.insert((35, "regresa".to_string()), Accion::Shift(34));
        m.insert((113, "si".to_string()), Accion::Reduce(31));
        m.insert((83, ")".to_string()), Accion::Reduce(44));
        m.insert((40, "]".to_string()), Accion::Reduce(20));
        m.insert((61, ")".to_string()), Accion::Reduce(55));
        m.insert((96, "cte_ent".to_string()), Accion::Shift(56));
        m.insert((69, ",".to_string()), Accion::Reduce(34));
        m.insert((74, "<".to_string()), Accion::Reduce(48));
        m.insert((54, "(".to_string()), Accion::Reduce(71));
        m.insert((64, ")".to_string()), Accion::Reduce(36));
        m.insert((83, "!=".to_string()), Accion::Reduce(44));
        m.insert((87, ")".to_string()), Accion::Reduce(76));
        m.insert((46, "]".to_string()), Accion::Reduce(21));
        m.insert((84, "(".to_string()), Accion::Shift(53));
        m.insert((45, ")".to_string()), Accion::Reduce(73));
        m.insert((72, "id".to_string()), Accion::Reduce(52));
        m.insert((91, "/".to_string()), Accion::Reduce(56));
        m.insert((35, "[".to_string()), Accion::Shift(35));
        m.insert((51, "cte_flot".to_string()), Accion::Shift(52));
        m.insert((58, "!=".to_string()), Accion::Reduce(58));
        m.insert((61, "==".to_string()), Accion::Reduce(55));
        m.insert((45, ">".to_string()), Accion::Reduce(73));
        m.insert((102, ";".to_string()), Accion::Shift(103));
        m.insert((126, "id".to_string()), Accion::Reduce(30));
        m.insert((48, "]".to_string()), Accion::Reduce(18));
        m.insert((85, ")".to_string()), Accion::Reduce(74));
        m.insert((62, ",".to_string()), Accion::Reduce(45));
        m.insert((127, ";".to_string()), Accion::Shift(128));
        m.insert((61, "/".to_string()), Accion::Reduce(55));
        m.insert((34, "cte_ent".to_string()), Accion::Shift(56));
        m.insert((70, "+".to_string()), Accion::Shift(57));
        m.insert((83, ",".to_string()), Accion::Reduce(44));
        m.insert((76, "!=".to_string()), Accion::Reduce(50));
        m.insert((119, "[".to_string()), Accion::Reduce(22));
        m.insert((4, "nula".to_string()), Accion::Reduce(3));
        m.insert((106, "-".to_string()), Accion::Shift(55));
        m.insert((92, "!=".to_string()), Accion::Reduce(57));
        m.insert((131, "}".to_string()), Accion::Reduce(2));
        m.insert((79, "(".to_string()), Accion::Reduce(46));
        m.insert((121, "escribe".to_string()), Accion::Reduce(23));
        m.insert((72, "cte_flot".to_string()), Accion::Reduce(52));
        m.insert((43, "}".to_string()), Accion::Reduce(15));
        m.insert((131, "nula".to_string()), Accion::Reduce(2));
        m.insert((35, "escribe".to_string()), Accion::Shift(38));
        m.insert((58, ")".to_string()), Accion::Reduce(58));
        m.insert((111, "{".to_string()), Accion::Shift(29));
        m.insert((53, "cte_flot".to_string()), Accion::Shift(52));
        m.insert((0, "programa".to_string()), Accion::Shift(1));
        m.insert((77, ";".to_string()), Accion::Reduce(49));
        m.insert((96, "(".to_string()), Accion::Shift(53));
        m.insert((106, "(".to_string()), Accion::Shift(53));
        m.insert((119, "]".to_string()), Accion::Reduce(22));
        m.insert((22, "{".to_string()), Accion::Reduce(62));
        m.insert((12, "nula".to_string()), Accion::Shift(10));
        m.insert((89, "<".to_string()), Accion::Reduce(59));
        m.insert((4, "entero".to_string()), Accion::Reduce(3));
        m.insert((50, "fin".to_string()), Accion::Reduce(13));
        m.insert((62, "==".to_string()), Accion::Reduce(45));
        m.insert((4, "inicio".to_string()), Accion::Reduce(3));
        m.insert((89, ">".to_string()), Accion::Reduce(59));
        m.insert((50, "sino".to_string()), Accion::Reduce(13));
        m.insert((90, "-".to_string()), Accion::Reduce(60));
        m.insert((115, "cte_flot".to_string()), Accion::Shift(52));
        m.insert((122, "cte_flot".to_string()), Accion::Shift(52));
        m.insert((117, "si".to_string()), Accion::Reduce(29));
        m.insert((115, "-".to_string()), Accion::Shift(55));
        m.insert((91, "*".to_string()), Accion::Reduce(56));
        m.insert((126, "[".to_string()), Accion::Reduce(30));
        m.insert((77, ">".to_string()), Accion::Reduce(49));
        m.insert((50, "si".to_string()), Accion::Reduce(13));
        m.insert((34, "cte_flot".to_string()), Accion::Shift(52));
        m.insert((12, "entero".to_string()), Accion::Shift(9));
        m.insert((83, "<".to_string()), Accion::Reduce(44));
        m.insert((131, "entero".to_string()), Accion::Reduce(2));
        m.insert((32, "id".to_string()), Accion::Shift(36));
        m.insert((133, ":".to_string()), Accion::Shift(134));
        m.insert((58, "-".to_string()), Accion::Reduce(58));
        m.insert((54, "!=".to_string()), Accion::Reduce(54));
        m.insert((126, "escribe".to_string()), Accion::Reduce(30));
        m.insert((54, "<".to_string()), Accion::Reduce(54));
        m.insert((76, "+".to_string()), Accion::Reduce(50));
        m.insert((62, ">".to_string()), Accion::Reduce(45));
        m.insert((8, ")".to_string()), Accion::Reduce(11));
        m.insert((65, "id".to_string()), Accion::Reduce(38));
        m.insert((119, "}".to_string()), Accion::Reduce(22));
        m.insert((100, "id".to_string()), Accion::Shift(54));
        m.insert((29, "escribe".to_string()), Accion::Reduce(3));
        m.insert((136, "nula".to_string()), Accion::Reduce(6));
        m.insert((56, "==".to_string()), Accion::Reduce(41));
        m.insert((88, "+".to_string()), Accion::Reduce(72));
        m.insert((40, "mientras".to_string()), Accion::Reduce(20));
        m.insert((95, "id".to_string()), Accion::Reduce(19));
        m.insert((131, "flotante".to_string()), Accion::Reduce(2));
        m.insert((113, "]".to_string()), Accion::Reduce(31));
        m.insert((77, "==".to_string()), Accion::Reduce(49));
        m.insert((24, ")".to_string()), Accion::Reduce(70));
        m.insert((43, "]".to_string()), Accion::Reduce(15));
        m.insert((50, "id".to_string()), Accion::Reduce(13));
        m.insert((48, "escribe".to_string()), Accion::Reduce(18));
        m.insert((76, "<".to_string()), Accion::Reduce(50));
        m.insert((103, "mientras".to_string()), Accion::Reduce(24));
        m.insert((11, ",".to_string()), Accion::Reduce(12));
        m.insert((36, "(".to_string()), Accion::Reduce(71));
        m.insert((29, "entero".to_string()), Accion::Reduce(3));
        m.insert((80, "-".to_string()), Accion::Shift(55));
        m.insert((55, "cte_flot".to_string()), Accion::Shift(52));
        m.insert((4, "escribe".to_string()), Accion::Reduce(3));
        m.insert((9, "id".to_string()), Accion::Reduce(10));
        m.insert((44, "regresa".to_string()), Accion::Reduce(16));
        m.insert((52, "!=".to_string()), Accion::Reduce(42));
        m.insert((48, "[".to_string()), Accion::Reduce(18));
        m.insert((60, ")".to_string()), Accion::Reduce(77));
        m.insert((116, ";".to_string()), Accion::Shift(117));
        m.insert((62, ";".to_string()), Accion::Reduce(45));
        m.insert((117, "]".to_string()), Accion::Reduce(29));
        m.insert((76, ";".to_string()), Accion::Reduce(50));
        m.insert((89, ")".to_string()), Accion::Reduce(59));
        m.insert((122, "+".to_string()), Accion::Shift(57));
        m.insert((50, "regresa".to_string()), Accion::Reduce(13));
        m.insert((103, "id".to_string()), Accion::Reduce(24));
        m.insert((128, "inicio".to_string()), Accion::Reduce(63));
        m.insert((58, ",".to_string()), Accion::Reduce(58));
        m.insert((80, "cte_flot".to_string()), Accion::Shift(52));
        m.insert((70, "(".to_string()), Accion::Shift(53));
        m.insert((43, "mientras".to_string()), Accion::Shift(33));
        m.insert((72, "cte_ent".to_string()), Accion::Reduce(52));
        m.insert((36, "=".to_string()), Accion::Shift(115));
        m.insert((75, "cte_flot".to_string()), Accion::Shift(52));
        m.insert((53, "cte_ent".to_string()), Accion::Shift(56));
        m.insert((74, ";".to_string()), Accion::Reduce(48));
        m.insert((81, "<".to_string()), Accion::Reduce(43));
        m.insert((125, "{".to_string()), Accion::Shift(29));
        m.insert((63, ";".to_string()), Accion::Reduce(50));
        m.insert((70, "cte_ent".to_string()), Accion::Shift(56));
        m.insert((61, "+".to_string()), Accion::Reduce(55));
        m.insert((63, "<".to_string()), Accion::Reduce(50));
        m.insert((66, "(".to_string()), Accion::Reduce(40));
        m.insert((107, ")".to_string()), Accion::Shift(108));
        m.insert((54, ">".to_string()), Accion::Reduce(54));
        m.insert((48, "id".to_string()), Accion::Reduce(18));
        m.insert((35, "}".to_string()), Accion::Reduce(15));
        m.insert((6, "vars".to_string()), Accion::Shift(5));
        m.insert((115, "cte_ent".to_string()), Accion::Shift(56));
        m.insert((92, ",".to_string()), Accion::Reduce(57));
        m
    };

    /// Tabla GOTO: (estado, no-terminal) -> estado_destino
    pub static ref TABLA_GOTO: HashMap<(usize, String), usize> = {
        let mut m = HashMap::new();
        m.insert((75, "<LLAMADA_ARGS>".to_string()), 45);
        m.insert((29, "<VARS_OPT>".to_string()), 32);
        m.insert((7, "<FUNCS_LIST>".to_string()), 15);
        m.insert((51, "<LLAMADA>".to_string()), 58);
        m.insert((84, "<TÉRMINO>".to_string()), 62);
        m.insert((84, "<CTE>".to_string()), 61);
        m.insert((100, "<CTE>".to_string()), 61);
        m.insert((96, "<OBJ_IMPRIME>".to_string()), 99);
        m.insert((96, "<EXP>".to_string()), 64);
        m.insert((0, "<Programa>".to_string()), 2);
        m.insert((32, "<IMPRIME>".to_string()), 40);
        m.insert((43, "<ESTATUTO>".to_string()), 43);
        m.insert((106, "<TÉRMINO>".to_string()), 62);
        m.insert((70, "<LLAMADA>".to_string()), 58);
        m.insert((115, "<EXP>".to_string()), 64);
        m.insert((32, "<ESTATUTO_LIST>".to_string()), 42);
        m.insert((32, "<ASIGNA>".to_string()), 44);
        m.insert((35, "<IMPRIME>".to_string()), 40);
        m.insert((80, "<LLAMADA_HEADER>".to_string()), 41);
        m.insert((106, "<LLAMADA_HEADER>".to_string()), 41);
        m.insert((122, "<LLAMADA_HEADER>".to_string()), 41);
        m.insert((57, "<CTE>".to_string()), 90);
        m.insert((100, "<LLAMADA_HEADER>".to_string()), 41);
        m.insert((115, "<LLAMADA>".to_string()), 58);
        m.insert((80, "<TÉRMINO>".to_string()), 82);
        m.insert((64, "<EXPRESION_PRIMA>".to_string()), 69);
        m.insert((34, "<FACTOR>".to_string()), 63);
        m.insert((51, "<EXPRESIÓN_OPT>".to_string()), 59);
        m.insert((84, "<LLAMADA_ARGS>".to_string()), 45);
        m.insert((122, "<EXP>".to_string()), 64);
        m.insert((34, "<LLAMADA_ARGS>".to_string()), 45);
        m.insert((122, "<CTE>".to_string()), 61);
        m.insert((43, "<RETURN>".to_string()), 46);
        m.insert((70, "<LLAMADA_HEADER>".to_string()), 41);
        m.insert((70, "<FACTOR>".to_string()), 63);
        m.insert((6, "<VARS_OPT>".to_string()), 131);
        m.insert((100, "<LLAMADA>".to_string()), 58);
        m.insert((25, "<ARG_LIST>".to_string()), 27);
        m.insert((43, "<IMPRIME>".to_string()), 40);
        m.insert((115, "<LLAMADA_ARGS>".to_string()), 45);
        m.insert((122, "<FACTOR>".to_string()), 63);
        m.insert((24, "<ARG_LIST_PRIMA>".to_string()), 26);
        m.insert((80, "<LLAMADA>".to_string()), 58);
        m.insert((43, "<LLAMADA_ARGS>".to_string()), 45);
        m.insert((100, "<EXPRESIÓN>".to_string()), 98);
        m.insert((34, "<LLAMADA>".to_string()), 58);
        m.insert((75, "<LLAMADA>".to_string()), 58);
        m.insert((32, "<LLAMADA>".to_string()), 39);
        m.insert((76, "<TERMINO_PRIMA>".to_string()), 77);
        m.insert((35, "<CONDICIÓN>".to_string()), 47);
        m.insert((80, "<FACTOR>".to_string()), 63);
        m.insert((64, "<OPERADOR>".to_string()), 70);
        m.insert((32, "<ESTATUTO>".to_string()), 43);
        m.insert((43, "<LLAMADA_HEADER>".to_string()), 41);
        m.insert((96, "<LLAMADA_ARGS>".to_string()), 45);
        m.insert((100, "<OBJ_IMPRIME>".to_string()), 104);
        m.insert((63, "<*/>".to_string()), 75);
        m.insert((60, "<EXPRESIÓN_LIST>".to_string()), 85);
        m.insert((106, "<LLAMADA>".to_string()), 58);
        m.insert((51, "<LLAMADA_ARGS>".to_string()), 45);
        m.insert((122, "<TÉRMINO>".to_string()), 62);
        m.insert((125, "<CUERPO>".to_string()), 126);
        m.insert((7, "<TIPO_OPT>".to_string()), 13);
        m.insert((51, "<EXPRESIÓN>".to_string()), 60);
        m.insert((32, "<CONDICIÓN>".to_string()), 47);
        m.insert((23, "<TIPO>".to_string()), 24);
        m.insert((53, "<LLAMADA>".to_string()), 58);
        m.insert((63, "<TERMINO_PRIMA>".to_string()), 74);
        m.insert((82, "<+->".to_string()), 80);
        m.insert((35, "<ASIGNA>".to_string()), 44);
        m.insert((18, "<ARG_LIST>".to_string()), 21);
        m.insert((5, "<VAR_LIST>".to_string()), 133);
        m.insert((53, "<EXP>".to_string()), 64);
        m.insert((75, "<CTE>".to_string()), 61);
        m.insert((4, "<VARS_OPT>".to_string()), 7);
        m.insert((53, "<EXPRESIÓN>".to_string()), 93);
        m.insert((62, "<EXP_PRIMA>".to_string()), 81);
        m.insert((62, "<+->".to_string()), 80);
        m.insert((70, "<LLAMADA_ARGS>".to_string()), 45);
        m.insert((100, "<LLAMADA_ARGS>".to_string()), 45);
        m.insert((35, "<CICLO>".to_string()), 48);
        m.insert((34, "<EXP>".to_string()), 64);
        m.insert((35, "<RETURN>".to_string()), 46);
        m.insert((53, "<LLAMADA_HEADER>".to_string()), 41);
        m.insert((35, "<LLAMADA>".to_string()), 39);
        m.insert((70, "<CTE>".to_string()), 61);
        m.insert((35, "<LLAMADA_HEADER>".to_string()), 41);
        m.insert((115, "<EXPRESIÓN>".to_string()), 116);
        m.insert((70, "<TÉRMINO>".to_string()), 62);
        m.insert((34, "<CTE>".to_string()), 61);
        m.insert((29, "<VARS>".to_string()), 6);
        m.insert((32, "<CICLO>".to_string()), 48);
        m.insert((32, "<RETURN>".to_string()), 46);
        m.insert((4, "<VARS>".to_string()), 6);
        m.insert((34, "<LLAMADA_HEADER>".to_string()), 41);
        m.insert((35, "<LLAMADA_ARGS>".to_string()), 45);
        m.insert((51, "<TÉRMINO>".to_string()), 62);
        m.insert((75, "<LLAMADA_HEADER>".to_string()), 41);
        m.insert((80, "<CTE>".to_string()), 61);
        m.insert((96, "<LLAMADA_HEADER>".to_string()), 41);
        m.insert((100, "<EXP>".to_string()), 64);
        m.insert((115, "<TÉRMINO>".to_string()), 62);
        m.insert((43, "<ASIGNA>".to_string()), 44);
        m.insert((104, "<IMPRIME_LIST>".to_string()), 105);
        m.insert((84, "<EXPRESIÓN>".to_string()), 86);
        m.insert((115, "<LLAMADA_HEADER>".to_string()), 41);
        m.insert((115, "<CTE>".to_string()), 61);
        m.insert((84, "<EXP>".to_string()), 64);
        m.insert((122, "<LLAMADA_ARGS>".to_string()), 45);
        m.insert((51, "<EXP>".to_string()), 64);
        m.insert((100, "<TÉRMINO>".to_string()), 62);
        m.insert((96, "<TÉRMINO>".to_string()), 62);
        m.insert((43, "<CONDICIÓN>".to_string()), 47);
        m.insert((55, "<CTE_OPT>".to_string()), 92);
        m.insert((7, "<FUNC_ARGS>".to_string()), 14);
        m.insert((106, "<EXP>".to_string()), 64);
        m.insert((7, "<TIPO>".to_string()), 16);
        m.insert((12, "<TIPO>".to_string()), 16);
        m.insert((122, "<EXPRESIÓN>".to_string()), 123);
        m.insert((111, "<CUERPO>".to_string()), 114);
        m.insert((35, "<ESTATUTO>".to_string()), 43);
        m.insert((57, "<CTE_OPT>".to_string()), 91);
        m.insert((84, "<LLAMADA>".to_string()), 58);
        m.insert((6, "<VARS>".to_string()), 6);
        m.insert((28, "<CUERPO>".to_string()), 30);
        m.insert((51, "<CTE>".to_string()), 61);
        m.insert((35, "<ESTATUTO_LIST>".to_string()), 118);
        m.insert((51, "<LLAMADA_HEADER>".to_string()), 41);
        m.insert((53, "<CTE>".to_string()), 61);
        m.insert((80, "<LLAMADA_ARGS>".to_string()), 45);
        m.insert((70, "<EXP>".to_string()), 71);
        m.insert((53, "<FACTOR>".to_string()), 63);
        m.insert((82, "<EXP_PRIMA>".to_string()), 83);
        m.insert((96, "<LLAMADA>".to_string()), 58);
        m.insert((115, "<FACTOR>".to_string()), 63);
        m.insert((134, "<TIPO>".to_string()), 135);
        m.insert((132, "<VAR_LIST_PRIMA>".to_string()), 138);
        m.insert((139, "<VAR_LIST_PRIMA>".to_string()), 140);
        m.insert((84, "<FACTOR>".to_string()), 63);
        m.insert((106, "<CTE>".to_string()), 61);
        m.insert((32, "<LLAMADA_HEADER>".to_string()), 41);
        m.insert((32, "<LLAMADA_ARGS>".to_string()), 45);
        m.insert((55, "<CTE>".to_string()), 90);
        m.insert((75, "<FACTOR>".to_string()), 76);
        m.insert((12, "<FUNCS_LIST>".to_string()), 130);
        m.insert((43, "<LLAMADA>".to_string()), 39);
        m.insert((53, "<LLAMADA_ARGS>".to_string()), 45);
        m.insert((96, "<FACTOR>".to_string()), 63);
        m.insert((100, "<FACTOR>".to_string()), 63);
        m.insert((12, "<FUNC_HEADER>".to_string()), 17);
        m.insert((109, "<CUERPO>".to_string()), 110);
        m.insert((122, "<LLAMADA>".to_string()), 58);
        m.insert((34, "<TÉRMINO>".to_string()), 62);
        m.insert((43, "<ESTATUTO_LIST>".to_string()), 49);
        m.insert((106, "<EXPRESIÓN>".to_string()), 107);
        m.insert((43, "<CICLO>".to_string()), 48);
        m.insert((110, "<SINO_OPT>".to_string()), 112);
        m.insert((34, "<EXPRESIÓN>".to_string()), 120);
        m.insert((99, "<IMPRIME_LIST>".to_string()), 101);
        m.insert((96, "<EXPRESIÓN>".to_string()), 98);
        m.insert((12, "<TIPO_OPT>".to_string()), 13);
        m.insert((53, "<TÉRMINO>".to_string()), 62);
        m.insert((86, "<EXPRESIÓN_LIST>".to_string()), 87);
        m.insert((106, "<LLAMADA_ARGS>".to_string()), 45);
        m.insert((7, "<FUNCS>".to_string()), 12);
        m.insert((12, "<FUNC_ARGS>".to_string()), 14);
        m.insert((96, "<CTE>".to_string()), 61);
        m.insert((14, "<CUERPO>".to_string()), 127);
        m.insert((18, "<ARG_OPT>".to_string()), 20);
        m.insert((7, "<FUNC_HEADER>".to_string()), 17);
        m.insert((12, "<FUNCS>".to_string()), 12);
        m.insert((51, "<FACTOR>".to_string()), 63);
        m.insert((76, "<*/>".to_string()), 75);
        m.insert((84, "<LLAMADA_HEADER>".to_string()), 41);
        m.insert((106, "<FACTOR>".to_string()), 63);
        m
    };

    /// Lista de producciones de la gramática
    pub static ref PRODUCCIONES: Vec<Regla> = vec![
        Regla { id: 0, cabeza: "<ProgramaPrime>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 1, cabeza: "<Programa>".to_string(), longitud_cuerpo: 8 },
        Regla { id: 2, cabeza: "<VARS_OPT>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 3, cabeza: "<VARS_OPT>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 4, cabeza: "<FUNCS_LIST>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 5, cabeza: "<FUNCS_LIST>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 6, cabeza: "<VARS>".to_string(), longitud_cuerpo: 5 },
        Regla { id: 7, cabeza: "<VAR_LIST>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 8, cabeza: "<VAR_LIST_PRIMA>".to_string(), longitud_cuerpo: 3 },
        Regla { id: 9, cabeza: "<VAR_LIST_PRIMA>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 10, cabeza: "<TIPO>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 11, cabeza: "<TIPO>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 12, cabeza: "<TIPO>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 13, cabeza: "<CUERPO>".to_string(), longitud_cuerpo: 4 },
        Regla { id: 14, cabeza: "<ESTATUTO_LIST>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 15, cabeza: "<ESTATUTO_LIST>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 16, cabeza: "<ESTATUTO>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 17, cabeza: "<ESTATUTO>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 18, cabeza: "<ESTATUTO>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 19, cabeza: "<ESTATUTO>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 20, cabeza: "<ESTATUTO>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 21, cabeza: "<ESTATUTO>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 22, cabeza: "<ESTATUTO>".to_string(), longitud_cuerpo: 3 },
        Regla { id: 23, cabeza: "<RETURN>".to_string(), longitud_cuerpo: 3 },
        Regla { id: 24, cabeza: "<IMPRIME>".to_string(), longitud_cuerpo: 6 },
        Regla { id: 25, cabeza: "<OBJ_IMPRIME>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 26, cabeza: "<OBJ_IMPRIME>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 27, cabeza: "<IMPRIME_LIST>".to_string(), longitud_cuerpo: 3 },
        Regla { id: 28, cabeza: "<IMPRIME_LIST>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 29, cabeza: "<ASIGNA>".to_string(), longitud_cuerpo: 4 },
        Regla { id: 30, cabeza: "<CICLO>".to_string(), longitud_cuerpo: 6 },
        Regla { id: 31, cabeza: "<CONDICIÓN>".to_string(), longitud_cuerpo: 8 },
        Regla { id: 32, cabeza: "<SINO_OPT>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 33, cabeza: "<SINO_OPT>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 34, cabeza: "<EXPRESIÓN>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 35, cabeza: "<EXPRESION_PRIMA>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 36, cabeza: "<EXPRESION_PRIMA>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 37, cabeza: "<OPERADOR>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 38, cabeza: "<OPERADOR>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 39, cabeza: "<OPERADOR>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 40, cabeza: "<OPERADOR>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 41, cabeza: "<CTE>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 42, cabeza: "<CTE>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 43, cabeza: "<EXP>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 44, cabeza: "<EXP_PRIMA>".to_string(), longitud_cuerpo: 3 },
        Regla { id: 45, cabeza: "<EXP_PRIMA>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 46, cabeza: "<+->".to_string(), longitud_cuerpo: 1 },
        Regla { id: 47, cabeza: "<+->".to_string(), longitud_cuerpo: 1 },
        Regla { id: 48, cabeza: "<TÉRMINO>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 49, cabeza: "<TERMINO_PRIMA>".to_string(), longitud_cuerpo: 3 },
        Regla { id: 50, cabeza: "<TERMINO_PRIMA>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 51, cabeza: "<*/>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 52, cabeza: "<*/>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 53, cabeza: "<FACTOR>".to_string(), longitud_cuerpo: 3 },
        Regla { id: 54, cabeza: "<FACTOR>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 55, cabeza: "<FACTOR>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 56, cabeza: "<FACTOR>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 57, cabeza: "<FACTOR>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 58, cabeza: "<FACTOR>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 59, cabeza: "<CTE_OPT>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 60, cabeza: "<CTE_OPT>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 61, cabeza: "<FUNC_HEADER>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 62, cabeza: "<FUNC_ARGS>".to_string(), longitud_cuerpo: 4 },
        Regla { id: 63, cabeza: "<FUNCS>".to_string(), longitud_cuerpo: 3 },
        Regla { id: 64, cabeza: "<TIPO_OPT>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 65, cabeza: "<TIPO_OPT>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 66, cabeza: "<ARG_OPT>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 67, cabeza: "<ARG_OPT>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 68, cabeza: "<ARG_LIST>".to_string(), longitud_cuerpo: 4 },
        Regla { id: 69, cabeza: "<ARG_LIST_PRIMA>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 70, cabeza: "<ARG_LIST_PRIMA>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 71, cabeza: "<LLAMADA_HEADER>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 72, cabeza: "<LLAMADA_ARGS>".to_string(), longitud_cuerpo: 4 },
        Regla { id: 73, cabeza: "<LLAMADA>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 74, cabeza: "<EXPRESIÓN_OPT>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 75, cabeza: "<EXPRESION_OPT>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 76, cabeza: "<EXPRESIÓN_LIST>".to_string(), longitud_cuerpo: 3 },
        Regla { id: 77, cabeza: "<EXPRESIÓN_LIST>".to_string(), longitud_cuerpo: 0 },
    ];
}
