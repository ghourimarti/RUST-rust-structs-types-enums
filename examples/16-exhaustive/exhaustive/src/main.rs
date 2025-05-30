enum WineGrapes {
    CabernetFranc,
    Tannat,
    Merlot,
}

fn taste_wine(grapes: WineGrapes) {
    match grapes {
        WineGrapes::CabernetFranc => println!("This is a Cabertnet Franc wine."),
        _ => println!("This is a NOT Cabertnet Franc wine."),

        // WineGrapes::Tannat => println!("This is a Tannat wine."),
        // WineGrapes::Merlot => println!("This is a Merlot wine."),
    }
}

fn main() {
    println!("<=====================================>");
    taste_wine(WineGrapes::CabernetFranc);
}
