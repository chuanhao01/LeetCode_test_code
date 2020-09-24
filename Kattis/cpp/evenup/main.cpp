#include <iostream>
#include <vector>
#include <unordered_set>
#include <stack>

int main(){
    int n;
    std::cin >> n;
    std::stack<int> cards;
    for(int i=0; i<n; i++){
        // Read stdin
        int x;
        std::cin >> x;
        if(cards.size() == 0){
            // Base case
            cards.push(x);
        }
        else{
            // Main
            if((cards.top() + x)%2 == 0){
                // Even
                cards.pop();
            }
            else{
                cards.push(x);
            }
        }
    }
    std::cout << cards.size();
    return 0;
}


