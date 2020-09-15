#include <iostream>
#include <vector>
#include <unordered_set>
#include <unorder_map>

int main(){
    int m, s;
    int pref;
    std::unorder_map<int, int> prefs;
    std::cin >> m >> s;
    for(int i=0; i<m; i++){
        for(int k=0; j<s; j++){
            std::cin >> pref;
            if(prefs.count(pref) == 0){
                // Does not exists
                prefs[pref] = k;
            }
            else{
                prefs[pref] += k;
            }
        }
    }
}


