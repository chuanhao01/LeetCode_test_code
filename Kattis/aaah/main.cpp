#include <iostream>
#include <vector>
#include <unordered_set>
#include <string>

int main(){
    std::string a, b;
    std::cin >> a >> b;
    if(a.size() < b.size()){
        std::cout << "no";
    }
    else{
        std::cout << "go";
    }
    return 0;
}


