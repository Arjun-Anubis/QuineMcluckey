use std::io;
use crate::structs:: Implicant;

pub const default_letters: [char; 4] = ['a','b','c','d'];

fn read_int() -> Result<i32, std::num::ParseIntError> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)
        .unwrap();
    let x = s.trim().parse()?;
    Ok(x)
}

pub fn load_n_from_stdin() -> i32 {
    println!("Please enter n: ");
    match read_int() {
        Ok(n) => { return n }
        Err(_e) => {panic!("Invalid value for n");}
    }
}

pub fn load_minterms_from_stdin( n: usize ) -> Vec<Implicant> {
    let mut implicants: Vec<Implicant> = Vec::new();
    println!("Enter minterms:");
    loop {
        match read_int() {
            Ok(minterm) => {
                let implicant : Implicant = Implicant {
                    representation: format!("{:0width$b}",minterm, width = n).chars().collect(),
                    unmerged : true
                };
                implicants.push(implicant)
            }
            Err(_e) => {break;}
        }
    }
    implicants
}

pub fn display_implicants_as_SOP( implicants: &Vec<Implicant> ) {
    for i in 0..implicants.len() {
        if i!=0 {
            print!( " + " );
        }
        print!("{}", implicants[i].as_product() ); 
    }
    print!( "\n" );
}
