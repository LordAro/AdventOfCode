#include <stdio.h>
#include <stdbool.h>
#include <inttypes.h>

int main(int argc, char **argv)
{
	if (argc < 2) {
		fprintf(stderr, "Incorrect number of arguments provided\n");
		return 1;
	}
	FILE *f = fopen(argv[1], "r");
	if (f == NULL) {
		fprintf(stderr, "Could not open input file\n");
		return 0;
	}

	uint32_t start, end;
	fscanf(f, "%u-%u", &start, &end);
	fclose(f);

	uint32_t p1_count = 0;
	uint32_t p2_count = 0;
	for (uint32_t n = start; n <= end; n++) {
		uint32_t ds[6] = {
			n / 100000,
			(n / 10000) % 10,
			(n / 1000) % 10,
			(n / 100) % 10,
			(n / 10) % 10,
			n % 10,
		};
		if (ds[0] <= ds[1] && ds[1] <= ds[2] && ds[2] <= ds[3] && ds[3] <= ds[4] && ds[4] <= ds[5]) {
			if (ds[0] == ds[1] || ds[1] == ds[2] || ds[2] == ds[3] || ds[3] == ds[4] || ds[4] == ds[5]) {
				p1_count++;
				if ((ds[0] == ds[1] && ds[1] != ds[2])
						|| (ds[0] != ds[1] && ds[1] == ds[2] && ds[2] != ds[3])
						|| (ds[1] != ds[2] && ds[2] == ds[3] && ds[3] != ds[4])
						|| (ds[2] != ds[3] && ds[3] == ds[4] && ds[4] != ds[5])
						|| (ds[3] != ds[4] && ds[4] == ds[5])) {
					p2_count++;
				}
			}
		}
	}
    printf("Number of possible passwords: %u\n", p1_count);
    printf("Number of possible passwords (part 2): %u\n", p2_count);
	return 0;
}
