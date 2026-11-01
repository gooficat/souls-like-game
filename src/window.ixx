module;
#include <cstdint>
export module window;
import math;

export class Window
{
  public:
	Window(std::uint16_t const width, std::uint16_t const height) : size{width, height}
	{
	}

  private:
	Math::Vec<std::uint16_t, 2> size;
};
