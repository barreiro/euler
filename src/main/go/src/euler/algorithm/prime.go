// COPYRIGHT (C) 2014 barreiro. All Rights Reserved.
// GoLang helper for Project Euler problems
package algorithm

import (
	"math"
	"math/bits"
)

var millerRabinFast, millerRabinBase = []int{2, 7, 61}, []int{2, 325, 9375, 28178, 450775, 9780504, 1795265022}

// calculates the prime factors of a given number. The result is a map where the keys are primes and the values are the occurrences
func PrimeFactors(n int) map[int]int {
	factorMap, generator, stop := make(map[int]int), GeneratorTrialDivision(), IntSqrt(n)

	for n != 1 {
		factor := generator()
		for ; n%factor == 0; n /= factor {
			factorMap[factor]++
		}

		if factor >= stop {
			// if the number is prime, or if there is still a remainder, add itself as a factor
			if n >= factor || len(factorMap) == 0 {
				factorMap[n] = 1
			}
			break
		}
	}
	return factorMap
}

func PrimeComposition(factormap map[int]int) int {
	sum := 1
	for base, exp := range factormap {
		sum *= Pow(base, exp)
	}
	return sum
}

// closure that generates primes based on the method of trial division
func GeneratorTrialDivision() func() int {

	var cache []int

	return func() int {
		// avoid put 2 into the cache. It's easy to skip even numbers altogether
		if cache == nil {
			cache = make([]int, 0)
			return 2
		}
		// 3 is the smallest value in the cache. It's put explicitly so that the last element can always be retrieved
		if len(cache) == 0 {
			cache = append(cache, 3)
			return 3
		}
		for candidate, stop := cache[len(cache)-1]+2, IntSqrt(cache[len(cache)-1]); ; candidate += 2 {
			small := candidate <= math.MaxInt32
			for _, prime := range cache {
				if small {
					if int32(candidate)%int32(prime) == 0 {
						break
					}
				} else if candidate%prime == 0 {
					break
				}

				if prime > stop {
					// won't find a prime factor, therefore the candidate is prime
					cache = append(cache, candidate)
					return candidate
				}
			}
		}
	}
}

// closure that generates primes prime numbers, starting with the one below N.
func PrimesLessThan(n int) func() int {

	candidate := n

	return func() int {
		for candidate--; ; candidate-- {
			if MillerRabin(candidate) {
				return candidate
			}
		}
	}
}

func MillerRabin(n int) bool {
	effectiveBase := millerRabinFast
	if n >= 4759123141 {
		effectiveBase = millerRabinBase
	}

	for _, b := range effectiveBase {
		if n > b && !millerRabinPass(b, n) {
			return false
		}
	}
	return true
	// return (n > 1) && ((n == 2) || stream(effectiveBase).allMatch(b- > n <= b || millerRabinPass(b, n)));
}

func millerRabinPass(b, n int) bool {
	s := bits.TrailingZeros64(uint64(n - 1))
	d := (n - 1) >> uint64(s)
	a := PowerMod(b, d, n)

	if a == 1 {
		return true
	}
	for i := 0; i < s-1; i++ {
		if a == n-1 {
			return true
		}
		if a == 1 {
			return false
		}
		a = PowerMod(a, 2, n)
	}
	return a == n-1
}
