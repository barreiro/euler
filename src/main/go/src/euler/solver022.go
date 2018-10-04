// COPYRIGHT (C) 2017 barreiro. All Rights Reserved.
// GoLang solvers for Project Euler problems
package euler

import (
	"bufio"
	"euler/util"
	"io"
	"sort"
	"unicode/utf8"
)

// Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.
// For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.
// What is the total of all the name scores in the file?

func Solver022() int {
	return solver022(5163)
}

func solver022(N int) int {
	file := util.OpenResource("problem022-data.txt")
	defer file.Close()
	return solver022reader(N, file)
}

func solver022reader(N int, reader io.Reader) int {
	scanner, names, sum := bufio.NewScanner(reader), make([]string, 0), 0
	scanner.Split(scanWords)

	for scanner.Scan() {
		names = append(names, scanner.Text())
	}
	sort.Strings(names)
	for i, name := range names {
		if i < N {
			for _, c := range name {
				sum += (i + 1) * int(c-'A'+1)
			}
		}
	}
	return sum
}

// This is the scan function, that takes '"' and ',' as separators
func scanWords(data []byte, atEOF bool) (advance int, token []byte, err error) {
	var r rune
	start := 0
	for width := 0; start < len(data); start += width {
		r, width = utf8.DecodeRune(data[start:])
		if r != '"' && r != ',' {
			break
		}
	}
	for width, i := 0, start; i < len(data); i += width {
		r, width = utf8.DecodeRune(data[i:])
		if r == '"' {
			return i + width, data[start:i], nil
		}
	}
	if atEOF && len(data) > start {
		return len(data), data[start:], nil
	}
	return start, nil, nil
}
