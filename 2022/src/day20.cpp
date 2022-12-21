#include <algorithm>
#include <fstream>
#include <iostream>
#include <numeric>
#include <vector>

std::pair<std::vector<int64_t>, std::vector<size_t>> unmix_numbers(std::vector<int64_t> mixed_numbers, std::vector<size_t> ordering)
{
	int64_t len = mixed_numbers.size();
	for (int i = 0; i < len; i++) {
		// there must be a better way of doing this...
		size_t current_index = std::distance(ordering.begin(), std::find(ordering.begin(), ordering.end(), i));
		int64_t n = mixed_numbers[current_index];

		if (n > 0) {
			// just rotate everything so that the number we're fiddling with is at the start - saves handling wrap arounds
			// todo: this shouldn't be necessary?
			std::rotate(mixed_numbers.begin(), mixed_numbers.begin() + current_index, mixed_numbers.end());
			std::rotate(ordering.begin(), ordering.begin() + current_index, ordering.end());

			// move forward n => rotate left 1 over an (n + 1)-sized group
			// but we still need to account for wrap arounds - need to act as if n isn't there when wrapping
			int64_t group_size = (n % len) + (n / len) + 1;
			while (group_size > len) group_size = group_size % len + group_size / len;
			std::rotate(mixed_numbers.begin(), mixed_numbers.begin() + 1, mixed_numbers.begin() + group_size);
			std::rotate(ordering.begin(), ordering.begin() + 1, ordering.begin() + group_size);
		} else if (n < 0) {
			// just rotate everything so that the number we're fiddling with is at the start (or, end) - saves handling wrap arounds
			// todo: this shouldn't be necessary?
			std::rotate(mixed_numbers.rbegin(), mixed_numbers.rbegin() + len - current_index - 1, mixed_numbers.rend());
			std::rotate(ordering.rbegin(), ordering.rbegin() + len - current_index - 1, ordering.rend());

			// move backward n => rotate right 1 over an (n + 1)-sized group
			int64_t group_size = ((len - n) % len) + (-n / len) + 1;
			while (group_size > len) group_size = group_size % len + group_size / len;
			std::rotate(mixed_numbers.rbegin(), mixed_numbers.rbegin() + 1, mixed_numbers.rbegin() + group_size);
			std::rotate(ordering.rbegin(), ordering.rbegin() + 1, ordering.rbegin() + group_size);
		}
	}

	return {mixed_numbers, ordering};
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

	std::vector<int64_t> initial_numbers;
	for (int64_t n = 0; input >> n; ) {
		initial_numbers.push_back(n);
	}

//	initial_numbers = {1, 2, -3, 3, -2, 0, 4};

	{
		std::vector<size_t> ordering(initial_numbers.size());
		std::iota(ordering.begin(), ordering.end(), 0);

		auto [unmixed, _] = unmix_numbers(initial_numbers, ordering);
		auto zero_index = std::distance(unmixed.begin(), std::find(unmixed.begin(), unmixed.end(), 0));
		auto zero_plus_1000 = unmixed.begin() + (zero_index + 1000) % unmixed.size();
		auto zero_plus_2000 = unmixed.begin() + (zero_index + 2000) % unmixed.size();
		auto zero_plus_3000 = unmixed.begin() + (zero_index + 3000) % unmixed.size();
		std::cout << "Sum of grove coordinates: " << *zero_plus_1000 + *zero_plus_2000 + *zero_plus_3000 << '\n';
	}

	{
		const int DECRYPTION_KEY = 811589153;
		std::vector<int64_t> multiplied = initial_numbers;
		std::transform(multiplied.cbegin(), multiplied.cend(), multiplied.begin(), [](int64_t i) { return i * DECRYPTION_KEY; });

		std::vector<size_t> ordering(initial_numbers.size());
		std::iota(ordering.begin(), ordering.end(), 0);

		std::vector<int64_t> unmixed = multiplied;
		for (int i = 0; i < 10; i++) {
			auto [unmixed_, ordering_] = unmix_numbers(unmixed, ordering);
			unmixed = std::move(unmixed_);
			ordering = std::move(ordering_);
		}

		auto zero_index = std::distance(unmixed.begin(), std::find(unmixed.begin(), unmixed.end(), 0));
		auto zero_plus_1000 = unmixed.begin() + (zero_index + 1000) % unmixed.size();
		auto zero_plus_2000 = unmixed.begin() + (zero_index + 2000) % unmixed.size();
		auto zero_plus_3000 = unmixed.begin() + (zero_index + 3000) % unmixed.size();
		std::cout << "Sum of grove coordinates after full decryption: " << *zero_plus_1000 + *zero_plus_2000 + *zero_plus_3000 << '\n';
	}
}
