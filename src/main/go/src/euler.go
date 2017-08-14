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

// ---

func main() {
	singleProblem(001, euler.Solver001())
	singleProblem(002, euler.Solver002())
	singleProblem(003, euler.Solver003())
}
