// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import "euler/algorithm"

// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10001st prime number?

func Solver007() int {
	return solver007(10001)
}

func solver007(N int) int {
	prime, generator := 0, algorithm.GeneratorTrialDivision()
	for i:= 1; i <= N; i++ {
		prime = generator()
	}
	return prime
}
