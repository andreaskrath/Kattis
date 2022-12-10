// be aware, there are two "Greetings!" problems on Kattis
// this is the solution to one of them.

package main

import (
	"fmt"
	"strings"
)

func main() {
	var input string

	fmt.Scan(&input)

	fmt.Printf("h%sy", strings.Repeat("e", 2*(strings.Count(input, "e"))))
}
