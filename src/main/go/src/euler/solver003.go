// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import (
	"euler/algoritm"
	"math"
)

// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

func Solver003() int {
	return solver003(600851475143)
}

func solver003(N int) int {
	return max(algoritm.PrimeFactors(N))
}

// biggest value present on a map
func max(factors map[int]int) int {
	max := math.MinInt64
	for f := range factors {
		if f > max {
			max = f
		}
	}
	return max
}
