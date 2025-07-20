// Modul plane_module obsahuje vše o letadlech
mod plane_module {
    pub struct Plane {
        pub model: String,
        pub altitude: u32,
        pub speed: u32,
    }

    pub enum PlaneType {
        Commercial,
        Private,
        Military,
    }

    impl Plane {
        pub fn print_info(&self) {
            println!("Model: {}", self.model);
            println!("Altitude: {} feet", self.altitude);
            println!("Speed: {} knots", self.speed);
        }
    }
}

// Modul target_module obsahuje vše o cílech
mod target_module {
    pub struct Target {
        pub name: String,
        pub adsb_id: String,
        pub mode_regime: String,
    }

    pub enum ModeRegime {
        ModeA,
        ModeC,
        ModeS,
        Mode5,
    }

    impl Target {
        pub fn print_info(&self) {
            println!("Target Name: {}", self.name);
            println!("ADSB ID: {}", self.adsb_id);
            println!("Mode Regime: {}", self.mode_regime);
        }
    }
}

use std::collections::{HashMap, HashSet};

// Importy
use plane_module::{Plane, PlaneType};
use target_module::{ModeRegime, Target};

// Vrací konstantní ID
fn plane_id() -> u32 {
    42
}

fn constant() -> u32 {
    let id = 42;
    println!("Plane ID is: {}", id);
    id
}

fn prochazeni_pole() {
    let mut pole = [1, 2, 3, 4, 5];

    for i in &mut pole {
        *i += 1;
    }

    println!("Updated array: {:?}", pole);
    println!("Pointer to first element: {}", &pole[0]);

    if let Some(first) = pole.first() {
        println!("First element of the array: {}", first);
    }

    let last = pole.last().unwrap_or(&0);
    println!("Last element of the array: {}", last);

    while let Some(last) = pole.last() {
        println!("Last element in while loop: {}", last);
        break;
    }

    loop {
        if let Some(last) = pole.last() {
            println!("Last element in loop: {}", last);
            break;
        } else {
            println!("Array is empty, breaking the loop.");
            break;
        }
    }
}

fn string_working() -> String {
    let mut s = String::from("Ciassss, ");
    s.push_str("Dominik!");
    println!("{}", s);
    s
}

fn string_working2() -> String {
    let mut m = String::from("Daidalos, ");
    m.push_str("Ikaros");
    println!("{}", m);
    m
}

fn string_working3() {
    let mut g = String::from("Jon");
    g.push_str(" Snow");
    println!("{}", g);
}

fn borrowing() {
    let mut s = String::from("Ahoj, světe!");
    pujceni_do_borrowing(&mut s);
    println!("Půjčený string: {}", s);
}

fn pujceni_do_borrowing(text: &mut String) {
    text.push_str(" - půjčeno!");
    println!("Text v borrowing: {}", text);
}

fn borrowing2() {
    let mut borr = String::from("winter is coming");
    pujceni_do_borrowing2(&mut borr);
    println!("Půjčený string 2: {}", borr);
}

fn pujceni_do_borrowing2(text: &mut String) {
    text.push_str(" - půjčeno!");
    println!("Text v borrowing2: {}", text);
}

fn easily_borrowing() {
    let a = 1;
    let b = &a;
    println!("Půjčená hodnota: {}", b);
}

fn lifetime_example<'a>(text: &'a str) -> &'a str {
    println!("Text v lifetime_example: {}", text);
    text
}

fn deleni_nulou(a: u64, b: u64) -> Result<u64, String> {
    if b == 0 {
        Err(String::from("Dělení nulou není povoleno!"))
    } else {
        Ok(a / b)
    }
}

fn option_example(pismeno: char, text: &str) -> Option<String> {
    if text.contains(pismeno) {
        Some(String::from("Písmeno nalezeno!"))
    } else {
        None
    }
}

// Vec<T> - dynamické pole
fn vektor_example() {
    let mut vektor: Vec<i32> = Vec::new();
    vektor.push(1);
    vektor.push(2);
    vektor.push(3);
    assert!(vektor.len() == 3);
    println!("Vektor obsahuje: {:?}", vektor);
    for elem in &vektor {
        println!("Prvek: {}", elem);
    }
}

// HashMap<K, V> - slovník klíč-hodnota
fn hashmap_example() {
    let mut hash = HashMap::new();
    hash.insert("key1", 10);
    hash.insert("key2", 20);
    hash.insert("key3", 30);
    println!("HashMap obsahuje: {:?}", hash);
    for (k, v) in &hash {
        println!("Klíč: {}, Hodnota: {}", k, v);
    }
}

// HashSet<T> - množina unikátních hodnot
fn hashset_example() {
    let mut hash_set = HashSet::new();
    hash_set.insert("jablko");
    hash_set.insert("banán");
    hash_set.insert("pomeranč");
    println!("HashSet obsahuje: {:?}", hash_set);
    for item in &hash_set {
        println!("Položka: {}", item);
    }
}

struct Pes;
struct Kocka;

trait Zvuk {
    fn vydat_zvuk(&self);
}

impl Zvuk for Pes {
    fn vydat_zvuk(&self) {
        println!("Haf Haf!");
    }
}

impl Zvuk for Kocka {
    fn vydat_zvuk(&self) {
        println!("Mňau Mňau!");
    }
}

fn main() {
    let pes = Pes;
    let kocka = Kocka;

    pes.vydat_zvuk();    // Vypíše: Haf Haf!
    kocka.vydat_zvuk();  // Vypíše: Mňau Mňau!

    hashset_example();
    hashmap_example();
    vektor_example();

    // Option
    let pismeno = 'a';
    let text = "Ahoj, světe!";
    if let Some(vysledek) = option_example(pismeno, text) {
        println!("Výsledek Option: {}", vysledek);
    }

    // Result - chyba
    match deleni_nulou(10, 0) {
        Ok(vysledek) => println!("Výsledek dělení: {}", vysledek),
        Err(e) => println!("Chyba: {}", e),
    }

    // Result - úspěch
    match deleni_nulou(10, 2) {
        Ok(vysledek) => println!("Výsledek dělení: {}", vysledek),
        Err(e) => println!("Chyba: {}", e),
    }

    // Lifetimes
    let c = lifetime_example("Hello, Rust!");
    println!("Text s životností: {}", c);

    // Borrowing
    easily_borrowing();
    borrowing();
    borrowing2();

    // Clone
    let s1 = String::from("Ahoj");
    let s2 = s1.clone();
    println!("{}", s1);
    println!("{}", s2);

    // Strings
    let text = string_working();
    println!("Vrácený text: {}", text);
    let text2 = string_working2();
    println!("Vrácený text 2: {}", text2);
    string_working3();

    // ID a pole
    let css = constant();
    println!("Constant Plane ID: {}", css);

    let id = plane_id();
    println!("Plane ID: {}", id);

    // Enum match
    let plane_type = PlaneType::Commercial;
    match plane_type {
        PlaneType::Commercial => println!("This is a commercial plane."),
        PlaneType::Private => println!("This is a private plane."),
        PlaneType::Military => println!("This is a military plane."),
    }

    let mode = ModeRegime::ModeS;
    match mode {
        ModeRegime::ModeA => println!("Mode A selected."),
        ModeRegime::ModeC => println!("Mode C selected."),
        ModeRegime::ModeS => println!("Mode S selected."),
        ModeRegime::Mode5 => println!("Mode 5 selected."),
    }

    let plane = Plane {
        model: String::from("Boeing 747"),
        altitude: 35000,
        speed: 560,
    };
    plane.print_info();

    // Pole
    prochazeni_pole();

    println!("Finished processing.");
}
