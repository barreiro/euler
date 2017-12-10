// COPYRIGHT (C) 2014 barreiro. All Rights Reserved.
// GoLang helper for Project Euler problems
package algorithm

import "math"

// Default value used as base for the numeric system. Used in methods that make array-based calculations. Default to the decimal system.
var DEFAULTRADIX = 10

// Table for fast lookup of powers of 10
var POW10 = []int{
	1,
	10,
	100,
	1000,
	10000,
	100000,
	1000000,
	10000000,
	100000000,
	1000000000,
	10000000000,
	100000000000,
	1000000000000,
	10000000000000,
	100000000000000,
	1000000000000000,
	10000000000000000,
	100000000000000000,
	1000000000000000000,
}

// Calculates an approximate of the square root
func IntSqrt(value int) int {
	result, one := 0, 1<<30

	// "one" starts at the highest power of four <= than the argument
	for one > value {
		one >>= 2
	}

	approx := value
	for ; one != 0; one, result = one>>2, result>>1 {
		if approx >= result+one {
			approx -= result + one
			result = result + (one << 1)
		}
	}

	// Rounding to nearest integer
	if approx > result {
		return result + 1
	} else {
		return result
	}
}

func Pow(base int, exp int) int {
	if base == 0 {
		if exp == 0 {
			return 1
		} else {
			return 0
		}
	}
	if base == 1 {
		return base
	}
	if base == 2 {
		return 1 << uint(exp)
	}
	if base == 10 {
		return Pow10(exp)
	}

	if exp == 0 {
		return 1
	}
	if exp == 1 {
		return base
	}
	if exp == 2 {
		return base * base
	}
	return squaring(base, exp);
}

func squaring(base int, exp int) int {
	result, squaringBase, squaringExp := 1, base, exp
	for ; squaringExp != 0; squaringExp, squaringBase = squaringExp/2, squaringBase*squaringBase {
		if squaringExp%2 != 0 {
			result *= squaringBase
		}
	}
	return result
}

// Convenience method to calculate the power when in base 10.
func Pow10(exp int) int {
	if exp < len(POW10) {
		return POW10[exp]
	} else {
		return -2
	}
}

// ---

func IsPalindrome(value int) bool {
	return isPalindromeDigits(ToDigits(value))
}

func isPalindromeDigits(digits []int) bool {
	for l := 0; l*2 < len(digits); l++ {
		if digits[l] != digits[len(digits)-l-1] {
			return false
		}
	}
	return true
}

// ---

func ToDigits(value int) []int {
	return toDigitsRadix(value, DEFAULTRADIX)
}

func toDigitsRadix(value int, radix int) []int {
	digits := make([]int, 0, 32/IntSqrt(radix))
	for ; value >= radix; value /= radix {
		digits = append(digits, value%radix)
	}
	return append(digits, value)
}

// ---

func Max(a int, b int) int {
	if a >= b {
		return a
	} else {
		return b
	}
}

func MaxArray(a ...int) int {
	max := math.MinInt64
	for _, i := range a {
		if i > max {
			max = i
		}
	}
	return max
}
