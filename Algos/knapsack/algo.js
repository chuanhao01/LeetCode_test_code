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
    }
};

module.exports = algo;