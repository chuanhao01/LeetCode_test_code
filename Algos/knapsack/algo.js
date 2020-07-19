const algo = {
    recur(weights, values, maxWeight){
        let boo = [];
        for(let i of values){
            boo.push(0);
        }
        // Own implementation
        // function re(i, c){
        //     if(i === -1 || c === 0){
        //         // Base case
        //         return 0;
        //     }
        //     else if(weights[i] > c){
        //         return re(i-1, c);
        //     }
        //     else{
        //         let temp1 = re(i-1, c);
        //         let temp2 = values[i] + re(i-1, c - weights[i]);
        //         if(temp2 > temp1){
        //             boo[i] = 1;
        //         }
        //         return Math.max(temp1, temp2);
        //     }
        // }
        // return re(weights.length - 1, maxWeight);

        // Better
        function re(i, c){
            if(i === 0 || c === 0){
                return 0;
            }
            else if(weights[i-1] > c){
                return re(i-1, c);
            }
            else{
                let temp1 = re(i-1, c);
                let temp2 = values[i-1] + re(i-1, c - weights[i-1]);
                if(temp2 > temp1){
                    boo[i-1] = 1;
                }
                return Math.max(temp1, temp2);
            }
        }
        let fv = re(weights.length - 1, maxWeight);
        console.log(boo);
        return fv;
    },
    dpmem(weights, values, maxWeight){
        let dp = [];
        let n = weights.length;
        for(let i=0; i<n;i++){
            let arr = [];
            for(let j=0; j<maxWeight+1; j++){
                arr.push(-1);
            }
            dp.push(arr);
        }
        function re(i, c){
            if(i !== 0){
                if(dp[i-1][c] > -1){
                    return dp[i-1][c];
                }
            }
            let result;
            if(i === 0 || c === 0){
                return 0;
            }
            else if(weights[i-1] > c){
                result = re(i-1, c);
            }
            else{
                let temp1 = re(i-1, c);
                let temp2 = values[i-1] + re(i-1, c - weights[i-1]);
                result = Math.max(temp1, temp2);
            }
            dp[i-1][c] = result;
            return result;
        }
        return re(weights.length, maxWeight);
    },
    dpBottom(weights, values, maxWeight){
        let dp = [];
        for(let i of weights){
            let arr = [];
            for(let j=0; j<maxWeight+1; j++){
                arr.push(-1);
            }
            dp.push(arr);
        }
        for(let i=0; i<weights.length; i++){
            for(let j=0; j<maxWeight+1; j++){
                if(i===0){
                    if(j<weights[i]){
                        dp[i][j] = 0;
                    }
                    else{
                        dp[i][j] = values[i];
                    }
                }
                else{
                    if(j < weights[i]){
                        // If we cannot take the current item
                        dp[i][j] = dp[i-1][j];
                    }
                    else{
                        let takeItem = values[i] + dp[i-1][j-weights[i]];
                        dp[i][j] = Math.max(takeItem, dp[i-1][j]);
                    }
                }
            }
        }
        return dp[weights.length-1][maxWeight];
    }
};

module.exports = algo;