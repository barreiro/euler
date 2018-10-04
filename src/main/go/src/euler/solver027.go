// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import (
	"euler/algorithm"
)

// Euler discovered the remarkable quadratic formula: n^2 + n + 41
// It turns out that the formula will produce 40 primes for the consecutive values n = 0 to 39.
// However, when n = 40, 402 + 40 + 41 = 40(40 + 1) + 41 is divisible by 41, and certainly when n = 41, 41^2 + 41 + 41 is clearly divisible by 41.
// The incredible formula  n^2 − 79n + 1601 was discovered, which produces 80 primes for the consecutive values n = 0 to 79.
// The product of the coefficients, −79 and 1601, is −126479.
//
// Considering quadratics of the form: n^2 + an + b, where |a| < 1000 and |b| < 1000 where |n| is the modulus/absolute value of n e.g. |11| = 11 and |−4| = 4
// Find the product of the coefficients, a and b, for the quadratic expression that produces the maximum number of primes for consecutive values of n, starting with n = 0.

const heegner = -163

func Solver027() int {
	return solver027(1000)
}

func solver027(N int) int {
	// Conjecture: a is odd negative and b is one of the 10% highest primes
	// The discriminant must be an Heegner number, in particular -163
	candidate, best, a := 0, 0, -N
	if N%2 == 0 {
		a += 1
	}

	for ; a < 0; a += 2 {
		generator, bound := algorithm.PrimesLessThan(N), N-N/10
		for b := generator(); b > bound; b = generator() {
			if a*a-4*b == heegner {
				for n := 0; algorithm.MillerRabin(n*n + a*n + b); n++ {
					if n > best {
						best, candidate = n, a*b
					}
				}
			}
		}
	}
	return candidate
}
