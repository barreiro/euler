// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

// The following iterative sequence is defined for the set of positive integers: n → n/2 (n is even) n → 3n + 1 (n is odd)
//
// Using the rule above and starting with 13, we generate the following sequence: 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
// Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
//
// Which starting number, under one million, produces the longest chain?
//
// NOTE: Once the chain starts the terms are allowed to go above one million.

func Solver014() int {
	return solver014(1000000)
}

func solver014(N int) int {
	cache, max, i := make([]int, N), 1, 2
	for cache[1] = 1; i < N; i++ {
		if collatzLength(i, cache) > cache[max] {
			max = i
		}
	}
	return max
}

func collatzLength(i int, cache []int) int {
	// Can't rely on the cache for everything but in many cases we can cut lots of recursion
	if i < len(cache) {
		if cache[i] == 0 {
			cache[i] = rawCollatzLength(i, cache)
		}
		return cache[i]
	}
	return rawCollatzLength(i, cache)
}

func rawCollatzLength(i int, cache []int) int {
	if i%2 == 0 {
		return collatzLength(i/2, cache) + 1
	} else {
		return collatzLength(3*i+1, cache) + 1
	}
}
