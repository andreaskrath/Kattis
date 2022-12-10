package main

import (
	"fmt"
)

func main() {
	var input float64
	fmt.Scan(&input)
	fmt.Println(int(input*1000.0*(5280.0/4854.0) + 0.5))
}
