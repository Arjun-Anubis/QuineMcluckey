// An implicant contains a string and an unmerged flag
#[derive(Clone)]
#[derive(Debug)]
pub struct Implicant {
    pub representation: Vec<char>,
    pub unmerged: bool
}

impl Implicant {

    fn merge( self: &mut Implicant, other: &mut Implicant, location: usize ) ->  Implicant {
        // Mark both implicants as merged, thats why we need the mutable reference
        self.unmerged = false;
        other.unmerged = false;

        // Create a clone of either of the implicants
        let mut rep: Vec<char> = self.representation.clone();

        // Replace the location with a -
        rep[location] = '-';

        // Create an implicant and return it
        Implicant { representation: rep, unmerged : true }
        
    }

    fn check_merge( self: &Implicant, other: &Implicant ) -> Option<usize> {
        // Only succeeds if there is a difference
        let mut found_difference = false;

        // Let the location be zero by defualt
        let mut location = 0;
         
        // Ensure that both implicant have the same length
        assert!( self.representation.len() == other.representation.len() );

        // Let the length be length
        let length = self.representation.len();

        // Iterate over the representation
        for i in 0..length {

            // If both aren't equal
            if self.representation[i] != other.representation[i] {
                if self.representation[i] == '-' || other.representation[i] == '-' {
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
    pub fn try_merge( self: &mut Implicant, other: &mut Implicant ) -> Option<Implicant> {
        match self.check_merge(&other)
        {
            Some(location) => {
                return Some( self.merge( other, location ) )
            },
            None => { return None }
        }
    }

    pub fn is_unmerged( self: &Implicant ) -> bool {
        self.unmerged
    }
    
    pub fn covers( self: &Implicant, other: &Implicant ) -> bool {
        for i in 0..self.representation.len() {
            if (self.representation[i] != other.representation[i]) && (self.representation[i] != '-') {
                return false
            }
        }
        true
    }


}
