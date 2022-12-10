// Stuck In A Time Loop
// https://open.kattis.com/problems/timeloop

package main

import "fmt"

func main() {
	var loopCounter int

	fmt.Scan(&loopCounter)

	for i := 0; i < loopCounter; i++ {
		fmt.Println(i+1, " Abracadabra")
	}
}
