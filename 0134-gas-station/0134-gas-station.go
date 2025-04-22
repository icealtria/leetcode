func canCompleteCircuit(gas []int, cost []int) int {
   	n := len(gas)
	total := 0
	curr := 0
	start := 0
	for i := 0; i < n; i++ {
		total += gas[i] - cost[i]
		curr += gas[i] - cost[i]
		if curr < 0 {
			start = i + 1
			curr = 0
		}
	}
	if total < 0 {
		return -1
	} else {
		return start
	} 
}