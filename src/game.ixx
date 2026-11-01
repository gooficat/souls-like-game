module;
export module game;
import window;
import input;
import render;

export class Game
{
  public:
	Game() : window(Window(640, 480)), renderer(Renderer(window)), input_handler(InputHandler())
	{
	}

  private:
	Window window;
	Renderer renderer;
	InputHandler input_handler;
};
