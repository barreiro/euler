/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import static java.util.Arrays.stream;
import static java.util.stream.LongStream.range;

/**
 * You are given the following information, but you may prefer to do some research for yourself.
 * <p>
 * 1 Jan 1900 was a Monday.
 * Thirty days has September, April, June and November. All the rest have thirty-one, Saving February alone, Which has twenty-eight, rain or shine. And on leap years, twenty-nine.
 * A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
 * <p>
 * How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
 *
 * @author barreiro
 */
public class Solver019 extends ProjectEulerSolver {

    static {
        // 1900 started on a monday, 1901 on a tuesday
        START_YEAR = 1901;
        START_DAY = 2;
    }

    private static long START_YEAR;
    private static long START_DAY;

    // Number of elapsed days in the first day of each month
    private static long[] DAYS_COMMON = new long[]{0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334};
    private static long[] DAYS_LEAP = new long[]{0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335};

    public Solver019() {
        this(100);
    }

    public Solver019(long n) {
        super(n);
    }

    /* --- */

    public long solve() {
        return range(START_YEAR, START_YEAR + N).map(y -> isLeap(y) ? sundaysLeap(startDay(y)) : sundaysCommon(startDay(y))).sum();
    }

    /**
     * A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
     *
     * @param year Year to evaluate
     * @return True if is a leap year
     */
    public static boolean isLeap(final long year) {
        return (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0);
    }

    private static long startDay(final long year) {
        return (START_DAY + range(START_YEAR, year).map(y -> isLeap(y) ? 366 : 365).sum()) % 7;
    }

    private static long sundaysCommon(final long startDay) {
        return stream(DAYS_COMMON).filter(d -> (startDay + d) % 7 == 0).count();
    }

    private static long sundaysLeap(final long startDay) {
        return stream(DAYS_LEAP).filter(d -> (startDay + d) % 7 == 0).count();
    }
}
