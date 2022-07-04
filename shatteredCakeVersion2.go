package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	var width, pieces int

	fmt.Scan(&width)
	fmt.Scan(&pieces)

	scanner := bufio.NewScanner(os.Stdin)
	scanner.Split(bufio.ScanWords)

	var area int
	for i := 0; i < pieces; i++ {
		num := 1
		scanner.Scan()
		num *= toInt(scanner.Bytes())
		scanner.Scan()
		num *= toInt(scanner.Bytes())
		area += num
	}

	fmt.Println(area / width)
}

func toInt(buf []byte) int {
	var n int
	for _, v := range buf {
		n = int(v - '0')
	}
	return n
}
