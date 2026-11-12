#include "level.h"
#include "error.h"
#include <stdio.h>

int load_level(struct world * world, char const * path)
{
	FILE * file;
	if (!path || !world)
	{
		return ERROR_NULL_POINTER_PASSED;
	}
	file = fopen(path, "rt");
	if (!file)
	{
		return ERROR_SUB_CALL_FAILED;
	}
	return 0;
}
