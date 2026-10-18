fn main() {

    //fonction pour découper le texte
    
    fn read_txt() {

        let texte = "Rust est un langage rapide. Rust permet de créer des logiciels fiables !";

        let sans_pontuation : String = texte.chars().filter(|c| !c.is_ascii_punctuation()).collect();

        let mot_miniscules = sans_pontuation.to_lowercase();

        for mot in mot_miniscules.split_whitespace(){
            println!("{}",mot);
                }
    }

    read_txt();

}
