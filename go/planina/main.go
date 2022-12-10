// Planina
// https://open.kattis.com/problems/planina

package main

import "fmt"

func main() {
	var a int
	output := 2

	fmt.Scan(&a)

	for i := 0; i < a; i++ {
		output += (output - 1)
	}

	fmt.Println(output * output)

}
