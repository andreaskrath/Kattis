// Dice Cup
// https://open.kattis.com/problems/dicecup

package main

import (
	"fmt"
	"math"
)

func main() {
	var diceOne, diceTwo int

	fmt.Scanf("%d %d", &diceOne, &diceTwo)

	if diceOne == diceTwo {
		fmt.Println(diceOne + 1)
	} else {
		for i := math.Min(float64(diceOne), float64(diceTwo)); i <= math.Max(float64(diceOne), float64(diceTwo)); i++ {
			fmt.Println(i + 1)
		}
	}
}
