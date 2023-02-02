#include "Iterator.h"

#include <stdlib.h>
#include <string.h>

typedef struct GuideIterator
{
	uint8_t Buffer[BUFFER_SIZE];
	FILE* File;

	int LinesCount;
	int CurrentLine;
} GuideIterator;

const Round INVALID_ROUND =
{
	.Opponent = '\0',
	.Player = '\0',
};

GuideIterator* NewIterator(const char* path)
{
	FILE* file = fopen(path, "r");
	if (file == NULL)
	{
		printf("failed to open file: %s\n", strerror(errno));
		return NULL;
	}

	GuideIterator* it = (GuideIterator*)malloc(sizeof(GuideIterator));
	if (it == NULL)
	{
		printf("failed to allocate iterator: %s\n", strerror(errno));
		return NULL;
	}

	memset(it->Buffer, 0, BUFFER_SIZE);
	it->File = file;
	it->CurrentLine = 0;
	it->LinesCount = 0;

	return it;
}

void DestroyIterator(GuideIterator* it)
{
	if (it == NULL)
	{
		printf("null iterator can`t be closed\n");
		return;
	}

	int result = fclose(it->File);
	if (result != 0)
	{
		printf("failed to close file: %s\n", strerror(errno));
		return;
	}

	free(it);
}

bool IsValidRound(Round round)
{
	return round.Opponent != '\0' && round.Player != '\0';
}

int ReadChunk(struct GuideIterator* it)
{
	if (it == NULL)
	{
		printf("can`t read chunk from null iterator\n");
		return 0;
	}

	const int lenght = fread(it->Buffer, sizeof(uint8_t), BUFFER_SIZE, it->File);
	if (ferror(it->File) != 0)
	{
		printf("failed to read file: %s\n", strerror(errno));
		return 0;
	}

	const int linesCount = lenght / LINE_LENGHT;
	return linesCount;
}

Round GetLine(struct GuideIterator* it)
{
	if (it == NULL)
	{
		printf("can`t read line from null iterator\n");
		return INVALID_ROUND;
	}

	const char* linePtr = it->Buffer + (it->CurrentLine * LINE_LENGHT);
	Round round =
	{
		.Opponent = linePtr[0],
		.Player = linePtr[2]
	};

	it->CurrentLine++;
	return round;
}

Round Next(struct GuideIterator* it)
{
	if (it == NULL)
	{
		printf("can`t read from null iterator\n");
		return INVALID_ROUND;
	}

	if (it->CurrentLine == it->LinesCount)
	{
		it->LinesCount = ReadChunk(it);
		it->CurrentLine = 0;

		if (it->LinesCount == 0)
		{
			return INVALID_ROUND;
		}
	}

	const Round round = GetLine(it);
	return round;
}
