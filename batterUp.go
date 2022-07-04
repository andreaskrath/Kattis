package main

import "fmt"

func main() {
	var input int
	fmt.Scan(&input)

	var battings []int
	var temp int
	for i := 0; i < input; i++ {
		fmt.Scan(&temp)
		battings = append(battings, temp)
	}

	var counter, bases float32
	for _, v := range battings {
		if v != -1 {
			bases += float32(v)
			counter++
		}
	}

	fmt.Println(bases / counter)
}
