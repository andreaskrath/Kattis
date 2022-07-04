package main

import "fmt"

func main() {
	var input, seconds int
	fmt.Scan(&input)

	// even = start (zero indexed)
	// uneven = stop
	var tempOne, tempTwo int
	for i := 0; i < input; i++ {
		if i%2 == 0 {
			fmt.Scan(&tempOne)
		} else {
			fmt.Scan(&tempTwo)
			seconds += tempTwo - tempOne
		}

	}

	if input%2 != 0 {
		fmt.Print("still running")
	} else {
		fmt.Println(seconds)
	}
}
