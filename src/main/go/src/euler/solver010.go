// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import (
	"euler/algorithm"
)

// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
// Find the sum of all the primes below two million.

func Solver010() int {
	return solver010(2000000)
}

func solver010(N int) int {
	sum, generator := 0, algorithm.GeneratorTrialDivision()
	for prime := 0; prime < N; prime = generator() {
		sum += prime
	}
	return sum
}
