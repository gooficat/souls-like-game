module;
export module render;
import window;

export class Renderer
{
  public:
	Renderer(Window &window) : window(window)
	{
	}

  private:
	Window &window;
};
