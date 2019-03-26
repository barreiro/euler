// COPYRIGHT (C) 2014 barreiro. All Rights Reserved.
// GoLang helper for Project Euler problems
package algorithm

import "math"

func HasFactorBelow(value int, roof int) bool {
	for l, floor := roof, IntSqrt(value); l > floor; l-- {
		if value%l == 0 && value/l < roof {
			return true
		}
	}
	return false
}

func NumberOfFactors(value int) int {
	factors, ceiling, perfect, small := 0, IntSqrt(value), Square(IntSqrt(value)) == value, value <= math.MaxInt32
	for i := ceiling; i > 0; i-- {
		if small {
			if int32(value) % int32(i) == 0 {
				factors ++
			}
		} else if value%i == 0 {
			factors ++
		}
	}

	// need to adjust the number of divisors if the number is a perfect square
	if perfect {
		return 2*factors - 1
	} else {
		return 2 * factors
	}
}

// defined according to problem 21: numbers less than n which divide evenly into n
func SumOfFactors(value int) int {
	sum, ceiling, perfect, small := 1, IntSqrt(value), Square(IntSqrt(value)) == value, value <= math.MaxInt32
	for i := ceiling; i > 1; i-- {
		if small {
			// optimization for small values
			if int32(value)%int32(i) == 0 {
				sum += i + value/i
			}
		} else  if value%i == 0 {
			sum += i + value/i
		}
	}

	// need to adjust the number of divisors if the number is a perfect square
	if perfect {
		return sum - ceiling
	} else {
		return sum
	}
}

func IsAbundant(value int) bool {
	return value < SumOfFactors(value)
}
