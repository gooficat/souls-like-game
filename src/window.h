#ifndef WINDOW_H
#define WINDOW_H

#define WINDOW_WIDTH 1280
#define WINDOW_HEIGHT 720

#include "error.h"

enum return_code window_open(void);
enum return_code window_update(void);
enum return_code window_close(void);

#endif
