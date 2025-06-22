use std::io;
use crate::structs:: Implicant;


fn read_int() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s)
        .unwrap();
    s.trim().parse().expect("Not a valid number")
}

pub fn load_n_from_stdin() -> i32 {
    println!("Please enter n: ");
    let n:i32 = read_int();
    return n
}

pub fn load_minterms_from_stdin( n: usize ) -> Vec<Implicant> {
    let mut size1_implicants: Vec<Implicant> = Vec::new();
    println!("Enter minterms: (enter negative number to end)");
    loop {
        let minterm: i32 = read_int();
        if minterm < 0 {
            break
        }
        let size1_implicant : Implicant = Implicant {
            representation: format!("{:0width$b}",minterm, width = n).chars().collect(),
            enable : true
        };
        size1_implicants.push(size1_implicant)
    }
    size1_implicants
}
