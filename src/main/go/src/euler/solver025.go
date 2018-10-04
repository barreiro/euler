// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import "math"

// The Fibonacci sequence is defined by the recurrence relation: Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
// Hence the first 12 terms will be:
// F1 = 1
// F2 = 1
// F3 = 2
// F4 = 3
// F5 = 5
// F6 = 8
// F7 = 13
// F8 = 21
// F9 = 34
// F10 = 55
// F11 = 89
// F12 = 144
// The 12th term, F12, is the first term to contain three digits.
// What is the first term in the Fibonacci sequence to contain 1000 digits?

func Solver025() int {
	return solver025(1000)
}

func solver025(N int) int {
	// Using the logarithm (base 10) of Binet's Formula (approximation)
	return int(math.Ceil((float64(N-1) + math.Log10(math.Sqrt(5))) / math.Log10(math.Phi)))
}
