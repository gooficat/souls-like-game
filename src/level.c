#include "level.h"
#include "error.h"
#include <stdio.h>

enum return_code load_level(struct world * world, char const * path)
{
	FILE * file;
	if (!path || !world)
	{
		return RETURN_CODE_NULL_POINTER_PASSED;
	}
	file = fopen(path, "rt");
	if (!file)
	{
		return RETURN_CODE_SUB_CALL_FAILED;
	}
	return RETURN_CODE_OK;
}
