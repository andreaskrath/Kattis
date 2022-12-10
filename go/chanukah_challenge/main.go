// Chanukah Challenge
// https://open.kattis.com/problems/chanukah

package main

import "fmt"

func main() {
	var dataSets int
	fmt.Scan(&dataSets)

	var tempInt, ignore int
	var candles []int
	for i := 0; i < dataSets; i++ {
		fmt.Scanf("%d %d", &ignore, &tempInt)
		candles = append(candles, findCandleAmount(tempInt))
	}

	for i, v := range candles {
		fmt.Println(i+1, v)
	}
}

func findCandleAmount(input int) int {
	sum := input

	for i := input; i > 0; i-- {
		sum += i
	}
	// basically we return the sum counting down to 0
	// and our base is our amount of days
	// giving us total amount of candles needed for n days
	return sum
}
