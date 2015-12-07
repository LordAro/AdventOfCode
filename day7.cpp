#include <iostream>
#include <fstream>
#include <sstream>
#include <map>
#include <algorithm>

bool is_numeric(const std::string &str)
{
	return std::all_of(str.begin(), str.end(), isdigit);
}

bool is_gate(const std::string &str)
{
	return str == "RSHIFT" || str == "LSHIFT" ||
		str == "OR" || str == "AND" || str == "NOT";
}

bool contains(const std::string &str, const std::string &substr)
{
	return str.find(substr) != std::string::npos;
}

std::map<std::string, std::string> gate_vals;
std::map<std::string, uint16_t> cached;

uint16_t process_gate(const std::string &gate)
{
	std::string operation = gate_vals.at(gate);
	auto search = cached.find(operation);
	if (search != cached.end()) {
		return search->second;
	}
	std::istringstream stream(operation);
	uint16_t processed;
	if (contains(operation, "RSHIFT") || contains(operation, "LSHIFT")) {
		std::string wire1, op;
		uint16_t shift;
		stream >> wire1 >> op >> shift;
		if (op == "RSHIFT") {
			processed = process_gate(wire1) >> shift;
		} else {
			processed = process_gate(wire1) << shift;
		}
	} else if (contains(operation, "AND") || contains(operation, "OR")) {
		std::string wire1str, op, wire2str;
		uint16_t wire1, wire2;
		stream >> wire1str >> op >> wire2str;
		// Sad horribleness
		if (is_numeric(wire1str)) {
			wire1 = static_cast<uint16_t>(std::stoi(wire1str));
		} else {
			wire1 = process_gate(wire1str);
		}
		if (is_numeric(wire2str)) {
			wire2 = static_cast<uint16_t>(std::stoi(wire2str));
		} else {
			wire2 = process_gate(wire2str);
		}
		if (op == "AND") {
			processed = wire1 & wire2;
		} else {
			processed = wire1 | wire2;
		}
	} else if (contains(operation, "NOT")) {
		std::string op, wire;
		stream >> op >> wire;
		processed = ~process_gate(wire);
	} else if (is_numeric(operation)) {
		uint16_t num;
		stream >> num;
		processed = num;
	} else {
		processed = process_gate(operation);
	}
	cached[operation] = processed;
	return processed;
}

int main()
{
	std::fstream input("day7.input");
	if (!input) {
		std::cerr << "Could not open input file\n";
		return 1;
	}

	std::string line;
	while (std::getline(input, line)) {
		size_t delim = line.find(" -> ");
		std::string operation = line.substr(0, delim);
		std::string output = line.substr(delim + 4, line.size() - (delim + 4));
		gate_vals[output] = operation;
	}
	uint16_t result = process_gate("a");
	std::cout << "Result of program on wire a: " << result << "\n";
	cached.clear();
	gate_vals["b"] = std::to_string(result);
	result = process_gate("a");
	std::cout << "Result of program on wire a with new value: " << result << "\n";
	return 0;
}
