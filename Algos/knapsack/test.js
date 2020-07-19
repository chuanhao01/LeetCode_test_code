const a = require('./algo');
const expect = require('chai').expect;

describe('Test algo', function(){
    it('Basic', function(){
        let w = [1, 2, 4, 2, 5];
        let v = [5, 3, 5, 3, 2];
        let mw = 10;
        let mv = a.recur(w, v, mw);
        expect(mv).to.be.equal(16);
    });
    it('dp mem', function(){
        let w = [1, 2, 4, 2, 5];
        let v = [5, 3, 5, 3, 2];
        let mw = 10;
        let mv = a.dpmem(w, v, mw);
        expect(mv).to.be.equal(16);
    });
    it('Bottom up', function(){
        let w = [1, 2, 4, 2, 5];
        let v = [5, 3, 5, 3, 2];
        let mw = 10;
        let mv = a.dpBottom(w, v, mw);
        expect(mv).to.be.equal(16);
    });
});