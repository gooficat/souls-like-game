#ifndef STATE_H
#define STATE_H

#include "error.h"
#include <stdint.h>
#define _Bool uint8_t
#include <stdbool.h>
#include <stdio.h>

#define MAX_SCENES 16
#define TARGET_FPS 30
#define GAME_NAME "My game"

#include <stdbool.h>

enum engine_state
{
	ENGINE_HALTED,
	ENGINE_RUNNING,
	ENGINE_PAUSED,
	ENGINE_CLOSING
};

enum scene_type
{
	SCENE_EMPTY,
	SCENE_MENU,
	SCENE_WORLD
};

struct scene
{
	enum scene_type type;
	bool is_loaded;
	char const * path;
	void * data; /* points to either a menu or a world. should be NULL if unloaded */
};

struct state
{
	enum engine_state state;
	struct scene scenes[MAX_SCENES];
	size_t current_scene;
	double delta_time;
	uint16_t target_fps;
};

extern struct state STATE;

enum return_code state_init(void);
enum return_code state_close(void);

#endif
