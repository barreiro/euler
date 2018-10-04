// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import "euler/algorithm"

//A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4.
//If all of the permutations are listed numerically or alphabetically, we call it lexicographic order.
//The lexicographic permutations of 0, 1 and 2 are:   012   021   102   120   201   210
//What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

var base = []int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}

func Solver024() int {
	return solver024(1000000)
}

func solver024(N int) int {
	return solver024slice(N, append(base[:0:0], base...))
}

func solver024slice(N int, unplaced []int) int {
	value, sum := N-1, 0
	// Use a kind of factorization of N over the factorials of the base, fixing an element of the permutation each round
	for l := len(unplaced) - 1; l > 0; l -- {
		f := algorithm.Factorial(l)
		sum += algorithm.Pow10(l) * unplaced[value/f]
		unplaced, value = append(unplaced[:value/f], unplaced[value/f+1:]...), value%f
	}
	return sum + unplaced[0]
}
