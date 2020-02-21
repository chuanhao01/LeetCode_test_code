#include <vector>

int FindOutlier(std::vector<int> arr){
    // Here 0 -> even, 1 -> odd
    int remainder = 0;
    if(arr[0] % 2 == 0 && arr[1] % 2 == 0){
        // Means the odd is out
        remainder = 0;
    }
    else if(arr[0] % 2 == 1 && arr[1] % 2 == 1){
        // Means even is out
        remainder = 1;
    }
    else{
        // Need to check third to see which is right
        if(arr[2] % 2 == 0){
            // odd is out
            remainder = 0;
        }
        else{
            // even is out
            remainder = 1;
        }
    }
    for(int i=0; i<arr.size(); i++){
        if(arr[i] % 2 != remainder || arr[i] % 2 != -remainder){
            return arr[i];
        }
    }
}