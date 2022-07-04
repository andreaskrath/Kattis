package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Split(bufio.ScanWords)

	table := make(map[byte]int)

	for i := 0; i < 5; i++ {
		scanner.Scan()
		card := scanner.Text()
		table[card[0]]++
	}

	max := table[0]
	for _, v := range table {
		if v > max {
			max = v
		}
	}

	fmt.Println(max)
}
