// Fifty Shades of Pink
// https://open.kattis.com/problems/fiftyshades

package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	var cases int
	fmt.Scan(&cases)

	scanner := bufio.NewScanner(os.Stdin)

	var counter int
	for i := 0; i < cases; i++ {
		scanner.Scan()
		s := strings.ToLower(scanner.Text())
		if strings.Contains(s, "pink") || strings.Contains(s, "rose") {
			counter++
		}
	}

	if counter != 0 {
		fmt.Println(counter)
	} else {
		fmt.Println("I must watch Star Wars with my daughter")
	}
}
