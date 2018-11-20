// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import (
	"euler/algorithm"
)

// Consider all integer combinations of ab for 2 ≤ a ≤ 5 and 2 ≤ b ≤ 5:
// 2^2=4,  2^3=8,   2^4=16,  2^5=32
// 3^2=9,  3^3=27,  3^4=81,  3^5=243
// 4^2=16, 4^3=64,  4^4=256, 4^5=1024
// 5^2=25, 5^3=125, 5^4=625, 5^5=3125
//
// If they are then placed in numerical order, with any repeats removed, we get the following sequence of 15 distinct terms: 4, 8, 9, 16, 25, 27, 32, 64, 81, 125, 243, 256, 625, 1024, 3125
// How many distinct terms are in the sequence generated by a^b for 2 ≤ a ≤ 100 and 2 ≤ b ≤ 100?

func Solver029() int {
	return solver029(100)
}

func solver029(N int) int {
	// a given number 2≤a≤N can only produce duplicates if 'a^n' can be expressed as '(b^i)^j' with n=i∗j and 2≤n,i,j≤N

	duplicates, unique := 0, make(map[int]bool)
	for b := algorithm.IntSqrt(N); b >= 2; b -- {
		for i := algorithm.IntSqrt(N); i >= 2; i-- {
			if a := algorithm.Pow(b, i); a <= N && unique[a] == false {
				duplicates, unique[a] = duplicates+(N/i)-1, true

				// the trivial duplicates that have i*j<=N are accounted. there may be an equivalent of the factorization that still satisfies the relation
				for j := N; j > N/i; j-- {
					factoredBase, factoredExp := factoredPower(a, j)
					for base, exp, k := factoredBase, factoredExp, 2; base < a; base, exp, k = base*factoredBase, factoredExp/k, k+1 {
						if base < a && exp <= N && factoredExp%exp == 0 {
							duplicates ++
							break
						}
					}
				}
			}
		}
	}
	return algorithm.Pow(N-1, 2) - duplicates
}

func factoredPower(base, power int) (int, int) {
	factoredBase, factoredExp, factors := 1, 0, algorithm.PrimeFactors(base)
	for k, v := range factors {
		factoredBase, factoredExp = factoredBase*k, factoredExp+v
	}
	return factoredBase, factoredExp * power / len(factors)
}
