// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import (
	"euler/algorithm"
)

// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which, a^2 + b^2 = c^2
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
//
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

func Solver009() int {
	return solver009(1000)
}

func solver009(N int) int {
	// solved with Euclides Formula --- a=m^2-n^2 --- b=2nm --- c=m^2+n^2 --- with m>n
	for m := 2; m < algorithm.IntSqrt(N); m++ {
		for n := 1; n < m; n++ {
			if a, b, c := m*m-n*n, 2*m*n, m*m+n*n; a+b+c == N {
				return a * b * c
			}
		}
	}
	return 0
}
