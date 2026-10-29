#include "window.h"
#include "log.h"
#include "main.h"
#include <GLFW/glfw3.h>

static GLFWwindow *window;

void window_init(int width, int height, char const * name) {
	glfwInit();
	window = glfwCreateWindow(width, height, name, NULL, NULL);
	if (NULL == window) {
		logger_log(LOG_CRITICAL, "Failed to open window!");
		game_close();
	}
	glfwShowWindow(window);
	glfwMakeContextCurrent(window);
}

void window_update(void) {
	glfwPollEvents();
	glfwSwapBuffers(window);
}

void window_close(void) {
	if (NULL != window) {
		glfwDestroyWindow(window);
		glfwTerminate();
	}
}
