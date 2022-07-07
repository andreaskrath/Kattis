package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var cases int
	fmt.Scan(&cases)

	scanner := bufio.NewScanner(os.Stdin)

	var tempStr string
	for i := 0; i < cases; i++ {
		outputStr := fmt.Sprintf("Case #%d: ", i+1)
		scanner.Scan()
		tempStr = scanner.Text()
		for _, v := range tempStr {
			val := t9Value(v)
			if len(outputStr) != 0 && val[0] == outputStr[len(outputStr)-1] {
				outputStr += " " + val
			} else {
				outputStr += val
			}
		}
		fmt.Println(outputStr)
		outputStr = ""
	}
}

func t9Value(s rune) string {
	var val string

	switch s {
	case 'a':
		val = "2"
	case 'b':
		val = "22"
	case 'c':
		val = "222"
	case 'd':
		val = "3"
	case 'e':
		val = "33"
	case 'f':
		val = "333"
	case 'g':
		val = "4"
	case 'h':
		val = "44"
	case 'i':
		val = "444"
	case 'j':
		val = "5"
	case 'k':
		val = "55"
	case 'l':
		val = "555"
	case 'm':
		val = "6"
	case 'n':
		val = "66"
	case 'o':
		val = "666"
	case 'p':
		val = "7"
	case 'q':
		val = "77"
	case 'r':
		val = "777"
	case 's':
		val = "7777"
	case 't':
		val = "8"
	case 'u':
		val = "88"
	case 'v':
		val = "888"
	case 'w':
		val = "9"
	case 'x':
		val = "99"
	case 'y':
		val = "999"
	case 'z':
		val = "9999"
	default:
		val = "0"
	}
	return val
}
