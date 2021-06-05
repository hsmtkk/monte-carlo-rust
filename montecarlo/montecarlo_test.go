package montecarlo_test

import(
	"testing"
	"github.com/stretchr/testify/assert"
	"github.com/hsmtkk/monte-carlo-rust/montecarlo"
)

func TestMonteCarlo(t *testing.T) {
	pi := montecarlo.New().Calculate(1000)
	assert.True(t, 3 < pi)
	assert.True(t, pi < 4)
}
