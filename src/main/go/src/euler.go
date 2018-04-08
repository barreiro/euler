// COPYRIGHT (C) 2014 barreiro. All Rights Reserved.
// Project Euler solvers implemented in GoLang
package main

import (
	"euler"
	"fmt"
	"os"
	"runtime"
	"time"
)

func singleProblem(n int, solver func() int) {
	runtime.GC()
	time.Sleep(100)

	start := time.Now()
	result := solver()
	elapsed := float64(time.Since(start))

	fmt.Printf("Solution for problem %03d is %12d ( took %9.3f ms )\n", n, result, elapsed / 1000000)
	os.Stdout.Sync()
}

// --- //

func main() {
	singleProblem(1, euler.Solver001)
	singleProblem(2, euler.Solver002)
	singleProblem(3, euler.Solver003)
	singleProblem(4, euler.Solver004)
	singleProblem(5, euler.Solver005)
	singleProblem(6, euler.Solver006)
	singleProblem(7, euler.Solver007)
	singleProblem(8, euler.Solver008)
	singleProblem(9, euler.Solver009)
	singleProblem(10, euler.Solver010)
	singleProblem(11, euler.Solver011)
	singleProblem(12, euler.Solver012)
	singleProblem(13, euler.Solver013)
	singleProblem(14, euler.Solver014)
	singleProblem(15, euler.Solver015)
	singleProblem(16, euler.Solver016)
	singleProblem(17, euler.Solver016)
}
