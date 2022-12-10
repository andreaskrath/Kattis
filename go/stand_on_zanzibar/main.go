// Stand on Zanzibar
// https://open.kattis.com/problems/zanzibar

package main

import "fmt"

func main() {
	var inputAmount int
	fmt.Scan(&inputAmount)

	for i := 0; i < inputAmount; i++ {
		oldYear, newYear, turtles := 0, 0, 0
		for {
			fmt.Scan(&newYear)
			if newYear == 0 { // terminates the sequence
				break
			}
			if oldYear == 0 { // skipping first comparison
				oldYear = newYear
				continue
			}

			if newYear-(oldYear*2) > 0 {
				turtles += newYear - (oldYear * 2)
			}
			oldYear = newYear
		}
		fmt.Println(turtles)
	}
}
