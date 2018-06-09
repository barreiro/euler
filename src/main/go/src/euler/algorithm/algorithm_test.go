// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package algorithm

import (
	"testing"
)

func assertInt(t *testing.T, actual int, expected int) {
	if expected != actual {
		t.Fatal(t.Name(), "Expected:", expected, "Actual:", actual)
	}
	t.Log(t.Name(), "Expected:", expected)
}

func assertTrue(t *testing.T, actual bool) {
	if !actual {
		t.Fatal(t.Name(), "Expected:", !actual, "Actual:", actual)
	}
	t.Log(t.Name(), "Expected:", actual)
}

func assertFalse(t *testing.T, actual bool) {
	if actual {
		t.Fatal(t.Name(), "Expected:", !actual, "Actual:", actual)
	}
	t.Log(t.Name(), "Expected:", actual)
}

func assertMap(t *testing.T, actual map[int]int, expected map[int]int) {
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
	assertInt(t, IntSqrt(4), 2)
	assertInt(t, IntSqrt(10000), 100)
	assertInt(t, IntSqrt(1787568), 1337)
}

func TestIsPalindrome(t *testing.T) {
	assertFalse(t, IsPalindrome(15))
	assertTrue(t, IsPalindrome(88))
	assertFalse(t, IsPalindrome(15846))
	assertTrue(t, IsPalindrome(84048))
}

// --- prime.go

func TestPrimeFactors(t *testing.T) {
	assertMap(t, PrimeFactors(4), map[int]int{2: 2})
	assertMap(t, PrimeFactors(21), map[int]int{3: 1, 7: 1})
	assertMap(t, PrimeFactors(23), map[int]int{23: 1})
	assertMap(t, PrimeFactors(840), map[int]int{2: 3, 3: 1, 5: 1, 7: 1})
	assertMap(t, PrimeFactors(1031), map[int]int{1031: 1})
}

// --- factor.go

func TestNumberOfFactors(t *testing.T) {
	assertInt(t, NumberOfFactors(4), 3)
	assertInt(t, NumberOfFactors(21), 4)
	assertInt(t, NumberOfFactors(23), 2)
	assertInt(t, NumberOfFactors(840), 32)
	assertInt(t, NumberOfFactors(1031), 2)
}

func TestSumOfFactors(t *testing.T) {
	assertInt(t, SumOfFactors(4), 7)
	assertInt(t, SumOfFactors(21), 32)
	assertInt(t, SumOfFactors(23), 24)
	assertInt(t, SumOfFactors(840), 2880)
	assertInt(t, SumOfFactors(1031), 1032)
}
