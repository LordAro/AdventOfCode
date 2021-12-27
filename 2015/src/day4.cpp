#include <openssl/md5.h>

#include <iostream>
#include <fstream>

std::string bytetostr(unsigned char array[MD5_DIGEST_LENGTH])
{
	char hexstr[MD5_DIGEST_LENGTH * 2 + 1];
	for (int i = 0; i < MD5_DIGEST_LENGTH; i++) {
		sprintf(hexstr + i * 2, "%02x", array[i]);
	}
	hexstr[MD5_DIGEST_LENGTH * 2] = 0;
	return std::string(hexstr);
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

	std::string secret;
	std::getline(input, secret);

	bool found5 = false;
	for (int i = 1; ; i++) {
		std::string newstr = secret + std::to_string(i);

		unsigned char result[MD5_DIGEST_LENGTH];
		MD5((const unsigned char *)newstr.c_str(), newstr.size(), result);

		std::string hexstr = bytetostr(result);

		if (!found5 && result[0] == 0 && result[1] == 0 && result[2] >> 4 == 0) {
			std::cout << newstr << ": " << bytetostr(result) << "\n";
			found5 = true;
		}
		if (result[0] == 0 && result[1] == 0 && result[2] == 0) {
			std::cout << newstr << ": " << bytetostr(result) << "\n";
			return 0;
		}

	}

	return 0;
}
