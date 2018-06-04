// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import (
	"euler/algorithm"
)

// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
// What is the sum of the digits of the number 2^1000?

func Solver016() int {
	return solver016(1000)
}

func solver016(N int) int {
	// each element is a digit. Each iteration we double every digit and adjust
	// N/3 is an estimate on the number of digits for each power
	values, radix, sum := make([]int, algorithm.Max(N/3+1, 1)), algorithm.DEFAULTRADIX, 0
	values[0] = 1
	for i := 0; i < N; i++ {
		// since carry never ripples we can iterate backwards, using less memory
		for j := i / 3; j >= 0; j-- {
			if values[j] *= 2; values[j] >= radix {
				// with radix > 2 can use increment and subtraction instead of divide and take the remainder
				values[j+1]++
				values[j] -= radix
			}
		}
	}
	for _, digit := range values {
		sum += digit
	}
	return sum
}
