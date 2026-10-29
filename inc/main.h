#ifndef MAIN_H
#define MAIN_H

#include <stdio.h>

enum game_state_state {
	GAME_STATE_MENU,
	GAME_STATE_RUNNING,
	GAME_STATE_PAUSED,
	GAME_STATE_QUITTING
};

struct game_state {
	enum game_state_state state;
	double frame_start;
	unsigned short tick_rate;
};

void game_init(void);
void game_run(void);
void game_close(void);

void game_start_frame(void);
void game_end_frame(void);

extern struct game_state STATE;

#endif
