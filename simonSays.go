package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	var inputLines int
	fmt.Scan(&inputLines)

	scanner := bufio.NewScanner(os.Stdin)
	for i := 0; i < inputLines; i++ {
		scanner.Scan()
		s := scanner.Text()
		if strings.Contains(s, "Simon says") {
			fmt.Println(strings.TrimPrefix(s, "Simon says "))
		}
	}
}
