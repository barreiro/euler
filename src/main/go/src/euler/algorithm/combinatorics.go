// COPYRIGHT (C) 2014 barreiro. All Rights Reserved.
// GoLang helper for Project Euler problems
package algorithm

// Method for calculation the combinations of a certain number of elements in a total number of places.
// Uses recursion instead of the formula with factorials.
func Choose(total, elements int) int {
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

func choose(total, elements int, cache [][]int) int {
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

// --- //

// Calculates the number of integer partition of a value, given a set of constrains.
func Partition(value int, constraints map[int]bool) int {
	cache := make([][]int, value+1, value+1)
	for i:=0;i<=value; i++ {
		cache[i] = make([]int, value+1, value+1)
	}
	return partition(value, value, 0, constraints, cache )
}

func partition(remaining, total, sum int, constrains map[int]bool, cache [][]int) int {
	if remaining == 0 {
		return 1
	}
	if cache[remaining][total] != 0 {
		return cache[remaining][total]
	}
	l := 0
	for c := Min( remaining, total ); c > 0; c-- {
		if constrains[c] {
			l += partition( remaining - c, c, sum + c, constrains, cache )
		}
	}
	cache[remaining][total] = l
	return l
}
