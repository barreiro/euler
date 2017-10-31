// COPYRIGHT (C) 2014 barreiro. All Rights Reserved.
// GoLang helper for Project Euler problems
package algorithm

// Calculates the prime factors
func PrimeFactors(n int) map[int]int {
	m := make(map[int]int)
	generator := generatorTrialDivision()
	subject := n
	stop := IntSqrt(n)

	for subject != 1 {
		factor := generator()
		for ; subject%factor == 0; subject /= factor {
			m[factor]++
		}

		if factor > stop {
			// if the number is prime, or if there is still a remainder, add itself as a factor
			if subject >= factor || len(m) == 0 {
				m[subject] = 1
			}
			break
		}
	}
	return m
}

// closure that generates primes based on the method of trial division
func generatorTrialDivision() func() int {

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
