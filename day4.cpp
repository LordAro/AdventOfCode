#include <openssl/md5.h>

#include <iostream>
#include <string>

std::string secret = "ckczppom";

std::string bytetostr(unsigned char array[MD5_DIGEST_LENGTH])
{
	char hexstr[MD5_DIGEST_LENGTH * 2 + 1];
	for (int i = 0; i < MD5_DIGEST_LENGTH; i++) {
		sprintf(hexstr + i * 2, "%02x", array[i]);
	}
	hexstr[MD5_DIGEST_LENGTH * 2] = 0;
	return std::string(hexstr);
}

int main()
{
	bool found5 = false;
	for (int i = 1; ; i++) {
		std::string newstr = secret + std::to_string(i);

		unsigned char result[MD5_DIGEST_LENGTH];
		MD5((const unsigned char *)newstr.c_str(), newstr.size(), result);

		std::string hexstr = bytetostr(result);

		if (!found5 && hexstr.substr(0, 5) == "00000") {
			std::cout << newstr << ": " << hexstr << "\n";
			found5 = true;
		}
		if (hexstr.substr(0, 6) == "000000") {
			std::cout << newstr << ": " << hexstr << "\n";
			return 0;
		}

	}

	return 0;
}
