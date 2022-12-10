#include <array>
#include <fstream>
#include <iostream>

int maybeGetSignalStrength(int cycleN, int registerX)
{
	if (cycleN % 40 == 20) {
		return cycleN * registerX;
	}
	return 0;
}

template<class T, size_t N>
void maybeDrawPixel(std::array<T, N> &display, int cycleN, int registerX)
{
	// confusingly, cycle 1-based, but sprite is 0-based
	int spritePos = cycleN - 1;
	if (((spritePos % 40) >= registerX - 1) && ((spritePos % 40) <= registerX + 1)) {
		display[spritePos] = true;
	}
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

	int registerX = 1;
	int cycleN = 1;

	std::array<bool, 40 * 6> display{};
	int signalStrengthSum = 0;

	std::string line;
	while (std::getline(input, line)) {
		std::string_view line_sv = line;
		int addxVal = 0;
		if (line_sv.substr(0, 4) == "addx") {
			addxVal = std::atoi(line_sv.substr(5).data()); // dirty using string_views in this way, but works for our purpose

			//addx 2 cycles
			maybeDrawPixel(display, cycleN, registerX);
			cycleN++;
			signalStrengthSum += maybeGetSignalStrength(cycleN, registerX);
		}

		maybeDrawPixel(display, cycleN, registerX);
		registerX += addxVal; // only takes effect *after* 2 cycles
		cycleN++;
		signalStrengthSum += maybeGetSignalStrength(cycleN, registerX);
	}

	std::cout << "Sum of signal strengths: " << signalStrengthSum << '\n';
	std::cout << "Display output:\n";

	for (size_t i = 0; i < display.size(); i++) {
		std::cout << (display[i] ? '#' : '.');
		if (i % 40 == 39) std::cout << '\n';
	}
}
