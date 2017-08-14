// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package algoritm

import (
	"testing"
)

func test(t *testing.T, actual int, expected int) {
	if expected != actual {
		t.Fatal(t.Name(), "Expected:", expected, "Actual:", actual)
	}
	t.Log(t.Name(), "Expected:", expected)
}

func testMap(t *testing.T, actual map[int]int, expected map[int]int) {
	if len(expected) != len(actual) {
		t.Fatal(t.Name(), "Expected elements:", len(expected), "Actual elements:", len(actual))
	}
	for k, v := range expected {
		if v != actual[k] {
			t.Fatal(t.Name(), "Key:", k, "Expected:", v, "Actual:", actual[v])
		}
	}
	t.Log(t.Name(), "Expected:", expected)
}

// --- long.go

func TestIntSqrt(t *testing.T) {
	test(t, IntSqrt(4), 2)
	test(t, IntSqrt(10000), 100)
	test(t, IntSqrt(1787568), 1337)
}

// --- primes.go

func TestPrimeFactors(t *testing.T) {
	testMap(t, PrimeFactors(4), map[int]int{2: 2})
	testMap(t, PrimeFactors(21), map[int]int{3: 1, 7: 1})
	testMap(t, PrimeFactors(23), map[int]int{23: 1})
	testMap(t, PrimeFactors(840), map[int]int{2: 3, 3: 1, 5: 1, 7: 1})
	testMap(t, PrimeFactors(1031), map[int]int{1031: 1})
}
