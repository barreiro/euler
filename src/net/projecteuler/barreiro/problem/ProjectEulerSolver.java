/* COPYRIGHT (C) 2014 barreiro. All Rights Reserved. */

package net.projecteuler.barreiro.problem;

/**
 * Abstract class for a solver for a particular euler problem.
 *
 * @author barreiro
 */
public abstract class ProjectEulerSolver {

    protected final long N;

    /**
     * It's very useful that for every problem solver we define some sort of dimension. Mostly for testing purposes.
     *
     * @param N Scale of the problem
     */
    protected ProjectEulerSolver(long N) {
        this.N = N;
    }

    /**
     * Solves all problems! The euler ones.
     *
     * @param args not used
     */
    public static void main(final String[] args) {
        final int PROBLEMS = 20;
        for (int problem = 1; problem <= PROBLEMS; problem++) {
            try {
                String solverClassName = String.format("%s%s%03d", ProjectEulerSolver.class.getPackage().getName(), ".Solver", problem);
                ProjectEulerSolver solverInstance = ProjectEulerSolver.class.cast(Class.forName(solverClassName).newInstance());
                if (solverInstance != null) {
                    System.out.printf("Solution for problem %03d is %d%n", problem, solverInstance.solve());
                }
            } catch (ClassNotFoundException e) {
                System.out.printf("ERROR: No implementation found for problem %d%n", problem);
            } catch (Exception e) {
                System.out.printf("ERROR: Exception during execution of problem %s: %s%n", problem, e.getMessage());
            }
        }
    }

    /* --- */

    /**
     * @return Solution for the problem.
     */
    public abstract long solve();

}
