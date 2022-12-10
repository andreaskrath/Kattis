// Magic Trick
// https://open.kattis.com/problems/magictrick

package main

import "fmt"

func main() {
	var inputString string
	var runeSlice []rune

	fmt.Scan(&inputString)

	runeSlice = []rune(inputString)

	for i, v := range runeSlice {
		for j := i + 1; j < len(runeSlice); j++ { // we dont need to check earlier than i, that would have been cought on first runthrough
			if v == runeSlice[j] {
				fmt.Println(0)
				return // exit on failure
			}
		}
	}

	fmt.Println(1)
}
