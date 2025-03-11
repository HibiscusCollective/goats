package testdata

import "time"

type Person struct {
	Name     string    `goats:"name"`
	Age      int       `goats:"age"`
	Birthday time.Time `goats:"birthday"`
}
