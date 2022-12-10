// Bijele
// https://open.kattis.com/problems/bijele

package main

import "fmt"

func main() {
	var pieces []int
	var tempInt int
	for i := 0; i < 6; i++ {
		fmt.Scan(&tempInt)
		pieces = append(pieces, tempInt)
	}

	properPieces := [6]int{1, 1, 2, 2, 2, 8}
	var outputString string
	for i, v := range pieces {
		outputString += fmt.Sprint(properPieces[i] - v)
		outputString += " "
	}

	fmt.Println(outputString)
}
