// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import (
	"euler/algorithm"
	"euler/util"
)

// The sequence of triangle numbers is generated by adding the natural numbers.
// So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28.
// The first ten terms would be: 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
//
// Let us list the factors of the first seven triangle numbers:
// 1: 1
// 3: 1,3
// 6: 1,2,3,6
// 10: 1,2,5,10
// 15: 1,3,5,15
// 21: 1,3,7,21
// 28: 1,2,4,7,14,28
// We can see that 28 is the first triangle number to have over five divisors.
//
// What is the value of the first triangle number to have over five hundred divisors?

func Solver012() int {
	return solver012(500)
}

func solver012(N int) int {
	return util.ParallelUnbound(N-1, algorithm.ArithmeticSum, func(n int) bool { return algorithm.NumberOfFactors(n) >= N })
}
