/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import net.projecteuler.barreiro.algorithm.Primes;

import java.util.Map;
import java.util.TreeMap;

/**
 * 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
 * What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
 *
 * @author barreiro
 */
public class Solver005 extends ProjectEulerSolver {

    public Solver005() {
        this(20);
    }

    public Solver005(long n) {
        super(n);
    }

    /* --- */

    public long solve() {
        Primes.Generator generator = new Primes.Generator();
        Map<Long, Long> factorMap = new TreeMap<>();

        for (long prime = 1; prime <= N; ) { // Pre-fill prime set up to N
            prime = generator.nextPrime();
            factorMap.put(prime, 0L);
        }

        for (long i = 2; i <= N; i++) { // Decompose all numbers in prime factors and collect the greatest
            for (Map.Entry<Long, Long> primeEntry : Primes.primeFactors(i, factorMap.keySet()).entrySet()) {
                if (factorMap.get(primeEntry.getKey()) < primeEntry.getValue()) {
                    factorMap.put(primeEntry.getKey(), primeEntry.getValue());
                }
            }
        }

        long candidate = 1;
        for (Map.Entry<Long, Long> factorEntry : factorMap.entrySet()) { // Compose from prime factors
            candidate *= Math.pow(factorEntry.getKey(), factorEntry.getValue());
        }
        return candidate;
    }

}
