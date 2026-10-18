use std::fs;


    //fonction pour découper le texte
    
    fn read_txt() {

        let contenu =  fs::read_to_string("/home/msh-cyber/Documents_texte/document1.txt");

        let texte = match contenu {
            Ok(t) => {
                println!("Lecture avec succès\n\n");
                t
            } 
            Err(erreur) => {
                println!("Erreur de lecture {}", erreur);
                return;
            }
        };

        let sans_pontuation : String = texte.chars().filter(|c| !c.is_ascii_punctuation()).collect();

        let mot_miniscules = sans_pontuation.to_lowercase();

        for mot in mot_miniscules.split_whitespace(){
            println!("{}",mot);
                }
    }

fn main() {
    read_txt();

}
