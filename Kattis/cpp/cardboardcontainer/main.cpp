#include <iostream>
#include <vector>
#include <unordered_set>
#include <unordered_map>
#include <string>
#include <stack>
#include <queue>
#include <cmath>

int main(){
    int v;
    std::cin >> v;
    int l = std::cbrt(v), w, h;
    while(true){
        if(v % l == 0){
            int temp = v / l;
            w = std::sqrt(temp);
            while(true){
                if(temp % w == 0){
                    h = temp / w;
                    break;
                }
                w--;
            }
            break;
        }
        l--;
    }
    std::cout << (2 * l * w) + (2 * l * h) + (2 * w * h);
    return 0;
}


