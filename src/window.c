#include "window.h"
#include "error.h"
#include "state.h"

/* glfw for now */
#include <GLFW/glfw3.h>

static GLFWwindow * internal_window = NULL;

static void window_close_callback(GLFWwindow * window)
{
	(void)window;
	STATE.state = ENGINE_CLOSING;
}

enum return_code window_open(void)
{
	if (internal_window)
	{
		return RETURN_CODE_ALREADY_EXISTS;
	}
	if (!glfwInit())
	{
		return RETURN_CODE_SUB_CALL_FAILED;
	}

	internal_window = glfwCreateWindow(WINDOW_WIDTH, WINDOW_HEIGHT, GAME_NAME, NULL, NULL);
	if (!internal_window)
	{
		return RETURN_CODE_SUB_CALL_FAILED;
	}
	glfwSetWindowCloseCallback(internal_window, window_close_callback);
	glfwMakeContextCurrent(internal_window);
	glfwShowWindow(internal_window);
	return RETURN_CODE_OK;
}

enum return_code window_update(void)
{
	glfwPollEvents();
	glfwSwapBuffers(internal_window);
	return RETURN_CODE_OK;
}

enum return_code window_close(void)
{
	if (internal_window != NULL)
	{
		return RETURN_CODE_ALREADY_EXISTS;
	}
	glfwDestroyWindow(internal_window);
	internal_window = NULL;
	glfwTerminate();
	return RETURN_CODE_OK;
}
