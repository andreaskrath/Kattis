package main

import "fmt"

func main() {
	var (
		testCases int
		trips     []int
	)
	fmt.Scan(&testCases)

	for i := 0; i < testCases; i++ {
		trips = append(trips, uniqueTrip())
	}

	for _, v := range trips {
		fmt.Println(v)
	}
}

func uniqueTrip() int {
	var indTrips int
	fmt.Scan(&indTrips)

	cities := make(map[string]int)
	var curCity string
	for i := 0; i < indTrips; i++ {
		fmt.Scan(&curCity)
		cities[curCity]++
	}

	return len(cities)
}
