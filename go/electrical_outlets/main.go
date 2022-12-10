// Electrical Outlets
// https://open.kattis.com/problems/electricaloutlets
package main

import "fmt"

func main() {
	var testCases int
	var appliances []int
	fmt.Scan(&testCases)
	for i := 0; i < testCases; i++ {
		appliances = append(appliances, findAppliances())
	}

	for _, v := range appliances {
		fmt.Println(v)
	}
}

func findAppliances() int {
	var powerStrips, outlets, tempInt int
	fmt.Scan(&powerStrips)
	for i := 0; i < powerStrips; i++ {
		fmt.Scan(&tempInt)
		outlets += tempInt
	}

	return outlets - (powerStrips - 1)
}
