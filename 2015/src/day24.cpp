#include <algorithm>
#include <array>
#include <cassert>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <iterator>
#include <numeric>
#include <set>
#include <unordered_set>
#include <vector>

uint64_t get_quantum_entanglement(const std::vector<int> &group)
{
	return std::accumulate(group.begin(), group.end(), 1ULL, std::multiplies<uint64_t>());
}

int get_group_weight(const std::vector<int> &group)
{
	return std::accumulate(group.begin(), group.end(), 0);
}

// recursively finds all elements of the powerset that total target (total / N)
void partitionN(const std::vector<int> &input, int target, std::vector<int> &sum, std::set<std::vector<int>> &partitions)
{
	if (target == 0) partitions.insert(sum);
	if (get_group_weight(input) < target) return;

	for (size_t i = 0; i < input.size(); i++) {
		int p = input[i];
		if (p > target) continue;
		std::vector<int> remaining_values(input.begin() + i + 1, input.end());

		auto sum_copy = sum;
		sum_copy.push_back(p);
		partitionN(remaining_values, target - p, sum_copy, partitions);
	}
}

std::array<std::vector<int>, 3> get_minimal_group3(const std::vector<std::vector<int>> &partitions, size_t num_inputs)
{
	for (auto i_it = partitions.cbegin(); i_it != partitions.cend(); ++i_it) {
		for (auto j_it = std::next(i_it); j_it != partitions.cend(); ++j_it) {
			if (std::find_first_of(j_it->begin(), j_it->end(), i_it->begin(), i_it->end()) != j_it->end()) continue;

			for (auto k_it = std::next(j_it); k_it != partitions.cend(); ++k_it) {
				if (i_it->size() + j_it->size() + k_it->size() != num_inputs) continue;

				if (std::find_first_of(k_it->begin(), k_it->end(), i_it->begin(), i_it->end()) != k_it->end()) continue;
				if (std::find_first_of(k_it->begin(), k_it->end(), j_it->begin(), j_it->end()) != k_it->end()) continue;

				std::unordered_set<int> s;
				s.insert(i_it->begin(), i_it->end());
				s.insert(j_it->begin(), j_it->end());
				s.insert(k_it->begin(), k_it->end());
				if (s.size() != num_inputs) continue;

				return {*i_it, *j_it, *k_it};
			}
		}
	}
	assert(false);
	return {}; // unreachable?
}

// TODO: Figure out if there's a decent way of doing this without (essentially) duplication
std::array<std::vector<int>, 4> get_minimal_group4(const std::vector<std::vector<int>> &partitions, size_t num_inputs)
{
	// all possible N-permutations
	// skip if the partition contains any numbers in the previous partitions
	for (auto i_it = partitions.cbegin(); i_it != partitions.cend(); ++i_it) {
		for (auto j_it = std::next(i_it); j_it != partitions.cend(); ++j_it) {
			if (std::find_first_of(j_it->begin(), j_it->end(), i_it->begin(), i_it->end()) != j_it->end()) continue;

			for (auto k_it = std::next(j_it); k_it != partitions.cend(); ++k_it) {
				if (std::find_first_of(k_it->begin(), k_it->end(), i_it->begin(), i_it->end()) != k_it->end()) continue;
				if (std::find_first_of(k_it->begin(), k_it->end(), j_it->begin(), j_it->end()) != k_it->end()) continue;

				for (auto l_it = std::next(k_it); l_it != partitions.cend(); ++l_it) {
					// cheaper than find_first_of
					if (i_it->size() + j_it->size() + k_it->size() + l_it->size() != num_inputs) continue;

					if (std::find_first_of(l_it->begin(), l_it->end(), i_it->begin(), i_it->end()) != l_it->end()) continue;
					if (std::find_first_of(l_it->begin(), l_it->end(), j_it->begin(), j_it->end()) != l_it->end()) continue;
					if (std::find_first_of(l_it->begin(), l_it->end(), k_it->begin(), k_it->end()) != l_it->end()) continue;

					std::unordered_set<int> s;
					s.insert(i_it->begin(), i_it->end());
					s.insert(j_it->begin(), j_it->end());
					s.insert(k_it->begin(), k_it->end());
					s.insert(l_it->begin(), l_it->end());
					if (s.size() != num_inputs) continue;

					return {*i_it, *j_it, *k_it, *l_it};
				}
			}
		}
	}
	assert(false);
	return {}; // unreachable?
}

template<int N>
std::vector<std::vector<int>> get_sorted_partitionsN(const std::vector<int> &inputs)
{
	int total_weight = get_group_weight(inputs);
	assert(total_weight % N == 0);

	// gets each individual partition
	std::set<std::vector<int>> partitions;
	std::vector<int> sum;
	partitionN(inputs, total_weight / N, sum, partitions);

	// sorting the partitions this way means we can just pick the first result
	std::vector<std::vector<int>> sorted_partitions(partitions.begin(), partitions.end());
	std::sort(
		sorted_partitions.begin(), sorted_partitions.end(),
		[](const std::vector<int> &a, const std::vector<int> &b) {
			return (a.size() != b.size()) ? a.size() < b.size() : get_quantum_entanglement(a) < get_quantum_entanglement(b);
		}
	);

	return sorted_partitions;
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

	std::vector<int> packages;

	std::string line;
	while (input >> line) {
		packages.push_back(std::stoi(line));
	}

	//packages = {1,2,3,4,5,7,8,9,10,11};

	// mildly faster at finding partitions if we sort in reverse order
	std::sort(packages.begin(), packages.end(), std::greater<int>());

	// p1
	auto sorted_partitions3 = get_sorted_partitionsN<3>(packages);
	std::cout << "3: " << sorted_partitions3.size() << '\n';
	auto balanced_partition3 = get_minimal_group3(sorted_partitions3, packages.size());

	std::cout << "Ideal package configuration (3-group): ";
	for (const auto &p : balanced_partition3) {
		for (const auto &e : p) {
			std::cout << e << ' ';
		}
		std::cout << ", ";
	}
	std::cout << "entanglement: " << get_quantum_entanglement(balanced_partition3.front()) << '\n';

	// p2
	auto sorted_partitions4 = get_sorted_partitionsN<4>(packages);
	std::cout << "4: " << sorted_partitions4.size() << '\n';
	auto balanced_partition4 = get_minimal_group4(sorted_partitions4, packages.size());

	std::cout << "Ideal package configuration (4-group): ";
	for (const auto &p : balanced_partition4) {
		for (const auto &e : p) {
			std::cout << e << ' ';
		}
		std::cout << ", ";
	}
	std::cout << "entanglement: " << get_quantum_entanglement(balanced_partition4.front()) << '\n';

	return 0;
}
