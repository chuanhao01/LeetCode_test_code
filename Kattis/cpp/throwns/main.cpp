#include <iostream>
#include <vector>
#include <unordered_set>
#include <stack>
#include <string>

int main(){
    int n, k;
    std::cin >> n >> k;
    std::stack<int> coms;
    bool undo = false;
    for(int i=0; i<k; i++){
        int p;
        std::cin >> p;
        if(!std::cin){
            // Is undo
            undo = true;
            i--;
        }
        else{
            if(undo){
                // p is m
                for(int j=0; j<p; j++){
                    coms.pop();
                }
            }
            else{
                coms.push(p);
            }
        }
    }
    std::cout << std::endl;
    int sum = 0;
    while(coms.size() > 0){
        std::cout << coms.top();
        std::cout << std::endl;
        sum += coms.top();
        coms.pop();
    }
    std::cout << std::endl;
    std::cout << sum % n;
    return 0;
}


