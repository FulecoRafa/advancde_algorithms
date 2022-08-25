package main

import (
  "fmt"
  "container/heap"
  "bufio"
  "os"
  "strings"
)

type Query struct {
  num, period, next_time int
}

func NewQuery(num int, period int) *Query {
  return &Query{num, period, period}
}

// Creates a new query from string of type "Register <id> <period>"
func NewQueryFromString(s string) *Query {
  var id, period int
  // fmt.Printf("New query from: %s\n", s)
  fmt.Sscanf(s, "Register %d %d", &id, &period)
  return NewQuery(id, period)
}

func (q *Query) NextTime() int {
  q.next_time += q.period
  return q.next_time
}

type PQ []*Query

func (pq PQ) Len() int { return len(pq) }

func (pq PQ) Less(i, j int) bool {
  this, other := pq[i], pq[j]
  if this.next_time == other.next_time {
    return this.num < other.num
  }
  return this.next_time < other.next_time
}

func (pq PQ) Swap(i, j int) {
  pq[i], pq[j] = pq[j], pq[i]
}

func (pq *PQ) Push(x interface{}) {
  *pq = append(*pq, x.(*Query))
}

func (pq *PQ) Pop() interface{} {
  old := *pq
  n := len(old)
  x := old[n-1]
  *pq = old[0 : n-1]
  return x
}

func main() {
  pq := make(PQ, 0)
  reader := bufio.NewReader(os.Stdin)
  // Read lines until '#' is read
  for {
    line, err := reader.ReadString('\n')
    if err != nil {
      panic(err)
    }
    if strings.HasPrefix(line, "#") {
      break
    }
    q := NewQueryFromString(line)
    heap.Push(&pq, q)
  }

  // Read number of queries
  var num_queries int
  line, err := reader.ReadString('\n')
  if err != nil {
    panic(err)
  }
  fmt.Sscanf(line, "%d\n", &num_queries)
  for i := 0; i < num_queries; i++ {
    q := heap.Pop(&pq).(*Query)
    fmt.Printf("%d\n", q.num)
    q.NextTime()
    heap.Push(&pq, q)
  }
}
