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
        std::string com;
        std::cin >> com;
        if(!std::isdigit(com)){
            // Is undo
            undo = true;
            i--;
        }
        else{
            int p = std::stoi(com);
            if(undo){
                
            }
        }
        // int p;
        // std::cin >> p;
        // if(!std::cin){
        //     // Is undo
        //     undo = true;
        //     std::cin.clear();
        // }
        // else{
        //     if(undo){
        //         // p is m
        //         for(int j=0; j<p; j++){
        //             coms.pop();
        //         }
        //         undo = false;
        //     }
        //     else{
        //         coms.push(p);
        //     }
        // }
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


