package main

func channel(num_threads uint64) {

	ch := make(chan uint64)

	for i := uint64(0); i < num_threads; i++  {
		go func(i uint64) {
			ch <- i
		}(i)
	}

	for i := uint64(0); i < num_threads; i++ {
		<- ch
	}
}

func main() {
	channel(10)
}
