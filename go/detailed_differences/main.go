// Detailed Differences
// https://open.kattis.com/problems/detaileddifferences

package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var cases int
	fmt.Scan(&cases)
	scanner := bufio.NewScanner(os.Stdin)

	for i := 0; i < cases; i++ {
		scanner.Scan()
		s1 := scanner.Text()
		scanner.Scan()
		s2 := scanner.Text()
		s3 := compareString(s1, s2)
		fmt.Print(s1, "\n", s2, "\n", s3, "\n\n")
	}
}

func compareString(s1, s2 string) string {
	var output string
	for i := 0; i < len(s1); i++ {
		if s1[i] == s2[i] {
			output += "."
		} else {
			output += "*"
		}
	}
	return output
}
