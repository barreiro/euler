// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import (
	"euler/algorithm"
)

// n! means n × (n − 1) × ... × 3 × 2 × 1
// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800, and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
// Find the sum of the digits in the number 100!

func Solver020() int {
	return solver020(100)
}

func solver020(N int) int {
	factorial, sum, ceiling := algorithm.ToDigits(1), 0, algorithm.Pow10(15)
	for n, carry := 1, 0; n <= N; n++ {
		for i, old := range factorial {
			value := old*n + carry

			// Adjust the buckets that grow beyond the ceiling value, carrying to next bucket
			if value > ceiling {
				carry, factorial[i] = value/ceiling, value%ceiling
			} else {
				carry, factorial[i] = 0, value
			}
		}
		if carry != 0 {
			// with a small ceiling values would probably need to split the carry into buckets
			factorial, carry = append(factorial, carry), 0
		}
	}
	for _, n := range factorial {
		sum += algorithm.ArraySum(algorithm.ToDigits(n))
	}
	return sum
}
