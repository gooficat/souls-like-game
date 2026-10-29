#include "main.h"
#include "log.h"
#include "window.h"

struct game_state STATE;


/* Could possibly move glfw to a comms category as it is used by game, renderer, and window */
void game_init(void) {
	STATE.state = GAME_STATE_MENU;
}

void game_run(void) {
	STATE.state = GAME_STATE_RUNNING;
	while (STATE.state != GAME_STATE_QUITTING) {
		window_update();
	}
}

void game_close(void) {
	STATE.state = GAME_STATE_QUITTING;
	window_close();
	logger_close();
}

void game_start_frame(void) {
	/* STATE.frame_start = */
}

void game_end_frame(void) {

}

int main(int argc, char **argv)
{
	(void)argc;
	(void)argv;
	game_init();
	game_run();
	game_close();
	return 0;
}
