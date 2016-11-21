/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import net.projecteuler.barreiro.algorithm.util.StreamUtils;

import java.io.BufferedInputStream;
import java.util.Scanner;
import java.util.Set;
import java.util.TreeSet;

import static java.util.regex.Pattern.compile;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.lazyStream;

/**
 * Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.
 * <p>
 * For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.
 * <p>
 * What is the total of all the name scores in the file?
 *
 * @author barreiro
 */
public class Solver022 extends ProjectEulerSolver {

    private final Scanner scanner;

    public Solver022() {
        this( 5163 );
    }

    public Solver022(long n) {
        super( n );
        this.scanner = new Scanner( new BufferedInputStream( Solver022.class.getResourceAsStream( "problem022-data.txt" ) ) );
    }

    public Solver022(long n, String data) {
        super( n );
        this.scanner = new Scanner( data );
    }

    // --- //

    private static class Counter {

        private long value = 1;

        private long increment() {
            return value++;
        }

    }

    public long solve() {
        Set<String> sortedNames = new TreeSet<>();
        Counter c = new Counter();

        lazyStream( scanner.useDelimiter( compile( ",|\"" ) ) ).filter( s -> !s.isEmpty() ).forEach( sortedNames::add );
        return sortedNames.stream().limit( N ).mapToLong( StreamUtils::letterSum ).reduce( 0, (l1, l2) -> l1 + l2 * c.increment() );
    }

}
