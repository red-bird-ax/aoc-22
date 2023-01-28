package naive

import (
    "fmt"
    "io"
    "log"
    "os"
    "strconv"
    "strings"
)

func Run(path string) {
    dataset, err := readDataset(path)
    if err != nil {
        log.Fatal("failed to read dataset: ", err)
    }

    var (
        maxCalories int64
        sumCalories int64

        offset int
    )

    for offset < len(dataset)  {
        index := offset + strings.Index(dataset[offset:], "\n")
        line := dataset[offset:index]

        if len(line) == 0 {
            if sumCalories > maxCalories {
                maxCalories = sumCalories
            }
            sumCalories = 0
        } else {
            calories, err := strconv.ParseInt(line, 10, 32)
            if err != nil {
                log.Fatal("failed to parse calories: ", err)
            }
            sumCalories += calories
        }

        offset = index + 1
    }

    fmt.Println("Max calories:", maxCalories)
}

func readDataset(path string) (string, error) {
    file, err := os.Open(path)
    if err != nil {
        return "", err
    }

    var builder strings.Builder
    if _, err = io.Copy(&builder, file); err != nil {
        return "", err
    }

    return builder.String(), nil
}