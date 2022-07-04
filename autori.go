package main

import (
	"fmt"
	"unicode"
)

func main() {
	var input string
	var runedInput []rune

	fmt.Scanln(&input)
	runedInput = []rune(input)

	var output string

	for _, value := range runedInput {
		if unicode.IsUpper(value) == true {
			output += string(value)
		}
	}

	fmt.Println(output)
}
