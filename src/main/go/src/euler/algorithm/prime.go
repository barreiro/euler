// COPYRIGHT (C) 2014 barreiro. All Rights Reserved.
// GoLang helper for Project Euler problems
package algorithm

// Calculates the prime factors
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
			for _, prime := range cache {
				if candidate%prime == 0 {
					// found a prime factor, thus candidate is not prime
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
