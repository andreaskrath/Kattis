package main

import (
	"fmt"
	"strings"
)

func main() {
	var input string

	fmt.Scan(&input)

	if strings.Contains(input, "ss") {
		fmt.Println("hiss")
	} else {
		fmt.Println("no hiss")
	}
}
