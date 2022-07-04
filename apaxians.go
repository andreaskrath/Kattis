package main

import "fmt"

func main() {
	var input string
	fmt.Scan(&input)

	var output string
	for i := 0; i < len(input); i++ {
		if i < len(input)-1 && input[i] == input[i+1] {
			continue
		} else {
			output += string(input[i])
		}
	}
	fmt.Println(output)
}
