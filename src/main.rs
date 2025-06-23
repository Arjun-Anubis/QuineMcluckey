mod ttyinput;
mod structs;

use structs::Implicant;

fn initialize(main: &mut Vec<Vec<Implicant>>, n: usize ) {
    for _k in 0..=n {
        main.push( Vec::new() );
    }
}

fn iterate_algo( source: &mut Vec<Implicant> , target: &mut Vec<Implicant> ) -> Option<usize> {
    // Record the number of merges
    let mut merge_count = 0;
    
    // Iterate pairs (i,j) in the source in a triangular fashion
    for i in 0.. source.len() {
        for j in i+1..source.len() {

            // Try to merge the terms at source(i,j)
            if let Ok([imp1, imp2]) = source.get_disjoint_mut([i,j]) {
                match imp1.try_merge( imp2 ) {


                    // If they merge store the result in target and increment the count
                    Some(merged_term) => {
                        target.push(merged_term);
                        merge_count += 1;
                    }

                    // If the don't merge then do nothing
                    None  => ()
                }
            }
        }
    }

    // Send an exit signal if there were no merges
    if merge_count == 0 {
        return None
    }
    // Return the merge count otherwise
    return Some(merge_count)
}

fn greedy_algorithm( minterms: &Vec<Implicant>, prime_implicants: &Vec<Implicant> ) -> Vec<Implicant> {
    let mut required_implicants : Vec<Implicant> = Vec::new();
    let mut working_minterms = minterms.clone();
    loop {
        let mut greatest_cover = 0;
        let mut index_of_best_fit: usize = 0;
        for i in 0..prime_implicants.len() {
            let  cover_count = find_cover_count( &prime_implicants[i], &working_minterms );
            if cover_count > greatest_cover {
                greatest_cover = cover_count;
                index_of_best_fit = i;
                working_minterms = remove_coverage( &working_minterms, &prime_implicants[i] );
            }
        }
        if greatest_cover == 0 {
            break;
        }
        required_implicants.push( prime_implicants[ index_of_best_fit ].clone() )
    }
    required_implicants
}

fn find_cover_count( implicant: &Implicant, minterms: &Vec<Implicant> ) -> i32 {
    let mut count = 0;
    for minterm in minterms {
        if implicant.covers(minterm) {
            count += 1;
        }
    }
    count
}

fn remove_coverage( current_minterms: &Vec<Implicant>, implicant: &Implicant ) -> Vec<Implicant> {
    let copy = current_minterms.clone();
    copy.into_iter().filter( |x| !( implicant.covers(x) ) ).collect()
}



fn main() {
    // We make a vector of a vector of implicants, main[size] has implicants of size 2^size
    let mut main : Vec<Vec<Implicant>> = Vec::new();

    // We load n from stdin
    let n : usize = ttyinput::load_n_from_stdin().try_into().unwrap();

    // The max size an implicant can have is 2^n, so we have to initialize main[0] to main [n] which
    // is n + 1 elements
    initialize(&mut main,n);

    // We load the size 1 implicants with the provided minterms
    let minterms = ttyinput::load_minterms_from_stdin( n );
    main[0] = minterms.clone();

    for k in 0..n {
        // First we split at mutable to make sure we can have two different mutable references to
        // main[k] and main [k+1] simultanouesly

        if let Ok([source, target]) = main.get_disjoint_mut([k,k+1]) {
            match iterate_algo(source, target) {
                Some(count) => {
                    // println!("Merged {n} terms of size {size}", n = count, size = 1 << k) 
                },
                None => { 
                    // println!("No further merges of size {size}, quitting...", size = 1 << k );
                    break;
                }
            }
        }
    }
    // Now we have the prime implicants, we can arrange them greedily since the actual problem is
    // NP complete


    // Flatten main and then filter to get only the prime implicants 
    let prime_implicants = main.into_iter().flatten().filter( |x| x.is_unmerged() ).collect();



    let required_implicants: Vec<Implicant> = greedy_algorithm( &minterms, &prime_implicants );

    ttyinput::display_implicants_as_SOP( &required_implicants );
}

