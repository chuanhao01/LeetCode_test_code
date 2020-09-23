#include <iostream>
#include <vector>
#include <unordered_set>
#include <unordered_map>

int main(){
    int m, s;
    std::cin >> m >> s;
    std::vector<std::vector<int>> prefs(m+1, std::vector<int>(s+1, 0));
    // Reading in stdin
    for(int i=0; i<m; i++){
        for(int j=0; j<s; j++){
            std::cin >> prefs[i][j];
        }
    }
    int done = 0;
    std::vector<int> seen(s+1);
    for(int j=0; j<s; j++){
        // Adding 1 to song that is seen
        for(int i=0; i<m; i++){
            seen[prefs[i][j]]++;
            if(seen[prefs[i][j]] == m){
                done++;
            }
        }
        if(done == (j+1)){
            std::cout << j+1 << std::endl;
            for(int ij=0; ij<s; ij++){
                if(seen[ij+1] == m){
                    std::cout << ij+1 << ' ';
                }
            }
            return 0;
        }
    }
    return 0;
}


