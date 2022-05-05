import {createRequire} from 'module';
const require = createRequire(import.meta.url);
 let  {encodeRailFenceCipher,decodeRailFenceCipher,longAddition} =  require('../index.node');
  import {listSquaredJs as listSquared , longAddition as longAddition2}  from "../jsVers.js";
import assert from 'assert';
let add = longAddition2;
describe("Testing rust-node addon", function(){
    it("should work with basic tests", function(){
      assert.strictEqual(encodeRailFenceCipher("Hello, World!", 3), "Hoo!el,Wrdl l");
      assert.strictEqual(encodeRailFenceCipher("Hello, World!", 2), "Hlo ol!el,Wrd");
  
      assert.strictEqual(encodeRailFenceCipher("", 3), "");
      assert.strictEqual(decodeRailFenceCipher("", 3), "");
  
      assert.strictEqual(encodeRailFenceCipher("WEAREDISCOVEREDFLEEATONCE", 3), "WECRLTEERDSOEEFEAOCAIVDEN");
      assert.strictEqual(decodeRailFenceCipher("WECRLTEERDSOEEFEAOCAIVDEN", 3), "WEAREDISCOVEREDFLEEATONCE");
  
      assert.strictEqual(encodeRailFenceCipher("WEAREDISCOVEREDFLEEATONCE", 4), "WIREEEDSEEEACAECVDLTNROFO");
      assert.strictEqual(decodeRailFenceCipher("WIREEEDSEEEACAECVDLTNROFO", 4), "WEAREDISCOVEREDFLEEATONCE");
  
      assert.strictEqual(encodeRailFenceCipher("WEAREDISCOVEREDFLEEATONCE", 5), "WCLEESOFECAIVDENRDEEAOERT");
      assert.strictEqual(decodeRailFenceCipher("WECRLTEERDSOEEFEAOCAIVDEN", 5), "WLSADOOTEEECEAEECRFINVEDR");
    });
    
    it("should work with random tests", function() {
      let lorem = 'Amet non facere minima iure unde, provident, \
      veritatis officiis asperiores ipsa eveniet sit! Deserunt \
      autem excepturi quibusdam iure unde! Porro alias distinctio \
      ipsam iure exercitationem molestiae. Voluptate fugiat quasi maiores!jk';
      
      let randomRailCount;
      
      function shuffle(string) {
        let parts = string.split('');
        for(let i = parts.length; i > 0;) {
          let random = parseInt(Math.random() * i);
          let temp = parts[--i];
          parts[i] = parts[random];
          parts[random] = temp;
        }
        return parts.join('');
      }
      
      function random() {
        return Math.floor(Math.random() * 50) + 2;
      }
      
      for (let i = 0; i < 20; i +=1) {
        randomRailCount = random();
        lorem = shuffle(lorem);
        
        assert.strictEqual(
          decodeRailFenceCipher(
            encodeRailFenceCipher(lorem, randomRailCount),
            randomRailCount
          ), 
          lorem
        );
      }
    });
  });
  describe("Rust listSquared ",function(){
   it(" Basic tests",function() {
    assert.deepEqual(listSquared(1, 250), [[1, 1], [42, 2500], [246, 84100]])
    assert.deepEqual(listSquared(42, 250), [[42, 2500], [246, 84100]])
    assert.deepEqual(listSquared(250, 500), [[287, 84100]])
    assert.deepEqual(listSquared(300, 600), [])
    assert.deepEqual(listSquared(600, 1500), [[728, 722500], [1434, 2856100]])
    assert.deepEqual(listSquared(1500, 1800), [[1673, 2856100]])
    assert.deepEqual(listSquared(1800, 2000), [[1880, 4884100]])
    assert.deepEqual(listSquared(2000, 2200), [])
    assert.deepEqual(listSquared(2200, 5000), [[4264, 24304900]])
    assert.deepEqual(listSquared(5000, 10000), [[6237, 45024100], [9799, 96079204], [9855, 113635600]])
    })
    
    it("Random tests",function(){
        function randint(a,b) {
            return Math.floor(Math.random() * (b - a + 1) + a);
        }
        
        function solAux130412(n) {
            let s = 0;
            let nf = 0;
            let res = [];
            for (let i = 1; i <= Math.floor(Math.sqrt(n)); i += 1)
                if (n % i === 0) {
                    s += i * i;
                    nf = n / i;
                    if (nf !== i)
                        s += nf * nf;
            }
            if (Math.pow(~~Math.sqrt(s), 2) === s) {
                res.push(n);
                res.push(s);
                return res;
            } else return null;
        }
    
        function solution130412(m, n) {
            let res = [];
            for (let i = m; i <= n; i++) {
                let r = solAux130412(i);
                if (r !== null) {
                    res.push(r);
                }
            }
            return res;
        }
    
        for (let i = 0; i < 50; i++){
            let m = randint(200, 1000);
            let n = randint(1100, 8000);
          
                    assert.deepEqual(listSquared(m, n),solution130412(m, n) )
                
            
        }
    })
})

describe('long Addition', function() {
  
  
  it('should handle small numbers', function() {

    assert.strictEqual(add('91', '19'), '110');
    assert.strictEqual(add('123456789', '987654322'), '1111111111');
    assert.strictEqual(add('999999999', '1'), '1000000000');
    
  
    for(let i = 0; i < 3; i++) {
      let  a =  Math.round(Math.random() * 1000000);
      let b = Math.round(Math.random() * 1000000);
           
      assert.equal(add( a.toString(), b.toString()), (a + b).toString());
    }
  });
  it('should handle big numbers', function() {

     let sum;
   sum = add('823094582094385190384102934810293481029348123094818923749817',
              '234758927345982475298347523984572983472398457293847594193837');

    assert.strictEqual(sum, '1057853509440367665682450458794866464501746580388666517943654');
    
    sum = add('234859234758913759182357398457398474598237459823745928347538',
              '987429134712934876249385134781395873198472398562384958739845');
    assert.strictEqual(sum, '1222288369471848635431742533238794347796709858386130887087383');
  
    sum = add('854694587458967459867923420398420394845873945734985374844444',
              '333333333333439439483948394839834938493843948394839432322229');
    assert.strictEqual(sum, '1188027920792406899351871815238255333339717894129824807166673');
  
    sum = add('666666665656566666666565656666666656565666666665656566666666',
              '464646464646464644646464646464646464646464646463463463463466');
    assert.strictEqual(sum, '1131313130303031311313030303131313121212131313129120030130132');
    
    sum = add('987429134712934876249385134781395873198472398562384958739845234859234758913759182357398457398474598237459823745928347538',
              '835743829547328954732895474893754893753281957319857432958432548937859483265893274891378593187431583942678439217431924789');
    assert.strictEqual(sum, '1823172964260263830982280609675150766951754355882242391698277783797094242179652457248777050585906182180138262963360272327');
  });
});