package main

import "fmt"

func main() {
	var numberAmount, tempInt int
	var numberSlice []int

	fmt.Scan(&numberAmount)

	for i := 0; i < numberAmount; i++ {
		fmt.Scan(&tempInt)
		numberSlice = append(numberSlice, tempInt)
	}

	for _, value := range numberSlice {
		if value%2 == 0 {
			fmt.Println(value, "is even")
		} else {
			fmt.Println(value, "is odd")
		}
	}

}
