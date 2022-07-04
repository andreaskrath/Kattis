package main

import "fmt"

func main() {
	var n int
	table := make(map[int]uint8)
	for i := 0; i < 10; i++ {
		fmt.Scan(&n)
		_, exists := table[n%42]
		if exists {
			continue
		} else {
			table[n%42]++
		}
	}
	fmt.Println(len(table))
}
