// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import "testing"

func assert(t *testing.T, actual int, expected int) {
	if expected != actual {
		t.Fatal(t.Name(), "Expected:", expected, "Actual:", actual)
	}
	t.Log(t.Name(), "Expected:", expected)
}

// ---

func TestSolver001(t *testing.T) {
	assert(t, Solver001(), 233168)

	assert(t, solver001(6), 8)
	assert(t, solver001(7), 14)
	assert(t, solver001(10), 23)
	assert(t, solver001(30), 195)
	assert(t, solver001(1000000000), 233333333166666668)
}

func TestSolver002(t *testing.T) {
	assert(t, Solver002(), 4613732)

	assert(t, solver002(100), 44)
	assert(t, solver002(1000000000000), 478361013020)
}

func TestSolver003(t *testing.T) {
	assert(t, Solver003(), 6857)

	assert(t, solver003(12), 3)
	assert(t, solver003(13195), 29)
	assert(t, solver003(1000000000031), 85302397)
}

func TestSolver004(t *testing.T) {
	assert(t, Solver004(), 906609)

	assert(t, solver004(1), 9)
	assert(t, solver004(2), 9009)
	assert(t, solver004(4), 99000099)
}