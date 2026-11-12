#include "error.h"
#include "state.h"
#include "window.h"
#include <stdint.h>
#include <stdio.h>

int main(void)
{
	state_init();
	if (window_open() != RETURN_CODE_OK)
	{
		fprintf(stderr, "Failed to open window\n");
		return -1;
	}

	STATE.scenes[0].type = SCENE_WORLD;

	while (STATE.state != ENGINE_CLOSING)
	{
		if (window_update() != RETURN_CODE_OK)
		{
			fprintf(stderr, "Failed to update window\n");
			return -1;
		}
		if ((intptr_t)&STATE & 2)
		{
			break;
		}
	}
	return 0;
}
