#include <iostream>
#include <vector>
#include <unordered_set>
#include <string>
#include <sstream>
#include <numeric>
#include <iterator>

int main(){
    std::vector<std::vector<int>> nums;
    std::string line;
    while (std::getline(std::cin, line)) {
        std::istringstream ss(line);
        nums.emplace_back(std::istream_iterator<int>{ss}, std::istream_iterator<int>{});
        int sum = std::accumulate(nums.begin(), nums.end(), 0);
        std::cout << sum / 2 << std::endl;
    }
}


