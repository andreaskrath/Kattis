package main

import "fmt"

func main() {
	monthDays := []int{31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31}
	var day, month int
	fmt.Scanf("%d %d", &day, &month)

	var totalDays int
	for i := 0; i < month-1; i++ { // zero indexing
		totalDays += monthDays[i]
	}
	totalDays += day - 1 // zero indexing

	switch totalDays % 7 {
	case 0:
		fmt.Println("Thursday")
	case 1:
		fmt.Println("Friday")
	case 2:
		fmt.Println("Saturday")
	case 3:
		fmt.Println("Sunday")
	case 4:
		fmt.Println("Monday")
	case 5:
		fmt.Println("Tuesday")
	case 6:
		fmt.Println("Wednesday")
	}
}
