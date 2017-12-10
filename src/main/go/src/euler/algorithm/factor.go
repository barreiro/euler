// COPYRIGHT (C) 2014 barreiro. All Rights Reserved.
// GoLang helper for Project Euler problems
package algorithm

func HasFactorBelow(value int, roof int) bool {
	for l := roof; l > IntSqrt(value); l-- {
		if value%l == 0 && value/l < roof {
			return true
		}
	}
	return false
}

func NumberOfFactors(value int) int {
	factors, ceiling := 0, IntSqrt(value)
	for i := 1; i <= ceiling; i++ {
		if value%i == 0 {
			factors += 2
		}
	}

	// We need to adjust the number of divisors if the number is a perfect square
	if ceiling*ceiling == value {
		return factors - 1
	} else {
		return factors
	}
}
