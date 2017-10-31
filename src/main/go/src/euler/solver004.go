// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import "euler/algorithm"

// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

func Solver004() int {
	return solver004(3)
}

func solver004(N int) int {
	for l := algorithm.Pow10(N) * algorithm.Pow10(N); ; l-- {
		if algorithm.IsPalindrome(l) && algorithm.HasFactorBelow(l, algorithm.Pow10(N)) {
			return l
		}
	}
}
