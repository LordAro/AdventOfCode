#include <iostream>
#include <fstream>

#include <picojson.h>

int total_nested_object(const picojson::value &v, bool ignore_red)
{
	int total = 0;
	if (v.is<double>()) {
		total = v.get<double>();
	} else if (v.is<picojson::array>()) {
		const picojson::array& a = v.get<picojson::array>();
		for (picojson::array::const_iterator i = a.begin(); i != a.end(); ++i) {
			total += total_nested_object(*i, ignore_red);
		}
	} else if (v.is<picojson::object>()) {
		const picojson::object& o = v.get<picojson::object>();
		for (picojson::object::const_iterator i = o.begin(); i != o.end(); ++i) {
			const picojson::value &val = i->second;
			if (ignore_red && val.is<std::string>() && val.to_str() == "red") {
				total = 0;
				break;
			}
			total += total_nested_object(val, ignore_red);
		}
	}
	return total;
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

	picojson::value v;
	input >> v;
	if (input.fail()) {
		std::cerr << picojson::get_last_error() << "\n";
		return 1;
	}

	int total = total_nested_object(v, false);
	std::cout << "Nested JSON object total: " << total << "\n";
	total = total_nested_object(v, true);
	std::cout << "Nested JSON object total (without 'red'): " << total << "\n";
	return 0;
}
