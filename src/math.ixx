module;
#include <array>
#include <cstddef>
export module math;

export namespace Math
{
template <typename T, std::size_t N> class Vec
{
  public:
	constexpr Vec operator=(T const val)
	{
		data = {val};
	}
	std::array<T, N> data;
};

}; // namespace Math
