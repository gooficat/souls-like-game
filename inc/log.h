#ifndef LOG_H
#define LOG_H

#include <stdio.h>

enum log_level {
	LOG_NOTICE,
	LOG_WARNING,
	LOG_CRITICAL
};

void logger_init(char const * log_file_path);
void logger_log(enum log_level level, char const * msg);
void logger_close(void);

#endif
