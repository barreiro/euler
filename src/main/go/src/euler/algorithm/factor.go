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
