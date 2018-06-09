// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import "euler/algorithm"

var input001 = []int{3, 5}

// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

func Solver001() int {
	return solver001(1000)
}

func solver001(N int) int {
	sum := 0
	for i, f := range input001 {
		sum += contribution(f, N-1)

		// remove the numbers that are appear twice as they are multiple of more than one factor
		for j, other := range input001 {
			if j < i {
				sum -= contribution(f*other, N-1)
			}
		}
	}
	return sum
}

func contribution(factor int, ceiling int) int {
	// the sum is the factor multiplied by the sum of the number of occurrences
	return factor * algorithm.ArithmeticSum(ceiling/factor)
}
