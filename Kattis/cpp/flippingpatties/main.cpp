#include <iostream>
#include <vector>
#include <unordered_map>

int main(){
    int n;
    int highest = 0;
    std::unordered_map<int, int> time;
    std::cin >> n;
    for(int i=0; i<n; i++){
        int d, t;
        std::cin >> d >> t;
        for(int j=0; j<3; j++){
            int x = t - d*j;
            if(time.count(x) == 0){
                time[x] = 1;
            }
            else{
                time[x]++;
            }
            highest = std::max(highest, time[x]);
        }
    }
    std::cout << (highest + 1) / 2;
    return 0;
}


