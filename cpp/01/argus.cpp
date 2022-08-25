#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <queue>

struct Query {
  int num;
  int period;
  int next_time;

  Query(int num, int period, int next_time) : num(num), period(period), next_time(next_time) {}

  /*
   * Creates a new query object from a string of type "Register <id> <period>"
   */
  Query(const std::string& s) {
    std::istringstream iss(s);
    std::string ignore;
    iss >> ignore >> num >> period;
    next_time = period;
  }

  void increase_time() {
    next_time += period;
  }

  // Overload > operator for priority queue
  bool operator>(const Query& other) const {
    if (next_time == other.next_time) {
      return num > other.num;
    }
    return next_time > other.next_time;
  }
};

int main (void) {
  // Store Queries in a priority queue to keep them in order
  // The priority is the period of the query, in ascending order
  std::priority_queue<Query, std::vector<Query>, std::greater<Query>> queries;
  // Read lines from stdin
  std::string line;
  while (std::getline(std::cin, line)) {
    // Exit condition
    if (line[0] == '#') {
      break;
    }
    // Parse line
    Query q(line);
    queries.push(q);
  }

  int n_queries;
  std::cin >> n_queries;

  for (int i = 0; i < n_queries; i++) {
    // Get next query
    Query q = queries.top();
    queries.pop();
    // Print query number
    std::cout << q.num << std::endl;
    // Add query to queue plus period
    q.increase_time();
    queries.push(q);
  }

  return 0;
}
