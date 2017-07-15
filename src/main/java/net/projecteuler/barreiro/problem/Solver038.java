/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import net.projecteuler.barreiro.algorithm.util.LongUtils;

import java.util.Comparator;
import java.util.function.LongPredicate;

import static java.util.Arrays.stream;
import static java.util.stream.LongStream.rangeClosed;
import static net.projecteuler.barreiro.algorithm.Combinatorics.partitionStream;
import static net.projecteuler.barreiro.algorithm.Combinatorics.permutationStream;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.intSqrt;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.longSet;

/**
 * Take the number 192 and multiply it by each of 1, 2, and 3:
 * 192 × 1 = 192
 * 192 × 2 = 384
 * 192 × 3 = 576
 * By concatenating each product we get the 1 to 9 pandigital, 192384576. We will call 192384576 the concatenated product of 192 and (1,2,3)
 * <p>
 * The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5, giving the pandigital, 918273645, which is the concatenated product of 9 and (1,2,3,4,5).
 * <p>
 * What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated product of an integer with (1,2, ... , n) where n > 1?
 *
 * @author barreiro
 */
public class Solver038 extends ProjectEulerSolver {

    private static final Comparator<long[]> REVERSE_COMPARATOR = (l1, l2) -> {
        for ( int i = l1.length - 1; i >= 0; i-- ) {
            if ( l1[i] != l2[i] ) {
                return (int) ( l2[i] - l1[i] );
            }
        }
        return 0;
    };

    public Solver038() {
        this( 9 );
    }

    public Solver038(long n) {
        super( n );
    }

    // --- //

    private boolean hasSameFactor(long... partition) {
        LongPredicate reverseNatural = p -> rangeClosed( 1, partition.length ).allMatch( n -> partition[(int) n - 1] / p == partition.length - n + 1 );
        return stream( partition ).filter( l -> stream( partition ).allMatch( p -> l != 0 && p % l == 0 && p / l < N ) ).anyMatch( reverseNatural );
    }

    private boolean solveSingle(long partitions, long... permutation) {
        return partitionStream( permutation, (int) partitions ).filter( l -> stream( l ).allMatch( a -> a > 10 ) ).anyMatch( this::hasSameFactor );
    }

    // different approach: generate pandigital numbers, order them and then try to find a multiplier from one of the possible partitions
    public long solve() {
        return permutationStream( longSet( rangeClosed( 1, N ) ) ).sorted( REVERSE_COMPARATOR ).filter( p -> rangeClosed( 2, intSqrt( N ) ).anyMatch( n -> solveSingle( n, p ) ) ).mapToLong( LongUtils::fromDigits ).findFirst().orElse( 0L );
    }
}
