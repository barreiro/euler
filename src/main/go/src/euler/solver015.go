// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import "euler/algorithm"

// Starting in the top left corner of a 2x2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.
// How many such routes are there through a 20x20 grid?

func Solver015() int {
	return solver015(20)
}

func solver015(N int) int {
	return algorithm.Choose(2*N, N)
}
