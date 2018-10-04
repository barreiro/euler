// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

// Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:
// 21 22 23 24 25
// 20  7  8  9 10
// 19  6  1  2 11
// 18  5  4  3 12
// 17 16 15 14 13
// It can be verified that the sum of the numbers on the diagonals is 101.
// What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?

func Solver028() int {
	return solver028(1001)
}

func solver028(N int) int {
	sum := 1
	for i := 3; i <= N; i += 2 {
		// sum of the left corners == right corners == 2*i*i - 3*(i-1)
		sum += 4*i*i - 6*(i-1)
	}
	return sum
}
