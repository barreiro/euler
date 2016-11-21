/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import java.util.function.LongPredicate;
import java.util.stream.DoubleStream;
import java.util.stream.LongStream;

import static java.util.stream.LongStream.range;

/**
 * The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to simplify it may incorrectly believe that 49/98 = 4/8, which is correct, is obtained by cancelling the 9s.
 * <p>
 * We shall consider fractions like, 30/50 = 3/5, to be trivial examples.
 * <p>
 * There are exactly four non-trivial examples of this type of fraction, less than one in value, and containing two digits in the numerator and denominator.
 * <p>
 * If the product of these four fractions is given in its lowest common terms, find the value of the denominator.
 *
 * @author barreiro
 */
public class Solver033 extends ProjectEulerSolver {

    private static final int BASE = 10;

    public Solver033() {
        this( 100 );
    }

    public Solver033(long n) {
        super( n );
    }

    // --- //

    private LongStream denominator() {
        return range( 1, N ).parallel();
    }

    private LongStream numerator(long max) {
        return range( 1, max ).parallel();
    }

    private static long product(DoubleStream stream) {
        return (long) stream.reduce( 1, (d1, d2) -> d1 * d2 );
    }

    /*
     * Calculates the contribution to a product.
     * In the context of this problem, we need double because of irreducible fractions like 2/5 -> 5/2
     */
    private static double contribution(DoubleStream stream) {
        return stream.findFirst().orElse( 1 );
    }

    /*
     * Predicate to check if a fraction is naive, i.e. the end of the numerator equals the beginning of the denominator
     */
    private static LongPredicate naive(long denominator) {
        return numerator -> range( 0, BASE ).anyMatch( radix -> naiveCancellation( numerator, denominator, radix ) );
    }

    /*
     * This is a bit over-optimized!
     * What happens here is that we first check if the end of the numerator is equal to the start of the denominator.
     * We then check to see if the fractions match, juggling a bit with the terms to avoid double calculation.
     *
     * => naiveNumerator = numerator / 10, naiveDenominator = denominator - radix * 10
     * => numerator / denominator == naiveNumerator / naiveDenominator;
     */
    public static boolean naiveCancellation(long n, long d, long r) {
        return n % 10 == r && d > r * 10 && n * ( d - r * BASE ) == d * ( n / BASE );
    }

    // --- //

    public long solve() {
        return product( denominator().mapToDouble( d -> contribution( numerator( d ).filter( naive( d ) ).mapToDouble( n -> (double) d / n ) ) ) );
    }

}
