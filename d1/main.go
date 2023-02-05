package main

import (
    "github.com/red-bird-ax/aoc22-d1/concurent"
    "github.com/red-bird-ax/aoc22-d1/naive"
)

const DatasetPath = "./dataset.txt"

func main() {
    naive.Run(DatasetPath)
    concurent.Run(DatasetPath)
}