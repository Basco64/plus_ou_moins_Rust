use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn input (message:&str) ->String{
    use std::io::Write;
    println!("{}",message);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim_end().to_string()
}



fn main() {
    
    println!("Bonjour.");

    let player_name = input("Veuillez entrer votre nom.");

    println!("Bienvenue {}.", player_name);

    let game = true;
    while game == true {
        println!("Devinez le nombre mystère en 6 coups!");


        let secret_number = rand::thread_rng().gen_range(1..101);

        let tries_max=6;
        let mut tries=0;
        
        println!("Veuillez entrer un nombre entre 1 et 100.");

        while tries < tries_max{
            
            let mut supposition = String::new();
            
            io::stdin()
                .read_line(&mut supposition)
                .expect("Echec de la lecture de l'entrée de l'utlisateur.");
                
                let supposition: u32 = match supposition.trim().parse(){
                    Ok(nombre) => nombre,
                    Err(_) => continue,
                };
            

            if supposition < 1 || supposition > 100{
                println!{"Le nombre proposé doit etre entre 1 et 100."}
                continue
            };

            println!("Votre nombre : {}",supposition);

            match supposition.cmp(&secret_number){
                Ordering::Less => println!("C'est plus ! "),
                Ordering::Greater => println!("C'est moins ! "),
                Ordering::Equal => {
                    println!("C'est gagné !");
                    println!("Bien joué {}!", player_name);
                    break;
                }
            }
            tries+=1;
            match tries {
                0|1|2|3|4  => {
                    println!("Il vous reste {} chances!", tries_max-tries)
                }
                5 => {
                    println!("Il ne vous reste qu'une seule chance!")
                }
                6 => {
                    println!("Perdu pour cette fois, n'hesitez pas a re-tenter votre chance. :-)")
                }
                i32::MIN..=-1_i32 | 6_i32..=i32::MAX => todo!() // s'assurer que tous les cas possibles sont traités en ajoutant un bras de correspondance avec un motif joker, un bras de correspondance avec plusieurs motifs ou comme indiqué, ou plusieurs bras de correspondance.
            };
        }

        println!("{}","-".repeat(50));
        
        let relaunch = input("Voules vous relancez une partie? (O pour continuer)");
        
        if relaunch=="o" || relaunch=="O"{
            continue
        }else{
            break;
        }
    }
}