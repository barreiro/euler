/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import java.util.Deque;
import java.util.LinkedList;
import java.util.List;

import static java.util.stream.LongStream.range;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.factorial;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.fromDigits;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.longArray;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.longList;

/**
 * A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4.
 * If all of the permutations are listed numerically or alphabetically, we call it lexicographic order.
 * The lexicographic permutations of 0, 1 and 2 are:
 * <p>
 * 012   021   102   120   201   210
 * <p>
 * What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
 *
 * @author barreiro
 */
public class Solver024 extends ProjectEulerSolver {

    private final int base;

    public Solver024() {
        this( 1000000 );
    }

    public Solver024(long n) {
        this( n, 10 );
    }

    public Solver024(long n, int base) {
        super( n );
        this.base = base;
    }

    // --- //

    public long solve() {
        List<Long> unplaced = longList( range( 0, base ) );
        Deque<Long> placed = new LinkedList<>();
        long value = N - 1;

        // Use a kind of factorization of N over the factorials. In the end convert the digits to a number.
        for ( long l = 0; l < base - 1; l++ ) {
            long f = factorial( base - 1 - l );
            long q = ( value / f );
            long digit = unplaced.get( (int) q );
            
            unplaced.remove( digit );
            placed.addFirst( digit );
            value %= f;
        }
        placed.addFirst( unplaced.get( 0 ) );

        return fromDigits( longArray( placed.stream() ) );
    }

}
