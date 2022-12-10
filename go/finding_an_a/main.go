package main

import (
	"fmt"
	"strings"
)

func main() {
	var input string

	fmt.Scanln(&input)

	aIndex := strings.Index(input, "a")

	runedInput := []rune(input)

	fmt.Println(string(runedInput[aIndex:]))
}
