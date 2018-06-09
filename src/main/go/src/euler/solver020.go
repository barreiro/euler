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
	for i, carry := 1, 0; i <= N; i++ {
		for n := 0; n < len(factorial); n++ {
			factorial[n] = factorial[n]*i + carry

			// Adjust the buckets that grow beyond the ceiling value, carrying to next bucket
			if carry = 0; factorial[n] > ceiling {
				carry, factorial[n] = factorial[n]/ceiling, factorial[n]%ceiling
			}
		}
		if carry != 0 {
			// with a small ceiling values would probably need to split the carry into buckets
			factorial, carry = append(factorial, carry), 0
		}
	}
	for _, n := range factorial {
		for _, digit := range algorithm.ToDigits(n) {
			sum += digit
		}
	}
	return sum
}
