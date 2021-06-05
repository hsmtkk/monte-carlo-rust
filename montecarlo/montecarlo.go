package montecarlo

import(
	"math/rand"
)

type Calculator interface {
	Calculate(count int) float64
}

type calculatorImpl struct {}

func New() Calculator {
	return &calculatorImpl{}
}

func (c *calculatorImpl) Calculate(count int) float64 {
	lt := 0
	for i:=0 ; i<count ; i++{
		x := rand.Float64()
		y := rand.Float64()
		z := x * x + y * y
		if z < 1.0 {
			lt++
		}
	}
	return 4.0 * float64(lt) / float64(count)
}