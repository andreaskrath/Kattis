package main

import "fmt"

func main() {
	var inputAmount int
	fmt.Scan(&inputAmount)

	for i := 0; i < inputAmount; i++ {
		num, output := 0, 1
		fmt.Scan(&num)
		for j := 1; j <= num; j++ {
			output *= j
		}
		a := fmt.Sprint(output)
		fmt.Println(string(a[len(a)-1]))
	}
}
