#include "state.h"
#include "error.h"
#include <string.h>

struct state STATE = {ENGINE_HALTED, {{SCENE_EMPTY, false, NULL, NULL}}, 0, 0.0, 0};

enum return_code state_init(void)
{
	if (STATE.state != ENGINE_HALTED)
	{
		return RETURN_CODE_ALREADY_EXISTS;
	}
	STATE.state = ENGINE_RUNNING;
	STATE.target_fps = TARGET_FPS;
	return RETURN_CODE_OK;
}

enum return_code state_close(void)
{
	if (STATE.state == ENGINE_HALTED)
	{
		return RETURN_CODE_ALREADY_EXISTS;
	}
	/* TODO save if needed */
	memset(&STATE, 0, sizeof STATE);
	return RETURN_CODE_OK;
}
