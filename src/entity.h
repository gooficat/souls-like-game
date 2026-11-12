#ifndef ENTITY_H
#define ENTITY_H

#define MAX_ENTITIES 512

#include <stdint.h>

#define ENTITY_MAX_HEALTH (INT16_MAX - 1)

struct transform
{
	float pos[3];
	float rot;
};

struct entity
{
	uint32_t uuid;
	struct transform transform;
	uint16_t health;
};

#endif
