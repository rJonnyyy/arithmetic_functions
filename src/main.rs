use std::io;

fn main() {
    //teilbarkeit
    let wahrheitswert: bool = prüfung_teilbarkeit();
    if wahrheitswert == true {
        println!("JUHU! Deine Zahl ist durch 4, 10 und 20 ganzzahlig teilbar!");
    } else {
        println!("Schade! Deine Zahl ist nicht durch 4, 10 und 20 teilbar!");
    }

    //primzahltest
    primzahltest();

    //primzahltesteteter
    primzahltestertester(5);
    primzahltestertester(17);
    primzahltestertester(25);
    primzahltestertester(19);
    primzahltestertester(111);
    primzahltestertester(121);

    //fakultät berechnen
    let fakultät = fakultät_berechnen(6);
    println!("Die Fakultät der Zahl ist {}", fakultät);
}

fn prüfung_teilbarkeit() -> bool {
    let mut benutzereingabe = String::new();
    println!("Gebe eine Zahl ein, das Programm überprüft dann auf Teilbarkeit");
    io::stdin()
        .read_line(&mut benutzereingabe)
        .expect("not a valid i32 integer!");
    let zahl: i32 = benutzereingabe
        .trim()
        .parse()
        .expect("not a valid i32 integer!");

    if zahl % 4 == 0 && zahl % 10 == 0 && zahl % 20 == 0 {
        true
    } else {
        false
    }
}

fn primzahltest() {
    let mut benutzereingabe = String::new();
    println!("Gebe eine (vermeintliche) Primzahl ein!");
    io::stdin()
        .read_line(&mut benutzereingabe)
        .expect("not a valid u32 integer");
    let testzahl: u32 = benutzereingabe.trim().parse().unwrap();

    let mut is_prim: bool = true;

    for number in 2..=(testzahl / 2) {
        if (testzahl % number) == 0 {
            is_prim = false;
            break;
        }
    }
    if is_prim == true {
        println!("Die Zahl {} ist eine Primzahl!", testzahl);
    } else {
        println!("Die Zahl {} ist keine Primzahl", testzahl);
    }
}

fn primzahltestertester(testzahl: u32) {
    let mut is_prim: bool = true;

    for number in 2..=(testzahl / 2) {
        //theoretisch reicht es bis hin zu testzahl.sqrt() + 1 zu iterieren, (jedoch Datentypfehler ig -> f64) es könnte dabei ein Fehler entstehen wenn bspw. die Wurzel von 37 gezogen wird und das Ergebnis ein f64 ist
        if (testzahl % number) == 0 {
            is_prim = false;
            break;
        }
    }
    if is_prim == true {
        println!("Die Zahl {} ist eine Primzahl!", testzahl);
    } else {
        println!("Die Zahl {} ist keine Primzahl", testzahl);
    }
}

fn fakultät_berechnen(spielzahl: u32) -> u32 {
    //! spielzahl = Zahl, von der die Fakultät berechnet werden soll z.B.3-> 3*2*1 || 1*2*3
    let mut fakultätszahl = spielzahl;
    for number in 1..spielzahl {
        fakultätszahl = fakultätszahl * number;
    }
    fakultätszahl
}

// fn binominalkoef_berechnen() {}

// fn summenzeichen_berechnen() {}
