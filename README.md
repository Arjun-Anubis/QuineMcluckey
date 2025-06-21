# Introduction
The Quine Mucluckey algorithm or the tabular method is an algorithm to find the minimal sum of products or product of sum representation for a given truth table in many variables
This can be used in place of a karnaugh map when the number of inputs is very large, it also ensures that the minimal form is reached. It follows essentially the same principle as a kmap to find the prime implicants of a truth table


# Working
The input to the algorithm is (a) the number of inputs _n_, and (b) the list of minterms given as numbers in radix 2 (so the ordering of the variables is fixed as according to the base the minterms are given in).

The output is the minimal sum of products representation or the minimal product of sums presentation as required given in the form of k (which is as of yet undetermined and depends on the minterms themselves) strings of length n where every character is +, - or . as according the variable in that place value is present as itself, in its complement or not present 
