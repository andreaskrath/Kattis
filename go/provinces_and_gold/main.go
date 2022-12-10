// Provinces and Gold
// https://open.kattis.com/problems/provincesandgold

package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Split(bufio.ScanWords)

	scanner.Scan()
	gold := 3 * uint8(toInt(scanner.Bytes())) // gold worth 3
	scanner.Scan()
	silver := 2 * uint8(toInt(scanner.Bytes())) // silver worth 2
	scanner.Scan()
	copper := uint8(toInt(scanner.Bytes()))

	handVal := gold + silver + copper
	var output string

	switch { // victory cards
	case handVal >= 8:
		output += "Province"
	case handVal >= 5:
		output += "Duchy"
	case handVal >= 2:
		output += "Estate"
	}

	if len(output) != 0 {
		output += " or "
	}

	switch { // treasure cards
	case handVal >= 6:
		output += "Gold"
	case handVal >= 3:
		output += "Silver"
	default:
		output += "Copper"
	}

	fmt.Println(output)
}

func toInt(buf []byte) int {
	var n int
	for _, v := range buf {
		n = int(v - '0')
	}
	return n
}
