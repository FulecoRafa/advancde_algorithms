#include <iostream>
#include <array>
#include <map>
#include <algorithm>

int main (void) {
  while (true) {
    int lines;
    std::cin >> lines;

    // Exit condition
    if (lines == 0) {
      return 0;
    }

    // Stores a line as a key and the value counts as the number of times the line is repeated
    std::map<std::array<int, 5>, int> frosh;

    // For each line
    for (int i = 0; i < lines; i++) {
      // Capture numbers in array for key
      std::array<int, 5> this_line;
      for (int j = 0; j < 5; j++) {
        std::cin >> this_line[j];
      }

      // Sort the array
      std::sort(this_line.begin(), this_line.end());

      // Increment key
      frosh[this_line]++;
    }

    // Find largest value in map
    int largest = 0;
    for (auto const& entry : frosh) {
      int value = entry.second;
      if (value > largest) {
        largest = value;
      }
    }

    // Count number of keys with largest value
    int count = 0;
    for (auto const& entry : frosh) {
      int value = entry.second;
      if (value == largest) {
        count += value;
      }
    }

    std::cout << count << std::endl;
  }
}
