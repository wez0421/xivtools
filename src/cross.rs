#[macro_use(lazy_static)]
use std::collections::HashSet;

lazy_static::lazy_static! {
    static ref cross_actions: HashSet<&'static str> =  {
        let mut h = HashSet::new();
        h.insert("brand of earth");
        h.insert("brand of fire");
        h.insert("brand of ice");
        h.insert("brand of lightning");
        h.insert("brand of water");
        h.insert("brand of wind");
        h.insert("byregot's blessing");
        h.insert("careful synthesis ii");
        h.insert("careful synthesis");
        h.insert("comfort zone");
        h.insert("flawless synthesis");
        h.insert("hasty touch");
        h.insert("ingenuity ii");
        h.insert("ingenuity");
        h.insert("innovation");
        h.insert("maker's mark");
        h.insert("manipulation");
        h.insert("muscle memory");
        h.insert("name of earth");
        h.insert("name of fire");
        h.insert("name of ice");
        h.insert("name of lightning");
        h.insert("name of water");
        h.insert("name of wind");
        h.insert("piece by piece");
        h.insert("rapid synthesis");
        h.insert("reclaim");
        h.insert("rumination");
        h.insert("steady hand ii");
        h.insert("tricks of the trade");
        h.insert("waste not ii");
        h.insert("waste not");
        h
    };
}
