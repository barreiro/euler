// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import (
	"euler/algorithm"
	"euler/util"
)

// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.
// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
// Evaluate the sum of all the amicable numbers under 10000.

func Solver021() int {
	return solver021(10000)
}

func solver021(N int) int {
	factorSum, amicableSum := make([]int, N), 0
	util.ParallelFill(factorSum, algorithm.SumOfFactors)
	for i, sum := range factorSum {
		if sum > 1 && sum < N && sum != i && factorSum[sum] == i {
			amicableSum += sum
		}
	}
	return amicableSum
}
