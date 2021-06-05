package main

import(
	"fmt"
	"log"
	"os"
	"strconv"
	"github.com/hsmtkk/monte-carlo-rust/montecarlo"
)

func main(){
	if len(os.Args) != 2 {
		log.Fatal("Usage: monte-carlo-rust count")
	}
	count, err := strconv.Atoi(os.Args[1])
	if err != nil {
		log.Fatalf("failed to parse as int; %s; %w", os.Args[1], err)
	}
	val := montecarlo.New().Calculate(count)
	fmt.Println(val)
}