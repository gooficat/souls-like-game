#include "log.h"
#include <stdarg.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>


static FILE *log_file;
static bool LOGGING_TO_STDOUT;

static char const * const LOG_LEVEL[] = {
	"NOTICE",
	"WARNING",
	"ERROR"
};

void logger_init(char const * log_file_path) {
	if (NULL == log_file_path) {
		goto log_to_stdout;
	}
	log_file = fopen(log_file_path, "at");
	if (!log_file) {
log_to_stdout:;
		log_file = stdout;
		logger_log(LOG_WARNING, "Failed to open logger file, routing logs to `stdout`");
	}
}

void logger_log(enum log_level level, char const * msg) {
	time_t current_time;
	char const * asc_time;
	char * asc_time_mut;
	size_t asc_time_len;
	current_time = time(NULL);
	asc_time = asctime(gmtime(&current_time));
	asc_time_len = strlen(asc_time);
	asc_time_mut = malloc(asc_time_len + 1);
	memcpy(asc_time_mut, asc_time, asc_time_len);
	asc_time_mut[asc_time_len - 1] = '\0';
	fprintf(log_file, "[%s]\n[%s]:\n\t%s\n\n", LOG_LEVEL[(size_t)level], asc_time_mut, msg);
	free(asc_time_mut);
}

void logger_close(void) {
	if (log_file && false == LOGGING_TO_STDOUT) {
		fclose(log_file);
	}
}
