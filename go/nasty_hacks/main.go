// Nasty Hands
// https://open.kattis.com/problems/nastyhacks

package main

import "fmt"

func main() {
	var inputAmount, revNoAd, revWithAd, costOfAd, revDiff int
	var outputSlice []string

	fmt.Scan(&inputAmount)

	for i := 0; i < inputAmount; i++ {
		fmt.Scanf("%d %d %d", &revNoAd, &revWithAd, &costOfAd)

		revDiff = revWithAd - revNoAd

		if revDiff > costOfAd {
			outputSlice = append(outputSlice, "advertise")
		} else if revDiff < costOfAd {
			outputSlice = append(outputSlice, "do not advertise")
		} else {
			outputSlice = append(outputSlice, "does not matter")
		}
	}

	for _, v := range outputSlice {
		fmt.Println(v)
	}
}
