// Kornislav
// https://open.kattis.com/problems/kornislav

package main

import (
	"fmt"
	"sort"
)

func main() {
	var lengths []int
	var temp int
	for i := 0; i < 4; i++ {
		fmt.Scan(&temp)
		lengths = append(lengths, temp)
	}
	sort.Ints(lengths)

	fmt.Println(lengths[0] * lengths[2]) // shortest and second longest lengths give us area of rectangle
}
