// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import "testing"

func test(t *testing.T, actual int, expected int) {
	if expected != actual {
		t.Fatal(t.Name(), "Expected:", expected, "Actual:", actual)
	}
	t.Log(t.Name(), "Expected:", expected)
}

// ---

func TestSolver001(t *testing.T) {
	test(t, Solver001(), 233168)

	test(t, solver001(6), 8)
	test(t, solver001(7), 14)
	test(t, solver001(10), 23)
	test(t, solver001(30), 195)
	test(t, solver001(1000000000), 233333333166666668)
}

func TestSolver002(t *testing.T) {
	test(t, Solver002(), 4613732)

	test(t, solver002(100), 44)
	test(t, solver002(1000000000000), 478361013020)
}

func TestSolver003(t *testing.T) {
	test(t, Solver003(), 6857)

	test(t, solver003(12), 3)
	test(t, solver003(13195), 29)
	test(t, solver003(1000000000031), 85302397)
}
