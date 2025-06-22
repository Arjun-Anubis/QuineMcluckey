mod ttyinput;
mod structs;

use structs::Implicant;

fn initialize(main: &mut Vec<Vec<Implicant>>, n: usize ) {
    let mut k = 0;
    while k <= n {
        main.push( Vec::new() );
        k+=1;
    }
}

fn iterate_algo( source: &mut Vec<Implicant> , target: &mut Vec<Implicant> ) -> Option<usize> {
    // Record the number of merges
    let mut merge_count = 0;
    
    // Iterate pairs (i,j) in the source in a triangular fashion
    for i in 0.. source.len() {
        for j in i+1..source.len() {

            // Try to merge the terms at source(i,j)
            match source[i].try_merge( &mut source[j] ) {


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

    // Send an exit signal if there were no merges
    if merge_count == 0 {
        return None
    }
    // Return the merge count otherwise
    return Some(merge_count)
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
    main[0] = ttyinput::load_minterms_from_stdin( n );

    for k in 0..n {
        // First we split at mutable to make sure we can have two different mutable references to
        // main[k] and main [k+1] simultanouesly
        let (left, right) = main.split_at_mut(k+1);

        match iterate_algo(&mut left[k], &mut right[0]) {
            Some(count) => { println!("Merged {n} terms of size {size}", n = count, size = 1 << k) },
            None => { 
                println!("No further merges of size {size}, quitting...", size = 1 << k );
                break;
            }
        }
    }
}

