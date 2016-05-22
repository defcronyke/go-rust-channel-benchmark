package main

import (
	"testing"
)

func BenchmarkGoChannel10uint64(b *testing.B) {
	for i := 0; i < b.N; i++ {
		channel(10)
	}
}

func BenchmarkGoChannel10000uint64(b *testing.B) {
	for i := 0; i < b.N; i++ {
		channel(10000)
	}
}
