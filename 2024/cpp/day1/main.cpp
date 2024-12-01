#include <iostream>
#include <vector>
#include <algorithm>
#include <numeric>

using std::vector;
using std::sort;
using std::transform_reduce;
using std::count;

int main() {
	using num_t = int;
	vector<num_t> a;
	vector<num_t> b;

	while(!std::cin.eof()) {
		num_t tmp;

		std::cin >> tmp;
		if(std::cin.fail()) break;
		a.push_back(tmp);
		std::cin >> tmp;
		b.push_back(tmp);
	}

	sort(a.begin(), a.end());
	sort(b.begin(), b.end());

	num_t result1 = transform_reduce(a.cbegin(), a.cend(), b.cbegin(), num_t{0}, std::plus{}, [](num_t a, num_t b) { return abs(a - b); });
	std::cout << "Result part 1: " << result1 << std::endl;

	num_t result2 = transform_reduce(a.cbegin(), a.cend(), num_t{0}, std::plus{}, [&b](num_t a) { return a * count(b.cbegin(), b.cend(), a); });
	std::cout << "Result part 2: " << result2 << std::endl;
}
