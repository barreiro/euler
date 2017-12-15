// COPYRIGHT (C) 2014 barreiro. All Rights Reserved.
// GoLang helper for Project Euler problems
package algorithm

// Method for calculation the combinations of a certain number of elements in a total number of places.
// Uses recursion instead of the formula with factorials.
func Choose(total int, elements int) int {
	// Take full advantage of symmetry
	elements = Min(elements, total-elements)
	if elements < 0 {
		return 0
	}
	if elements == 0 {
		return 1
	}
	cache := make([][]int, total+1)
	for i := range cache {
		cache[i] = make([]int, elements+1)
	}
	return choose(total, elements, cache)
}

func choose(total int, elements int, cache [][]int) int {
	elements = Min(elements, total-elements)
	if elements == 1 {
		return total
	}
	if cache[total][elements] != 0 {
		return cache[total][elements]
	}
	value := choose(total-1, elements-1, cache) + choose(total-1, elements, cache)
	cache[total][elements] = value
	return value
}
