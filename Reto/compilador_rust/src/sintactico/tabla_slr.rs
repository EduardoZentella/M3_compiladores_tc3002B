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
        m.insert((55, ")".to_string()), Accion::Reduce(41));
        m.insert((9, ";".to_string()), Accion::Reduce(12));
        m.insert((66, "cte_ent".to_string()), Accion::Reduce(51));
        m.insert((34, "mientras".to_string()), Accion::Shift(36));
        m.insert((45, "[".to_string()), Accion::Reduce(21));
        m.insert((91, "/".to_string()), Accion::Reduce(53));
        m.insert((62, ";".to_string()), Accion::Reduce(45));
        m.insert((93, ";".to_string()), Accion::Reduce(57));
        m.insert((69, "-".to_string()), Accion::Reduce(48));
        m.insert((95, ">".to_string()), Accion::Reduce(56));
        m.insert((88, ",".to_string()), Accion::Reduce(44));
        m.insert((93, "!=".to_string()), Accion::Reduce(57));
        m.insert((115, "si".to_string()), Accion::Reduce(31));
        m.insert((125, "si".to_string()), Accion::Reduce(30));
        m.insert((51, "[".to_string()), Accion::Reduce(13));
        m.insert((15, "id".to_string()), Accion::Shift(19));
        m.insert((52, "-".to_string()), Accion::Shift(56));
        m.insert((92, "==".to_string()), Accion::Reduce(59));
        m.insert((89, "<".to_string()), Accion::Reduce(72));
        m.insert((108, "id".to_string()), Accion::Shift(54));
        m.insert((129, "inicio".to_string()), Accion::Reduce(3));
        m.insert((4, "{".to_string()), Accion::Reduce(3));
        m.insert((139, "flotante".to_string()), Accion::Reduce(6));
        m.insert((133, "flotante".to_string()), Accion::Reduce(63));
        m.insert((81, ",".to_string()), Accion::Shift(79));
        m.insert((68, "(".to_string()), Accion::Shift(58));
        m.insert((134, "flotante".to_string()), Accion::Reduce(2));
        m.insert((135, ":".to_string()), Accion::Reduce(9));
        m.insert((120, "}".to_string()), Accion::Reduce(22));
        m.insert((137, "letrero".to_string()), Accion::Shift(9));
        m.insert((28, ")".to_string()), Accion::Reduce(68));
        m.insert((61, "*".to_string()), Accion::Reduce(55));
        m.insert((42, "si".to_string()), Accion::Reduce(18));
        m.insert((48, "*".to_string()), Accion::Reduce(73));
        m.insert((46, "]".to_string()), Accion::Reduce(20));
        m.insert((4, "[".to_string()), Accion::Reduce(3));
        m.insert((54, ">".to_string()), Accion::Reduce(54));
        m.insert((87, "!=".to_string()), Accion::Reduce(45));
        m.insert((102, "cte_ent".to_string()), Accion::Shift(55));
        m.insert((115, "regresa".to_string()), Accion::Reduce(31));
        m.insert((129, "{".to_string()), Accion::Reduce(3));
        m.insert((130, "{".to_string()), Accion::Shift(31));
        m.insert((56, "cte_flot".to_string()), Accion::Shift(57));
        m.insert((134, "inicio".to_string()), Accion::Reduce(2));
        m.insert((43, "]".to_string()), Accion::Reduce(15));
        m.insert((64, "==".to_string()), Accion::Shift(74));
        m.insert((57, "*".to_string()), Accion::Reduce(42));
        m.insert((58, "cte_flot".to_string()), Accion::Shift(57));
        m.insert((54, "+".to_string()), Accion::Reduce(54));
        m.insert((95, ",".to_string()), Accion::Reduce(56));
        m.insert((6, "inicio".to_string()), Accion::Reduce(3));
        m.insert((51, "si".to_string()), Accion::Reduce(13));
        m.insert((6, "regresa".to_string()), Accion::Reduce(3));
        m.insert((67, "-".to_string()), Accion::Reduce(52));
        m.insert((97, "regresa".to_string()), Accion::Reduce(19));
        m.insert((38, "id".to_string()), Accion::Shift(54));
        m.insert((65, "==".to_string()), Accion::Reduce(50));
        m.insert((7, "entero".to_string()), Accion::Shift(8));
        m.insert((76, "-".to_string()), Accion::Shift(56));
        m.insert((108, "-".to_string()), Accion::Shift(56));
        m.insert((129, "mientras".to_string()), Accion::Reduce(3));
        m.insert((34, "id".to_string()), Accion::Shift(35));
        m.insert((70, "/".to_string()), Accion::Shift(67));
        m.insert((34, "[".to_string()), Accion::Shift(37));
        m.insert((51, "id".to_string()), Accion::Reduce(13));
        m.insert((31, "si".to_string()), Accion::Reduce(3));
        m.insert((62, "==".to_string()), Accion::Reduce(45));
        m.insert((72, "cte_flot".to_string()), Accion::Reduce(37));
        m.insert((16, "letrero".to_string()), Accion::Shift(9));
        m.insert((62, ",".to_string()), Accion::Reduce(45));
        m.insert((66, "cte_flot".to_string()), Accion::Reduce(51));
        m.insert((7, "letrero".to_string()), Accion::Shift(9));
        m.insert((25, "entero".to_string()), Accion::Shift(8));
        m.insert((65, "!=".to_string()), Accion::Reduce(50));
        m.insert((91, "-".to_string()), Accion::Reduce(53));
        m.insert((94, ",".to_string()), Accion::Reduce(60));
        m.insert((54, ";".to_string()), Accion::Reduce(54));
        m.insert((44, "mientras".to_string()), Accion::Reduce(17));
        m.insert((4, "}".to_string()), Accion::Reduce(3));
        m.insert((85, "!=".to_string()), Accion::Reduce(43));
        m.insert((62, "-".to_string()), Accion::Shift(84));
        m.insert((86, "cte_flot".to_string()), Accion::Shift(57));
        m.insert((79, "(".to_string()), Accion::Shift(58));
        m.insert((65, ",".to_string()), Accion::Reduce(50));
        m.insert((84, "-".to_string()), Accion::Reduce(47));
        m.insert((42, "[".to_string()), Accion::Reduce(18));
        m.insert((93, "+".to_string()), Accion::Reduce(57));
        m.insert((37, "escribe".to_string()), Accion::Shift(40));
        m.insert((47, "(".to_string()), Accion::Shift(52));
        m.insert((134, "letrero".to_string()), Accion::Reduce(2));
        m.insert((138, ";".to_string()), Accion::Shift(139));
        m.insert((143, ":".to_string()), Accion::Reduce(8));
        m.insert((6, "si".to_string()), Accion::Reduce(3));
        m.insert((31, "letrero".to_string()), Accion::Reduce(3));
        m.insert((70, "!=".to_string()), Accion::Reduce(50));
        m.insert((54, "*".to_string()), Accion::Reduce(54));
        m.insert((94, "-".to_string()), Accion::Reduce(60));
        m.insert((114, ";".to_string()), Accion::Shift(115));
        m.insert((123, "haz".to_string()), Accion::Shift(124));
        m.insert((61, "/".to_string()), Accion::Reduce(55));
        m.insert((66, "-".to_string()), Accion::Reduce(51));
        m.insert((4, "letrero".to_string()), Accion::Reduce(3));
        m.insert((40, "(".to_string()), Accion::Shift(98));
        m.insert((61, ";".to_string()), Accion::Reduce(55));
        m.insert((69, "<".to_string()), Accion::Reduce(48));
        m.insert((16, "flotante".to_string()), Accion::Shift(11));
        m.insert((7, "inicio".to_string()), Accion::Reduce(5));
        m.insert((105, "id".to_string()), Accion::Reduce(24));
        m.insert((86, "cte_ent".to_string()), Accion::Shift(55));
        m.insert((88, "<".to_string()), Accion::Reduce(44));
        m.insert((125, "id".to_string()), Accion::Reduce(30));
        m.insert((31, "}".to_string()), Accion::Reduce(3));
        m.insert((31, "[".to_string()), Accion::Reduce(3));
        m.insert((129, "id".to_string()), Accion::Reduce(3));
        m.insert((76, "id".to_string()), Accion::Shift(54));
        m.insert((43, "escribe".to_string()), Accion::Shift(40));
        m.insert((93, ")".to_string()), Accion::Reduce(57));
        m.insert((85, "<".to_string()), Accion::Reduce(43));
        m.insert((82, ")".to_string()), Accion::Reduce(76));
        m.insert((142, ":".to_string()), Accion::Reduce(9));
        m.insert((19, "(".to_string()), Accion::Reduce(61));
        m.insert((95, "*".to_string()), Accion::Reduce(56));
        m.insert((71, "-".to_string()), Accion::Reduce(49));
        m.insert((133, "entero".to_string()), Accion::Reduce(63));
        m.insert((74, "-".to_string()), Accion::Reduce(39));
        m.insert((62, "<".to_string()), Accion::Reduce(45));
        m.insert((35, "=".to_string()), Accion::Shift(126));
        m.insert((71, "<".to_string()), Accion::Reduce(49));
        m.insert((97, "[".to_string()), Accion::Reduce(19));
        m.insert((58, "id".to_string()), Accion::Shift(54));
        m.insert((86, "(".to_string()), Accion::Shift(58));
        m.insert((91, ">".to_string()), Accion::Reduce(53));
        m.insert((127, ";".to_string()), Accion::Shift(128));
        m.insert((42, "regresa".to_string()), Accion::Reduce(18));
        m.insert((44, "id".to_string()), Accion::Reduce(17));
        m.insert((134, "{".to_string()), Accion::Reduce(2));
        m.insert((129, "}".to_string()), Accion::Reduce(3));
        m.insert((94, ";".to_string()), Accion::Reduce(60));
        m.insert((31, "escribe".to_string()), Accion::Reduce(3));
        m.insert((44, "si".to_string()), Accion::Reduce(17));
        m.insert((94, "/".to_string()), Accion::Reduce(60));
        m.insert((83, "cte_ent".to_string()), Accion::Reduce(46));
        m.insert((139, "regresa".to_string()), Accion::Reduce(6));
        m.insert((29, ")".to_string()), Accion::Reduce(69));
        m.insert((92, ">".to_string()), Accion::Reduce(59));
        m.insert((34, "escribe".to_string()), Accion::Shift(40));
        m.insert((20, "id".to_string()), Accion::Shift(21));
        m.insert((55, "!=".to_string()), Accion::Reduce(41));
        m.insert((64, "<".to_string()), Accion::Shift(73));
        m.insert((64, ",".to_string()), Accion::Reduce(36));
        m.insert((69, ";".to_string()), Accion::Reduce(48));
        m.insert((90, ")".to_string()), Accion::Shift(91));
        m.insert((6, "}".to_string()), Accion::Reduce(3));
        m.insert((108, "cte_flot".to_string()), Accion::Shift(57));
        m.insert((125, "regresa".to_string()), Accion::Reduce(30));
        m.insert((45, "]".to_string()), Accion::Reduce(21));
        m.insert((140, "id".to_string()), Accion::Shift(142));
        m.insert((86, "+".to_string()), Accion::Shift(53));
        m.insert((142, ",".to_string()), Accion::Shift(140));
        m.insert((59, "!=".to_string()), Accion::Reduce(58));
        m.insert((87, "+".to_string()), Accion::Shift(83));
        m.insert((46, "escribe".to_string()), Accion::Reduce(20));
        m.insert((119, "]".to_string()), Accion::Shift(120));
        m.insert((48, ")".to_string()), Accion::Reduce(73));
        m.insert((37, "mientras".to_string()), Accion::Shift(36));
        m.insert((73, "(".to_string()), Accion::Reduce(38));
        m.insert((54, ")".to_string()), Accion::Reduce(54));
        m.insert((87, "==".to_string()), Accion::Reduce(45));
        m.insert((120, "escribe".to_string()), Accion::Reduce(22));
        m.insert((16, "nula".to_string()), Accion::Shift(10));
        m.insert((131, "}".to_string()), Accion::Shift(132));
        m.insert((83, "id".to_string()), Accion::Reduce(46));
        m.insert((128, "]".to_string()), Accion::Reduce(29));
        m.insert((7, "flotante".to_string()), Accion::Shift(11));
        m.insert((79, "cte_flot".to_string()), Accion::Shift(57));
        m.insert((139, "vars".to_string()), Accion::Reduce(6));
        m.insert((84, "cte_flot".to_string()), Accion::Reduce(47));
        m.insert((45, "id".to_string()), Accion::Reduce(21));
        m.insert((42, "id".to_string()), Accion::Reduce(18));
        m.insert((56, "cte_ent".to_string()), Accion::Shift(55));
        m.insert((115, "mientras".to_string()), Accion::Reduce(31));
        m.insert((76, "cte_flot".to_string()), Accion::Shift(57));
        m.insert((51, "]".to_string()), Accion::Reduce(13));
        m.insert((26, ")".to_string()), Accion::Reduce(70));
        m.insert((46, "si".to_string()), Accion::Reduce(20));
        m.insert((4, "si".to_string()), Accion::Reduce(3));
        m.insert((129, "letrero".to_string()), Accion::Reduce(3));
        m.insert((59, ")".to_string()), Accion::Reduce(58));
        m.insert((118, "mientras".to_string()), Accion::Reduce(23));
        m.insert((94, "==".to_string()), Accion::Reduce(60));
        m.insert((6, "entero".to_string()), Accion::Reduce(3));
        m.insert((31, "inicio".to_string()), Accion::Reduce(3));
        m.insert((43, "si".to_string()), Accion::Shift(39));
        m.insert((93, "<".to_string()), Accion::Reduce(57));
        m.insert((125, "escribe".to_string()), Accion::Reduce(30));
        m.insert((61, "-".to_string()), Accion::Reduce(55));
        m.insert((92, ";".to_string()), Accion::Reduce(59));
        m.insert((85, ",".to_string()), Accion::Reduce(43));
        m.insert((126, "cte_flot".to_string()), Accion::Shift(57));
        m.insert((57, ",".to_string()), Accion::Reduce(42));
        m.insert((134, "[".to_string()), Accion::Reduce(2));
        m.insert((129, "flotante".to_string()), Accion::Reduce(3));
        m.insert((69, "+".to_string()), Accion::Reduce(48));
        m.insert((93, "*".to_string()), Accion::Reduce(57));
        m.insert((134, "}".to_string()), Accion::Reduce(2));
        m.insert((107, ")".to_string()), Accion::Reduce(27));
        m.insert((77, ",".to_string()), Accion::Reduce(34));
        m.insert((129, "[".to_string()), Accion::Reduce(3));
        m.insert((9, ",".to_string()), Accion::Reduce(12));
        m.insert((69, "!=".to_string()), Accion::Reduce(48));
        m.insert((95, "+".to_string()), Accion::Reduce(56));
        m.insert((42, "}".to_string()), Accion::Reduce(18));
        m.insert((31, "flotante".to_string()), Accion::Reduce(3));
        m.insert((128, "escribe".to_string()), Accion::Reduce(29));
        m.insert((105, "escribe".to_string()), Accion::Reduce(24));
        m.insert((44, "escribe".to_string()), Accion::Reduce(17));
        m.insert((79, "cte_ent".to_string()), Accion::Shift(55));
        m.insert((92, "*".to_string()), Accion::Reduce(59));
        m.insert((128, "si".to_string()), Accion::Reduce(29));
        m.insert((52, "+".to_string()), Accion::Shift(53));
        m.insert((26, ",".to_string()), Accion::Shift(27));
        m.insert((115, "id".to_string()), Accion::Reduce(31));
        m.insert((49, "mientras".to_string()), Accion::Reduce(16));
        m.insert((88, "==".to_string()), Accion::Reduce(44));
        m.insert((97, "si".to_string()), Accion::Reduce(19));
        m.insert((91, "<".to_string()), Accion::Reduce(53));
        m.insert((38, "cte_ent".to_string()), Accion::Shift(55));
        m.insert((91, ",".to_string()), Accion::Reduce(53));
        m.insert((59, "-".to_string()), Accion::Reduce(58));
        m.insert((62, ")".to_string()), Accion::Reduce(45));
        m.insert((48, ";".to_string()), Accion::Reduce(73));
        m.insert((91, "*".to_string()), Accion::Reduce(53));
        m.insert((109, ")".to_string()), Accion::Shift(110));
        m.insert((11, ";".to_string()), Accion::Reduce(11));
        m.insert((65, ">".to_string()), Accion::Reduce(50));
        m.insert((61, "==".to_string()), Accion::Reduce(55));
        m.insert((52, "(".to_string()), Accion::Shift(58));
        m.insert((61, ">".to_string()), Accion::Reduce(55));
        m.insert((48, "==".to_string()), Accion::Reduce(73));
        m.insert((101, ",".to_string()), Accion::Shift(102));
        m.insert((75, "+".to_string()), Accion::Reduce(40));
        m.insert((48, "<".to_string()), Accion::Reduce(73));
        m.insert((93, "/".to_string()), Accion::Reduce(57));
        m.insert((118, "escribe".to_string()), Accion::Reduce(23));
        m.insert((121, "cte_flot".to_string()), Accion::Shift(57));
        m.insert((6, "{".to_string()), Accion::Reduce(3));
        m.insert((57, "<".to_string()), Accion::Reduce(42));
        m.insert((84, "+".to_string()), Accion::Reduce(47));
        m.insert((87, ">".to_string()), Accion::Reduce(45));
        m.insert((92, ",".to_string()), Accion::Reduce(59));
        m.insert((101, ")".to_string()), Accion::Reduce(28));
        m.insert((102, "id".to_string()), Accion::Shift(54));
        m.insert((98, "cte_ent".to_string()), Accion::Shift(55));
        m.insert((45, "mientras".to_string()), Accion::Reduce(21));
        m.insert((57, ")".to_string()), Accion::Reduce(42));
        m.insert((37, "}".to_string()), Accion::Reduce(15));
        m.insert((81, ")".to_string()), Accion::Reduce(77));
        m.insert((8, "id".to_string()), Accion::Reduce(10));
        m.insert((96, "]".to_string()), Accion::Reduce(14));
        m.insert((105, "si".to_string()), Accion::Reduce(24));
        m.insert((68, "+".to_string()), Accion::Shift(53));
        m.insert((121, "cte_ent".to_string()), Accion::Shift(55));
        m.insert((87, ")".to_string()), Accion::Reduce(45));
        m.insert((52, "cte_ent".to_string()), Accion::Shift(55));
        m.insert((105, "[".to_string()), Accion::Reduce(24));
        m.insert((8, ")".to_string()), Accion::Reduce(10));
        m.insert((66, "+".to_string()), Accion::Reduce(51));
        m.insert((95, "==".to_string()), Accion::Reduce(56));
        m.insert((106, ")".to_string()), Accion::Reduce(28));
        m.insert((44, "regresa".to_string()), Accion::Reduce(17));
        m.insert((53, "id".to_string()), Accion::Shift(92));
        m.insert((77, ")".to_string()), Accion::Reduce(34));
        m.insert((72, "cte_ent".to_string()), Accion::Reduce(37));
        m.insert((89, "==".to_string()), Accion::Reduce(72));
        m.insert((93, "==".to_string()), Accion::Reduce(57));
        m.insert((69, "==".to_string()), Accion::Reduce(48));
        m.insert((91, ")".to_string()), Accion::Reduce(53));
        m.insert((95, ";".to_string()), Accion::Reduce(56));
        m.insert((51, ";".to_string()), Accion::Reduce(13));
        m.insert((68, "cte_flot".to_string()), Accion::Shift(57));
        m.insert((108, "(".to_string()), Accion::Shift(58));
        m.insert((134, "escribe".to_string()), Accion::Reduce(2));
        m.insert((74, "+".to_string()), Accion::Reduce(39));
        m.insert((94, ")".to_string()), Accion::Reduce(60));
        m.insert((11, ",".to_string()), Accion::Reduce(11));
        m.insert((120, "si".to_string()), Accion::Reduce(22));
        m.insert((139, "escribe".to_string()), Accion::Reduce(6));
        m.insert((31, "mientras".to_string()), Accion::Reduce(3));
        m.insert((60, ")".to_string()), Accion::Shift(89));
        m.insert((110, "entonces".to_string()), Accion::Shift(111));
        m.insert((113, "{".to_string()), Accion::Shift(31));
        m.insert((17, "id".to_string()), Accion::Reduce(65));
        m.insert((37, "id".to_string()), Accion::Shift(35));
        m.insert((83, "-".to_string()), Accion::Reduce(46));
        m.insert((46, "}".to_string()), Accion::Reduce(20));
        m.insert((35, "(".to_string()), Accion::Reduce(71));
        m.insert((95, "/".to_string()), Accion::Reduce(56));
        m.insert((115, "}".to_string()), Accion::Reduce(31));
        m.insert((121, "-".to_string()), Accion::Shift(56));
        m.insert((44, "]".to_string()), Accion::Reduce(17));
        m.insert((124, "{".to_string()), Accion::Shift(31));
        m.insert((57, "+".to_string()), Accion::Reduce(42));
        m.insert((71, "!=".to_string()), Accion::Reduce(49));
        m.insert((51, "regresa".to_string()), Accion::Reduce(13));
        m.insert((54, ",".to_string()), Accion::Reduce(54));
        m.insert((54, "/".to_string()), Accion::Reduce(54));
        m.insert((55, ";".to_string()), Accion::Reduce(41));
        m.insert((57, "==".to_string()), Accion::Reduce(42));
        m.insert((59, "+".to_string()), Accion::Reduce(58));
        m.insert((86, "id".to_string()), Accion::Shift(54));
        m.insert((102, "+".to_string()), Accion::Shift(53));
        m.insert((102, "cte_flot".to_string()), Accion::Shift(57));
        m.insert((49, "id".to_string()), Accion::Reduce(16));
        m.insert((67, "+".to_string()), Accion::Reduce(52));
        m.insert((48, "-".to_string()), Accion::Reduce(73));
        m.insert((71, ">".to_string()), Accion::Reduce(49));
        m.insert((105, "mientras".to_string()), Accion::Reduce(24));
        m.insert((31, "nula".to_string()), Accion::Reduce(3));
        m.insert((61, ",".to_string()), Accion::Reduce(55));
        m.insert((115, "escribe".to_string()), Accion::Reduce(31));
        m.insert((75, "cte_ent".to_string()), Accion::Reduce(40));
        m.insert((37, "]".to_string()), Accion::Reduce(15));
        m.insert((115, "]".to_string()), Accion::Reduce(31));
        m.insert((98, "id".to_string()), Accion::Shift(54));
        m.insert((128, "}".to_string()), Accion::Reduce(29));
        m.insert((46, "[".to_string()), Accion::Reduce(20));
        m.insert((43, "id".to_string()), Accion::Shift(35));
        m.insert((48, ">".to_string()), Accion::Reduce(73));
        m.insert((87, "<".to_string()), Accion::Reduce(45));
        m.insert((128, "regresa".to_string()), Accion::Reduce(29));
        m.insert((129, "vars".to_string()), Accion::Shift(5));
        m.insert((50, "}".to_string()), Accion::Shift(51));
        m.insert((139, "mientras".to_string()), Accion::Reduce(6));
        m.insert((16, "entero".to_string()), Accion::Shift(8));
        m.insert((59, ">".to_string()), Accion::Reduce(58));
        m.insert((121, "id".to_string()), Accion::Shift(54));
        m.insert((125, "}".to_string()), Accion::Reduce(30));
        m.insert((92, "<".to_string()), Accion::Reduce(59));
        m.insert((139, "letrero".to_string()), Accion::Reduce(6));
        m.insert((85, "==".to_string()), Accion::Reduce(43));
        m.insert((31, "entero".to_string()), Accion::Reduce(3));
        m.insert((84, "id".to_string()), Accion::Reduce(47));
        m.insert((25, "flotante".to_string()), Accion::Shift(11));
        m.insert((137, "flotante".to_string()), Accion::Shift(11));
        m.insert((67, "cte_ent".to_string()), Accion::Reduce(52));
        m.insert((70, "<".to_string()), Accion::Reduce(50));
        m.insert((85, ")".to_string()), Accion::Reduce(43));
        m.insert((23, ")".to_string()), Accion::Reduce(66));
        m.insert((62, ">".to_string()), Accion::Reduce(45));
        m.insert((80, ")".to_string()), Accion::Reduce(74));
        m.insert((89, "/".to_string()), Accion::Reduce(72));
        m.insert((97, "mientras".to_string()), Accion::Reduce(19));
        m.insert((93, "-".to_string()), Accion::Reduce(57));
        m.insert((74, "cte_ent".to_string()), Accion::Reduce(39));
        m.insert((108, "+".to_string()), Accion::Shift(53));
        m.insert((132, ";".to_string()), Accion::Shift(133));
        m.insert((31, "vars".to_string()), Accion::Shift(5));
        m.insert((112, ";".to_string()), Accion::Reduce(33));
        m.insert((118, "id".to_string()), Accion::Reduce(23));
        m.insert((99, ",".to_string()), Accion::Reduce(26));
        m.insert((32, "fin".to_string()), Accion::Shift(33));
        m.insert((97, "}".to_string()), Accion::Reduce(19));
        m.insert((106, ",".to_string()), Accion::Shift(102));
        m.insert((76, "cte_ent".to_string()), Accion::Shift(55));
        m.insert((120, "id".to_string()), Accion::Reduce(22));
        m.insert((94, "<".to_string()), Accion::Reduce(60));
        m.insert((139, "nula".to_string()), Accion::Reduce(6));
        m.insert((8, ";".to_string()), Accion::Reduce(10));
        m.insert((42, "]".to_string()), Accion::Reduce(18));
        m.insert((89, ",".to_string()), Accion::Reduce(72));
        m.insert((93, ">".to_string()), Accion::Reduce(57));
        m.insert((139, "inicio".to_string()), Accion::Reduce(6));
        m.insert((120, "]".to_string()), Accion::Reduce(22));
        m.insert((10, "id".to_string()), Accion::Reduce(64));
        m.insert((134, "id".to_string()), Accion::Reduce(2));
        m.insert((74, "(".to_string()), Accion::Reduce(39));
        m.insert((4, "regresa".to_string()), Accion::Reduce(3));
        m.insert((38, "-".to_string()), Accion::Shift(56));
        m.insert((62, "!=".to_string()), Accion::Reduce(45));
        m.insert((95, "<".to_string()), Accion::Reduce(56));
        m.insert((33, "$".to_string()), Accion::Reduce(1));
        m.insert((43, "regresa".to_string()), Accion::Shift(38));
        m.insert((129, "escribe".to_string()), Accion::Reduce(3));
        m.insert((46, "id".to_string()), Accion::Reduce(20));
        m.insert((72, "id".to_string()), Accion::Reduce(37));
        m.insert((54, "==".to_string()), Accion::Reduce(54));
        m.insert((42, "mientras".to_string()), Accion::Reduce(18));
        m.insert((30, "{".to_string()), Accion::Shift(31));
        m.insert((70, "-".to_string()), Accion::Reduce(50));
        m.insert((49, "escribe".to_string()), Accion::Reduce(16));
        m.insert((58, "(".to_string()), Accion::Shift(58));
        m.insert((72, "(".to_string()), Accion::Reduce(37));
        m.insert((45, "escribe".to_string()), Accion::Reduce(21));
        m.insert((73, "-".to_string()), Accion::Reduce(38));
        m.insert((73, "+".to_string()), Accion::Reduce(38));
        m.insert((89, "!=".to_string()), Accion::Reduce(72));
        m.insert((98, "+".to_string()), Accion::Shift(53));
        m.insert((139, "}".to_string()), Accion::Reduce(6));
        m.insert((105, "]".to_string()), Accion::Reduce(24));
        m.insert((70, ",".to_string()), Accion::Reduce(50));
        m.insert((102, "(".to_string()), Accion::Shift(58));
        m.insert((126, "cte_ent".to_string()), Accion::Shift(55));
        m.insert((68, "-".to_string()), Accion::Shift(56));
        m.insert((75, "cte_flot".to_string()), Accion::Reduce(40));
        m.insert((38, "(".to_string()), Accion::Shift(58));
        m.insert((46, "regresa".to_string()), Accion::Reduce(20));
        m.insert((126, "(".to_string()), Accion::Shift(58));
        m.insert((57, "/".to_string()), Accion::Reduce(42));
        m.insert((78, ",".to_string()), Accion::Reduce(35));
        m.insert((83, "+".to_string()), Accion::Reduce(46));
        m.insert((139, "id".to_string()), Accion::Reduce(6));
        m.insert((65, ";".to_string()), Accion::Reduce(50));
        m.insert((73, "cte_ent".to_string()), Accion::Reduce(38));
        m.insert((4, "id".to_string()), Accion::Reduce(3));
        m.insert((54, "(".to_string()), Accion::Reduce(71));
        m.insert((6, "id".to_string()), Accion::Reduce(3));
        m.insert((59, "==".to_string()), Accion::Reduce(58));
        m.insert((38, "+".to_string()), Accion::Shift(53));
        m.insert((70, ";".to_string()), Accion::Reduce(50));
        m.insert((95, ")".to_string()), Accion::Reduce(56));
        m.insert((117, ";".to_string()), Accion::Shift(118));
        m.insert((99, ")".to_string()), Accion::Reduce(26));
        m.insert((37, "regresa".to_string()), Accion::Shift(38));
        m.insert((20, ")".to_string()), Accion::Reduce(67));
        m.insert((126, "-".to_string()), Accion::Shift(56));
        m.insert((51, "}".to_string()), Accion::Reduce(13));
        m.insert((88, ">".to_string()), Accion::Reduce(44));
        m.insert((34, "regresa".to_string()), Accion::Shift(38));
        m.insert((97, "]".to_string()), Accion::Reduce(19));
        m.insert((100, ")".to_string()), Accion::Reduce(25));
        m.insert((11, "id".to_string()), Accion::Reduce(11));
        m.insert((31, "id".to_string()), Accion::Reduce(3));
        m.insert((43, "mientras".to_string()), Accion::Shift(36));
        m.insert((6, "mientras".to_string()), Accion::Reduce(3));
        m.insert((64, "!=".to_string()), Accion::Shift(75));
        m.insert((120, "[".to_string()), Accion::Reduce(22));
        m.insert((65, "-".to_string()), Accion::Reduce(50));
        m.insert((89, "-".to_string()), Accion::Reduce(72));
        m.insert((134, "entero".to_string()), Accion::Reduce(2));
        m.insert((14, "(".to_string()), Accion::Shift(20));
        m.insert((118, "si".to_string()), Accion::Reduce(23));
        m.insert((87, "-".to_string()), Accion::Shift(84));
        m.insert((139, "[".to_string()), Accion::Reduce(6));
        m.insert((4, "nula".to_string()), Accion::Reduce(3));
        m.insert((59, "/".to_string()), Accion::Reduce(58));
        m.insert((76, "+".to_string()), Accion::Shift(53));
        m.insert((91, "+".to_string()), Accion::Reduce(53));
        m.insert((4, "vars".to_string()), Accion::Shift(5));
        m.insert((78, ";".to_string()), Accion::Reduce(35));
        m.insert((128, "mientras".to_string()), Accion::Reduce(29));
        m.insert((98, "letrero".to_string()), Accion::Shift(99));
        m.insert((51, "mientras".to_string()), Accion::Reduce(13));
        m.insert((69, ">".to_string()), Accion::Reduce(48));
        m.insert((87, ";".to_string()), Accion::Reduce(45));
        m.insert((55, "+".to_string()), Accion::Reduce(41));
        m.insert((43, "}".to_string()), Accion::Reduce(15));
        m.insert((126, "+".to_string()), Accion::Shift(53));
        m.insert((125, "[".to_string()), Accion::Reduce(30));
        m.insert((48, "/".to_string()), Accion::Reduce(73));
        m.insert((71, ";".to_string()), Accion::Reduce(49));
        m.insert((53, "cte_flot".to_string()), Accion::Shift(57));
        m.insert((3, ";".to_string()), Accion::Shift(4));
        m.insert((73, "id".to_string()), Accion::Reduce(38));
        m.insert((54, "<".to_string()), Accion::Reduce(54));
        m.insert((34, "}".to_string()), Accion::Reduce(15));
        m.insert((89, ">".to_string()), Accion::Reduce(72));
        m.insert((96, "}".to_string()), Accion::Reduce(14));
        m.insert((133, "nula".to_string()), Accion::Reduce(63));
        m.insert((4, "flotante".to_string()), Accion::Reduce(3));
        m.insert((94, "*".to_string()), Accion::Reduce(60));
        m.insert((48, ",".to_string()), Accion::Reduce(73));
        m.insert((100, ",".to_string()), Accion::Reduce(25));
        m.insert((8, ",".to_string()), Accion::Reduce(10));
        m.insert((0, "programa".to_string()), Accion::Shift(1));
        m.insert((134, "nula".to_string()), Accion::Reduce(2));
        m.insert((98, "-".to_string()), Accion::Shift(56));
        m.insert((71, ")".to_string()), Accion::Reduce(49));
        m.insert((6, "letrero".to_string()), Accion::Reduce(3));
        m.insert((92, "!=".to_string()), Accion::Reduce(59));
        m.insert((118, "}".to_string()), Accion::Reduce(23));
        m.insert((4, "entero".to_string()), Accion::Reduce(3));
        m.insert((77, ";".to_string()), Accion::Reduce(34));
        m.insert((93, ",".to_string()), Accion::Reduce(57));
        m.insert((55, ">".to_string()), Accion::Reduce(41));
        m.insert((88, ";".to_string()), Accion::Reduce(44));
        m.insert((22, ")".to_string()), Accion::Shift(24));
        m.insert((79, "+".to_string()), Accion::Shift(53));
        m.insert((125, "mientras".to_string()), Accion::Reduce(30));
        m.insert((102, "letrero".to_string()), Accion::Shift(99));
        m.insert((134, "regresa".to_string()), Accion::Reduce(2));
        m.insert((58, "+".to_string()), Accion::Shift(53));
        m.insert((27, "id".to_string()), Accion::Shift(21));
        m.insert((12, "{".to_string()), Accion::Shift(129));
        m.insert((64, ">".to_string()), Accion::Shift(72));
        m.insert((36, "(".to_string()), Accion::Shift(121));
        m.insert((65, ")".to_string()), Accion::Reduce(50));
        m.insert((71, "+".to_string()), Accion::Reduce(49));
        m.insert((105, "}".to_string()), Accion::Reduce(24));
        m.insert((25, "letrero".to_string()), Accion::Shift(9));
        m.insert((94, "+".to_string()), Accion::Reduce(60));
        m.insert((2, "$".to_string()), Accion::Accept);
        m.insert((45, "regresa".to_string()), Accion::Reduce(21));
        m.insert((116, ";".to_string()), Accion::Reduce(32));
        m.insert((133, "inicio".to_string()), Accion::Reduce(63));
        m.insert((92, ")".to_string()), Accion::Reduce(59));
        m.insert((122, ")".to_string()), Accion::Shift(123));
        m.insert((37, "[".to_string()), Accion::Shift(37));
        m.insert((38, "cte_flot".to_string()), Accion::Shift(57));
        m.insert((73, "cte_flot".to_string()), Accion::Reduce(38));
        m.insert((45, "}".to_string()), Accion::Reduce(21));
        m.insert((69, ",".to_string()), Accion::Reduce(48));
        m.insert((55, "<".to_string()), Accion::Reduce(41));
        m.insert((37, "si".to_string()), Accion::Shift(39));
        m.insert((70, "==".to_string()), Accion::Reduce(50));
        m.insert((118, "regresa".to_string()), Accion::Reduce(23));
        m.insert((69, ")".to_string()), Accion::Reduce(48));
        m.insert((112, "sino".to_string()), Accion::Shift(113));
        m.insert((128, "id".to_string()), Accion::Reduce(29));
        m.insert((6, "nula".to_string()), Accion::Reduce(3));
        m.insert((45, "si".to_string()), Accion::Reduce(21));
        m.insert((43, "[".to_string()), Accion::Shift(37));
        m.insert((6, "vars".to_string()), Accion::Shift(5));
        m.insert((67, "(".to_string()), Accion::Reduce(52));
        m.insert((4, "inicio".to_string()), Accion::Reduce(3));
        m.insert((39, "(".to_string()), Accion::Shift(108));
        m.insert((115, "[".to_string()), Accion::Reduce(31));
        m.insert((133, "letrero".to_string()), Accion::Reduce(63));
        m.insert((95, "-".to_string()), Accion::Reduce(56));
        m.insert((129, "entero".to_string()), Accion::Reduce(3));
        m.insert((85, ";".to_string()), Accion::Reduce(43));
        m.insert((118, "[".to_string()), Accion::Reduce(23));
        m.insert((58, "-".to_string()), Accion::Shift(56));
        m.insert((129, "nula".to_string()), Accion::Reduce(3));
        m.insert((105, "regresa".to_string()), Accion::Reduce(24));
        m.insert((59, "<".to_string()), Accion::Reduce(58));
        m.insert((48, "+".to_string()), Accion::Reduce(73));
        m.insert((91, "!=".to_string()), Accion::Reduce(53));
        m.insert((1, "id".to_string()), Accion::Shift(3));
        m.insert((6, "flotante".to_string()), Accion::Reduce(3));
        m.insert((34, "]".to_string()), Accion::Reduce(15));
        m.insert((75, "(".to_string()), Accion::Reduce(40));
        m.insert((65, "*".to_string()), Accion::Shift(66));
        m.insert((42, "escribe".to_string()), Accion::Reduce(18));
        m.insert((70, "+".to_string()), Accion::Reduce(50));
        m.insert((121, "+".to_string()), Accion::Shift(53));
        m.insert((71, "==".to_string()), Accion::Reduce(49));
        m.insert((128, "[".to_string()), Accion::Reduce(29));
        m.insert((87, ",".to_string()), Accion::Reduce(45));
        m.insert((13, "inicio".to_string()), Accion::Shift(30));
        m.insert((4, "mientras".to_string()), Accion::Reduce(3));
        m.insert((59, ",".to_string()), Accion::Reduce(58));
        m.insert((76, "(".to_string()), Accion::Shift(58));
        m.insert((34, "si".to_string()), Accion::Shift(39));
        m.insert((63, ",".to_string()), Accion::Shift(79));
        m.insert((49, "[".to_string()), Accion::Reduce(16));
        m.insert((118, "]".to_string()), Accion::Reduce(23));
        m.insert((125, "]".to_string()), Accion::Reduce(30));
        m.insert((46, "mientras".to_string()), Accion::Reduce(20));
        m.insert((137, "entero".to_string()), Accion::Shift(8));
        m.insert((48, "!=".to_string()), Accion::Reduce(73));
        m.insert((9, "id".to_string()), Accion::Reduce(12));
        m.insert((16, "inicio".to_string()), Accion::Reduce(5));
        m.insert((66, "(".to_string()), Accion::Reduce(51));
        m.insert((70, "*".to_string()), Accion::Shift(66));
        m.insert((56, "id".to_string()), Accion::Shift(92));
        m.insert((94, ">".to_string()), Accion::Reduce(60));
        m.insert((59, ";".to_string()), Accion::Reduce(58));
        m.insert((78, ")".to_string()), Accion::Reduce(35));
        m.insert((86, "-".to_string()), Accion::Shift(56));
        m.insert((4, "escribe".to_string()), Accion::Reduce(3));
        m.insert((57, ">".to_string()), Accion::Reduce(42));
        m.insert((79, "id".to_string()), Accion::Shift(54));
        m.insert((111, "{".to_string()), Accion::Shift(31));
        m.insert((129, "regresa".to_string()), Accion::Reduce(3));
        m.insert((49, "si".to_string()), Accion::Reduce(16));
        m.insert((91, "==".to_string()), Accion::Reduce(53));
        m.insert((55, "-".to_string()), Accion::Reduce(41));
        m.insert((74, "id".to_string()), Accion::Reduce(39));
        m.insert((66, "id".to_string()), Accion::Reduce(51));
        m.insert((6, "[".to_string()), Accion::Reduce(3));
        m.insert((54, "-".to_string()), Accion::Reduce(54));
        m.insert((129, "si".to_string()), Accion::Reduce(3));
        m.insert((9, ")".to_string()), Accion::Reduce(12));
        m.insert((24, "{".to_string()), Accion::Reduce(62));
        m.insert((49, "regresa".to_string()), Accion::Reduce(16));
        m.insert((55, "==".to_string()), Accion::Reduce(41));
        m.insert((61, ")".to_string()), Accion::Reduce(55));
        m.insert((72, "+".to_string()), Accion::Reduce(37));
        m.insert((120, "mientras".to_string()), Accion::Reduce(22));
        m.insert((44, "[".to_string()), Accion::Reduce(17));
        m.insert((57, "!=".to_string()), Accion::Reduce(42));
        m.insert((54, "!=".to_string()), Accion::Reduce(54));
        m.insert((57, ";".to_string()), Accion::Reduce(42));
        m.insert((136, ":".to_string()), Accion::Shift(137));
        m.insert((31, "regresa".to_string()), Accion::Reduce(3));
        m.insert((51, "escribe".to_string()), Accion::Reduce(13));
        m.insert((84, "cte_ent".to_string()), Accion::Reduce(47));
        m.insert((63, ")".to_string()), Accion::Reduce(77));
        m.insert((139, "{".to_string()), Accion::Reduce(6));
        m.insert((55, ",".to_string()), Accion::Reduce(41));
        m.insert((88, ")".to_string()), Accion::Reduce(44));
        m.insert((139, "entero".to_string()), Accion::Reduce(6));
        m.insert((89, "+".to_string()), Accion::Reduce(72));
        m.insert((89, ")".to_string()), Accion::Reduce(72));
        m.insert((97, "id".to_string()), Accion::Reduce(19));
        m.insert((104, ";".to_string()), Accion::Shift(105));
        m.insert((120, "regresa".to_string()), Accion::Reduce(22));
        m.insert((97, "escribe".to_string()), Accion::Reduce(19));
        m.insert((6, "escribe".to_string()), Accion::Reduce(3));
        m.insert((68, "cte_ent".to_string()), Accion::Shift(55));
        m.insert((103, ")".to_string()), Accion::Shift(104));
        m.insert((141, ":".to_string()), Accion::Reduce(7));
        m.insert((62, "+".to_string()), Accion::Shift(83));
        m.insert((52, "id".to_string()), Accion::Shift(54));
        m.insert((49, "}".to_string()), Accion::Reduce(16));
        m.insert((7, "nula".to_string()), Accion::Shift(10));
        m.insert((57, "-".to_string()), Accion::Reduce(42));
        m.insert((98, "(".to_string()), Accion::Shift(58));
        m.insert((65, "/".to_string()), Accion::Shift(67));
        m.insert((70, ">".to_string()), Accion::Reduce(50));
        m.insert((126, "id".to_string()), Accion::Shift(54));
        m.insert((135, ",".to_string()), Accion::Shift(140));
        m.insert((52, "cte_flot".to_string()), Accion::Shift(57));
        m.insert((55, "*".to_string()), Accion::Reduce(41));
        m.insert((67, "id".to_string()), Accion::Reduce(52));
        m.insert((53, "cte_ent".to_string()), Accion::Shift(55));
        m.insert((61, "+".to_string()), Accion::Reduce(55));
        m.insert((65, "+".to_string()), Accion::Reduce(50));
        m.insert((83, "cte_flot".to_string()), Accion::Reduce(46));
        m.insert((89, ";".to_string()), Accion::Reduce(72));
        m.insert((68, "id".to_string()), Accion::Shift(54));
        m.insert((92, "/".to_string()), Accion::Reduce(59));
        m.insert((55, "/".to_string()), Accion::Reduce(41));
        m.insert((95, "!=".to_string()), Accion::Reduce(56));
        m.insert((98, "cte_flot".to_string()), Accion::Shift(57));
        m.insert((59, "*".to_string()), Accion::Reduce(58));
        m.insert((21, ":".to_string()), Accion::Shift(25));
        m.insert((67, "cte_flot".to_string()), Accion::Reduce(52));
        m.insert((75, "id".to_string()), Accion::Reduce(40));
        m.insert((89, "*".to_string()), Accion::Reduce(72));
        m.insert((51, "fin".to_string()), Accion::Reduce(13));
        m.insert((91, ";".to_string()), Accion::Reduce(53));
        m.insert((11, ")".to_string()), Accion::Reduce(11));
        m.insert((65, "<".to_string()), Accion::Reduce(50));
        m.insert((71, ",".to_string()), Accion::Reduce(49));
        m.insert((72, "-".to_string()), Accion::Reduce(37));
        m.insert((108, "cte_ent".to_string()), Accion::Shift(55));
        m.insert((134, "mientras".to_string()), Accion::Reduce(2));
        m.insert((5, "id".to_string()), Accion::Shift(135));
        m.insert((85, ">".to_string()), Accion::Reduce(43));
        m.insert((92, "-".to_string()), Accion::Reduce(59));
        m.insert((121, "(".to_string()), Accion::Shift(58));
        m.insert((18, "inicio".to_string()), Accion::Reduce(4));
        m.insert((61, "!=".to_string()), Accion::Reduce(55));
        m.insert((92, "+".to_string()), Accion::Reduce(59));
        m.insert((64, ")".to_string()), Accion::Reduce(36));
        m.insert((83, "(".to_string()), Accion::Reduce(46));
        m.insert((94, "!=".to_string()), Accion::Reduce(60));
        m.insert((75, "-".to_string()), Accion::Reduce(40));
        m.insert((88, "!=".to_string()), Accion::Reduce(44));
        m.insert((134, "si".to_string()), Accion::Reduce(2));
        m.insert((64, ";".to_string()), Accion::Reduce(36));
        m.insert((58, "cte_ent".to_string()), Accion::Shift(55));
        m.insert((102, "-".to_string()), Accion::Shift(56));
        m.insert((139, "si".to_string()), Accion::Reduce(6));
        m.insert((79, "-".to_string()), Accion::Shift(56));
        m.insert((84, "(".to_string()), Accion::Reduce(47));
        m.insert((70, ")".to_string()), Accion::Reduce(50));
        m.insert((74, "cte_flot".to_string()), Accion::Reduce(39));
        m.insert((41, ";".to_string()), Accion::Shift(97));
        m.insert((31, "{".to_string()), Accion::Reduce(3));
        m.insert((44, "}".to_string()), Accion::Reduce(17));
        m.insert((51, "sino".to_string()), Accion::Reduce(13));
        m.insert((61, "<".to_string()), Accion::Reduce(55));
        m.insert((49, "]".to_string()), Accion::Reduce(16));
        m
    };

    /// Tabla GOTO: (estado, no-terminal) -> estado_destino
    pub static ref TABLA_GOTO: HashMap<(usize, String), usize> = {
        let mut m = HashMap::new();
        m.insert((126, "<LLAMADA_HEADER>".to_string()), 47);
        m.insert((124, "<CUERPO>".to_string()), 125);
        m.insert((6, "<VARS>".to_string()), 6);
        m.insert((63, "<EXPRESIÓN_LIST>".to_string()), 80);
        m.insert((38, "<CTE>".to_string()), 61);
        m.insert((52, "<FACTOR>".to_string()), 65);
        m.insert((126, "<EXPRESIÓN>".to_string()), 127);
        m.insert((126, "<CTE>".to_string()), 61);
        m.insert((70, "<TERMINO_PRIMA>".to_string()), 71);
        m.insert((98, "<CTE>".to_string()), 61);
        m.insert((129, "<VARS_OPT>".to_string()), 130);
        m.insert((52, "<LLAMADA_HEADER>".to_string()), 47);
        m.insert((53, "<CTE_OPT>".to_string()), 95);
        m.insert((86, "<CTE>".to_string()), 61);
        m.insert((121, "<EXPRESIÓN>".to_string()), 122);
        m.insert((25, "<TIPO>".to_string()), 26);
        m.insert((34, "<IMPRIME>".to_string()), 46);
        m.insert((7, "<FUNC_HEADER>".to_string()), 14);
        m.insert((43, "<RETURN>".to_string()), 45);
        m.insert((4, "<VARS>".to_string()), 6);
        m.insert((5, "<VAR_LIST>".to_string()), 136);
        m.insert((16, "<TIPO>".to_string()), 17);
        m.insert((34, "<LLAMADA_HEADER>".to_string()), 47);
        m.insert((37, "<CICLO>".to_string()), 42);
        m.insert((31, "<VARS>".to_string()), 6);
        m.insert((58, "<CTE>".to_string()), 61);
        m.insert((87, "<+->".to_string()), 86);
        m.insert((98, "<LLAMADA>".to_string()), 59);
        m.insert((102, "<LLAMADA_ARGS>".to_string()), 48);
        m.insert((108, "<EXPRESIÓN>".to_string()), 109);
        m.insert((126, "<LLAMADA_ARGS>".to_string()), 48);
        m.insert((64, "<EXPRESION_PRIMA>".to_string()), 77);
        m.insert((70, "<*/>".to_string()), 68);
        m.insert((37, "<LLAMADA_ARGS>".to_string()), 48);
        m.insert((43, "<LLAMADA_HEADER>".to_string()), 47);
        m.insert((111, "<CUERPO>".to_string()), 112);
        m.insert((34, "<CONDICIÓN>".to_string()), 44);
        m.insert((34, "<RETURN>".to_string()), 45);
        m.insert((62, "<+->".to_string()), 86);
        m.insert((102, "<LLAMADA>".to_string()), 59);
        m.insert((98, "<EXPRESIÓN>".to_string()), 100);
        m.insert((37, "<RETURN>".to_string()), 45);
        m.insert((86, "<LLAMADA_ARGS>".to_string()), 48);
        m.insert((7, "<TIPO>".to_string()), 17);
        m.insert((68, "<FACTOR>".to_string()), 70);
        m.insert((76, "<LLAMADA>".to_string()), 59);
        m.insert((16, "<FUNC_HEADER>".to_string()), 14);
        m.insert((112, "<SINO_OPT>".to_string()), 114);
        m.insert((58, "<TÉRMINO>".to_string()), 62);
        m.insert((79, "<LLAMADA_ARGS>".to_string()), 48);
        m.insert((34, "<LLAMADA_ARGS>".to_string()), 48);
        m.insert((38, "<EXP>".to_string()), 64);
        m.insert((52, "<LLAMADA>".to_string()), 59);
        m.insert((52, "<EXPRESIÓN_OPT>".to_string()), 60);
        m.insert((38, "<LLAMADA_ARGS>".to_string()), 48);
        m.insert((38, "<LLAMADA_HEADER>".to_string()), 47);
        m.insert((52, "<TÉRMINO>".to_string()), 62);
        m.insert((53, "<CTE>".to_string()), 94);
        m.insert((20, "<ARG_OPT>".to_string()), 22);
        m.insert((31, "<VARS_OPT>".to_string()), 34);
        m.insert((58, "<LLAMADA>".to_string()), 59);
        m.insert((58, "<FACTOR>".to_string()), 65);
        m.insert((52, "<LLAMADA_ARGS>".to_string()), 48);
        m.insert((76, "<FACTOR>".to_string()), 65);
        m.insert((79, "<LLAMADA_HEADER>".to_string()), 47);
        m.insert((98, "<FACTOR>".to_string()), 65);
        m.insert((102, "<EXPRESIÓN>".to_string()), 100);
        m.insert((108, "<EXP>".to_string()), 64);
        m.insert((121, "<EXP>".to_string()), 64);
        m.insert((108, "<FACTOR>".to_string()), 65);
        m.insert((102, "<TÉRMINO>".to_string()), 62);
        m.insert((38, "<LLAMADA>".to_string()), 59);
        m.insert((108, "<LLAMADA>".to_string()), 59);
        m.insert((98, "<EXP>".to_string()), 64);
        m.insert((7, "<FUNC_ARGS>".to_string()), 12);
        m.insert((34, "<ESTATUTO_LIST>".to_string()), 50);
        m.insert((58, "<LLAMADA_HEADER>".to_string()), 47);
        m.insert((43, "<ESTATUTO>".to_string()), 43);
        m.insert((38, "<TÉRMINO>".to_string()), 62);
        m.insert((98, "<TÉRMINO>".to_string()), 62);
        m.insert((38, "<FACTOR>".to_string()), 65);
        m.insert((43, "<CONDICIÓN>".to_string()), 44);
        m.insert((98, "<LLAMADA_ARGS>".to_string()), 48);
        m.insert((126, "<EXP>".to_string()), 64);
        m.insert((76, "<CTE>".to_string()), 61);
        m.insert((76, "<LLAMADA_HEADER>".to_string()), 47);
        m.insert((121, "<CTE>".to_string()), 61);
        m.insert((20, "<ARG_LIST>".to_string()), 23);
        m.insert((86, "<LLAMADA>".to_string()), 59);
        m.insert((86, "<LLAMADA_HEADER>".to_string()), 47);
        m.insert((79, "<LLAMADA>".to_string()), 59);
        m.insert((27, "<ARG_LIST>".to_string()), 29);
        m.insert((81, "<EXPRESIÓN_LIST>".to_string()), 82);
        m.insert((142, "<VAR_LIST_PRIMA>".to_string()), 143);
        m.insert((38, "<EXPRESIÓN>".to_string()), 117);
        m.insert((79, "<TÉRMINO>".to_string()), 62);
        m.insert((64, "<OPERADOR>".to_string()), 76);
        m.insert((79, "<EXPRESIÓN>".to_string()), 81);
        m.insert((102, "<EXP>".to_string()), 64);
        m.insert((37, "<LLAMADA_HEADER>".to_string()), 47);
        m.insert((16, "<FUNC_ARGS>".to_string()), 12);
        m.insert((52, "<EXPRESIÓN>".to_string()), 63);
        m.insert((129, "<VARS>".to_string()), 6);
        m.insert((26, "<ARG_LIST_PRIMA>".to_string()), 28);
        m.insert((76, "<TÉRMINO>".to_string()), 62);
        m.insert((135, "<VAR_LIST_PRIMA>".to_string()), 141);
        m.insert((34, "<LLAMADA>".to_string()), 41);
        m.insert((16, "<TIPO_OPT>".to_string()), 15);
        m.insert((56, "<CTE_OPT>".to_string()), 93);
        m.insert((76, "<LLAMADA_ARGS>".to_string()), 48);
        m.insert((106, "<IMPRIME_LIST>".to_string()), 107);
        m.insert((43, "<ESTATUTO_LIST>".to_string()), 96);
        m.insert((43, "<LLAMADA>".to_string()), 41);
        m.insert((43, "<LLAMADA_ARGS>".to_string()), 48);
        m.insert((101, "<IMPRIME_LIST>".to_string()), 103);
        m.insert((7, "<FUNCS_LIST>".to_string()), 13);
        m.insert((52, "<CTE>".to_string()), 61);
        m.insert((37, "<ESTATUTO_LIST>".to_string()), 119);
        m.insert((56, "<CTE>".to_string()), 94);
        m.insert((79, "<EXP>".to_string()), 64);
        m.insert((7, "<TIPO_OPT>".to_string()), 15);
        m.insert((68, "<LLAMADA_ARGS>".to_string()), 48);
        m.insert((62, "<EXP_PRIMA>".to_string()), 85);
        m.insert((98, "<OBJ_IMPRIME>".to_string()), 101);
        m.insert((121, "<LLAMADA>".to_string()), 59);
        m.insert((30, "<CUERPO>".to_string()), 32);
        m.insert((37, "<LLAMADA>".to_string()), 41);
        m.insert((79, "<CTE>".to_string()), 61);
        m.insert((43, "<IMPRIME>".to_string()), 46);
        m.insert((76, "<EXP>".to_string()), 78);
        m.insert((130, "<CUERPO>".to_string()), 131);
        m.insert((137, "<TIPO>".to_string()), 138);
        m.insert((34, "<ASIGNA>".to_string()), 49);
        m.insert((86, "<FACTOR>".to_string()), 65);
        m.insert((34, "<CICLO>".to_string()), 42);
        m.insert((16, "<FUNCS_LIST>".to_string()), 18);
        m.insert((58, "<LLAMADA_ARGS>".to_string()), 48);
        m.insert((65, "<TERMINO_PRIMA>".to_string()), 69);
        m.insert((68, "<LLAMADA>".to_string()), 59);
        m.insert((6, "<VARS_OPT>".to_string()), 134);
        m.insert((58, "<EXPRESIÓN>".to_string()), 90);
        m.insert((108, "<LLAMADA_ARGS>".to_string()), 48);
        m.insert((108, "<TÉRMINO>".to_string()), 62);
        m.insert((43, "<ASIGNA>".to_string()), 49);
        m.insert((113, "<CUERPO>".to_string()), 116);
        m.insert((52, "<EXP>".to_string()), 64);
        m.insert((126, "<FACTOR>".to_string()), 65);
        m.insert((34, "<ESTATUTO>".to_string()), 43);
        m.insert((43, "<CICLO>".to_string()), 42);
        m.insert((98, "<LLAMADA_HEADER>".to_string()), 47);
        m.insert((121, "<LLAMADA_HEADER>".to_string()), 47);
        m.insert((121, "<FACTOR>".to_string()), 65);
        m.insert((86, "<TÉRMINO>".to_string()), 87);
        m.insert((108, "<LLAMADA_HEADER>".to_string()), 47);
        m.insert((16, "<FUNCS>".to_string()), 16);
        m.insert((58, "<EXP>".to_string()), 64);
        m.insert((102, "<CTE>".to_string()), 61);
        m.insert((121, "<LLAMADA_ARGS>".to_string()), 48);
        m.insert((121, "<TÉRMINO>".to_string()), 62);
        m.insert((126, "<TÉRMINO>".to_string()), 62);
        m.insert((108, "<CTE>".to_string()), 61);
        m.insert((7, "<FUNCS>".to_string()), 16);
        m.insert((37, "<IMPRIME>".to_string()), 46);
        m.insert((87, "<EXP_PRIMA>".to_string()), 88);
        m.insert((79, "<FACTOR>".to_string()), 65);
        m.insert((4, "<VARS_OPT>".to_string()), 7);
        m.insert((0, "<Programa>".to_string()), 2);
        m.insert((68, "<CTE>".to_string()), 61);
        m.insert((102, "<LLAMADA_HEADER>".to_string()), 47);
        m.insert((126, "<LLAMADA>".to_string()), 59);
        m.insert((37, "<CONDICIÓN>".to_string()), 44);
        m.insert((37, "<ESTATUTO>".to_string()), 43);
        m.insert((68, "<LLAMADA_HEADER>".to_string()), 47);
        m.insert((37, "<ASIGNA>".to_string()), 49);
        m.insert((102, "<OBJ_IMPRIME>".to_string()), 106);
        m.insert((102, "<FACTOR>".to_string()), 65);
        m.insert((65, "<*/>".to_string()), 68);
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
        Regla { id: 63, cabeza: "<FUNCS>".to_string(), longitud_cuerpo: 6 },
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
