// Seven Wonders
// https://open.kattis.com/problems/sevenwonders

package main

import (
	"fmt"
)

func main() {
	var input string
	fmt.Scan(&input)

	countTable := make(map[rune]int)
	for _, v := range input {
		countTable[v]++
	}

	var points int
	minimum := 51
	for _, v := range countTable {
		if v < minimum {
			minimum = v
		}
		points += v * v
	}
	if len(countTable) == 3 {
		points += minimum * 7
	}

	fmt.Println(points)
}
