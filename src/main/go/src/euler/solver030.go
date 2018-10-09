// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import "euler/algorithm"

//Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:
//1634 = 1^4 + 6^4 + 3^4 + 4^4
//8208 = 8^4 + 2^4 + 0^4 + 8^4
//9474 = 9^4 + 4^4 + 7^4 + 4^4
//As 1 = 14 is not a sum it is not included.
//
//The sum of these numbers is 1634 + 8208 + 9474 = 19316.
//Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.

func Solver030() int {
	return solver030(5)
}

func solver030(N int) int {
	sum, lower, upper := 0, algorithm.Pow(9, N/2), N * algorithm.Pow(9, N)
	for n := upper; n > lower; n -- {
		digitSum := 0
		for _, digit := range algorithm.ToDigits(n) {
			digitSum += algorithm.Pow(digit, N)
		}
		if digitSum == n {
			sum += n
		}
	}
	return sum
}
