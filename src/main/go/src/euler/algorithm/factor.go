// COPYRIGHT (C) 2014 barreiro. All Rights Reserved.
// GoLang helper for Project Euler problems
package algorithm

func HasFactorBelow(value int, roof int) bool {
	for l, floor := roof, IntSqrt(value); l > floor; l-- {
		if value%l == 0 && value/l < roof {
			return true
		}
	}
	return false
}

func NumberOfFactors(value int) int {
	factors, ceiling, perfect := 0, IntSqrt(value), Square(IntSqrt(value)) == value
	for i := ceiling; i > 0; i-- {
		if value%i == 0 {
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

func SumOfFactors(value int) int {
	sum, ceiling, perfect := 1, IntSqrt(value), Square(IntSqrt(value)) == value
	for i := ceiling; i > 1; i-- {
		if value%i == 0 {
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
