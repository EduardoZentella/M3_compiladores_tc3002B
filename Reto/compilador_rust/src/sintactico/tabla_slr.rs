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
        m.insert((56, ";".to_string()), Accion::Reduce(38));
        m.insert((71, ">".to_string()), Accion::Reduce(50));
        m.insert((54, "-".to_string()), Accion::Reduce(39));
        m.insert((43, "id".to_string()), Accion::Shift(32));
        m.insert((19, "}".to_string()), Accion::Reduce(16));
        m.insert((19, "id".to_string()), Accion::Reduce(16));
        m.insert((18, "(".to_string()), Accion::Shift(29));
        m.insert((22, "[".to_string()), Accion::Reduce(15));
        m.insert((68, "<".to_string()), Accion::Reduce(42));
        m.insert((82, "haz".to_string()), Accion::Shift(83));
        m.insert((85, "id".to_string()), Accion::Reduce(54));
        m.insert((89, ")".to_string()), Accion::Reduce(23));
        m.insert((28, ";".to_string()), Accion::Reduce(12));
        m.insert((22, "]".to_string()), Accion::Reduce(15));
        m.insert((69, "!=".to_string()), Accion::Reduce(41));
        m.insert((92, "letrero".to_string()), Accion::Shift(89));
        m.insert((20, "[".to_string()), Accion::Reduce(17));
        m.insert((56, "-".to_string()), Accion::Reduce(38));
        m.insert((6, "inicio".to_string()), Accion::Reduce(2));
        m.insert((69, "<".to_string()), Accion::Reduce(41));
        m.insert((88, "-".to_string()), Accion::Shift(30));
        m.insert((30, "(".to_string()), Accion::Reduce(44));
        m.insert((14, "]".to_string()), Accion::Reduce(14));
        m.insert((59, "cte_flot".to_string()), Accion::Reduce(35));
        m.insert((52, "mientras".to_string()), Accion::Reduce(28));
        m.insert((21, "}".to_string()), Accion::Shift(28));
        m.insert((57, ")".to_string()), Accion::Reduce(51));
        m.insert((28, "}".to_string()), Accion::Reduce(12));
        m.insert((61, "id".to_string()), Accion::Reduce(34));
        m.insert((34, "-".to_string()), Accion::Shift(30));
        m.insert((99, "}".to_string()), Accion::Reduce(20));
        m.insert((40, ")".to_string()), Accion::Reduce(52));
        m.insert((100, ",".to_string()), Accion::Shift(104));
        m.insert((36, "cte_ent".to_string()), Accion::Reduce(53));
        m.insert((11, "si".to_string()), Accion::Shift(18));
        m.insert((55, ";".to_string()), Accion::Reduce(55));
        m.insert((66, "(".to_string()), Accion::Shift(33));
        m.insert((75, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((24, "id".to_string()), Accion::Shift(16));
        m.insert((96, ")".to_string()), Accion::Reduce(25));
        m.insert((56, "+".to_string()), Accion::Reduce(38));
        m.insert((80, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((63, ")".to_string()), Accion::Reduce(31));
        m.insert((41, "cte_ent".to_string()), Accion::Reduce(49));
        m.insert((23, "]".to_string()), Accion::Reduce(19));
        m.insert((55, "+".to_string()), Accion::Reduce(55));
        m.insert((61, "+".to_string()), Accion::Reduce(34));
        m.insert((77, ")".to_string()), Accion::Reduce(69));
        m.insert((17, "(".to_string()), Accion::Shift(80));
        m.insert((60, "+".to_string()), Accion::Reduce(36));
        m.insert((30, "id".to_string()), Accion::Reduce(44));
        m.insert((33, "id".to_string()), Accion::Reduce(54));
        m.insert((72, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((87, "[".to_string()), Accion::Reduce(26));
        m.insert((87, "si".to_string()), Accion::Reduce(26));
        m.insert((24, "[".to_string()), Accion::Shift(14));
        m.insert((71, ",".to_string()), Accion::Reduce(50));
        m.insert((45, ",".to_string()), Accion::Reduce(47));
        m.insert((62, "id".to_string()), Accion::Reduce(37));
        m.insert((68, ")".to_string()), Accion::Reduce(42));
        m.insert((22, "escribe".to_string()), Accion::Reduce(15));
        m.insert((64, "-".to_string()), Accion::Shift(30));
        m.insert((8, "inicio".to_string()), Accion::Shift(10));
        m.insert((72, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((28, "escribe".to_string()), Accion::Reduce(12));
        m.insert((68, "+".to_string()), Accion::Shift(31));
        m.insert((35, "<".to_string()), Accion::Shift(59));
        m.insert((61, "cte_flot".to_string()), Accion::Reduce(34));
        m.insert((23, "escribe".to_string()), Accion::Reduce(19));
        m.insert((19, "[".to_string()), Accion::Reduce(16));
        m.insert((50, "{".to_string()), Accion::Shift(11));
        m.insert((76, ")".to_string()), Accion::Reduce(66));
        m.insert((28, "[".to_string()), Accion::Reduce(12));
        m.insert((80, "+".to_string()), Accion::Shift(31));
        m.insert((27, "]".to_string()), Accion::Reduce(13));
        m.insert((34, "!=".to_string()), Accion::Reduce(42));
        m.insert((71, "==".to_string()), Accion::Reduce(50));
        m.insert((0, "programa".to_string()), Accion::Shift(1));
        m.insert((57, ">".to_string()), Accion::Reduce(51));
        m.insert((23, "mientras".to_string()), Accion::Reduce(19));
        m.insert((51, ";".to_string()), Accion::Shift(52));
        m.insert((46, ",".to_string()), Accion::Reduce(46));
        m.insert((66, "+".to_string()), Accion::Shift(31));
        m.insert((84, "[".to_string()), Accion::Reduce(27));
        m.insert((79, ")".to_string()), Accion::Reduce(65));
        m.insert((28, "mientras".to_string()), Accion::Reduce(12));
        m.insert((40, "!=".to_string()), Accion::Reduce(52));
        m.insert((80, "(".to_string()), Accion::Shift(33));
        m.insert((88, "id".to_string()), Accion::Shift(32));
        m.insert((93, ")".to_string()), Accion::Shift(94));
        m.insert((62, "(".to_string()), Accion::Reduce(37));
        m.insert((33, "-".to_string()), Accion::Shift(30));
        m.insert((46, "!=".to_string()), Accion::Reduce(46));
        m.insert((61, "-".to_string()), Accion::Reduce(34));
        m.insert((57, ";".to_string()), Accion::Reduce(51));
        m.insert((90, ")".to_string()), Accion::Reduce(25));
        m.insert((74, ",".to_string()), Accion::Shift(75));
        m.insert((75, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((26, "id".to_string()), Accion::Reduce(18));
        m.insert((60, "id".to_string()), Accion::Reduce(36));
        m.insert((102, "<TIPO>;".to_string()), Accion::Shift(103));
        m.insert((46, ")".to_string()), Accion::Reduce(46));
        m.insert((31, "(".to_string()), Accion::Reduce(43));
        m.insert((67, ",".to_string()), Accion::Reduce(40));
        m.insert((38, ")".to_string()), Accion::Shift(47));
        m.insert((84, "escribe".to_string()), Accion::Reduce(27));
        m.insert((37, "cte_ent".to_string()), Accion::Shift(56));
        m.insert((34, "+".to_string()), Accion::Shift(31));
        m.insert((28, "id".to_string()), Accion::Reduce(12));
        m.insert((14, "mientras".to_string()), Accion::Shift(17));
        m.insert((81, ")".to_string()), Accion::Shift(82));
        m.insert((1, "id".to_string()), Accion::Shift(3));
        m.insert((60, "-".to_string()), Accion::Reduce(36));
        m.insert((24, "]".to_string()), Accion::Reduce(14));
        m.insert((61, "(".to_string()), Accion::Reduce(34));
        m.insert((71, "!=".to_string()), Accion::Reduce(50));
        m.insert((54, ";".to_string()), Accion::Reduce(39));
        m.insert((95, "escribe".to_string()), Accion::Reduce(21));
        m.insert((11, "]".to_string()), Accion::Reduce(14));
        m.insert((34, ")".to_string()), Accion::Reduce(42));
        m.insert((107, ":".to_string()), Accion::Reduce(8));
        m.insert((45, ")".to_string()), Accion::Reduce(47));
        m.insert((43, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((97, ")".to_string()), Accion::Reduce(24));
        m.insert((43, "+".to_string()), Accion::Shift(31));
        m.insert((45, "*".to_string()), Accion::Shift(42));
        m.insert((57, "+".to_string()), Accion::Reduce(51));
        m.insert((40, ",".to_string()), Accion::Reduce(52));
        m.insert((35, ")".to_string()), Accion::Reduce(33));
        m.insert((39, ";".to_string()), Accion::Reduce(47));
        m.insert((46, "<".to_string()), Accion::Reduce(46));
        m.insert((59, "(".to_string()), Accion::Reduce(35));
        m.insert((71, "<".to_string()), Accion::Reduce(50));
        m.insert((79, "==".to_string()), Accion::Reduce(65));
        m.insert((85, "-".to_string()), Accion::Shift(30));
        m.insert((106, ",".to_string()), Accion::Shift(104));
        m.insert((13, "$".to_string()), Accion::Reduce(1));
        m.insert((44, "!=".to_string()), Accion::Reduce(45));
        m.insert((57, "!=".to_string()), Accion::Reduce(51));
        m.insert((35, "==".to_string()), Accion::Shift(60));
        m.insert((84, "si".to_string()), Accion::Reduce(27));
        m.insert((42, "+".to_string()), Accion::Reduce(48));
        m.insert((100, ":".to_string()), Accion::Reduce(9));
        m.insert((74, ")".to_string()), Accion::Reduce(69));
        m.insert((29, "-".to_string()), Accion::Shift(30));
        m.insert((39, "<".to_string()), Accion::Reduce(47));
        m.insert((64, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((14, "escribe".to_string()), Accion::Shift(15));
        m.insert((29, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((91, ")".to_string()), Accion::Reduce(22));
        m.insert((22, "si".to_string()), Accion::Reduce(15));
        m.insert((14, "si".to_string()), Accion::Shift(18));
        m.insert((71, "+".to_string()), Accion::Reduce(50));
        m.insert((46, ";".to_string()), Accion::Reduce(46));
        m.insert((39, "!=".to_string()), Accion::Reduce(47));
        m.insert((45, "!=".to_string()), Accion::Reduce(47));
        m.insert((69, ">".to_string()), Accion::Reduce(41));
        m.insert((44, ",".to_string()), Accion::Reduce(45));
        m.insert((59, "+".to_string()), Accion::Reduce(35));
        m.insert((48, "{".to_string()), Accion::Shift(11));
        m.insert((73, ")".to_string()), Accion::Shift(79));
        m.insert((35, ",".to_string()), Accion::Reduce(33));
        m.insert((62, "-".to_string()), Accion::Reduce(37));
        m.insert((58, "<".to_string()), Accion::Reduce(56));
        m.insert((20, "id".to_string()), Accion::Reduce(17));
        m.insert((26, "]".to_string()), Accion::Reduce(18));
        m.insert((54, "*".to_string()), Accion::Reduce(39));
        m.insert((34, "<".to_string()), Accion::Reduce(42));
        m.insert((79, "*".to_string()), Accion::Reduce(65));
        m.insert((45, "-".to_string()), Accion::Reduce(47));
        m.insert((88, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((30, "+".to_string()), Accion::Reduce(44));
        m.insert((35, ">".to_string()), Accion::Shift(61));
        m.insert((54, "==".to_string()), Accion::Reduce(39));
        m.insert((67, "<".to_string()), Accion::Reduce(40));
        m.insert((98, "]".to_string()), Accion::Shift(99));
        m.insert((72, "(".to_string()), Accion::Shift(33));
        m.insert((55, "*".to_string()), Accion::Reduce(55));
        m.insert((55, "/".to_string()), Accion::Reduce(55));
        m.insert((22, "mientras".to_string()), Accion::Reduce(15));
        m.insert((52, "}".to_string()), Accion::Reduce(28));
        m.insert((72, "+".to_string()), Accion::Shift(31));
        m.insert((33, "(".to_string()), Accion::Shift(33));
        m.insert((20, "}".to_string()), Accion::Reduce(17));
        m.insert((31, "cte_ent".to_string()), Accion::Reduce(43));
        m.insert((37, "id".to_string()), Accion::Shift(55));
        m.insert((42, "-".to_string()), Accion::Reduce(48));
        m.insert((62, "cte_ent".to_string()), Accion::Reduce(37));
        m.insert((66, "-".to_string()), Accion::Shift(30));
        m.insert((35, "!=".to_string()), Accion::Shift(62));
        m.insert((99, "[".to_string()), Accion::Reduce(20));
        m.insert((59, "-".to_string()), Accion::Reduce(35));
        m.insert((95, "id".to_string()), Accion::Reduce(21));
        m.insert((75, "(".to_string()), Accion::Shift(33));
        m.insert((56, "*".to_string()), Accion::Reduce(38));
        m.insert((16, "=".to_string()), Accion::Shift(85));
        m.insert((69, ")".to_string()), Accion::Reduce(41));
        m.insert((80, "id".to_string()), Accion::Shift(32));
        m.insert((62, "+".to_string()), Accion::Reduce(37));
        m.insert((12, "fin".to_string()), Accion::Shift(13));
        m.insert((87, "escribe".to_string()), Accion::Reduce(26));
        m.insert((40, ";".to_string()), Accion::Reduce(52));
        m.insert((26, "[".to_string()), Accion::Reduce(18));
        m.insert((11, "id".to_string()), Accion::Shift(16));
        m.insert((31, "id".to_string()), Accion::Reduce(43));
        m.insert((58, "-".to_string()), Accion::Reduce(56));
        m.insert((45, "==".to_string()), Accion::Reduce(47));
        m.insert((32, "(".to_string()), Accion::Shift(72));
        m.insert((52, "escribe".to_string()), Accion::Reduce(28));
        m.insert((41, "-".to_string()), Accion::Reduce(49));
        m.insert((27, "}".to_string()), Accion::Reduce(13));
        m.insert((57, "==".to_string()), Accion::Reduce(51));
        m.insert((99, "]".to_string()), Accion::Reduce(20));
        m.insert((26, "}".to_string()), Accion::Reduce(18));
        m.insert((25, ";".to_string()), Accion::Shift(26));
        m.insert((40, "+".to_string()), Accion::Reduce(52));
        m.insert((64, "(".to_string()), Accion::Shift(33));
        m.insert((49, ";".to_string()), Accion::Reduce(30));
        m.insert((57, ",".to_string()), Accion::Reduce(51));
        m.insert((34, ",".to_string()), Accion::Reduce(42));
        m.insert((42, "(".to_string()), Accion::Reduce(48));
        m.insert((99, "si".to_string()), Accion::Reduce(20));
        m.insert((54, ",".to_string()), Accion::Reduce(39));
        m.insert((39, ")".to_string()), Accion::Reduce(47));
        m.insert((23, "[".to_string()), Accion::Reduce(19));
        m.insert((55, "-".to_string()), Accion::Reduce(55));
        m.insert((56, "<".to_string()), Accion::Reduce(38));
        m.insert((57, "-".to_string()), Accion::Reduce(51));
        m.insert((68, "!=".to_string()), Accion::Reduce(42));
        m.insert((104, "id".to_string()), Accion::Shift(106));
        m.insert((45, "+".to_string()), Accion::Reduce(47));
        m.insert((39, "-".to_string()), Accion::Reduce(47));
        m.insert((86, ";".to_string()), Accion::Shift(87));
        m.insert((96, ",".to_string()), Accion::Shift(92));
        m.insert((72, "-".to_string()), Accion::Shift(30));
        m.insert((58, ")".to_string()), Accion::Reduce(56));
        m.insert((5, "id".to_string()), Accion::Shift(100));
        m.insert((85, "+".to_string()), Accion::Shift(31));
        m.insert((36, "id".to_string()), Accion::Reduce(53));
        m.insert((67, ">".to_string()), Accion::Reduce(40));
        m.insert((58, ">".to_string()), Accion::Reduce(56));
        m.insert((58, ";".to_string()), Accion::Reduce(56));
        m.insert((78, ")".to_string()), Accion::Reduce(68));
        m.insert((26, "mientras".to_string()), Accion::Reduce(18));
        m.insert((79, "-".to_string()), Accion::Reduce(65));
        m.insert((24, "si".to_string()), Accion::Shift(18));
        m.insert((49, "sino".to_string()), Accion::Shift(50));
        m.insert((71, "*".to_string()), Accion::Reduce(50));
        m.insert((4, "{".to_string()), Accion::Reduce(3));
        m.insert((24, "escribe".to_string()), Accion::Shift(15));
        m.insert((58, "==".to_string()), Accion::Reduce(56));
        m.insert((54, "+".to_string()), Accion::Reduce(39));
        m.insert((7, "inicio".to_string()), Accion::Reduce(5));
        m.insert((61, "cte_ent".to_string()), Accion::Reduce(34));
        m.insert((68, "-".to_string()), Accion::Shift(30));
        m.insert((99, "mientras".to_string()), Accion::Reduce(20));
        m.insert((65, ")".to_string()), Accion::Reduce(32));
        m.insert((40, "-".to_string()), Accion::Reduce(52));
        m.insert((33, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((95, "si".to_string()), Accion::Reduce(21));
        m.insert((60, "cte_ent".to_string()), Accion::Reduce(36));
        m.insert((23, "id".to_string()), Accion::Reduce(19));
        m.insert((92, "-".to_string()), Accion::Shift(30));
        m.insert((64, "+".to_string()), Accion::Shift(31));
        m.insert((2, "$".to_string()), Accion::Accept);
        m.insert((11, "[".to_string()), Accion::Shift(14));
        m.insert((90, ",".to_string()), Accion::Shift(92));
        m.insert((11, "escribe".to_string()), Accion::Shift(15));
        m.insert((59, "cte_ent".to_string()), Accion::Reduce(35));
        m.insert((55, ",".to_string()), Accion::Reduce(55));
        m.insert((42, "cte_ent".to_string()), Accion::Reduce(48));
        m.insert((56, "==".to_string()), Accion::Reduce(38));
        m.insert((79, "/".to_string()), Accion::Reduce(65));
        m.insert((95, "[".to_string()), Accion::Reduce(21));
        m.insert((58, "+".to_string()), Accion::Reduce(56));
        m.insert((79, ";".to_string()), Accion::Reduce(65));
        m.insert((46, "-".to_string()), Accion::Reduce(46));
        m.insert((39, "+".to_string()), Accion::Reduce(47));
        m.insert((40, ">".to_string()), Accion::Reduce(52));
        m.insert((71, ")".to_string()), Accion::Reduce(50));
        m.insert((55, ")".to_string()), Accion::Reduce(55));
        m.insert((57, "<".to_string()), Accion::Reduce(51));
        m.insert((3, ";".to_string()), Accion::Shift(4));
        m.insert((45, "<".to_string()), Accion::Reduce(47));
        m.insert((46, "==".to_string()), Accion::Reduce(46));
        m.insert((55, "!=".to_string()), Accion::Reduce(55));
        m.insert((68, "==".to_string()), Accion::Reduce(42));
        m.insert((103, "inicio".to_string()), Accion::Reduce(6));
        m.insert((54, "<".to_string()), Accion::Reduce(39));
        m.insert((64, "id".to_string()), Accion::Shift(32));
        m.insert((79, ">".to_string()), Accion::Reduce(65));
        m.insert((62, "cte_flot".to_string()), Accion::Reduce(37));
        m.insert((29, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((29, "id".to_string()), Accion::Shift(32));
        m.insert((55, ">".to_string()), Accion::Reduce(55));
        m.insert((84, "mientras".to_string()), Accion::Reduce(27));
        m.insert((52, "]".to_string()), Accion::Reduce(28));
        m.insert((99, "id".to_string()), Accion::Reduce(20));
        m.insert((44, ">".to_string()), Accion::Reduce(45));
        m.insert((59, "id".to_string()), Accion::Reduce(35));
        m.insert((79, "<".to_string()), Accion::Reduce(65));
        m.insert((103, "{".to_string()), Accion::Reduce(6));
        m.insert((31, "-".to_string()), Accion::Reduce(43));
        m.insert((34, "==".to_string()), Accion::Reduce(42));
        m.insert((52, "[".to_string()), Accion::Reduce(28));
        m.insert((43, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((19, "mientras".to_string()), Accion::Reduce(16));
        m.insert((31, "+".to_string()), Accion::Reduce(43));
        m.insert((28, "fin".to_string()), Accion::Reduce(12));
        m.insert((41, "cte_flot".to_string()), Accion::Reduce(49));
        m.insert((53, ";".to_string()), Accion::Reduce(29));
        m.insert((69, "==".to_string()), Accion::Reduce(41));
        m.insert((66, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((92, "+".to_string()), Accion::Shift(31));
        m.insert((41, "(".to_string()), Accion::Reduce(49));
        m.insert((36, "cte_flot".to_string()), Accion::Reduce(53));
        m.insert((20, "]".to_string()), Accion::Reduce(17));
        m.insert((24, "}".to_string()), Accion::Reduce(14));
        m.insert((67, "!=".to_string()), Accion::Reduce(40));
        m.insert((33, "+".to_string()), Accion::Shift(31));
        m.insert((9, "inicio".to_string()), Accion::Reduce(4));
        m.insert((101, ":".to_string()), Accion::Shift(102));
        m.insert((28, "si".to_string()), Accion::Reduce(12));
        m.insert((87, "]".to_string()), Accion::Reduce(26));
        m.insert((47, "entonces".to_string()), Accion::Shift(48));
        m.insert((87, "id".to_string()), Accion::Reduce(26));
        m.insert((39, "==".to_string()), Accion::Reduce(47));
        m.insert((80, "-".to_string()), Accion::Shift(30));
        m.insert((91, ",".to_string()), Accion::Reduce(22));
        m.insert((19, "si".to_string()), Accion::Reduce(16));
        m.insert((79, "!=".to_string()), Accion::Reduce(65));
        m.insert((85, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((42, "cte_flot".to_string()), Accion::Reduce(48));
        m.insert((35, ";".to_string()), Accion::Reduce(33));
        m.insert((63, ",".to_string()), Accion::Reduce(31));
        m.insert((72, "id".to_string()), Accion::Reduce(54));
        m.insert((10, "{".to_string()), Accion::Shift(11));
        m.insert((19, "escribe".to_string()), Accion::Reduce(16));
        m.insert((20, "mientras".to_string()), Accion::Reduce(17));
        m.insert((39, "/".to_string()), Accion::Shift(41));
        m.insert((57, "/".to_string()), Accion::Reduce(51));
        m.insert((75, "id".to_string()), Accion::Shift(32));
        m.insert((58, ",".to_string()), Accion::Reduce(56));
        m.insert((23, "}".to_string()), Accion::Reduce(19));
        m.insert((14, "[".to_string()), Accion::Shift(14));
        m.insert((45, ";".to_string()), Accion::Reduce(47));
        m.insert((22, "}".to_string()), Accion::Reduce(15));
        m.insert((41, "+".to_string()), Accion::Reduce(49));
        m.insert((79, "+".to_string()), Accion::Reduce(65));
        m.insert((83, "{".to_string()), Accion::Shift(11));
        m.insert((89, ",".to_string()), Accion::Reduce(23));
        m.insert((67, ")".to_string()), Accion::Reduce(40));
        m.insert((26, "si".to_string()), Accion::Reduce(18));
        m.insert((75, "-".to_string()), Accion::Shift(30));
        m.insert((69, ";".to_string()), Accion::Reduce(41));
        m.insert((57, "*".to_string()), Accion::Reduce(51));
        m.insert((29, "(".to_string()), Accion::Shift(33));
        m.insert((84, "]".to_string()), Accion::Reduce(27));
        m.insert((43, "(".to_string()), Accion::Shift(33));
        m.insert((28, "sino".to_string()), Accion::Reduce(12));
        m.insert((106, ":".to_string()), Accion::Reduce(9));
        m.insert((84, "id".to_string()), Accion::Reduce(27));
        m.insert((34, ";".to_string()), Accion::Reduce(42));
        m.insert((11, "mientras".to_string()), Accion::Shift(17));
        m.insert((34, ">".to_string()), Accion::Reduce(42));
        m.insert((44, "+".to_string()), Accion::Reduce(45));
        m.insert((84, "}".to_string()), Accion::Reduce(27));
        m.insert((60, "cte_flot".to_string()), Accion::Reduce(36));
        m.insert((23, "si".to_string()), Accion::Reduce(19));
        m.insert((67, ";".to_string()), Accion::Reduce(40));
        m.insert((40, "==".to_string()), Accion::Reduce(52));
        m.insert((24, "mientras".to_string()), Accion::Shift(17));
        m.insert((29, "+".to_string()), Accion::Shift(31));
        m.insert((63, ";".to_string()), Accion::Reduce(31));
        m.insert((58, "*".to_string()), Accion::Reduce(56));
        m.insert((19, "]".to_string()), Accion::Reduce(16));
        m.insert((39, ">".to_string()), Accion::Reduce(47));
        m.insert((30, "cte_ent".to_string()), Accion::Reduce(44));
        m.insert((39, ",".to_string()), Accion::Reduce(47));
        m.insert((15, "(".to_string()), Accion::Shift(88));
        m.insert((44, ")".to_string()), Accion::Reduce(45));
        m.insert((44, "<".to_string()), Accion::Reduce(45));
        m.insert((22, "id".to_string()), Accion::Reduce(15));
        m.insert((68, ";".to_string()), Accion::Reduce(42));
        m.insert((94, ";".to_string()), Accion::Shift(95));
        m.insert((92, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((70, ")".to_string()), Accion::Shift(71));
        m.insert((40, "<".to_string()), Accion::Reduce(52));
        m.insert((37, "cte_flot".to_string()), Accion::Shift(54));
        m.insert((46, "+".to_string()), Accion::Reduce(46));
        m.insert((56, "/".to_string()), Accion::Reduce(38));
        m.insert((14, "id".to_string()), Accion::Shift(16));
        m.insert((30, "-".to_string()), Accion::Reduce(44));
        m.insert((54, ")".to_string()), Accion::Reduce(39));
        m.insert((28, "]".to_string()), Accion::Reduce(12));
        m.insert((65, ";".to_string()), Accion::Reduce(32));
        m.insert((56, ",".to_string()), Accion::Reduce(38));
        m.insert((40, "/".to_string()), Accion::Reduce(52));
        m.insert((40, "*".to_string()), Accion::Reduce(52));
        m.insert((55, "==".to_string()), Accion::Reduce(55));
        m.insert((65, ",".to_string()), Accion::Reduce(32));
        m.insert((52, "id".to_string()), Accion::Reduce(28));
        m.insert((88, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((64, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((20, "escribe".to_string()), Accion::Reduce(17));
        m.insert((92, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((52, "si".to_string()), Accion::Reduce(28));
        m.insert((41, "id".to_string()), Accion::Reduce(49));
        m.insert((71, "-".to_string()), Accion::Reduce(50));
        m.insert((46, ">".to_string()), Accion::Reduce(46));
        m.insert((92, "(".to_string()), Accion::Shift(33));
        m.insert((99, "escribe".to_string()), Accion::Reduce(20));
        m.insert((58, "/".to_string()), Accion::Reduce(56));
        m.insert((42, "id".to_string()), Accion::Reduce(48));
        m.insert((44, ";".to_string()), Accion::Reduce(45));
        m.insert((6, "{".to_string()), Accion::Reduce(2));
        m.insert((44, "==".to_string()), Accion::Reduce(45));
        m.insert((69, ",".to_string()), Accion::Reduce(41));
        m.insert((11, "}".to_string()), Accion::Reduce(14));
        m.insert((66, "id".to_string()), Accion::Reduce(54));
        m.insert((95, "}".to_string()), Accion::Reduce(21));
        m.insert((14, "}".to_string()), Accion::Reduce(14));
        m.insert((30, "cte_flot".to_string()), Accion::Reduce(44));
        m.insert((66, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((45, "/".to_string()), Accion::Shift(41));
        m.insert((58, "!=".to_string()), Accion::Reduce(56));
        m.insert((87, "mientras".to_string()), Accion::Reduce(26));
        m.insert((33, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((92, "id".to_string()), Accion::Shift(32));
        m.insert((20, "si".to_string()), Accion::Reduce(17));
        m.insert((4, "vars".to_string()), Accion::Shift(5));
        m.insert((88, "letrero".to_string()), Accion::Shift(89));
        m.insert((54, "!=".to_string()), Accion::Reduce(39));
        m.insert((43, "-".to_string()), Accion::Shift(30));
        m.insert((68, ">".to_string()), Accion::Reduce(42));
        m.insert((56, ">".to_string()), Accion::Reduce(38));
        m.insert((75, "+".to_string()), Accion::Shift(31));
        m.insert((88, "+".to_string()), Accion::Shift(31));
        m.insert((87, "}".to_string()), Accion::Reduce(26));
        m.insert((54, "/".to_string()), Accion::Reduce(39));
        m.insert((56, ")".to_string()), Accion::Reduce(38));
        m.insert((80, "cte_ent".to_string()), Accion::Reduce(54));
        m.insert((105, ":".to_string()), Accion::Reduce(7));
        m.insert((79, ",".to_string()), Accion::Reduce(65));
        m.insert((88, "(".to_string()), Accion::Shift(33));
        m.insert((60, "(".to_string()), Accion::Reduce(36));
        m.insert((16, "(".to_string()), Accion::Shift(72));
        m.insert((44, "-".to_string()), Accion::Reduce(45));
        m.insert((56, "!=".to_string()), Accion::Reduce(38));
        m.insert((77, ",".to_string()), Accion::Shift(75));
        m.insert((67, "==".to_string()), Accion::Reduce(40));
        m.insert((71, "/".to_string()), Accion::Reduce(50));
        m.insert((54, ">".to_string()), Accion::Reduce(39));
        m.insert((71, ";".to_string()), Accion::Reduce(50));
        m.insert((39, "*".to_string()), Accion::Shift(42));
        m.insert((95, "mientras".to_string()), Accion::Reduce(21));
        m.insert((31, "cte_flot".to_string()), Accion::Reduce(43));
        m.insert((4, "inicio".to_string()), Accion::Reduce(3));
        m.insert((45, ">".to_string()), Accion::Reduce(47));
        m.insert((26, "escribe".to_string()), Accion::Reduce(18));
        m.insert((95, "]".to_string()), Accion::Reduce(21));
        m.insert((55, "<".to_string()), Accion::Reduce(55));
        m.insert((68, ",".to_string()), Accion::Reduce(42));
        m.insert((85, "cte_flot".to_string()), Accion::Reduce(54));
        m.insert((85, "(".to_string()), Accion::Shift(33));
        m
    };

    /// Tabla GOTO: (estado, no-terminal) -> estado_destino
    pub static ref TABLA_GOTO: HashMap<(usize, String), usize> = {
        let mut m = HashMap::new();
        m.insert((74, "<EXPRESIÓN_LIST>".to_string()), 76);
        m.insert((75, "<+-_OPT>".to_string()), 37);
        m.insert((72, "<+-_OPT>".to_string()), 37);
        m.insert((66, "<LLAMADA>".to_string()), 40);
        m.insert((64, "<+->".to_string()), 36);
        m.insert((33, "<FACTOR>".to_string()), 39);
        m.insert((92, "<OBJ_IMPRIME>".to_string()), 96);
        m.insert((64, "<TÉRMINO>".to_string()), 34);
        m.insert((85, "<+->".to_string()), 36);
        m.insert((14, "<IMPRIME>".to_string()), 23);
        m.insert((85, "<LLAMADA>".to_string()), 40);
        m.insert((75, "<EXP>".to_string()), 35);
        m.insert((11, "<ESTATUTO>".to_string()), 24);
        m.insert((37, "<CTE>".to_string()), 58);
        m.insert((11, "<CONDICIÓN>".to_string()), 19);
        m.insert((33, "<EXP>".to_string()), 35);
        m.insert((75, "<EXPRESIÓN>".to_string()), 77);
        m.insert((14, "<CICLO>".to_string()), 20);
        m.insert((24, "<ASIGNA>".to_string()), 22);
        m.insert((72, "<FACTOR>".to_string()), 39);
        m.insert((45, "<*/>".to_string()), 43);
        m.insert((39, "<TÉRMINO'>".to_string()), 44);
        m.insert((66, "<FACTOR>".to_string()), 39);
        m.insert((88, "<EXPRESIÓN>".to_string()), 91);
        m.insert((100, "<VAR_LIST'>".to_string()), 105);
        m.insert((92, "<EXP>".to_string()), 35);
        m.insert((80, "<TÉRMINO>".to_string()), 34);
        m.insert((11, "<CICLO>".to_string()), 20);
        m.insert((24, "<CICLO>".to_string()), 20);
        m.insert((85, "<FACTOR>".to_string()), 39);
        m.insert((88, "<+-_OPT>".to_string()), 37);
        m.insert((11, "<IMPRIME>".to_string()), 23);
        m.insert((14, "<ESTATUTO_LIST>".to_string()), 98);
        m.insert((11, "<ESTATUTO_LIST>".to_string()), 21);
        m.insert((64, "<EXP>".to_string()), 65);
        m.insert((88, "<LLAMADA>".to_string()), 40);
        m.insert((92, "<EXPRESIÓN>".to_string()), 91);
        m.insert((50, "<CUERPO>".to_string()), 53);
        m.insert((64, "<FACTOR>".to_string()), 39);
        m.insert((75, "<LLAMADA>".to_string()), 40);
        m.insert((43, "<+->".to_string()), 36);
        m.insert((14, "<ESTATUTO>".to_string()), 24);
        m.insert((85, "<TÉRMINO>".to_string()), 34);
        m.insert((33, "<LLAMADA>".to_string()), 40);
        m.insert((34, "<EXP’>".to_string()), 67);
        m.insert((68, "<EXP’>".to_string()), 69);
        m.insert((33, "<TÉRMINO>".to_string()), 34);
        m.insert((80, "<+-_OPT>".to_string()), 37);
        m.insert((43, "<LLAMADA>".to_string()), 40);
        m.insert((72, "<EXPRESIÓN_OPT>".to_string()), 73);
        m.insert((7, "<FUNCS><FUNCS_LIST>".to_string()), 9);
        m.insert((80, "<+->".to_string()), 36);
        m.insert((66, "<TÉRMINO>".to_string()), 68);
        m.insert((48, "<CUERPO>".to_string()), 49);
        m.insert((10, "<CUERPO>".to_string()), 12);
        m.insert((66, "<+->".to_string()), 36);
        m.insert((4, "<VARS_OPT>".to_string()), 7);
        m.insert((85, "<EXP>".to_string()), 35);
        m.insert((14, "<LLAMADA>".to_string()), 25);
        m.insert((29, "<FACTOR>".to_string()), 39);
        m.insert((64, "<LLAMADA>".to_string()), 40);
        m.insert((43, "<FACTOR>".to_string()), 45);
        m.insert((72, "<LLAMADA>".to_string()), 40);
        m.insert((29, "<LLAMADA>".to_string()), 40);
        m.insert((5, "<VAR_LIST>".to_string()), 101);
        m.insert((64, "<+-_OPT>".to_string()), 37);
        m.insert((29, "<+->".to_string()), 36);
        m.insert((72, "<+->".to_string()), 36);
        m.insert((68, "<+->".to_string()), 66);
        m.insert((80, "<FACTOR>".to_string()), 39);
        m.insert((35, "<EXPRESIÓN'>".to_string()), 63);
        m.insert((72, "<TÉRMINO>".to_string()), 34);
        m.insert((0, "<Programa>".to_string()), 2);
        m.insert((88, "<+->".to_string()), 36);
        m.insert((75, "<TÉRMINO>".to_string()), 34);
        m.insert((92, "<LLAMADA>".to_string()), 40);
        m.insert((29, "<EXPRESIÓN>".to_string()), 38);
        m.insert((80, "<LLAMADA>".to_string()), 40);
        m.insert((72, "<EXPRESIÓN>".to_string()), 74);
        m.insert((11, "<LLAMADA>".to_string()), 25);
        m.insert((85, "<EXPRESIÓN>".to_string()), 86);
        m.insert((24, "<CONDICIÓN>".to_string()), 19);
        m.insert((33, "<EXPRESIÓN>".to_string()), 70);
        m.insert((33, "<+-_OPT>".to_string()), 37);
        m.insert((80, "<EXPRESIÓN>".to_string()), 81);
        m.insert((24, "<ESTATUTO_LIST>".to_string()), 27);
        m.insert((35, "<OPERADOR>".to_string()), 64);
        m.insert((34, "<+->".to_string()), 66);
        m.insert((14, "<ASIGNA>".to_string()), 22);
        m.insert((24, "<IMPRIME>".to_string()), 23);
        m.insert((39, "<*/>".to_string()), 43);
        m.insert((29, "<EXP>".to_string()), 35);
        m.insert((29, "<+-_OPT>".to_string()), 37);
        m.insert((92, "<TÉRMINO>".to_string()), 34);
        m.insert((45, "<TÉRMINO'>".to_string()), 46);
        m.insert((83, "<CUERPO>".to_string()), 84);
        m.insert((72, "<EXP>".to_string()), 35);
        m.insert((37, "<CTE_OPT>".to_string()), 57);
        m.insert((92, "<FACTOR>".to_string()), 39);
        m.insert((24, "<LLAMADA>".to_string()), 25);
        m.insert((4, "<VARS>".to_string()), 6);
        m.insert((66, "<+-_OPT>".to_string()), 37);
        m.insert((92, "<+->".to_string()), 36);
        m.insert((96, "<IMPRIME_LIST>".to_string()), 97);
        m.insert((77, "<EXPRESIÓN_LIST>".to_string()), 78);
        m.insert((29, "<TÉRMINO>".to_string()), 34);
        m.insert((88, "<OBJ_IMPRIME>".to_string()), 90);
        m.insert((90, "<IMPRIME_LIST>".to_string()), 93);
        m.insert((49, "<SINO_OPT>".to_string()), 51);
        m.insert((43, "<+-_OPT>".to_string()), 37);
        m.insert((11, "<ASIGNA>".to_string()), 22);
        m.insert((88, "<FACTOR>".to_string()), 39);
        m.insert((80, "<EXP>".to_string()), 35);
        m.insert((88, "<TÉRMINO>".to_string()), 34);
        m.insert((85, "<+-_OPT>".to_string()), 37);
        m.insert((14, "<CONDICIÓN>".to_string()), 19);
        m.insert((24, "<ESTATUTO>".to_string()), 24);
        m.insert((33, "<+->".to_string()), 36);
        m.insert((92, "<+-_OPT>".to_string()), 37);
        m.insert((88, "<EXP>".to_string()), 35);
        m.insert((106, "<VAR_LIST'>".to_string()), 107);
        m.insert((75, "<+->".to_string()), 36);
        m.insert((7, "<FUNCS_LIST>".to_string()), 8);
        m.insert((75, "<FACTOR>".to_string()), 39);
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
        Regla { id: 6, cabeza: "<VARS>".to_string(), longitud_cuerpo: 4 },
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
