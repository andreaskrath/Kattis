package main

import (
	"fmt"
	"sync"
)

type Photo struct {
	time     int
	distance int
}

var wg sync.WaitGroup

func main() {
	var photographAmount int
	fmt.Scan(&photographAmount)

	var photographs []Photo
	var tempPhoto Photo
	for i := 0; i < photographAmount; i++ {
		fmt.Scanf("%d %d", &tempPhoto.time, &tempPhoto.distance)
		photographs = append(photographs, tempPhoto)
	}

	veloCh := make(chan int, photographAmount)
	wg.Add(photographAmount)
	for i := 0; i < photographAmount; i++ {
		go func(index int) {
			defer wg.Done()
			veloCh <- findVelocity(photographs, index)
		}(i)
	}
	wg.Wait()
	close(veloCh)

	var output int
	for v := range veloCh {
		if v > output {
			output = v
		}
	}

	fmt.Println(output)
}

func findVelocity(arr []Photo, startIndex int) int {
	var maxVelocity int
	for i := startIndex + 1; i < len(arr); i++ {
		tempTimeDiff := arr[i].time - arr[startIndex].time
		tempDistDiff := arr[i].distance - arr[startIndex].distance

		velocity := 0
		if tempTimeDiff != 0 {
			velocity = tempDistDiff / tempTimeDiff
		}

		if velocity > maxVelocity {
			maxVelocity = velocity
		}
	}
	return maxVelocity
}
