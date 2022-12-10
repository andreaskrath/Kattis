// GCVWR
// https://open.kattis.com/problems/gcvwr

package main

import "fmt"

func main() {
	var gross, truck, items int

	fmt.Scanf("%d %d %d", &gross, &truck, &items)

	remaining := (float64(gross) - float64(truck)) * 0.9

	var tempInt float64
	for i := 0; i < items; i++ {
		fmt.Scan(&tempInt)
		remaining -= tempInt
	}

	fmt.Println(remaining)
}
