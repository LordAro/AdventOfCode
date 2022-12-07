#include <cassert>
#include <fstream>
#include <iostream>
#include <map>
#include <memory>
#include <sstream>
#include <vector>

struct Directory {
	std::string path;
	Directory *parent;
	std::map<std::string, std::unique_ptr<Directory>> dirs;
	std::map<std::string, size_t> files;
};

size_t get_directory_size(const Directory *dir)
{
	size_t size = 0;
	for (const auto &[name, child] : dir->dirs) size += get_directory_size(child.get());

	for (const auto &[name, file_size] : dir->files) size += file_size;
	return size;
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

	std::vector<Directory *> filesystem;
	auto root = std::make_unique<Directory>();
	Directory *cur_dir = root.get();
	filesystem.push_back(cur_dir);

	std::string line;
	std::getline(input, line); // first line makes sure we're at the root

	while (std::getline(input, line)) {
		if (line[0] == '$') {
			if (line[2] == 'c' && line[3] == 'd') {
				if (line[5] == '.' && line[6] == '.') {
					cur_dir = cur_dir->parent;
				} else {
					cur_dir = cur_dir->dirs.at(line.substr(5)).get();
				}
			} else if (line[2] == 'l' && line[3] == 's') {
				// don't need to do anything here
			} else {
				std::cerr << "Got unexpected command: " << line << '\n';
				assert(false);
			}
		} else {
			if (line.substr(0, 3) == "dir") {
				auto name = line.substr(4);
				auto new_dir = std::make_unique<Directory>();
				new_dir->path = name;
				new_dir->parent = cur_dir;
				filesystem.push_back(new_dir.get());
				cur_dir->dirs[name] = std::move(new_dir);
			} else {
				size_t size; std::string filename;
				std::stringstream ss(line);
				ss >> size >> filename;
				cur_dir->files[filename] = size;
			}
		}
	}

	size_t total_small_size = 0; // p1

	const size_t total_unused = 70'000'000 - get_directory_size(root.get()); // p2
	const size_t deletion_needed = 30'000'000 - total_unused;
	size_t smallest_deletion_candidate = 70'000'000;

	for (const auto &d : filesystem) {
		size_t size = get_directory_size(d);
		if (size < 100'000) {
			total_small_size += size;
		}

		if (size > deletion_needed) {
			smallest_deletion_candidate = std::min(smallest_deletion_candidate, size);
		}
	}

	std::cout << "Total size of small directories: " << total_small_size << '\n';
	std::cout << "Size of smallest deletion candidate: " << smallest_deletion_candidate << '\n';
}
