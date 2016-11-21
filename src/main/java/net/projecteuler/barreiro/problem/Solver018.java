/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.StringReader;
import java.util.ArrayList;
import java.util.List;

import static java.lang.Integer.valueOf;
import static net.projecteuler.barreiro.algorithm.util.StreamUtils.maxLong;

/**
 * By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is 23.
 * <pre style="text-align: center">
 * 3
 * 7 4
 * 2 4 6
 * 8 5 9 3</pre>
 * That is, 3 + 7 + 4 + 9 = 23.
 * Find the maximum total from top to bottom of the triangle below:
 * <p>
 * NOTE: As there are only 16384 routes, it is possible to solve this problem by trying every route.
 * However, Problem 67, is the same challenge with a triangle containing one-hundred rows; it cannot be solved by brute force, and requires a clever method! ;o)
 *
 * @author barreiro
 */
public class Solver018 extends ProjectEulerSolver {

    private static final String INPUT_STR = "" +
            "75\n" +
            "95 64\n" +
            "17 47 82\n" +
            "18 35 87 10\n" +
            "20 04 82 47 65\n" +
            "19 01 23 75 03 34\n" +
            "88 02 77 73 07 63 67\n" +
            "99 65 04 28 06 16 70 92\n" +
            "41 41 26 56 83 40 80 70 33\n" +
            "41 48 72 33 47 32 37 16 94 29\n" +
            "53 71 44 65 25 43 91 52 97 51 14\n" +
            "70 11 33 28 77 73 17 78 39 68 17 57\n" +
            "91 71 52 38 17 14 91 43 58 50 27 29 48\n" +
            "63 66 04 68 89 53 67 30 73 16 69 87 40 31\n" +
            "04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

    private final String data;

    public Solver018() {
        this( 15 );
    }

    public Solver018(long n) {
        this( n, INPUT_STR );
    }

    public Solver018(long n, String rawData) {
        super( n );
        data = rawData;
    }

    // --- //

    private static final class Node<T> {
        private final T element;
        private List<Node<T>> children;

        private Node(T element) {
            this.element = element;
        }

        private void addChild(Node<T> child) {
            if ( this.children == null ) {
                children = new ArrayList<>();
            }
            children.add( child );
        }

        private boolean isLeaf() {
            return this.children == null;
        }
    }

    private Node<Integer> parse(String rawData) {
        List<List<Node<Integer>>> data = new ArrayList<>();

        // For each line read the numbers and add as children of the previous line
        try ( BufferedReader reader = new BufferedReader( new StringReader( rawData ) ) ) {
            for ( String line = reader.readLine(); line != null; line = reader.readLine() ) {
                List<Node<Integer>> elements = new ArrayList<>();
                for ( String elementStr : line.split( " " ) ) {
                    Node<Integer> element = new Node<>( valueOf( elementStr ) );
                    elements.add( element );
                    if ( !data.isEmpty() ) {
                        List<Node<Integer>> parentList = data.get( data.size() - 1 );
                        if ( elements.size() > 1 ) {
                            parentList.get( elements.size() - 2 ).addChild( element );
                        }
                        if ( elements.size() <= parentList.size() ) {
                            parentList.get( elements.size() - 1 ).addChild( element );
                        }
                    }
                }
                data.add( elements );
                if ( data.size() == N ) {
                    break;
                }
            }
        } catch ( IOException e ) {
            // Finish
        }
        return data.get( 0 ).get( 0 );
    }

    // --- //

    // Recursive call on a node that returns the value of the node, plus the highest of its children
    private static long bestSum(Node<Integer> node) {
        return node.element + ( node.isLeaf() ? 0 : maxLong( node.children.stream().mapToLong( Solver018::bestSum ) ) );
    }

    public long solve() {
        return bestSum( parse( data ) );
    }

}
