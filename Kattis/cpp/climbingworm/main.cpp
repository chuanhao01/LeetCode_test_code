#include <iostream>
#include <vector>
#include <unordered_set>
#include <unordered_map>
#include <string>
#include <stack>
#include <queue>

int main(){
    int a, b, h;
    std::cin >> a >> b >> h;
    int time = 0, climb = 0;
    while(true){
        time++;
        climb += a;
        if(climb >= h){
            std::cout << time;
            break;
        }
        climb -= b;
    }
    return 0;
}


