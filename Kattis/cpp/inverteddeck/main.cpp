#include <iostream>
#include <vector>
#include <unordered_set>

int main(){
    int n, v;
    int start, stop;
    int prev;
    int flip = 0;
    bool asc = true;
    std::cin >> n;
    for(int i=0; i<n; i++){
        std::cin >> v;
        if(i != 0){
            if(asc){
                // asc
                if(prev >= v){
                    if(flip > 0){
                        std::cout << "impossible";
                        return 0;
                    }
                    start = i;
                    flip = 1;
                    asc = false;
                }
            }
            else{
                // desc
                if(prev < v){
                    stop = i;
                    asc = true;
                }
            }
        }
        prev = v;
    }
    if(flip == 0){
        std::cout << "0 0";
        return 0;
    }
    std::cout << start << ' ' << stop;
    return 0;
}


