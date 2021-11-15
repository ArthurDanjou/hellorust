fn main() {
    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("{}", s);

    let s2 = "Hello";
    //s2.push_str(", World!"); Error
    println!("{}", s2);

    let s3 = s;
    println!("{}", s3);
    //println!("{}", s); The value is borrowed

    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    let (s5, taille) = calculer_taille(s4);
    println!("s5 = {}, taille = {}", s5, taille);

    let taille2 = taille_sans_possession(&s5);
    println!("s5 = {}, taille = {}", s5, taille2);
}

fn calculer_taille(s: String) -> (String, usize) {
    let taille = s.len();
    (s, taille)
}

fn taille_sans_possession(s: &String) -> usize {
    s.len()
}