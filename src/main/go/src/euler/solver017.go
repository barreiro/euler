// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

// If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
// NOTE: Do not count spaces or hyphens.

// number of letters for zero (0), one, two [...] nineteen and zero (0), ten, twenty [...] ninety
var lookupOnes, lookupTens = [20]int{0, 3, 3, 5, 4, 4, 3, 5, 5, 4, 3, 6, 6, 8, 8, 7, 7, 9, 8, 8}, [10]int{0, 3, 6, 6, 5, 5, 5, 7, 6, 6}

// number of letters for 'and' 'thousand and' and 'hundred and'
const andCount, thousandCount, hundredCount = len("and"), len("thousand") + andCount, len("hundred") + andCount

func Solver017() int {
	return solver017(1000)
}

func solver017(N int) int {
	sum := 0
	for i := 1; i <= N; i++ {
		sum += letterCount(i)
	}
	return sum
}

func letterCount(i int) int {
	// the number of letters of the thousands, then the hundreds, and finally lookupOnes tens and ones
	sum := 0
	if thousand := i / 1000; thousand > 0 {
		sum += lookupOnes[thousand] + thousandCount
	}
	if hundred := i % 1000 / 100; hundred > 0 {
		sum += lookupOnes[hundred] + hundredCount
	}

	if ten, one := i%100/10, i%10; ten == 0 && one == 0 {
		return sum - andCount
	} else if ten > 1 {
		return sum + lookupTens[ten] + lookupOnes[one]
	} else {
		return sum + lookupOnes[i%100]
	}
}
