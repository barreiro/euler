// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import "euler/algorithm"

// The sum of the squares of the first ten natural numbers is, 1^2 + 2^2 + ... + 10^2 = 385
// The square of the sum of the first ten natural numbers is, (1 + 2 + ... + 10)^2 = 55^2 = 3025
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
//
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

func Solver006() int {
	return solver006(100)
}

func solver006(N int) int {
	// using Faulhaber's Formula for the square of the sum and Gauss's Formula for the sum of the squares
	return ( algorithm.Pow(N, 4)+2*algorithm.Pow(N, 3)+algorithm.Pow(N, 2) )/4 - N*( N+1 )*( 2*N+1 )/6
}
