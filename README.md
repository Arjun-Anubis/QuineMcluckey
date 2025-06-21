# Introduction
The Quine Mucluckey algorithm or the tabular method is an algorithm to find the minimal sum of products or product of sum representation for a given truth table in many variables
This can be used in place of a karnaugh map when the number of inputs is very large, it also ensures that the minimal form is reached. It follows essentially the same principle as a kmap to find the prime implicants of a truth table


# Principle
The input to the algorithm is (a) the number of inputs _n_, and (b) the list of minterms given as numbers in radix 2 (so the ordering of the variables is fixed as according to the base the minterms are given in).

The output is the minimal sum of products representation or the minimal product of sums presentation as required given in the form of k (which is as of yet undetermined and depends on the minterms themselves) strings of length n where every character is +, - or . as according the variable in that place value is present as itself, in its complement or not present 

# Algorithm

The algorithm is divided into two parts (a) finding the prime implicants and (b) using the prime implicants to find a sum of products representation

For (a) there is a an outer loop that runs r <= _n_ times so as to generate implicants of sizes from
$$
2^0 - 2^{n}
$$
Let k run from 1 to n, in every iteration we combine implicants of size k to form implicants of size k+1, thus we loop over every pair of implicants (we need only loop over (a,b) and (b,a) need not be covered, for every pair we first compare their masks then we check if they have a 1 bit difference when masked, if that is the case we delete both implicants and make a new implicant of size k + 1 with one more - in the location of the one bit difference all of which can be done using bit operations

