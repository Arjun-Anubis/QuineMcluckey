# Introduction
The Quine Mucluckey algorithm or the tabular method is an algorithm to find the minimal sum of products or product of sum representation for a given truth table in many variables
This can be used in place of a karnaugh map when the number of inputs is very large, it also ensures that the minimal form is reached. It follows essentially the same principle as a kmap to find the prime implicants of a truth table


# Principle
The input to the algorithm is (a) the number of inputs _n_, and (b) the list of minterms given as numbers in radix 2 (so the ordering of the variables is fixed as according to the base the minterms are given in). The output is the minimal sum of products required to impliment those minterms

# Algorithm

The algorithm is divided into two parts (a) finding the prime implicants and (b) using the prime implicants to find a sum of products representation

## Finding prime implicants

The principle is that _neighbouring_ implicants of the same size can be _merged_ to form an implicant with twice the size, thus we loop over the size 1 implicants and then merge any neighbouring ones into size 2 implicants, then loop over the size 2 implicants and merge any neighbouring ones into size 4 implicants and so on. The biggest possible implicant is size 2^n so we continue till we reach that size or when there are no more merges possible at the current size.

Two implicants are said to be _neighbouring_, if they share the same mask, i.e. they have dashes in the same locations, and if they differ in only 1 bit.
_Merging_ two implicants means that we take the two implicants and create an implicant of double the size which has a '-' in the bit of difference

At the end of this procdure, the remaining implicants that have not been merged, i.e. are _unmerged_ are called the prime implicants, because they can not be merged further, i.e. there does not exist a larger implicant that covers them.

## Finding a sum of products

The prime implicants themselves represent a product, however, they overcover the required function, hence we must find a minimal set of implicants that covers all the minterms required. We do this using a greedy algorithm, first we look at whicch implicant covers the most minterms, then we pick that implicant and remove all the minterms it covers from the list, next we look at the remaining minterms, and find the implicant that covers the most remaining minterms, then repeat this procdure till all terms are covered. Thuis algorithm guantees a solution, but it may not be the minimal solution.
