/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static java.util.stream.LongStream.rangeClosed;
import static net.projecteuler.barreiro.algorithm.Combinatorics.partitionStream;
import static net.projecteuler.barreiro.algorithm.Combinatorics.permutationStream;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.longSet;

/**
 * We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.
 * <p>
 * The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand, multiplier, and product is 1 through 9 pandigital.
 * <p>
 * Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital.
 * <p>
 * HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.
 *
 * @author barreiro
 */
public class Solver032 extends ProjectEulerSolver {

    public Solver032() {
        this( 9 );
    }

    public Solver032(long n) {
        super( n );
    }

    // --- //

    public long solve() {
        return permutationStream( longSet( rangeClosed( 1, N ) ) ).mapToLong( p -> partitionStream( p, 3 ).filter( l -> l[0] == l[1] * l[2] ).mapToLong( l -> l[0] ).sum() ).distinct().sum();
    }

}
