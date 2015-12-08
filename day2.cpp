#include <iostream>
#include <fstream>
#include <tuple>
#include <algorithm>

int surface_area(int l, int w, int h)
{
	return 2*l*w + 2*w*h + 2*h*l;
}

int slack_area(int l, int w, int h)
{
	int a = l*w;
	int b = w*h;
	int c = h*l;

	return std::min({a, b, c});
}

std::tuple<int, int, int> get_sides(const std::string &line)
{
	size_t first = line.find_first_of('x');
	size_t last = line.find_last_of('x');
	int a = std::stoi(line.substr(0, first));
	int b = std::stoi(line.substr(first+1, last));
	int c = std::stoi(line.substr(last+1, line.size()));
	return std::make_tuple(a, b, c);
}

int shortest_perimeter(int l, int w, int h)
{
	int a = l+l+w+w;
	int b = w+w+h+h;
	int c = l+l+h+h;

	return std::min({a, b, c});
}

int volume(int l, int w, int h)
{
	return l*w*h;
}

int main(int argc, char **argv)
{
	if (argc != 2) {
		std::cerr << "Incorrect number of arguments provided\n";
		return 1;
	}
	std::fstream input(argv[1]);
	if (!input) {
		std::cerr << "Could not open input file\n";
		return 1;
	}

	int total_area = 0;
	int total_ribbon = 0;

	std::string line;
	while (input >> line) {
		int l, w, h;
		std::tie(l, w, h) = get_sides(line);

		int area = surface_area(l, w, h) + slack_area(l, w, h);
		total_area += area;
		int ribbon = shortest_perimeter(l, w, h) + volume(l, w, h);
		total_ribbon += ribbon;
	}

	std::cout << "Total square feet of wrapping paper: " << total_area << "\n";
	std::cout << "Total length of ribbon: " << total_ribbon << "\n";
	return 0;
}
