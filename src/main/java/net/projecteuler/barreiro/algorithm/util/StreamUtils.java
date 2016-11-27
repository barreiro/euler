/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.algorithm.util;

import java.util.Iterator;
import java.util.List;
import java.util.PrimitiveIterator;
import java.util.PrimitiveIterator.OfLong;
import java.util.Set;
import java.util.Spliterator;
import java.util.stream.IntStream;
import java.util.stream.LongStream;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

import static java.util.Arrays.stream;
import static java.util.Spliterator.IMMUTABLE;
import static java.util.Spliterator.NONNULL;
import static java.util.Spliterators.spliterator;
import static java.util.Spliterators.spliteratorUnknownSize;
import static java.util.stream.Collectors.toList;
import static java.util.stream.Collectors.toSet;
import static java.util.stream.LongStream.iterate;
import static java.util.stream.StreamSupport.longStream;
import static java.util.stream.StreamSupport.stream;

/**
 * Utility methods to work with streams.
 *
 * @author barreiro
 */
public final class StreamUtils {

    private StreamUtils() {
    }

    /**
     * Utility Method to create a stream from an iterator. Elements are added lazily.
     *
     * @return A long stream with the iterated longs
     */
    public static LongStream lazyStream(OfLong iterator) {
        return longStream( spliteratorUnknownSize( iterator, IMMUTABLE | NONNULL ), false );
    }

    /**
     * Utility Method to create a stream from an iterator. Elements are added lazily.
     *
     * @return A stream with the iterated objects
     */
    public static <E> Stream<E> lazyStream(Iterator<E> iterator) {
        return stream( spliteratorUnknownSize( iterator, IMMUTABLE | NONNULL ), false );
    }

    /**
     * Creates a stream is descending order
     *
     * @return A stream with the specified range
     */
    public static LongStream rangeReverse(long startExclusive, long endInclusive) {
        return iterate( startExclusive - 1, l -> l - 1 ).limit( startExclusive - endInclusive );
    }

    /**
     * Creates an infinite parallel long stream.
     *
     * @return An infinite stream
     */
    public static LongStream infiniteParallelStream() {
        return iterate( 1L, l -> l + 1 ).parallel();
    }

    /**
     * Creates an infinite parallel long stream with a lower bound
     *
     * @return An infinite stream
     */
    public static LongStream infiniteParallelStream(long start) {
        return iterate( start, l -> l + 1 ).parallel();
    }

    // --- //

    /**
     * Returns a List view of a LongStream
     *
     * @return a List view
     */
    public static List<Long> longList(LongStream stream) {
        return stream.mapToObj( Long::valueOf ).collect( toList() );
    }

    /**
     * Returns a Set view of a LongStream
     *
     * @return a Set view
     */
    public static Set<Long> longSet(LongStream stream) {
        return stream.mapToObj( Long::valueOf ).collect( toSet() );
    }

    /**
     * Returns a Set view of an IntStream
     *
     * @return a Set view
     */
    public static Set<Integer> intSet(IntStream stream) {
        return stream.mapToObj( Integer::valueOf ).collect( toSet() );
    }

    /**
     * Returns an array view of a Stream of Long
     *
     * @return an array view
     */
    public static long[] longArray(Stream<Long> stream) {
        return stream.mapToLong( Long::valueOf ).toArray();
    }

    /**
     * Returns a IntStream from numeric Strings
     *
     * @return a IntStream from the values in the array
     */
    public static IntStream intStream(String... array) {
        return stream( array ).mapToInt( Integer::valueOf );
    }

    // --- //

    /**
     * Returns the highest value on a stream of longs
     *
     * @return the highest value on the stream
     */
    public static long maxLong(Stream<Long> stream) {
        return stream.mapToLong( Long::valueOf ).max().orElse( Long.MIN_VALUE );
    }

    /**
     * Returns the highest value on a stream of longs
     *
     * @return the highest value on the stream
     */
    public static long maxLong(LongStream stream) {
        return stream.max().orElse( Long.MIN_VALUE );
    }

    /**
     * Returns the first value on a stream of longs
     *
     * @return the first value on the stream
     */
    public static long firstLong(LongStream stream) {
        return stream.findFirst().orElseThrow( () -> new ArithmeticException( "A value was expected on a stream" ) );
    }

    /**
     * Finds an non zero element on the stream. To be used as na alternative to filter.
     *
     * @return An non zero element, or zero if no element is found
     */
    public static long notZero(LongStream stream) {
        return stream.filter( l -> l != 0 ).findAny().orElse( 0 );
    }

    // --- //

    /**
     * Calculates the sum of a digit stream
     *
     * @return The sum
     */
    public static long digitSum(Stream<long[]> stream) {
        return stream.mapToLong( LongUtils::fromDigits ).sum();
    }

    /**
     * Calculates the sum of the even elements of a stream
     *
     * @return The sum
     */
    public static long evenSum(LongStream stream) {
        return stream.filter( l -> l % 2 == 0 ).sum();
    }

    /**
     * Reduces a long stream by applying the product reduction.
     *
     * @return The product
     */
    public static long product(LongStream stream) {
        return stream.reduce( 1, (l1, l2) -> l1 * l2 );
    }

    /**
     * Calculates the product of digits in a char sequence
     *
     * @return The product
     */
    public static long charProduct(CharSequence charSequence) {
        return product( charSequence.chars().mapToLong( c -> c - '0' ) );
    }

    /**
     * Calculates the product of letters in a char sequence of uppercase chars.
     *
     * @return The sum
     */
    public static long letterSum(CharSequence charSequence) {
        return charSequence.chars().mapToLong( c -> c - 'A' + 1 ).sum();
    }

}
