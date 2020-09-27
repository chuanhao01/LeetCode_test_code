#include <iostream>
#include <vector>
#include <unordered_set>
#include <unordered_map>
#include <string>
#include <stack>
#include <queue>

void testCase(){
    int s, d;
    std::cin >> s >> d;
    int a = (s + d) / 2;
    int b = a - d;
    if(a*2 == (s + d) && b >= 0){
        std::cout << a << ' ' << b << std::endl;
    }
    else{
        std::cout << "impossible" << std::endl;
    }
}

int main(){
    int n;
    std::cin >> n;
    while(n--) testCase();
    return 0;
}


