// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

var factors = []int{3, 5}

// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

func Solver001() int {
	return solver001(1000)
}

func solver001(N int) int {
	sum := 0
	for i := 0; i < len(factors); i++ {
		sum += contribution(N, factors[i])

		// remove the numbers that are appear twice as they are multiple of more than one factor
		for j := i + 1; j < len(factors); j++ {
			sum -= contribution(N, factors[i]*factors[j])
		}
	}
	return sum
}

func contribution(N int, f int) int {
	// count the number of existing factors and then multiply it by the arithmetic progression to find the sum
	count := (N - 1) / f
	return f * count * (count + 1) / 2
}
