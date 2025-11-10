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
        m.insert((61, "-".to_string()), Accion::Reduce(37));
        m.insert((22, "[".to_string()), Accion::Reduce(16));
        m.insert((76, ")".to_string()), Accion::Reduce(68));
        m.insert((59, ">".to_string()), Accion::Reduce(42));
        m.insert((106, "inicio".to_string()), Accion::Reduce(6));
        m.insert((54, "+".to_string()), Accion::Reduce(38));
        m.insert((53, ")".to_string()), Accion::Reduce(39));
        m.insert((41, "+".to_string()), Accion::Reduce(47));
        m.insert((51, ";".to_string()), Accion::Reduce(46));
        m.insert((90, "+".to_string()), Accion::Shift(33));
        m.insert((4, "{".to_string()), Accion::Reduce(3));
        m.insert((50, ";".to_string()), Accion::Reduce(47));
        m.insert((64, "cte_flot".to_string()), Accion::Reduce(35));
        m.insert((57, ")".to_string()), Accion::Reduce(40));
        m.insert((30, "escribe".to_string()), Accion::Reduce(20));
        m.insert((12, "fin".to_string()), Accion::Shift(13));
        m.insert((57, "==".to_string()), Accion::Reduce(40));
        m.insert((57, "!=".to_string()), Accion::Reduce(40));
        m.insert((41, "*".to_string()), Accion::Shift(46));
        m.insert((55, ",".to_string()), Accion::Reduce(51));
        m.insert((80, "id".to_string()), Accion::Reduce(26));
        m.insert((50, ")".to_string()), Accion::Reduce(47));
        m.insert((45, "mientras".to_string()), Accion::Reduce(27));
        m.insert((33, "cte_ent".to_string()), Accion::Reduce(43));
        m.insert((62, "(".to_string()), Accion::Reduce(36));
        m.insert((49, "id".to_string()), Accion::Shift(32));
        m.insert((34, "-".to_string()), Accion::Shift(35));
        m.insert((54, "/".to_string()), Accion::Reduce(38));
        m.insert((1, "id".to_string()), Accion::Shift(3));
        m.insert((23, "]".to_string()), Accion::Reduce(17));
        m.insert((73, "(".to_string()), Accion::Shift(34));
        m.insert((30, "id".to_string()), Accion::Reduce(20));
        m.insert((21, "}".to_string()), Accion::Reduce(14));
        m.insert((54, "==".to_string()), Accion::Reduce(38));
        m.insert((20, "[".to_string()), Accion::Reduce(19));
        m.insert((7, "inicio".to_string()), Accion::Reduce(2));
        m.insert((22, "escribe".to_string()), Accion::Reduce(16));
        m.insert((38, ",".to_string()), Accion::Reduce(52));
        m.insert((55, ")".to_string()), Accion::Reduce(51));
        m.insert((56, ",".to_string()), Accion::Reduce(56));
        m.insert((97, "id".to_string()), Accion::Reduce(21));
        m.insert((72, ",".to_string()), Accion::Shift(73));
        m.insert((59, "+".to_string()), Accion::Shift(33));
        m.insert((51, "+".to_string()), Accion::Reduce(46));
        m.insert((22, "mientras".to_string()), Accion::Reduce(16));
        m.insert((58, "-".to_string()), Accion::Shift(35));
        m.insert((18, "[".to_string()), Accion::Shift(18));
        m.insert((51, "!=".to_string()), Accion::Reduce(46));
        m.insert((64, "cte_ent".to_string()), Accion::Reduce(35));
        m.insert((59, "<".to_string()), Accion::Reduce(42));
        m.insert((31, "(".to_string()), Accion::Shift(34));
        m.insert((33, "+".to_string()), Accion::Reduce(43));
        m.insert((38, "*".to_string()), Accion::Reduce(52));
        m.insert((82, ")".to_string()), Accion::Shift(83));
        m.insert((40, "cte_flot".to_string()), Accion::Shift(53));
        m.insert((21, "mientras".to_string()), Accion::Shift(17));
        m.insert((38, "+".to_string()), Accion::Reduce(52));
        m.insert((63, "cte_flot".to_string()), Accion::Reduce(34));
        m.insert((38, ";".to_string()), Accion::Reduce(52));
        m.insert((41, ">".to_string()), Accion::Reduce(47));
        m.insert((52, ")".to_string()), Accion::Reduce(55));
        m.insert((96, ";".to_string()), Accion::Shift(97));
        m.insert((78, "-".to_string()), Accion::Shift(35));
        m.insert((77, ">".to_string()), Accion::Reduce(65));
        m.insert((70, "id".to_string()), Accion::Shift(32));
        m.insert((69, "-".to_string()), Accion::Reduce(50));
        m.insert((28, "si".to_string()), Accion::Reduce(12));
        m.insert((38, "==".to_string()), Accion::Reduce(52));
        m.insert((94, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((57, ",".to_string()), Accion::Reduce(40));
        m.insert((64, "-".to_string()), Accion::Reduce(35));
        m.insert((69, ",".to_string()), Accion::Reduce(50));
        m.insert((73, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((74, ")".to_string()), Accion::Reduce(66));
        m.insert((65, ")".to_string()), Accion::Reduce(31));
        m.insert((46, "+".to_string()), Accion::Reduce(48));
        m.insert((36, ",".to_string()), Accion::Reduce(33));
        m.insert((58, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((66, "+".to_string()), Accion::Shift(33));
        m.insert((4, "inicio".to_string()), Accion::Reduce(3));
        m.insert((25, "]".to_string()), Accion::Reduce(15));
        m.insert((33, "-".to_string()), Accion::Reduce(43));
        m.insert((38, "<".to_string()), Accion::Reduce(52));
        m.insert((62, "+".to_string()), Accion::Reduce(36));
        m.insert((103, ";".to_string()), Accion::Reduce(10));
        m.insert((26, "si".to_string()), Accion::Reduce(18));
        m.insert((52, "<".to_string()), Accion::Reduce(55));
        m.insert((18, "id".to_string()), Accion::Shift(16));
        m.insert((108, ":".to_string()), Accion::Reduce(7));
        m.insert((35, "(".to_string()), Accion::Reduce(44));
        m.insert((43, "haz".to_string()), Accion::Shift(44));
        m.insert((41, ",".to_string()), Accion::Reduce(47));
        m.insert((58, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((104, ")".to_string()), Accion::Reduce(11));
        m.insert((77, ",".to_string()), Accion::Reduce(65));
        m.insert((30, "si".to_string()), Accion::Reduce(20));
        m.insert((58, "(".to_string()), Accion::Shift(34));
        m.insert((90, "(".to_string()), Accion::Shift(34));
        m.insert((55, "*".to_string()), Accion::Reduce(51));
        m.insert((55, "+".to_string()), Accion::Reduce(51));
        m.insert((61, "id".to_string()), Accion::Reduce(37));
        m.insert((27, "}".to_string()), Accion::Reduce(13));
        m.insert((59, "-".to_string()), Accion::Shift(35));
        m.insert((47, "-".to_string()), Accion::Reduce(49));
        m.insert((65, ",".to_string()), Accion::Reduce(31));
        m.insert((88, "}".to_string()), Accion::Reduce(28));
        m.insert((97, "]".to_string()), Accion::Reduce(21));
        m.insert((105, ";".to_string()), Accion::Shift(106));
        m.insert((34, "id".to_string()), Accion::Reduce(54));
        m.insert((46, "id".to_string()), Accion::Reduce(48));
        m.insert((25, "id".to_string()), Accion::Reduce(15));
        m.insert((63, "cte_ent".to_string()), Accion::Reduce(34));
        m.insert((66, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((97, "mientras".to_string()), Accion::Reduce(21));
        m.insert((28, "sino".to_string()), Accion::Reduce(12));
        m.insert((101, ":".to_string()), Accion::Shift(102));
        m.insert((44, "{".to_string()), Accion::Shift(11));
        m.insert((63, "id".to_string()), Accion::Reduce(34));
        m.insert((39, "cte_flot".to_string()), Accion::Reduce(53));
        m.insert((26, "[".to_string()), Accion::Reduce(18));
        m.insert((45, "si".to_string()), Accion::Reduce(27));
        m.insert((16, "(".to_string()), Accion::Shift(70));
        m.insert((21, "[".to_string()), Accion::Shift(18));
        m.insert((69, "<".to_string()), Accion::Reduce(50));
        m.insert((102, "entero".to_string()), Accion::Shift(103));
        m.insert((48, ">".to_string()), Accion::Reduce(45));
        m.insert((31, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((81, "+".to_string()), Accion::Shift(33));
        m.insert((94, "-".to_string()), Accion::Shift(35));
        m.insert((62, "-".to_string()), Accion::Reduce(36));
        m.insert((48, ";".to_string()), Accion::Reduce(45));
        m.insert((70, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((64, "+".to_string()), Accion::Reduce(35));
        m.insert((91, ")".to_string()), Accion::Reduce(23));
        m.insert((60, ")".to_string()), Accion::Reduce(41));
        m.insert((109, ":".to_string()), Accion::Reduce(9));
        m.insert((89, ";".to_string()), Accion::Reduce(29));
        m.insert((55, ";".to_string()), Accion::Reduce(51));
        m.insert((48, "<".to_string()), Accion::Reduce(45));
        m.insert((60, ">".to_string()), Accion::Reduce(41));
        m.insert((18, "escribe".to_string()), Accion::Shift(14));
        m.insert((53, "+".to_string()), Accion::Reduce(39));
        m.insert((48, "!=".to_string()), Accion::Reduce(45));
        m.insert((56, ")".to_string()), Accion::Reduce(56));
        m.insert((91, ",".to_string()), Accion::Reduce(23));
        m.insert((18, "si".to_string()), Accion::Shift(15));
        m.insert((100, ":".to_string()), Accion::Reduce(9));
        m.insert((37, ",".to_string()), Accion::Reduce(42));
        m.insert((90, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((81, "-".to_string()), Accion::Shift(35));
        m.insert((38, "!=".to_string()), Accion::Reduce(52));
        m.insert((56, "/".to_string()), Accion::Reduce(56));
        m.insert((94, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((80, "si".to_string()), Accion::Reduce(26));
        m.insert((21, "escribe".to_string()), Accion::Shift(14));
        m.insert((49, "+".to_string()), Accion::Shift(33));
        m.insert((47, "cte_ent".to_string()), Accion::Reduce(49));
        m.insert((50, ",".to_string()), Accion::Reduce(47));
        m.insert((10, "{".to_string()), Accion::Shift(11));
        m.insert((49, "-".to_string()), Accion::Shift(35));
        m.insert((48, "+".to_string()), Accion::Reduce(45));
        m.insert((41, "!=".to_string()), Accion::Reduce(47));
        m.insert((79, ";".to_string()), Accion::Shift(80));
        m.insert((99, ")".to_string()), Accion::Reduce(24));
        m.insert((80, "escribe".to_string()), Accion::Reduce(26));
        m.insert((53, "/".to_string()), Accion::Reduce(39));
        m.insert((56, "!=".to_string()), Accion::Reduce(56));
        m.insert((31, "-".to_string()), Accion::Shift(35));
        m.insert((57, ">".to_string()), Accion::Reduce(40));
        m.insert((102, "flotante".to_string()), Accion::Shift(104));
        m.insert((73, "id".to_string()), Accion::Shift(32));
        m.insert((36, ">".to_string()), Accion::Shift(63));
        m.insert((18, "}".to_string()), Accion::Reduce(14));
        m.insert((8, "inicio".to_string()), Accion::Shift(10));
        m.insert((28, "[".to_string()), Accion::Reduce(12));
        m.insert((20, "si".to_string()), Accion::Reduce(19));
        m.insert((5, "id".to_string()), Accion::Shift(100));
        m.insert((45, "}".to_string()), Accion::Reduce(27));
        m.insert((70, "(".to_string()), Accion::Shift(34));
        m.insert((54, "!=".to_string()), Accion::Reduce(38));
        m.insert((88, "si".to_string()), Accion::Reduce(28));
        m.insert((38, "-".to_string()), Accion::Reduce(52));
        m.insert((11, "id".to_string()), Accion::Shift(16));
        m.insert((52, ";".to_string()), Accion::Reduce(55));
        m.insert((11, "}".to_string()), Accion::Reduce(14));
        m.insert((90, "letrero".to_string()), Accion::Shift(91));
        m.insert((56, "==".to_string()), Accion::Reduce(56));
        m.insert((78, "(".to_string()), Accion::Shift(34));
        m.insert((77, "<".to_string()), Accion::Reduce(65));
        m.insert((51, ")".to_string()), Accion::Reduce(46));
        m.insert((53, "==".to_string()), Accion::Reduce(39));
        m.insert((104, ";".to_string()), Accion::Reduce(11));
        m.insert((28, "]".to_string()), Accion::Reduce(12));
        m.insert((56, "+".to_string()), Accion::Reduce(56));
        m.insert((11, "[".to_string()), Accion::Shift(18));
        m.insert((69, "==".to_string()), Accion::Reduce(50));
        m.insert((7, "{".to_string()), Accion::Reduce(2));
        m.insert((34, "(".to_string()), Accion::Shift(34));
        m.insert((94, "(".to_string()), Accion::Shift(34));
        m.insert((45, "]".to_string()), Accion::Reduce(27));
        m.insert((45, "[".to_string()), Accion::Reduce(27));
        m.insert((53, "-".to_string()), Accion::Reduce(39));
        m.insert((59, "!=".to_string()), Accion::Reduce(42));
        m.insert((11, "si".to_string()), Accion::Shift(15));
        m.insert((60, ";".to_string()), Accion::Reduce(41));
        m.insert((60, ",".to_string()), Accion::Reduce(41));
        m.insert((62, "cte_ent".to_string()), Accion::Reduce(36));
        m.insert((25, "escribe".to_string()), Accion::Reduce(15));
        m.insert((31, "+".to_string()), Accion::Shift(33));
        m.insert((58, "+".to_string()), Accion::Shift(33));
        m.insert((50, "-".to_string()), Accion::Reduce(47));
        m.insert((53, ";".to_string()), Accion::Reduce(39));
        m.insert((81, "id".to_string()), Accion::Shift(32));
        m.insert((52, "/".to_string()), Accion::Reduce(55));
        m.insert((103, "id".to_string()), Accion::Reduce(10));
        m.insert((23, "si".to_string()), Accion::Reduce(17));
        m.insert((78, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((57, ";".to_string()), Accion::Reduce(40));
        m.insert((39, "id".to_string()), Accion::Reduce(53));
        m.insert((4, "vars".to_string()), Accion::Shift(5));
        m.insert((17, "(".to_string()), Accion::Shift(31));
        m.insert((28, "id".to_string()), Accion::Reduce(12));
        m.insert((51, ">".to_string()), Accion::Reduce(46));
        m.insert((58, "id".to_string()), Accion::Shift(32));
        m.insert((54, ">".to_string()), Accion::Reduce(38));
        m.insert((63, "(".to_string()), Accion::Reduce(34));
        m.insert((20, "escribe".to_string()), Accion::Reduce(19));
        m.insert((45, "escribe".to_string()), Accion::Reduce(27));
        m.insert((29, "]".to_string()), Accion::Shift(30));
        m.insert((50, "+".to_string()), Accion::Reduce(47));
        m.insert((37, "+".to_string()), Accion::Shift(33));
        m.insert((50, "*".to_string()), Accion::Shift(46));
        m.insert((77, "-".to_string()), Accion::Reduce(65));
        m.insert((60, "<".to_string()), Accion::Reduce(41));
        m.insert((94, "id".to_string()), Accion::Shift(32));
        m.insert((81, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((52, "+".to_string()), Accion::Reduce(55));
        m.insert((103, ",".to_string()), Accion::Reduce(10));
        m.insert((42, ")".to_string()), Accion::Shift(43));
        m.insert((77, "!=".to_string()), Accion::Reduce(65));
        m.insert((75, ",".to_string()), Accion::Shift(73));
        m.insert((20, "}".to_string()), Accion::Reduce(19));
        m.insert((48, "==".to_string()), Accion::Reduce(45));
        m.insert((66, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((78, "+".to_string()), Accion::Shift(33));
        m.insert((73, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((35, "-".to_string()), Accion::Reduce(44));
        m.insert((97, "si".to_string()), Accion::Reduce(21));
        m.insert((61, "(".to_string()), Accion::Reduce(37));
        m.insert((52, ">".to_string()), Accion::Reduce(55));
        m.insert((15, "(".to_string()), Accion::Shift(81));
        m.insert((70, "-".to_string()), Accion::Shift(35));
        m.insert((25, "[".to_string()), Accion::Reduce(15));
        m.insert((40, "id".to_string()), Accion::Shift(52));
        m.insert((23, "id".to_string()), Accion::Reduce(17));
        m.insert((59, ";".to_string()), Accion::Reduce(42));
        m.insert((53, "!=".to_string()), Accion::Reduce(39));
        m.insert((97, "[".to_string()), Accion::Reduce(21));
        m.insert((107, "id".to_string()), Accion::Shift(109));
        m.insert((54, "-".to_string()), Accion::Reduce(38));
        m.insert((46, "cte_ent".to_string()), Accion::Reduce(48));
        m.insert((30, "mientras".to_string()), Accion::Reduce(20));
        m.insert((69, "!=".to_string()), Accion::Reduce(50));
        m.insert((104, "id".to_string()), Accion::Reduce(11));
        m.insert((94, "letrero".to_string()), Accion::Shift(91));
        m.insert((64, "(".to_string()), Accion::Reduce(35));
        m.insert((85, ";".to_string()), Accion::Reduce(30));
        m.insert((31, "id".to_string()), Accion::Reduce(54));
        m.insert((41, ")".to_string()), Accion::Reduce(47));
        m.insert((52, "!=".to_string()), Accion::Reduce(55));
        m.insert((57, "<".to_string()), Accion::Reduce(40));
        m.insert((104, ",".to_string()), Accion::Reduce(11));
        m.insert((56, ">".to_string()), Accion::Reduce(56));
        m.insert((97, "}".to_string()), Accion::Reduce(21));
        m.insert((2, "$".to_string()), Accion::Accept);
        m.insert((50, "<".to_string()), Accion::Reduce(47));
        m.insert((88, "escribe".to_string()), Accion::Reduce(28));
        m.insert((46, "-".to_string()), Accion::Reduce(48));
        m.insert((77, ";".to_string()), Accion::Reduce(65));
        m.insert((87, ";".to_string()), Accion::Shift(88));
        m.insert((100, ",".to_string()), Accion::Shift(107));
        m.insert((47, "(".to_string()), Accion::Reduce(49));
        m.insert((22, "}".to_string()), Accion::Reduce(16));
        m.insert((23, "}".to_string()), Accion::Reduce(17));
        m.insert((30, "[".to_string()), Accion::Reduce(20));
        m.insert((28, "}".to_string()), Accion::Reduce(12));
        m.insert((97, "escribe".to_string()), Accion::Reduce(21));
        m.insert((63, "+".to_string()), Accion::Reduce(34));
        m.insert((60, "==".to_string()), Accion::Reduce(41));
        m.insert((33, "id".to_string()), Accion::Reduce(43));
        m.insert((81, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((106, "{".to_string()), Accion::Reduce(6));
        m.insert((53, ",".to_string()), Accion::Reduce(39));
        m.insert((48, "-".to_string()), Accion::Reduce(45));
        m.insert((9, "inicio".to_string()), Accion::Reduce(4));
        m.insert((21, "id".to_string()), Accion::Shift(16));
        m.insert((64, "id".to_string()), Accion::Reduce(35));
        m.insert((77, "+".to_string()), Accion::Reduce(65));
        m.insert((19, "}".to_string()), Accion::Shift(28));
        m.insert((35, "cte_flot".to_string()), Accion::Reduce(44));
        m.insert((20, "id".to_string()), Accion::Reduce(19));
        m.insert((37, "-".to_string()), Accion::Shift(35));
        m.insert((73, "+".to_string()), Accion::Shift(33));
        m.insert((21, "]".to_string()), Accion::Reduce(14));
        m.insert((49, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((55, "!=".to_string()), Accion::Reduce(51));
        m.insert((88, "mientras".to_string()), Accion::Reduce(28));
        m.insert((30, "}".to_string()), Accion::Reduce(20));
        m.insert((28, "mientras".to_string()), Accion::Reduce(12));
        m.insert((27, "]".to_string()), Accion::Reduce(13));
        m.insert((36, ")".to_string()), Accion::Reduce(33));
        m.insert((69, "+".to_string()), Accion::Reduce(50));
        m.insert((55, "/".to_string()), Accion::Reduce(51));
        m.insert((72, ")".to_string()), Accion::Reduce(69));
        m.insert((11, "mientras".to_string()), Accion::Shift(17));
        m.insert((61, "cte_ent".to_string()), Accion::Reduce(37));
        m.insert((38, "/".to_string()), Accion::Reduce(52));
        m.insert((67, ")".to_string()), Accion::Reduce(32));
        m.insert((75, ")".to_string()), Accion::Reduce(69));
        m.insert((26, "]".to_string()), Accion::Reduce(18));
        m.insert((26, "}".to_string()), Accion::Reduce(18));
        m.insert((93, ",".to_string()), Accion::Reduce(22));
        m.insert((22, "]".to_string()), Accion::Reduce(16));
        m.insert((36, "==".to_string()), Accion::Shift(62));
        m.insert((46, "cte_flot".to_string()), Accion::Reduce(48));
        m.insert((69, "*".to_string()), Accion::Reduce(50));
        m.insert((67, ";".to_string()), Accion::Reduce(32));
        m.insert((25, "}".to_string()), Accion::Reduce(15));
        m.insert((28, ";".to_string()), Accion::Reduce(12));
        m.insert((39, "cte_ent".to_string()), Accion::Reduce(53));
        m.insert((47, "+".to_string()), Accion::Reduce(49));
        m.insert((90, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((26, "mientras".to_string()), Accion::Reduce(18));
        m.insert((94, "+".to_string()), Accion::Shift(33));
        m.insert((37, "==".to_string()), Accion::Reduce(42));
        m.insert((50, "!=".to_string()), Accion::Reduce(47));
        m.insert((90, "-".to_string()), Accion::Shift(35));
        m.insert((80, "mientras".to_string()), Accion::Reduce(26));
        m.insert((54, "<".to_string()), Accion::Reduce(38));
        m.insert((83, "entonces".to_string()), Accion::Shift(84));
        m.insert((103, ")".to_string()), Accion::Reduce(10));
        m.insert((98, ")".to_string()), Accion::Reduce(25));
        m.insert((51, ",".to_string()), Accion::Reduce(46));
        m.insert((41, "-".to_string()), Accion::Reduce(47));
        m.insert((49, "(".to_string()), Accion::Shift(34));
        m.insert((90, "id".to_string()), Accion::Reduce(54));
        m.insert((30, "]".to_string()), Accion::Reduce(20));
        m.insert((88, "]".to_string()), Accion::Reduce(28));
        m.insert((53, ">".to_string()), Accion::Reduce(39));
        m.insert((62, "id".to_string()), Accion::Reduce(36));
        m.insert((41, "/".to_string()), Accion::Shift(47));
        m.insert((77, "*".to_string()), Accion::Reduce(65));
        m.insert((80, "[".to_string()), Accion::Reduce(26));
        m.insert((36, "<".to_string()), Accion::Shift(64));
        m.insert((37, "<".to_string()), Accion::Reduce(42));
        m.insert((41, "<".to_string()), Accion::Reduce(47));
        m.insert((53, "*".to_string()), Accion::Reduce(39));
        m.insert((40, "cte_ent".to_string()), Accion::Shift(54));
        m.insert((69, "/".to_string()), Accion::Reduce(50));
        m.insert((33, "cte_flot".to_string()), Accion::Reduce(43));
        m.insert((69, ";".to_string()), Accion::Reduce(50));
        m.insert((35, "cte_ent".to_string()), Accion::Reduce(44));
        m.insert((48, ")".to_string()), Accion::Reduce(45));
        m.insert((51, "-".to_string()), Accion::Reduce(46));
        m.insert((59, ")".to_string()), Accion::Reduce(42));
        m.insert((61, "+".to_string()), Accion::Reduce(37));
        m.insert((37, ")".to_string()), Accion::Reduce(42));
        m.insert((78, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((47, "cte_flot".to_string()), Accion::Reduce(49));
        m.insert((52, "*".to_string()), Accion::Reduce(55));
        m.insert((67, ",".to_string()), Accion::Reduce(32));
        m.insert((6, "inicio".to_string()), Accion::Reduce(5));
        m.insert((62, "cte_flot".to_string()), Accion::Reduce(36));
        m.insert((50, "/".to_string()), Accion::Shift(47));
        m.insert((92, ",".to_string()), Accion::Shift(94));
        m.insert((51, "==".to_string()), Accion::Reduce(46));
        m.insert((55, "==".to_string()), Accion::Reduce(51));
        m.insert((73, "-".to_string()), Accion::Shift(35));
        m.insert((20, "mientras".to_string()), Accion::Reduce(19));
        m.insert((92, ")".to_string()), Accion::Reduce(25));
        m.insert((110, ":".to_string()), Accion::Reduce(8));
        m.insert((77, ")".to_string()), Accion::Reduce(65));
        m.insert((25, "si".to_string()), Accion::Reduce(15));
        m.insert((54, ")".to_string()), Accion::Reduce(38));
        m.insert((28, "fin".to_string()), Accion::Reduce(12));
        m.insert((36, ";".to_string()), Accion::Reduce(33));
        m.insert((56, ";".to_string()), Accion::Reduce(56));
        m.insert((59, "==".to_string()), Accion::Reduce(42));
        m.insert((68, ")".to_string()), Accion::Shift(69));
        m.insert((34, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((23, "[".to_string()), Accion::Reduce(17));
        m.insert((88, "id".to_string()), Accion::Reduce(28));
        m.insert((26, "escribe".to_string()), Accion::Reduce(18));
        m.insert((61, "cte_flot".to_string()), Accion::Reduce(37));
        m.insert((47, "id".to_string()), Accion::Reduce(49));
        m.insert((78, "id".to_string()), Accion::Reduce(54));
        m.insert((86, "{".to_string()), Accion::Shift(11));
        m.insert((109, ",".to_string()), Accion::Shift(107));
        m.insert((59, ",".to_string()), Accion::Reduce(42));
        m.insert((54, ",".to_string()), Accion::Reduce(38));
        m.insert((80, "]".to_string()), Accion::Reduce(26));
        m.insert((60, "!=".to_string()), Accion::Reduce(41));
        m.insert((70, "+".to_string()), Accion::Shift(33));
        m.insert((88, "[".to_string()), Accion::Reduce(28));
        m.insert((56, "*".to_string()), Accion::Reduce(56));
        m.insert((52, "==".to_string()), Accion::Reduce(55));
        m.insert((66, "id".to_string()), Accion::Reduce(54));
        m.insert((38, ">".to_string()), Accion::Reduce(52));
        m.insert((18, "mientras".to_string()), Accion::Shift(17));
        m.insert((63, "-".to_string()), Accion::Reduce(34));
        m.insert((35, "+".to_string()), Accion::Reduce(44));
        m.insert((48, ",".to_string()), Accion::Reduce(45));
        m.insert((32, "(".to_string()), Accion::Shift(70));
        m.insert((52, ",".to_string()), Accion::Reduce(55));
        m.insert((45, "id".to_string()), Accion::Reduce(27));
        m.insert((65, ";".to_string()), Accion::Reduce(31));
        m.insert((77, "==".to_string()), Accion::Reduce(65));
        m.insert((23, "mientras".to_string()), Accion::Reduce(17));
        m.insert((80, "}".to_string()), Accion::Reduce(26));
        m.insert((0, "programa".to_string()), Accion::Shift(1));
        m.insert((14, "(".to_string()), Accion::Shift(90));
        m.insert((35, "id".to_string()), Accion::Reduce(44));
        m.insert((23, "escribe".to_string()), Accion::Reduce(17));
        m.insert((51, "<".to_string()), Accion::Reduce(46));
        m.insert((24, ";".to_string()), Accion::Shift(26));
        m.insert((25, "mientras".to_string()), Accion::Reduce(15));
        m.insert((49, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((54, ";".to_string()), Accion::Reduce(38));
        m.insert((66, "(".to_string()), Accion::Shift(34));
        m.insert((95, ")".to_string()), Accion::Shift(96));
        m.insert((34, "+".to_string()), Accion::Shift(33));
        m.insert((20, "]".to_string()), Accion::Reduce(19));
        m.insert((93, ")".to_string()), Accion::Reduce(22));
        m.insert((22, "si".to_string()), Accion::Reduce(16));
        m.insert((16, "=".to_string()), Accion::Shift(78));
        m.insert((36, "!=".to_string()), Accion::Shift(61));
        m.insert((3, ";".to_string()), Accion::Shift(4));
        m.insert((22, "id".to_string()), Accion::Reduce(16));
        m.insert((26, "id".to_string()), Accion::Reduce(18));
        m.insert((50, ">".to_string()), Accion::Reduce(47));
        m.insert((66, "-".to_string()), Accion::Shift(35));
        m.insert((55, "<".to_string()), Accion::Reduce(51));
        m.insert((56, "-".to_string()), Accion::Reduce(56));
        m.insert((18, "]".to_string()), Accion::Reduce(14));
        m.insert((33, "(".to_string()), Accion::Reduce(43));
        m.insert((38, ")".to_string()), Accion::Reduce(52));
        m.insert((46, "(".to_string()), Accion::Reduce(48));
        m.insert((85, "sino".to_string()), Accion::Shift(86));
        m.insert((21, "si".to_string()), Accion::Shift(15));
        m.insert((77, "/".to_string()), Accion::Reduce(65));
        m.insert((41, "==".to_string()), Accion::Reduce(47));
        m.insert((69, ")".to_string()), Accion::Reduce(50));
        m.insert((11, "]".to_string()), Accion::Reduce(14));
        m.insert((55, "-".to_string()), Accion::Reduce(51));
        m.insert((50, "==".to_string()), Accion::Reduce(47));
        m.insert((98, ",".to_string()), Accion::Shift(94));
        m.insert((71, ")".to_string()), Accion::Shift(77));
        m.insert((53, "<".to_string()), Accion::Reduce(39));
        m.insert((54, "*".to_string()), Accion::Reduce(38));
        m.insert((28, "escribe".to_string()), Accion::Reduce(12));
        m.insert((56, "<".to_string()), Accion::Reduce(56));
        m.insert((52, "-".to_string()), Accion::Reduce(55));
        m.insert((84, "{".to_string()), Accion::Shift(11));
        m.insert((41, ";".to_string()), Accion::Reduce(47));
        m.insert((34, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((81, "(".to_string()), Accion::Shift(34));
        m.insert((37, ">".to_string()), Accion::Reduce(42));
        m.insert((11, "escribe".to_string()), Accion::Shift(14));
        m.insert((69, ">".to_string()), Accion::Reduce(50));
        m.insert((70, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((31, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((55, ">".to_string()), Accion::Reduce(51));
        m.insert((13, "$".to_string()), Accion::Reduce(1));
        m.insert((37, ";".to_string()), Accion::Reduce(42));
        m.insert((37, "!=".to_string()), Accion::Reduce(42));
        m
    };

    /// Tabla GOTO: (estado, no-terminal) -> estado_destino
    pub static ref TABLA_GOTO: HashMap<(usize, String), usize> = {
        let mut m = HashMap::new();
        m.insert((59, "<EXP’>".to_string()), 60);
        m.insert((66, "<+-_OPT>".to_string()), 40);
        m.insert((73, "<EXP>".to_string()), 36);
        m.insert((31, "<+-_OPT>".to_string()), 40);
        m.insert((90, "<+->".to_string()), 39);
        m.insert((31, "<TÉRMINO>".to_string()), 37);
        m.insert((34, "<TÉRMINO>".to_string()), 37);
        m.insert((78, "<TÉRMINO>".to_string()), 37);
        m.insert((34, "<FACTOR>".to_string()), 41);
        m.insert((86, "<CUERPO>".to_string()), 89);
        m.insert((78, "<+-_OPT>".to_string()), 40);
        m.insert((90, "<FACTOR>".to_string()), 41);
        m.insert((92, "<IMPRIME_LIST>".to_string()), 95);
        m.insert((73, "<+-_OPT>".to_string()), 40);
        m.insert((31, "<+->".to_string()), 39);
        m.insert((34, "<+->".to_string()), 39);
        m.insert((58, "<+-_OPT>".to_string()), 40);
        m.insert((34, "<EXP>".to_string()), 36);
        m.insert((50, "<*/>".to_string()), 49);
        m.insert((78, "<LLAMADA>".to_string()), 38);
        m.insert((41, "<*/>".to_string()), 49);
        m.insert((78, "<EXPRESIÓN>".to_string()), 79);
        m.insert((11, "<IMPRIME>".to_string()), 20);
        m.insert((21, "<ESTATUTO>".to_string()), 21);
        m.insert((31, "<FACTOR>".to_string()), 41);
        m.insert((90, "<EXP>".to_string()), 36);
        m.insert((94, "<OBJ_IMPRIME>".to_string()), 98);
        m.insert((78, "<EXP>".to_string()), 36);
        m.insert((81, "<+->".to_string()), 39);
        m.insert((21, "<IMPRIME>".to_string()), 20);
        m.insert((73, "<TÉRMINO>".to_string()), 37);
        m.insert((18, "<CONDICIÓN>".to_string()), 22);
        m.insert((40, "<CTE>".to_string()), 56);
        m.insert((81, "<+-_OPT>".to_string()), 40);
        m.insert((66, "<EXP>".to_string()), 67);
        m.insert((85, "<SINO_OPT>".to_string()), 87);
        m.insert((40, "<CTE_OPT>".to_string()), 55);
        m.insert((70, "<EXP>".to_string()), 36);
        m.insert((90, "<TÉRMINO>".to_string()), 37);
        m.insert((102, "<TIPO>".to_string()), 105);
        m.insert((81, "<FACTOR>".to_string()), 41);
        m.insert((21, "<CICLO>".to_string()), 23);
        m.insert((41, "<TÉRMINO'>".to_string()), 48);
        m.insert((94, "<EXPRESIÓN>".to_string()), 93);
        m.insert((98, "<IMPRIME_LIST>".to_string()), 99);
        m.insert((34, "<+-_OPT>".to_string()), 40);
        m.insert((10, "<CUERPO>".to_string()), 12);
        m.insert((66, "<TÉRMINO>".to_string()), 37);
        m.insert((70, "<LLAMADA>".to_string()), 38);
        m.insert((21, "<CONDICIÓN>".to_string()), 22);
        m.insert((37, "<EXP’>".to_string()), 57);
        m.insert((36, "<OPERADOR>".to_string()), 66);
        m.insert((18, "<ESTATUTO_LIST>".to_string()), 29);
        m.insert((90, "<+-_OPT>".to_string()), 40);
        m.insert((72, "<EXPRESIÓN_LIST>".to_string()), 74);
        m.insert((81, "<TÉRMINO>".to_string()), 37);
        m.insert((94, "<LLAMADA>".to_string()), 38);
        m.insert((75, "<EXPRESIÓN_LIST>".to_string()), 76);
        m.insert((36, "<EXPRESIÓN'>".to_string()), 65);
        m.insert((18, "<CICLO>".to_string()), 23);
        m.insert((81, "<EXPRESIÓN>".to_string()), 82);
        m.insert((49, "<FACTOR>".to_string()), 50);
        m.insert((59, "<+->".to_string()), 58);
        m.insert((70, "<FACTOR>".to_string()), 41);
        m.insert((94, "<+-_OPT>".to_string()), 40);
        m.insert((94, "<EXP>".to_string()), 36);
        m.insert((81, "<LLAMADA>".to_string()), 38);
        m.insert((58, "<+->".to_string()), 39);
        m.insert((73, "<FACTOR>".to_string()), 41);
        m.insert((18, "<LLAMADA>".to_string()), 24);
        m.insert((78, "<FACTOR>".to_string()), 41);
        m.insert((21, "<LLAMADA>".to_string()), 24);
        m.insert((37, "<+->".to_string()), 58);
        m.insert((50, "<TÉRMINO'>".to_string()), 51);
        m.insert((44, "<CUERPO>".to_string()), 45);
        m.insert((18, "<ESTATUTO>".to_string()), 21);
        m.insert((49, "<+->".to_string()), 39);
        m.insert((58, "<TÉRMINO>".to_string()), 59);
        m.insert((66, "<+->".to_string()), 39);
        m.insert((4, "<VARS_OPT>".to_string()), 6);
        m.insert((73, "<+->".to_string()), 39);
        m.insert((11, "<ASIGNA>".to_string()), 25);
        m.insert((4, "<VARS>".to_string()), 7);
        m.insert((31, "<LLAMADA>".to_string()), 38);
        m.insert((31, "<EXP>".to_string()), 36);
        m.insert((90, "<LLAMADA>".to_string()), 38);
        m.insert((84, "<CUERPO>".to_string()), 85);
        m.insert((11, "<CICLO>".to_string()), 23);
        m.insert((58, "<FACTOR>".to_string()), 41);
        m.insert((94, "<FACTOR>".to_string()), 41);
        m.insert((18, "<ASIGNA>".to_string()), 25);
        m.insert((78, "<+->".to_string()), 39);
        m.insert((73, "<LLAMADA>".to_string()), 38);
        m.insert((21, "<ASIGNA>".to_string()), 25);
        m.insert((0, "<Programa>".to_string()), 2);
        m.insert((6, "<FUNCS><FUNCS_LIST>".to_string()), 9);
        m.insert((49, "<+-_OPT>".to_string()), 40);
        m.insert((6, "<FUNCS_LIST>".to_string()), 8);
        m.insert((90, "<EXPRESIÓN>".to_string()), 93);
        m.insert((58, "<LLAMADA>".to_string()), 38);
        m.insert((70, "<+->".to_string()), 39);
        m.insert((31, "<EXPRESIÓN>".to_string()), 42);
        m.insert((94, "<+->".to_string()), 39);
        m.insert((109, "<VAR_LIST'>".to_string()), 110);
        m.insert((11, "<ESTATUTO_LIST>".to_string()), 19);
        m.insert((66, "<LLAMADA>".to_string()), 38);
        m.insert((90, "<OBJ_IMPRIME>".to_string()), 92);
        m.insert((11, "<CONDICIÓN>".to_string()), 22);
        m.insert((73, "<EXPRESIÓN>".to_string()), 75);
        m.insert((81, "<EXP>".to_string()), 36);
        m.insert((5, "<VAR_LIST>".to_string()), 101);
        m.insert((34, "<EXPRESIÓN>".to_string()), 68);
        m.insert((21, "<ESTATUTO_LIST>".to_string()), 27);
        m.insert((11, "<ESTATUTO>".to_string()), 21);
        m.insert((49, "<LLAMADA>".to_string()), 38);
        m.insert((70, "<EXPRESIÓN_OPT>".to_string()), 71);
        m.insert((11, "<LLAMADA>".to_string()), 24);
        m.insert((100, "<VAR_LIST'>".to_string()), 108);
        m.insert((34, "<LLAMADA>".to_string()), 38);
        m.insert((70, "<EXPRESIÓN>".to_string()), 72);
        m.insert((70, "<TÉRMINO>".to_string()), 37);
        m.insert((70, "<+-_OPT>".to_string()), 40);
        m.insert((18, "<IMPRIME>".to_string()), 20);
        m.insert((94, "<TÉRMINO>".to_string()), 37);
        m.insert((66, "<FACTOR>".to_string()), 41);
        m
    };

    /// Lista de producciones de la gramática
    pub static ref PRODUCCIONES: Vec<Regla> = vec![
        Regla { id: 0, cabeza: "<ProgramaPrime>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 1, cabeza: "<Programa>".to_string(), longitud_cuerpo: 8 },
        Regla { id: 2, cabeza: "<VARS_OPT>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 3, cabeza: "<VARS_OPT>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 4, cabeza: "<FUNCS_LIST>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 5, cabeza: "<FUNCS_LIST>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 6, cabeza: "<VARS>".to_string(), longitud_cuerpo: 5 },
        Regla { id: 7, cabeza: "<VAR_LIST>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 8, cabeza: "<VAR_LIST'>".to_string(), longitud_cuerpo: 3 },
        Regla { id: 9, cabeza: "<VAR_LIST'>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 10, cabeza: "<TIPO>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 11, cabeza: "<TIPO>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 12, cabeza: "<CUERPO>".to_string(), longitud_cuerpo: 3 },
        Regla { id: 13, cabeza: "<ESTATUTO_LIST>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 14, cabeza: "<ESTATUTO_LIST>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 15, cabeza: "<ESTATUTO>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 16, cabeza: "<ESTATUTO>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 17, cabeza: "<ESTATUTO>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 18, cabeza: "<ESTATUTO>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 19, cabeza: "<ESTATUTO>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 20, cabeza: "<ESTATUTO>".to_string(), longitud_cuerpo: 3 },
        Regla { id: 21, cabeza: "<IMPRIME>".to_string(), longitud_cuerpo: 6 },
        Regla { id: 22, cabeza: "<OBJ_IMPRIME>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 23, cabeza: "<OBJ_IMPRIME>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 24, cabeza: "<IMPRIME_LIST>".to_string(), longitud_cuerpo: 3 },
        Regla { id: 25, cabeza: "<IMPRIME_LIST>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 26, cabeza: "<ASIGNA>".to_string(), longitud_cuerpo: 4 },
        Regla { id: 27, cabeza: "<CICLO>".to_string(), longitud_cuerpo: 6 },
        Regla { id: 28, cabeza: "<CONDICIÓN>".to_string(), longitud_cuerpo: 8 },
        Regla { id: 29, cabeza: "<SINO_OPT>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 30, cabeza: "<SINO_OPT>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 31, cabeza: "<EXPRESIÓN>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 32, cabeza: "<EXPRESIÓN'>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 33, cabeza: "<EXPRESIÓN'>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 34, cabeza: "<OPERADOR>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 35, cabeza: "<OPERADOR>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 36, cabeza: "<OPERADOR>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 37, cabeza: "<OPERADOR>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 38, cabeza: "<CTE>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 39, cabeza: "<CTE>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 40, cabeza: "<EXP>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 41, cabeza: "<EXP’>".to_string(), longitud_cuerpo: 3 },
        Regla { id: 42, cabeza: "<EXP’>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 43, cabeza: "<+->".to_string(), longitud_cuerpo: 1 },
        Regla { id: 44, cabeza: "<+->".to_string(), longitud_cuerpo: 1 },
        Regla { id: 45, cabeza: "<TÉRMINO>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 46, cabeza: "<TÉRMINO'>".to_string(), longitud_cuerpo: 3 },
        Regla { id: 47, cabeza: "<TÉRMINO'>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 48, cabeza: "<*/>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 49, cabeza: "<*/>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 50, cabeza: "<FACTOR>".to_string(), longitud_cuerpo: 3 },
        Regla { id: 51, cabeza: "<FACTOR>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 52, cabeza: "<FACTOR>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 53, cabeza: "<+-_OPT>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 54, cabeza: "<+-_OPT>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 55, cabeza: "<CTE_OPT>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 56, cabeza: "<CTE_OPT>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 57, cabeza: "<FUNCS>".to_string(), longitud_cuerpo: 10 },
        Regla { id: 58, cabeza: "<TIPO_OPT>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 59, cabeza: "<TIPO_OPT>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 60, cabeza: "<ARG_OPT>".to_string(), longitud_cuerpo: 1 },
        Regla { id: 61, cabeza: "<ARG_OPT>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 62, cabeza: "<ARG_LIST>".to_string(), longitud_cuerpo: 4 },
        Regla { id: 63, cabeza: "<ARG_LIST’>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 64, cabeza: "<ARG_LIST’>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 65, cabeza: "<LLAMADA>".to_string(), longitud_cuerpo: 4 },
        Regla { id: 66, cabeza: "<EXPRESIÓN_OPT>".to_string(), longitud_cuerpo: 2 },
        Regla { id: 67, cabeza: "<EXPRESION_OPT>".to_string(), longitud_cuerpo: 0 },
        Regla { id: 68, cabeza: "<EXPRESIÓN_LIST>".to_string(), longitud_cuerpo: 3 },
        Regla { id: 69, cabeza: "<EXPRESIÓN_LIST>".to_string(), longitud_cuerpo: 0 },
    ];
}
