// Job Expenses
// https://open.kattis.com/problems/jobexpenses

package main

import "fmt"

func main() {
	var count, tempInt, expenses int

	fmt.Scan(&count)

	for i := 0; i < count; i++ {
		fmt.Scan(&tempInt)

		if tempInt < 0 {
			expenses += tempInt
		}
	}

	expenses *= -1
	fmt.Println(expenses)

}
