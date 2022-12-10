package main

import "fmt"

func main() {
	var inputAmount int
	var beatAmount, secondAmount, tempFloat float32
	var minRate, heartRate, maxRate []float32

	fmt.Scan(&inputAmount)

	for i := 0; i < inputAmount; i++ {
		fmt.Scanf("%f %f", &beatAmount, &secondAmount)
		tempFloat = (60 * beatAmount) / secondAmount
		heartRate = append(heartRate, tempFloat)
		minRate = append(minRate, (tempFloat - (1/beatAmount)*tempFloat))
		maxRate = append(maxRate, (tempFloat + (1/beatAmount)*tempFloat))
	}

	for i := 0; i < len(heartRate); i++ {
		fmt.Println(minRate[i], heartRate[i], maxRate[i])
	}
}
