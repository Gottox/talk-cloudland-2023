package main

import (
	"fmt"
	"sort"
)

type Actor struct {
	Name        string
	YearOfBirth int
}

func (a Actor) to_string() string {
	return fmt.Sprintf("%s (%d)", a.Name, a.YearOfBirth)
}

func main() {
	println("Incomplete Movie Database (IMDB)")
	println("-------------------")

	actors := []Actor{
		{Name: "Tilda Swinton", YearOfBirth: 1960},
		{Name: "Tom Hanks", YearOfBirth: 1956},
		{Name: "Helena Bonham Carter", YearOfBirth: 1966},
		{Name: "Sandra Bullock", YearOfBirth: 1964},
		{Name: "Leonardo DiCaprio", YearOfBirth: 1974},
		{Name: "Tom Hiddleston", YearOfBirth: 1981},
		{Name: "Scarlett Johansson", YearOfBirth: 1984},
		{Name: "Liam Neeson", YearOfBirth: 1952},
		{Name: "Johnny Depp", YearOfBirth: 1963},
	}

	tilda_swinton := &actors[0]
	for _, actor := range actors {
		println(actor.to_string())
	}
	println("-------------------")
	println("First actor:", tilda_swinton.to_string())
	println("-------------------")
	println("... Sorting ...")
	sort.Slice(actors, func(i, j int) bool {
		return actors[i].YearOfBirth < actors[j].YearOfBirth
	})
	println("-------------------")

	for _, actor := range actors {
		println(actor.to_string())
	}
}
