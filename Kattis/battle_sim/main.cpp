#include <iostream>
#include <vector>
#include <unordered_set>
#include <string>

int main(){
    std::string in;
    std::cin >> in;
    const int length = in.size();
    std::string out = "";
    int i=0;
    while(i<length){
        if(i<length-2){
            if(in[i] != in[i+1] && in[i+1] != in[i+2] && in[i] != in[i+2]){
                out += "C";
                i += 3;
                continue;
            }
        }
        if(in[i] == 'R'){
            out += "S";
        }
        else if(in[i] == 'B'){
            out += "K";
        }
        else{
            out += "H";
        }
        i++;
    }
    std::cout << out;
    return 0;
}


