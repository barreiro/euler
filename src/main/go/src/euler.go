// COPYRIGHT (C) 2014 barreiro. All Rights Reserved.
// Project Euler solvers implemented in GoLang
package main

import (
	"euler"
	"fmt"
	"time"
)

func singleProblem(n int, a interface{}) {
	start := time.Now()
	result := a
	elapsed := time.Since(start)

	fmt.Printf("Solution for problem %03d is %12d ( took %6d ns )\n", n, result, elapsed)
}

// --- //

func main() {
	singleProblem(1, euler.Solver001())
	singleProblem(2, euler.Solver002())
	singleProblem(3, euler.Solver003())
	singleProblem(4, euler.Solver004())
	singleProblem(5, euler.Solver005())
	singleProblem(6, euler.Solver006())
	singleProblem(7, euler.Solver007())
	singleProblem(8, euler.Solver008())
	singleProblem(9, euler.Solver009())
	singleProblem(10, euler.Solver010())
}
