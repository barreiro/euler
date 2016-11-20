/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm;

import java.util.Iterator;
import java.util.Set;
import java.util.stream.Stream;

import static java.lang.Math.min;
import static java.util.stream.IntStream.range;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.pow;
import static net.projecteuler.barreiro.algorithm.util.LongUtils.toDigits;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.lazyStream;

/**
 * Utility methods for solving problems related to combinatorial logic
 *
 * @author barreiro
 */
public final class Combinatorics {

    private Combinatorics() {}

    /**
     * Method for calculation the combinations of a certain number of elements in a total number of places.
     * Uses recursion instead of the formula with factorials.
     *
     * @param totalL  Number of places
     * @param chooseL Number of elements
     * @return The number of combinations
     */
    public static long choose(long totalL, long chooseL) {
        // Take full advantage of symmetry
        int total = (int) totalL, choose = (int) min( chooseL, totalL - chooseL );
        return choose < 0 ? 0 : choose == 0 ? 1 : choose( total, choose, new long[total + 1][choose + 1] );
    }

    private static long choose(int total, int choose, long[][] cache) {
        if ( ( choose = min( choose, total - choose ) ) == 1 ) {
            return total;
        }
        if ( cache[total][choose] != 0 ) {
            return cache[total][choose];
        }

        long value = choose( total - 1, choose - 1, cache ) + choose( total - 1, choose, cache );

        cache[total][choose] = value;
        return value;
    }

    // --- //

    /**
     * Calculates the number of integer partition of a value, given a set of constrains.
     *
     * @param valueL     Number to partition
     * @param constrains Set of values that are allowed on the partition
     * @return The number of different partitions
     */
    public static long partition(long valueL, Set<Integer> constrains) {
        int value = (int) valueL;
        return partition( value, value, 0, constrains, new long[value + 1][value + 1] );
    }

    private static long partition(int remaining, int total, long sum, Set<Integer> constrains, long[][] cache) {
        if ( remaining == 0 ) {
            return 1;
        }
        if ( cache[remaining][total] != 0 ) {
            return cache[remaining][total];
        }

        long l = 0;
        for ( int c = min( remaining, total ); c > 0; c-- ) {
            if ( c <= remaining && constrains.contains( c ) ) {
                l += partition( remaining - c, c, sum + c, constrains, cache );
            }
        }
        cache[remaining][total] = l;
        return l;
    }

    // --- //

    /**
     * Creates a Stream where each value is a permutation from the digits of a number
     *
     * @param value number to be permuted
     * @return Stream of all possible permutations
     */
    public static Stream<long[]> rotationStream(long value) {
        return lazyStream( new Rotator( toDigits( value ) ) );
    }

    /**
     * Creates a Stream where each value is a permutation from a set of elements
     *
     * @param value set to be permuted
     * @return Stream of all possible permutations
     */
    public static Stream<long[]> permutationStream(Set<Long> value) {
        return lazyStream( new Permutator( value ) );
    }

    /**
     * Creates a Stream where each value is a 3-way partition from an array of elements
     *
     * @param value array to be partitioned
     * @param n number of partitions
     * @return Stream of all possible permutations
     */
    public static Stream<long[]> partitionStream(long[] value, int n) {
        return lazyStream( new Partitioner( value, n ) );
    }

    // --- //

    private static final class Rotator implements Iterator<long[]> {

        private final long[] permutation;
        private long rotations;

        private Rotator(long[] set) {
            this.permutation = new long[set.length];
            int index = 0;
            for ( long l : set ) {
                this.permutation[index++] = l;
            }
        }

        public boolean hasNext() {
            return rotations < permutation.length;
        }

        public long[] next() {
            long swap = permutation[permutation.length - 1];
            System.arraycopy( permutation, 0, permutation, 1, permutation.length - 1 );
            permutation[0] = swap;
            rotations++;
            return permutation;
        }
    }

    private static final class Permutator implements Iterator<long[]> {

        private final long[] set;
        private long[] permutation;
        private boolean[] fixed, forward;

        private Permutator(Set<Long> set) {
            this.set = new long[set.size()];
            int index = 0;
            for ( Long l : set ) {
                this.set[index++] = l.intValue();
            }
        }

        private static void swap(long[] array, int a, int b) {
            long swap = array[a];
            array[a] = array[b];
            array[b] = swap;
        }

        private static void swap(boolean[] array, int a, int b) {
            boolean swap = array[a];
            array[a] = array[b];
            array[b] = swap;
        }

        /**
         * Implementation of Steinhaus–Johnson–Trotter algorithm with Even's speedup
         */
        public long[] next() {
            if ( permutation == null ) {
                permutation = new long[set.length];
                fixed = new boolean[set.length];
                fixed[0] = true;
                forward = new boolean[set.length];
                range( 0, permutation.length ).forEach( l -> permutation[l] = set[l] );
                return permutation.clone();
            }
            long max = -1;
            int next, index = -1;
            for ( int i = 0; i < permutation.length; i++ ) {
                if ( !fixed[i] && permutation[i] > max ) {
                    max = permutation[i];
                    index = i;
                }
            }
            next = index + ( forward[index] ? 1 : -1 );
            swap( permutation, index, next );
            swap( fixed, index, next );
            swap( forward, index, next );

            if ( next == 0 || next == permutation.length - 1 || permutation[next + ( forward[next] ? 1 : -1 )] > max ) {
                fixed[next] = true;
            }

            for ( int i = 0; i < permutation.length; i++ ) {
                if ( permutation[i] > max ) {
                    forward[i] = !forward[i];
                    fixed[i] = false;
                }
            }
            return permutation.clone();
        }

        public boolean hasNext() {
            if ( fixed == null ) {
                return true;
            }
            for ( boolean b : fixed ) {
                if ( !b ) {
                    return true;
                }
            }
            return false;
        }
    }

    private static final class Partitioner implements Iterator<long[]> {

        private final long[] array;
        private final int[] pivot;

        private Partitioner(long[] array, int partitions) {
            this.array = array;
            this.pivot = new int[partitions + 1];
            this.pivot[partitions] = array.length;
            for ( int j = 0; j < partitions - 1; j++ ) {
                pivot[j] = j;
            }
        }

        private static long fromDigits(long[] digits, int from, int to) {
            long sum = 0;
            for ( int i = from; i < to; i++ ) {
                sum += digits[i] * pow( 10, i - from );
            }
            return sum;
        }

        public long[] next() {
            long[] result = new long[pivot.length - 1];
            for ( int i = 0; i < pivot.length - 1; i++ ) {
                result[i] = fromDigits( array, pivot[i], pivot[i + 1] );
            }
            for ( int i = pivot.length - 1; i > 1; i-- ) {
                if ( pivot[i] - 1 != pivot[i - 1] ) {
                    pivot[i - 1]++;
                    if ( i != pivot.length - 1 ) {
                        pivot[i] = pivot[i - 1] + 1;
                    }
                    break;
                }
            }
            return result;
        }

        public boolean hasNext() {
            for ( int i = pivot.length - 1; i > 1; i-- ) {
                if ( pivot[i] - 1 != pivot[i - 1] ) {
                    return true;
                }
            }
            return false;
        }
    }

}
