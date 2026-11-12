#ifndef WORLD_H
#define WORLD_H

#include "entity.h"
#include <stdint.h>

struct map_mesh
{
	void * placeholder;
};

struct map_coll
{
	/* a triangle consists of 3 points in 3 dimensional space */
	float (*tris)[9];
};

struct world
{
	/* entities */
	struct entity entities[MAX_ENTITIES];
	struct map
	{
		/* static mesh */
		struct map_mesh mesh;
		/* collider */
		struct map_coll coll;
	} map;
	uint_fast8_t time; /* time of day */
					   /* player entity id */
};

#endif
