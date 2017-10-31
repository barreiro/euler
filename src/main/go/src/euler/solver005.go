// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import "euler/algorithm"

// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

func Solver005() int {
	return solver005(20)
}

func solver005(N int) int {
	factorsOfSmallest := make(map[int]int)

	for l := 2; l < N; l++ {
		for factor, count := range algorithm.PrimeFactors(l) {
			factorsOfSmallest[factor] = algorithm.Max(factorsOfSmallest[factor], count)
		}
	}

	sum := 1
	for base, exp := range factorsOfSmallest {
		sum *= algorithm.Pow(base, exp)
	}
	return sum
}
