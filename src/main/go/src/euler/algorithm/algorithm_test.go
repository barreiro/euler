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

func assertSlice(t *testing.T, actual []int, expected []int) {
	if len(expected) != len(actual) {
		t.Fatal(t.Name(), "Expected elements:", len(expected), "Actual elements:", len(actual))
	}
	for i := range expected {
		if expected[i] != actual[i] {
			t.Fatal(t.Name(), "Index:", i, "Expected:", expected[i], "Actual:", actual[i])
		}
	}
	t.Log(t.Name(), "Expected:", expected)
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

func TestModularExponentiation(t *testing.T) {
	assertInt(t, 445, PowerMod(4, 13, 497))
}

func TestModularExponentiationSmall(t *testing.T) {
	assertInt(t, 1, PowerMod(3, 0, 7));
	assertInt(t, 3, PowerMod(3, 1, 7));
	assertInt(t, 2, PowerMod(3, 2, 7));
	assertInt(t, 6, PowerMod(3, 3, 7));
	assertInt(t, 4, PowerMod(3, 4, 7));
	assertInt(t, 5, PowerMod(3, 5, 7));
	assertInt(t, 1, PowerMod(3, 6, 7));
}

// --- prime.go

func TestPrimeFactors(t *testing.T) {
	assertMap(t, PrimeFactors(4), map[int]int{2: 2})
	assertMap(t, PrimeFactors(21), map[int]int{3: 1, 7: 1})
	assertMap(t, PrimeFactors(23), map[int]int{23: 1})
	assertMap(t, PrimeFactors(840), map[int]int{2: 3, 3: 1, 5: 1, 7: 1})
	assertMap(t, PrimeFactors(1031), map[int]int{1031: 1})
}

func TestPrimeGeneratorReverse(t *testing.T) {
	actual, generator := make([]int, 0, 15), PrimesLessThan(36)
	for p := generator(); p > 1; p = generator() {
		actual = append(actual, p)
	}
	assertSlice(t, actual, []int{31, 29, 23, 19, 17, 13, 11, 7, 5, 3, 2})
}

func TestMillerRabin(t *testing.T) {
	assertTrue(t, millerRabin(2) && millerRabin(3) && millerRabin(5) && millerRabin(7) && millerRabin(11) && millerRabin(13))
	assertFalse(t, millerRabin(4) || millerRabin(6) || millerRabin(8) || millerRabin(9) || millerRabin(10) || millerRabin(12))
}

func TestMillerRabinCarmichael(t *testing.T) {
	assertFalse(t, millerRabin(561) || millerRabin(1105) || millerRabin(1729) || millerRabin(2465) || millerRabin(2821) || millerRabin(6601))
	assertFalse(t, millerRabin(101101) || millerRabin(252601) || millerRabin(314821) || millerRabin(340561) || millerRabin(410041) || millerRabin(512461))
}

func TestMillerRabinLong(t *testing.T) {
	assertFalse(t, millerRabin(154639673381) || millerRabin(585226005592931977) || millerRabin(7999252175582851) || millerRabin(55245642489451))
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
	assertInt(t, SumOfFactors(4), 3)
	assertInt(t, SumOfFactors(21), 11)
	assertInt(t, SumOfFactors(23), 1)
	assertInt(t, SumOfFactors(840), 2040)
	assertInt(t, SumOfFactors(1031), 1)
}
