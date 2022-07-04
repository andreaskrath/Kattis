package main

import "fmt"

func main() {
	var input float64 // percentage of switch points bet on options one
	fmt.Scan(&input)

	totalPoints := 50.0
	betOnePoints := totalPoints * (input / 100)
	betTwoPoints := totalPoints * (1 - (input / 100))

	fmt.Println(totalPoints / betOnePoints)
	fmt.Println(totalPoints / betTwoPoints)
}
