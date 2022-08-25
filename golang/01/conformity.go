package main

import (
  "fmt"
  "sort"
)


func main() {
  for {
    // Store line number in a variable
    var lines int
    fmt.Scanf("%d\n", &lines)

    if lines == 0 {
      break
    }

    // Map in which key is five int array and value is the number of times the array is found
    var frosh = make(map[[5]int]int)

    for i := 0; i < lines; i++ {
      // Read line with five numbers and store them in array
      var numbers [5]int
      fmt.Scanf("%d %d %d %d %d\n", &numbers[0], &numbers[1], &numbers[2], &numbers[3], &numbers[4])

      // Sort array of integers
      sort.Ints(numbers[:])

      // Increment value in map
      frosh[numbers]++
    }

    // Find the largest value in the map
    largest := 0
    for _, value := range frosh {
      if value > largest {
        largest = value
      }
    }

    // Count number of times the largest value is found in the map
    count := 0
    for _, value := range frosh {
      if value == largest {
        count += value
      }
    }

    // Print number of times the largest value is found
    fmt.Printf("%d\n", count)
  }
}
