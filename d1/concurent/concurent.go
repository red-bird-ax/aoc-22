package concurent

import (
    "fmt"
    "io"
    "log"
    "os"
    "strconv"
    "strings"

    "github.com/red-bird-ax/aoc22-d1/cache"
)

const BufferSize = 1024

type Result struct {
    PartOne int64
    PartTwo int64
}

type Communication struct {
    Errors   chan error
    Result   chan Result
    Chunks   chan string
    Lines    chan string
}

func Run(path string) {
    comm := Communication{
        Errors:   make(chan error),
        Result:   make(chan Result),
        Chunks:   make(chan string, 8),
        Lines:    make(chan string, 8),
    }

    go readFile(path, &comm)
    go splitLines(&comm)
    go calculateCalories(&comm)

    select {
    case err := <-comm.Errors :
        log.Fatal(err)

    case result := <-comm.Result:
        fmt.Printf("Max calories: %d, top three sum: %d\n", result.PartOne, result.PartTwo)
    }
}

func readFile(path string, comm *Communication) {
    defer close(comm.Chunks)

    file, err := os.Open(path)
    if err != nil {
        comm.Errors <- err
        return
    }

    var (
        buffer [BufferSize]byte
        offset int64
    )

    for {
        lenght, err := file.ReadAt(buffer[:], offset)
        if lenght > 0 {
            comm.Chunks <- string(buffer[:lenght])
            offset += int64(lenght)
        }

        if err != nil {
            if err != io.EOF {
                comm.Errors <- err
            }
            return
        }
    }
}

func splitLines(comm *Communication) {
    defer close(comm.Lines)

    var prefix string

    ChunksLoop:
    for chunk := range comm.Chunks {
        offset := 0
        for {
            index := offset + strings.IndexRune(chunk[offset:], '\n')
            if offset > index {
                prefix = chunk[offset:]
                continue ChunksLoop
            } else {
                if prefix == "" {
                    comm.Lines <- chunk[offset:index]
                } else {
                    line := prefix + chunk[offset:index]
                    prefix = ""
                    comm.Lines <- line
                }
                offset = index + 1
            }
        }
    }
}

func calculateCalories(comm *Communication) {
    topThree := cache.New[int64](3)
    var sum, max int64

    for line := range comm.Lines {
        if line == "" {
            cache.TryPush(topThree, sum)
            if sum > max {
                max = sum
            }
            sum = 0
        } else {
            calories, err := strconv.ParseInt(line, 10, 64)
            if err != nil {
                comm.Errors <- err
                return
            }
            sum += calories
        }
    }

    sum = 0
    for _, element := range cache.GetElements(topThree) {
        sum += element
    }

    comm.Result <- Result{
        PartOne: max,
        PartTwo: sum,
    }
}