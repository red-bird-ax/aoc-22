package cache

import "golang.org/x/exp/constraints"

type Cache[T constraints.Ordered] struct {
    buffer []T
    size   int
}

func New[T constraints.Ordered](size int) *Cache[T] {
    return &Cache[T]{
        buffer: make([]T, size, size),
        size:   0,
    }
}

func TryPush[T constraints.Ordered](cache *Cache[T], value T) bool {
    if cache.size < len(cache.buffer) {
        cache.buffer[cache.size] = value
        cache.size++
        return true
    }

    minIdx := 0
    for idx := 0; idx < cache.size; idx++ {
        element := cache.buffer[idx]
        if element < cache.buffer[minIdx] {
            minIdx = idx
        }
    }

    if value > cache.buffer[minIdx] {
        cache.buffer[minIdx] = value
        return true
    }

    return false
}

func GetElements[T constraints.Ordered](cache *Cache[T]) []T {
    elements := make([]T, len(cache.buffer))
    copy(elements, cache.buffer)
    return elements
}