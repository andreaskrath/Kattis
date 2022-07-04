package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	var cases int
	fmt.Scan(&cases)

	scanner := bufio.NewScanner(os.Stdin)

	for i := 0; i < cases; i++ {
		scanner.Scan()
		s := scanner.Text()
		if s == "P=NP" {
			fmt.Println("skipped")
		} else {
			split := strings.Split(s, "+")
			fmt.Println(makeInt(split[0]) + makeInt(split[1]))
		}
	}
}

func makeInt(s string) int {
	n, err := strconv.Atoi(s)
	if err != nil {
		os.Exit(0)
	}
	return n
}
