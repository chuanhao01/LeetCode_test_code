#include <iostream>
#include <vector>
#include <unordered_set>

int main(){
    int n;
    int i=1;
    while(std::cin>>n){
        int min;
        int max;
        for(int ni=0; ni<n; ni++){
            // take in a num
            int x;
            std::cin>>x;
            if(ni == 0){
                min = x;
                max = x;
            }
            else{
                if(x < min){
                    min = x;
                }
                if(x > max){
                    max = x;
                }
            }
        }
        std::cout<<"Case " << i << ": " << min << ' ' << max << ' ' << max-min << std::endl;
        i++; 
    }
}


