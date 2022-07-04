package main

import (
	"fmt"
	"os"
	"sort"
)

func main() {
	var knots int
	fmt.Scan(&knots)
	var knotsToLearn []int
	var temp int
	for i := 0; i < knots; i++ {
		fmt.Scan(&temp)
		knotsToLearn = append(knotsToLearn, temp)
	}
	var knotsKnown []int
	for i := 0; i < knots-1; i++ {
		fmt.Scan(&temp)
		knotsKnown = append(knotsKnown, temp)
	}
	sort.Ints(knotsToLearn)
	sort.Ints(knotsKnown)
	for i := 0; i < len(knotsToLearn); i++ {
		if i == len(knotsToLearn)-1 || knotsKnown[i] != knotsToLearn[i] {
			fmt.Println(knotsToLearn[i])
			os.Exit(0)
		}
	}
}
