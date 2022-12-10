// Quality-Adjusted Life-Year
// https://open.kattis.com/problems/qaly
package main

import "fmt"

func main() {
	var lineInput int
	var quality, years, output float32

	fmt.Scan(&lineInput)
	for i := 0; i < lineInput; i++ {
		fmt.Scanf("%f %f", &quality, &years)
		output += quality * years
	}

	fmt.Println(output)
}
