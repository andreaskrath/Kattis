package main

import "fmt"

func main() {
	var timeLeave, classStart, transitRoutes int
	fmt.Scanf("%d %d %d", &timeLeave, &classStart, &transitRoutes)

	var tempInt int
	var busStopTime []int
	for i := 0; i < transitRoutes+1; i++ {
		fmt.Scan(&tempInt)
		busStopTime = append(busStopTime, tempInt)
	}

	var busRideTime []int
	for i := 0; i < transitRoutes; i++ {
		fmt.Scan(&tempInt)
		busRideTime = append(busRideTime, tempInt)
	}

	var busIntervalTime []int
	for i := 0; i < transitRoutes; i++ {
		fmt.Scan(&tempInt)
		busIntervalTime = append(busIntervalTime, tempInt)
	}

	totalRideTime := timeLeave
	for i := 0; i < transitRoutes; i++ {
		totalRideTime += busStopTime[i]            // adding walk to bus stop
		if totalRideTime%busIntervalTime[i] != 0 { // checking if there is a wait time or not
			totalRideTime += busIntervalTime[i] - (totalRideTime % busIntervalTime[i]) // adding potential wait at bus stop
		}
		totalRideTime += busRideTime[i]
	}
	totalRideTime += busStopTime[len(busStopTime)-1]

	if totalRideTime > classStart {
		fmt.Println("no")
	} else {
		fmt.Println("yes")
	}
}

// s = time he leaves house
// t = time first class starts
// n = amount of transit routes he takes
// d = time he walks
// i = buss number i
// b = amount of time spent riding bus i
// c = bus intervals (first one leaves at time 0)

// INPUT FORMAT
// s t n
// d d d .. (n+1 amount) - first number is time from house to bus stop 1 (last is from last stop to class)
// b b (n amount) - denoting the amount of time it takes to ride bus i
// c (n amount) - denoting bus travel intervals
