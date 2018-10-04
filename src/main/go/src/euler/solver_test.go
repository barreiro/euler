// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import (
	"strings"
	"testing"
)

func assert(t *testing.T, actual int, expected int) {
	if expected != actual {
		t.Fatal(t.Name(), "Expected:", expected, "Actual:", actual)
	}
	t.Log(t.Name(), "Expected:", expected)
}

func assertTrue(t *testing.T, expected bool) {
	if !expected {
		t.Fatal(t.Name(), "Expected:", !expected, "Actual:", expected)
	}
	t.Log(t.Name(), "Expected:", expected)
}

func assertFalse(t *testing.T, expected bool) {
	if expected {
		t.Fatal(t.Name(), "Expected:", !expected, "Actual:", expected)
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

	assert(t, solver009(10), 0)
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
	assert(t, solver012(10), 120)
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

func TestSolver014(t *testing.T) {
	assert(t, Solver014(), 837799)

	assert(t, solver014(5), 3)
	assert(t, solver014(100), 97)
	assert(t, solver014(5000000), 3732423)
}

func TestSolver015(t *testing.T) {
	assert(t, Solver015(), 137846528820)

	assert(t, solver015(1), 2)
	assert(t, solver015(2), 6)
	assert(t, solver015(3), 20)
	assert(t, solver015(5), 252)
	assert(t, solver015(10), 184756)
	assert(t, solver015(30), 118264581564861424)
}

func TestSolver016(t *testing.T) {
	assert(t, Solver016(), 1366)

	assert(t, solver016(0), 1)
	assert(t, solver016(1), 2)
	assert(t, solver016(4), 7)
	assert(t, solver016(8), 13)
	assert(t, solver016(10), 7)
	assert(t, solver016(33), 62)
	assert(t, solver016(65), 86)
	assert(t, solver016(100), 115)
	assert(t, solver016(200), 256)
	assert(t, solver016(500), 679)
	assert(t, solver016(10000), 13561)
}

func TestSolver017(t *testing.T) {
	assert(t, Solver017(), 21124)

	assert(t, solver017(5), 19)
	assert(t, solver017(19), 106)
	assert(t, solver017(19999), 737203)

	// for a more comprehensive set of tests on the algorithm, check the java version
}

func TestSolver018(t *testing.T) {
	assert(t, Solver018(), 1074)

	assert(t, solver018(1), 75)
	assert(t, solver018(2), 170)
	assert(t, solver018(10), 696)
}

func TestSolver019(t *testing.T) {
	assert(t, Solver019(), 171)

	assert(t, solver019(1), 2)
	assert(t, solver019(2), 3)
	assert(t, solver019(3), 6)
	assert(t, solver019(4), 7)
	assert(t, solver019(10), 17)
	assert(t, solver019(10000), 17200)

	assertFalse(t, isLeap(2001))
	assertTrue(t, isLeap(2012))
	assertTrue(t, isLeap(2000))
	assertFalse(t, isLeap(1900))
}

func TestSolver020(t *testing.T) {
	assert(t, Solver020(), 648)

	assert(t, solver020(1), 1)
	assert(t, solver020(2), 2)
	assert(t, solver020(3), 6)
	assert(t, solver020(4), 6)
	assert(t, solver020(10), 27)
	assert(t, solver020(1000), 10539)
}

func TestSolver021(t *testing.T) {
	assert(t, Solver021(), 31626)

	assert(t, solver021(300), 504)
	assert(t, solver021(200000), 2896242)
}

func TestSolver022(t *testing.T) {
	assert(t, Solver022(), 871198282)

	assert(t, solver022(5), 496)
	assert(t, solver022(938), 26819198)

	assert(t, solver022reader(1, strings.NewReader("COLIN")), 53)
	assert(t, solver022reader(1, strings.NewReader("LUIS")), 61)
	assert(t, solver022reader(1, strings.NewReader("BARREIRO")), 86)
	assert(t, solver022reader(2, strings.NewReader("\"LUIS\",\"BARREIRO\"")), 208)
}

func TestSolver023(t *testing.T) {
	assert(t, Solver023(), 4179871)

	assert(t, solver023(23), 276)
	assert(t, solver023(24), 276)
	assert(t, solver023(25), 301)
	assert(t, solver023(50000), 4179871)
}
