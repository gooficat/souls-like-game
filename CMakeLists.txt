cmake_minimum_required(VERSION 4.3)
project(souls-like-game CXX)
set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
file(GLOB_RECURSE SRCS CONFIGURE_DEPENDS
	src/*.cxx)
file(GLOB_RECURSE MODS CONFIGURE_DEPENDS
	src/*.ixx)
add_executable(souls-like-game ${SRCS})
target_sources(souls-like-game
	PRIVATE
		FILE_SET modules TYPE CXX_MODULES
		BASE_DIRS src
		FILES ${MODS}
)
target_include_directories(souls-like-game PRIVATE src)
