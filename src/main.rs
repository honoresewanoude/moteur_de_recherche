use std::collections::HashMap;
use std::fs;
//use std::collections;


    //fonction pour découper le texte
    
    fn read_txt(chemin :&str) -> Vec<String> {

        let contenu =  fs::read_to_string(chemin);

        let texte = match contenu {
            Ok(t) => {
                //println!("Lecture avec succès\n\n");
                t
            } 
            Err(erreur) => {
                println!("Erreur de lecture {}", erreur);
                return Vec::new()
            }
        };

        let sans_pontuation : String = texte.chars().filter(|c| !c.is_ascii_punctuation()).collect();

        let mot_miniscules = sans_pontuation.to_lowercase();

        let mot: Vec<String> =  mot_miniscules.split_whitespace().map(|mot| mot.to_string()).collect();
        mot
                
    }

    /**fn vecteur (){
        let mut index: HashMap<String , Vec<usize>> = HashMap::new();
        index.insert("rust".to_string(), vec![1,2]);
        index.insert("rapide".to_string(), vec![1,3]);
        println!("{:?}",index["rapide"]);
    }**/

    //Fonction pour ajouter clé / valeur 

    fn ajouter_document(index: &mut HashMap<String , Vec<usize>>, mot : &str, id : usize){
        let document = index.entry(mot.to_string()).or_insert_with(Vec::new);
        if !document.contains(&id){document.push(id);}
    }



fn main() {
    
    //vecteur();

    let mut index: HashMap<String, Vec<usize>> = HashMap::new();

    let document = vec![(1,"/home/msh-cyber/Documents_texte/document1.txt".to_string()),
                                          (2,"/home/msh-cyber/Documents_texte/document1.txt".to_string()),];
    
   

    for (id,chemin) in &document{
         let resss = read_txt(&chemin);
          for mot in resss {
            ajouter_document(&mut index, &mot, *id);
            }

    }
    
    //println!("{:?}",index);

    let  mot_recher = "rust";

    if let Some(id) = index.get(mot_recher){
        println!("{:?}",id);
    }else{
        println!("Le mot n'existe pas");
    }

}



