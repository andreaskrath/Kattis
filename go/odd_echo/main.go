// Odd Echo
// https://open.kattis.com/problems/oddecho

package main

import "fmt"

func main() {
	var wordCount int
	var tempString string
	var outputSlice []string

	fmt.Scanln(&wordCount)
	for i := 0; i < wordCount; i++ {
		fmt.Scanln(&tempString)

		if i%2 == 0 {
			outputSlice = append(outputSlice, tempString)
		}
	}

	for _, value := range outputSlice {
		fmt.Println(value)
	}
}
