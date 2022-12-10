package main

import "fmt"

func main() {
	var mbPlan, knownMonths, mbUsage, totalUsage int

	fmt.Scan(&mbPlan)
	fmt.Scan(&knownMonths)

	for i := 0; i < knownMonths; i++ {
		fmt.Scan(&mbUsage)
		totalUsage += mbUsage
	}

	fmt.Println(mbPlan*(knownMonths+1) - totalUsage)
}
