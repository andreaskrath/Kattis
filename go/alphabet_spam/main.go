package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	buffer := make([]byte, 150000)
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Buffer(buffer, 150000)
	scanner.Scan()
	s := scanner.Bytes()
	var whiteSpace, lowerCase, upperCase, symbols float32
	for _, v := range s {
		switch {
		case v == 95:
			whiteSpace++
		case v >= 65 && v <= 90:
			upperCase++
		case v >= 97 && v <= 122:
			lowerCase++
		default:
			symbols++
		}
	}
	l := float32(len(s))
	fmt.Println(whiteSpace / l)
	fmt.Println(lowerCase / l)
	fmt.Println(upperCase / l)
	fmt.Println(symbols / l)
}
