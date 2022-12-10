// Spavanac
// https://open.kattis.com/problems/spavanac

package main

import "fmt"

func main() {
	var hours, minutes int
	fmt.Scanf("%d %d", &hours, &minutes)

	if minutes-45 < 0 {
		minutes = 60 - (-1 * (minutes - 45))
		if hours-1 < 0 {
			hours = 24
		}
		fmt.Println(hours-1, minutes)
	} else {
		fmt.Println(hours, minutes-45)
	}
}
