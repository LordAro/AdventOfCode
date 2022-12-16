#include <fstream>
#include <iostream>
#include <map>
#include <vector>

struct Valve {
	std::string name;
	int flow_rate;
	std::vector<std::string> links;
};

// debugging
std::ostream &operator<<(std::ostream &os, const Valve &v)
{
	os << v.name << " (" << v.flow_rate << ") - ";
	for (const auto &l : v.links) os << l << ", ";
	return os;
}

std::vector<std::string> split_by(const std::string &s, std::string_view delim)
{
	std::vector<std::string> res;

	size_t last = 0;
	size_t next = 0;
	while ((next = s.find(delim, last)) != std::string::npos) {
		res.push_back(s.substr(last, next - last));
		last = next + delim.size();
	}
	{
		res.push_back(s.substr(last));
	}
	return res;
}

std::istream &operator>>(std::istream &is, Valve &valve)
{
	std::string s;
	is >> s; // Valve
	is >> valve.name;
	is >> s >> s; // has flow
	is.ignore(6); // ' rate='
	is >> valve.flow_rate;
	is.ignore(2); // '; '
	is >> s >> s >> s >> s; // tunnels lead to valve(s)
	is.ignore(1); // ' '
	std::string links;
	std::getline(is, links);
	valve.links = split_by(links, ", ");
	return is;
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

	std::map<std::string, Valve> valves;
	for (Valve v; input >> v; ) {
		valves[v.name] = v;
		std::cout << v << '\n';
	}
}
