// Cold-puter Science
// https://open.kattis.com/problems/cold

package main

import (
	"fmt"
)

func main() {
	var numberOfTemps, curTemp int
	var sliceOfTemps []int

	fmt.Scanln(&numberOfTemps)

	for i := 0; i < numberOfTemps; i++ {
		fmt.Scan(&curTemp)
		sliceOfTemps = append(sliceOfTemps, curTemp)
	}

	var minusTempCount int

	for _, value := range sliceOfTemps {
		if value < 0 {
			minusTempCount++
		}
	}
	fmt.Println(minusTempCount)
}
