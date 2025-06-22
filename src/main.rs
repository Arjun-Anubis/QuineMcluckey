use std::io;

// An implicant contains a string and an enable flag
#[derive(Debug)]
struct Implicant {
    representation: Vec<char>,
    enable: bool
}
fn read_int() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s)
        .unwrap();
    s.trim().parse().expect("Not a valid number")
}

fn load_n_from_stdin() -> i32 {
    println!("Please enter n: ");
    let n:i32 = read_int();
    return n
}


fn initialize(main: &mut Vec<Vec<Implicant>>, n: i32) {
    let mut k = 0;
    while k <= n {
        main.push( Vec::new() );
        k+=1;
    }
}

fn load_minterms_from_stdin( n: usize ) -> Vec<Implicant> {
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

fn perform_merges( source: &mut Vec<Implicant> , target: &mut Vec<Implicant> ) -> i32 {
    let mut merge_count = 0;
    for i in 0.. source.len() {
        for j in i+1..source.len() {
            // println!("Checking {:?} and {:?}", source[i], source[j] );
            match check_merge( &source[i], &source[j] )
            {
                Some(location) => {
                    // println!("Merging at location {}", location );
                    source[i].enable = false;
                    source[j].enable = false;
                    target.push( merge( &source[i], &source[j], location) );
                    merge_count+=1;
                },
                None => {

                    // println!("Merge not possible");
                }
            }
        }
    }
    println!("Merge count is {}", merge_count);
    return merge_count
}

fn check_merge( implicant1: &Implicant, implicant2: &Implicant ) -> Option<usize> {
    let mut found_difference = false;
    let mut location = 0;
    if true {
        assert!( implicant1.representation.len() == implicant2.representation.len() );
        let length = implicant1.representation.len();
        for i in 0..length {
            if implicant1.representation[i] != implicant2.representation[i] {
                if implicant1.representation[i] == '-' || implicant2.representation[i] == '-' {
                    return None
                }
                if found_difference {
                    return None
                }
                location = i;
                found_difference = true
            }
        }
        if found_difference {
            return Some(location)
        } 
        return None
    }
    return None
}
// This will create a merged implicant to be pushed into the target vector
fn merge( implicant1: &Implicant, _implicant2: &Implicant, location: usize ) ->  Implicant {
    let mut rep: Vec<char> = implicant1.representation.clone();
    rep[location] = '-';
    let merged_implicant : Implicant = Implicant { representation: rep, enable : true };
    return merged_implicant;
    
}


fn main() {
    // We make a vector of a vector of implicants, main[size] has implicants of size 2^size
    let mut main : Vec<Vec<Implicant>> = Vec::new();
    // We load n from stdin
    let n : i32 = load_n_from_stdin();

    // The max size an implicant can have is 2^n, so we have to initialize main[0] to main [n] which
    // is n + 1 elements
    initialize(&mut main,n);
    main[0] = load_minterms_from_stdin( n.try_into().unwrap() );


    // This loop can happen up-to n times from k=0 to k=n-1, since k+1=n and main is initialised up-to
    // main[n]
    let mut k: usize = 0;
    while k < n.try_into().unwrap() {
        
        println!("Size 2^{}", k);
        // First we split at mutable to make sure we can have two different mutable references to
        // main[k] and main [k+1] simultanouesly
        let (left, right) = main.split_at_mut(k+1);
        // If there are zero merges then the algorithm is done
        if perform_merges(&mut left[k], &mut right[0]) == 0 {
            break;
        }
        k+=1;
    }
    println!("Final State");
    println!("Main is {:#?}", main );
}

