#include <iostream>
#include <vector>
#include <unordered_set>

int main(){
    int q;
    // int ans [q];
    std::cin >> q;
    for(int i=0; i<q; i++){
        // Init
        int k;
        std::cin >> k;
        int seen [1000] = {0};
        int n = 2;
        int fn1 = 1 % k, fn2 = fn1;

        while(true){
            // Get fn
            int fn = (fn1 + fn2) % k;
            // Check if seen
            if(seen[fn] != 0){
                // ans[i] = fn;
                std::cout << seen[fn] << std::endl;
                break;
            }
            seen[fn] = n;
            // Update fib
            fn2 = fn1;
            fn1 = fn;
            n++;
        }
    }
    return 0;
}


