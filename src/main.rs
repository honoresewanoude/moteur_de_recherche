use std::collections::HashMap;
use std::fs;
//use std::collections;


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

    /**fn vecteur (){
        let mut index: HashMap<String , Vec<usize>> = HashMap::new();
        index.insert("rust".to_string(), vec![1,2]);
        index.insert("rapide".to_string(), vec![1,3]);
        println!("{:?}",index["rapide"]);
    }**/



    fn ajouter_document(index: &mut HashMap<String , Vec<usize>>, mot : &str, id : usize){
        let document = index.entry(mot.to_string()).or_insert_with(Vec::new);
        if !document.contains(&id){document.push(id);}
    }



fn main() {
    read_txt();
    //vecteur();

    let mut index: HashMap<String, Vec<usize>> = HashMap::new();
    ajouter_document(&mut index, "rust", 3);
    ajouter_document(&mut index, "rapide", 4);
    ajouter_document(&mut index, "rust", 8);
    ajouter_document(&mut index, "logiciel", 3);

    println!("{:?}",index);

}
