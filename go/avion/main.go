// Avion
// https://open.kattis.com/problems/avion

package main

import (
	"fmt"
	"strings"
)

func main() {
	var curStr, outputStr string

	for i := 0; i < 5; i++ {
		fmt.Scanln(&curStr)
		if strings.Contains(curStr, "FBI") {
			outputStr += fmt.Sprint(i + 1)
			outputStr += " "
		}
	}

	if len(outputStr) == 0 {
		fmt.Println("HE GOT AWAY!")
	} else {
		fmt.Println(outputStr)
	}
}
