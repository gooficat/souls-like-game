#ifndef STATE_H
#define STATE_H

#define MAX_SCENES 16

#include <stdbool.h>

enum scene_type
{
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
	struct scene scenes[MAX_SCENES];
};

#endif
