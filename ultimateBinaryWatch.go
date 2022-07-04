package main

import "fmt"

func main() {
	var s string
	fmt.Scan(&s)

	lines := make([]string, 4)
	for i, v := range s {
		switch v {
		case '0':
			lines[0] += "."
			lines[1] += "."
			lines[2] += "."
			lines[3] += "."
		case '1':
			lines[0] += "."
			lines[1] += "."
			lines[2] += "."
			lines[3] += "*"
		case '2':
			lines[0] += "."
			lines[1] += "."
			lines[2] += "*"
			lines[3] += "."
		case '3':
			lines[0] += "."
			lines[1] += "."
			lines[2] += "*"
			lines[3] += "*"
		case '4':
			lines[0] += "."
			lines[1] += "*"
			lines[2] += "."
			lines[3] += "."
		case '5':
			lines[0] += "."
			lines[1] += "*"
			lines[2] += "."
			lines[3] += "*"
		case '6':
			lines[0] += "."
			lines[1] += "*"
			lines[2] += "*"
			lines[3] += "."
		case '7':
			lines[0] += "."
			lines[1] += "*"
			lines[2] += "*"
			lines[3] += "*"
		case '8':
			lines[0] += "*"
			lines[1] += "."
			lines[2] += "."
			lines[3] += "."
		case '9':
			lines[0] += "*"
			lines[1] += "."
			lines[2] += "."
			lines[3] += "*"
		}
		if i == 0 {
			lines[0] += " "
			lines[1] += " "
			lines[2] += " "
			lines[3] += " "
		}
		if i == 1 {
			lines[0] += "   "
			lines[1] += "   "
			lines[2] += "   "
			lines[3] += "   "
		}
		if i == 2 {
			lines[0] += " "
			lines[1] += " "
			lines[2] += " "
			lines[3] += " "
		}
	}
	for _, v := range lines {
		fmt.Println(v)
	}
}
