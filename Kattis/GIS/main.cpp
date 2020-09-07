#include <iostream>
#include <vector>
#include <unordered_set>

int main(){
    int N, count=0;
    std::cin>>N;
    int min, g;
    std::vector <int> gis;
    while(count<N){
        std::cin>>g;
        if(count == 0){
            // Base case
            min = g;
            gis.push_back(g);
        }
        else{
            if(g > min){
                gis.push_back(g);
                min = g;
            }
        }
        count++;
    }
    std::cout << gis.size() << std::endl;
    for(int i=0; i<gis.size(); i++){
        std::cout << gis[i] << ' ';
    }
}


