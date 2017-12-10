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

func TestSolver005(t *testing.T) {
	assert(t, Solver005(), 232792560)

	assert(t, solver005(6), 60)
	assert(t, solver005(10), 2520)
	assert(t, solver005(20), 232792560)
	assert(t, solver005(40), 5342931457063200)
}

func TestSolver006(t *testing.T) {
	assert(t, Solver006(), 25164150)

	assert(t, solver006(5), 170)
	assert(t, solver006(10), 2640)
	assert(t, solver006(200), 401323300)
}

func TestSolver007(t *testing.T) {
	assert(t, Solver007(), 104743)

	assert(t, solver007(1), 2)
	assert(t, solver007(6), 13)
	assert(t, solver007(100000), 1299709)
}

func TestSolver008(t *testing.T) {
	assert(t, Solver008(), 23514624000)

	assert(t, solver008(1), 9)
	assert(t, solver008(4), 5832)
	assert(t, solver008(20), 240789749760000)
}

func TestSolver009(t *testing.T) {
	assert(t, Solver009(), 31875000)

	assert(t, solver009(12), 60)
	assert(t, solver009(20000), 265387500000)
}

func TestSolver010(t *testing.T) {
	assert(t, Solver010(), 142913828922)

	assert(t, solver010(5), 5)
	assert(t, solver010(10), 17)
	assert(t, solver010(1000000), 37550402023)
}

func TestSolver011(t *testing.T) {
	assert(t, Solver011(), 70600674)

	assert(t, solver011(1), 99)
	assert(t, solver011(2), 9603)
	assert(t, solver011(3), 811502)
	assert(t, solver011(5), 3318231678)
	assert(t, solver011(6), 140975907072)
}

func TestSolver012(t *testing.T) {
	assert(t, Solver012(), 76576500)

	assert(t, solver012(4), 6)
	assert(t, solver012(5), 28)
	assert(t, solver012(100), 73920)
}

func TestSolver013(t *testing.T) {
	assert(t, Solver013(), 5537376230)

	assert(t, solver013(1), 5)
	assert(t, solver013(2), 55)
	assert(t, solver013(3), 553)
	assert(t, solver013(4), 5537)
	assert(t, solver013(5), 55373)
	assert(t, solver013(15), 553737623039087)
}
