// COPYRIGHT (C) 2014 barreiro. All Rights Reserved.
// GoLang helper for Project Euler problems
package algoritm

// Calculates an approximate of the square root
func IntSqrt(value int) int {
	result := 0
	one := 1 << 30

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
	}
	return result
}
