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
        std::string temp;
        if(! (std::cin >> p)){
            // Err undo
            std::cin.clear();
            std::cin >> temp;
            undo = true;
            i--;
        }
        else{
            if(undo){
                // p is m
                for(int j=0; j<p; j++){
                    coms.pop();
                }
                undo = false;
            }
            else{
                coms.push(p);
            }
        }
    }
    int sum = 0;
    while(coms.size() > 0){
        if((coms.top() % n) != 0) sum += coms.top();
        coms.pop();
    }
    if(sum < 0) std::cout << n + (sum % n);
    else std::cout << sum % n;
    return 0;
}


