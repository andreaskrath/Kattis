package main

import "fmt"

func main() {
	var inputLines int
	fmt.Scan(&inputLines)

	var newTime, oldTime, newVal, oldVal, totalVal float32

	fmt.Scanf("%f %f", &oldTime, &oldVal) //cheating
	for i := 0; i < inputLines-1; i++ {
		fmt.Scanf("%f %f", &newTime, &newVal)
		totalVal += ((newVal + oldVal) / 2.0) * (newTime - oldTime)
	}
	fmt.Println(totalVal)
}
