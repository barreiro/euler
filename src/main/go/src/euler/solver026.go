// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import (
	"euler/algorithm"
)

// A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:
// 1/2  =  0.5
// 1/3  =  0.(3)
// 1/4  =  0.25
// 1/5  =  0.2
// 1/6  =  0.1(6)
// 1/7  =  0.(142857)
// 1/8  =  0.125
// 1/9  =  0.(1)
// 1/10 =  0.1
// Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.
// Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.

func Solver026() int {
	return solver026(1000)
}

func solver026(N int) int {
	// For primes: if 10 is a primitive root modulo p, the recurring cycle is equal to p − 1; if not is a factor of p − 1
	for generator := algorithm.PrimesLessThan(N); ; {
		if p := generator(); isPrimeRootTen(p) {
			return p
		}
	}
}

func isPrimeRootTen(p int) bool {
	factors := algorithm.PrimeFactors(p - 1)
	for f := range factors {
		if algorithm.PowerMod(10, (p-1)/f, p) == 1 {
			return false
		}
	}
	return true
}
