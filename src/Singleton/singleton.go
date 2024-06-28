package main

import (
	"fmt"
	"sync"
)

type Singleton struct{}

var instance *Singleton
var once sync.Once

func GetInstance() *Singleton {
	once.Do(func() {
		instance = &Singleton{}
	})
	return instance
}

func main() {
	singleton1 := GetInstance()
	singleton2 := GetInstance()

	fmt.Println(singleton1 == singleton2)  // Output: true
}
