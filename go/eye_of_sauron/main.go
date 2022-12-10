// Eye of Sauron
// https://open.kattis.com/problems/eyeofsauron

package main

import (
	"fmt"
	"strings"
)

func main() {
	var inputString string

	fmt.Scan(&inputString)

	splitInput := strings.SplitAfter(inputString, "(")

	if strings.Count(splitInput[0], "|") == strings.Count(splitInput[1], "|") {
		fmt.Println("correct")
	} else {
		fmt.Println("fix")
	}
}
