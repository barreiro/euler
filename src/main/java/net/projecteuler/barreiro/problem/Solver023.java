/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import net.projecteuler.barreiro.algorithm.Factorization;

import java.util.Set;

import static java.util.stream.LongStream.rangeClosed;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.longSet;

/**
 * A perfect number is a number for which the sum of its proper divisors is exactly equal to the number.
 * For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.
 * <p>
 * A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant if this sum exceeds n.
 * <p>
 * As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as the sum of two abundant numbers is 24.
 * By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers.
 * However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.
 * <p>
 * Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.
 *
 * @author barreiro
 */
public class Solver023 extends ProjectEulerSolver {

    public Solver023() {
        this( 28123 );
    }

    public Solver023(long n) {
        super( n );
    }

    // --- //

    public long solve() {
        Set<Long> abundant = longSet( rangeClosed( 1, N ).parallel().filter( Factorization::isAbundant ) );
        return rangeClosed( 1, N ).parallel().filter( n -> abundant.stream().filter( a -> a <= n / 2 ).noneMatch( a -> abundant.contains( n - a ) ) ).sum();
    }

}
