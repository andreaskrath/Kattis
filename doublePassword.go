package main

import "fmt"

func main() {
	var passOne, passTwo string

	fmt.Scan(&passOne)
	fmt.Scan(&passTwo)

	passCombinations := 1
	for i := 0; i < len(passOne); i++ {
		if passOne[i] == passTwo[i] {
			passCombinations *= 1
		} else {
			passCombinations *= 2
		}
	}

	fmt.Println(passCombinations)
}
