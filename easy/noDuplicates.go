package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)

	table := make(map[string]int)

	scanner.Scan()
	words := strings.Split(scanner.Text(), " ")

	for _, v := range words {
		table[v]++
		if table[v] > 1 {
			fmt.Println("no")
			os.Exit(0)
		}
	}
	fmt.Println("yes")
}
