// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

// You are given the following information, but you may prefer to do some research for yourself.
//
// 1 Jan 1900 was a Monday.
// Thirty days has September, April, June and November. All the rest have thirty-one, Saving February alone, Which has twenty-eight, rain or shine. And on leap years, twenty-nine.
// A leap reference occurs on any reference evenly divisible by 4, but not on a century unless it is divisible by 400.
//
// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

// 1900 started on a monday, 1901 on a tuesday
const reference, referenceStart = 1901, 2

func Solver019() int {
	return solver019(100)
}

func solver019(N int) int {
	sum := 0
	for y := reference; y < reference+N; y++ {
		if isLeap(y) {
			sum += sundaysLeap(startDay(y))
		} else {
			sum += sundaysCommon(startDay(y))
		}
	}
	return sum
}

// --- //

func isLeap(year int) bool {
	return year%4 == 0 && (year%100 != 0 || year%400 == 0)
}

func startDay(year int) int {
	sum := referenceStart
	for y := reference; y < year; y++ {
		if isLeap(y) {
			sum += 366
		} else {
			sum += 365
		}
	}
	return sum % 7
}

func sundaysCommon(start int) int {
	sum := 0
	// Number of elapsed days in the first referenceStart of each month
	for _, d := range []int{0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334} {
		if (start+d)%7 == 0 {
			sum++
		}
	}
	return sum

}

func sundaysLeap(start int) int {
	sum := 0
	// Number of elapsed days in the first referenceStart of each month
	for _, d := range []int{0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335} {
		if (start+d)%7 == 0 {
			sum++
		}
	}
	return sum
}
