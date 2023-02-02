#pragma once

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>

#define LINE_LENGHT 4
#define BUFFER_SIZE LINE_LENGHT * 10

typedef struct
{
	char Opponent;
	char Player;
} Round;

struct GuideIterator;

struct GuideIterator* NewIterator(const char* path);

void DestroyIterator(struct GuideIterator* it);

Round Next(struct GuideIterator* it);

bool IsValidRound(Round round);