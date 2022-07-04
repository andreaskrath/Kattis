package main

import "fmt"

func main() {
	var lines int
	fmt.Scan(&lines)
	for {
		speed, time, totalDistance, formerTime := 0, 0, 0, 0
		for i := 0; i < lines; i++ {
			fmt.Scanf("%d %d", &speed, &time)
			totalDistance += speed * (time - formerTime)
			formerTime = time
		}
		fmt.Println(totalDistance, "miles")
		fmt.Scan(&lines)
		if lines == -1 {
			break
		}
	}
}
