#include <iostream>
#include <vector>
#include <unordered_set>
#include <stack>

int main(){
    int n;
    std::cin >> n;
    std::stack<int> socks;
    for(int i=0; i<2*n; i++){
        int a;
        std::cin >> a;
        if(socks.size() == 0){
            // If empty
            socks.push(a);
        }
        else{
            if(socks.top() == a){
                socks.pop();
            }
            else{
                socks.push(a);
            }
        }
    }
    if(socks.size() != 0){
        std::cout << "impossible";
    }
    else{
        std::cout << 2 * n;
    }
    return 0;
}


