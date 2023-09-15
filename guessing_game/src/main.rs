
use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main (){
println!("Adivine el numero");

let secret_number =rand::thread_rng().gen_range(1, 101);

println!("El numero secreto es ...{}" , secret_number);

loop{
    print!("Ingrese un Numero :");
    
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error al leer el dato.");

    let guess: u32 = match guess.trim().parse(){
        Ok(num )=> num,
        Err(_) => continue, 
    };

    println!("......Su numero es {}", guess );
    
    match guess.cmp(&secret_number){
        Ordering::Less => print!("{}", "Muy bajo!".red()),
        Ordering::Greater => print!("{}", "Muy alto!".red()),
        Ordering::Equal=> {
            print!("{}, Muy Bien Ganaste! :" , guess.to_string().red() );
            break;
            

        }
    }


}


}