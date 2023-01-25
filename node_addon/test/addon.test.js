import {createRequire} from 'module';
const require = createRequire(import.meta.url);
 let  {encodeRailFenceCipher,decodeRailFenceCipher,longAddition,toRoman,fromRoman,listSquared,alphametics} =  require('../index.node');
  import {listSquaredJs , longAddition as longAddition2,RomanNumerals as RomanNumerals2}  from "../jsVers";
  
 
import assert from 'assert';
let add = longAddition;
let Roman = {toRoman,fromRoman};
let RomanNumerals = RomanNumerals2;

describe("encodeRailFenceCipher", function(){
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
  describe("listSquared ",function(){
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
describe("Roman Numbers Helper", () => {

		it("sample tests", () => {
			assert.strictEqual(
				RomanNumerals.toRoman(1000),
				"M"
			);
			assert.strictEqual(
				RomanNumerals.toRoman(4),
				"IV"
			);
			assert.strictEqual(RomanNumerals.toRoman(1), "I");
			assert.strictEqual(
				RomanNumerals.toRoman(1990),
				"MCMXC"
			);
			assert.strictEqual(
				RomanNumerals.toRoman(2008),
				"MMVIII"
			);

			assert.strictEqual(
				RomanNumerals.fromRoman("XXI"),
				21
			);
			assert.strictEqual(
				RomanNumerals.fromRoman("I"),
				1
			);
			assert.strictEqual(
				RomanNumerals.fromRoman("IV"),
				4
			);
			assert.strictEqual(
				RomanNumerals.fromRoman("MMVIII"),
				2008
			);
			assert.strictEqual(
				RomanNumerals.fromRoman("MDCLXVI"),
				1666
			);
		});
	

	const solution = {
		Values: [
			["M", 1000],
			["CM", 900],
			["D", 500],
			["CD", 400],
			["C", 100],
			["XC", 90],
			["L", 50],
			["XL", 40],
			["X", 10],
			["IX", 9],
			["V", 5],
			["IV", 4],
			["I", 1],
		],

		fromRoman: function (str) {
			var result = 0;
			for (var i = 0; i < this.Values.length; ++i) {
				var pair = this.Values[i];
				var key = pair[0];
				var value = pair[1];
				var regex = RegExp("^" + key);
				while (str.match(regex)) {
					result += value;
					str = str.replace(regex, "");
				}
			}
			return result;
		},

		toRoman: function (n) {
			var value = "";
			for (
				var i = 0;
				n > 0 && i < this.Values.length;
				i++
			) {
				while (n >= this.Values[i][1]) {
					value += this.Values[i][0];
					n -= this.Values[i][1];
				}
			}
			return value;
		},
	};

	function randInt(min, max) {
		min = Math.ceil(min);
		max = Math.floor(max);
		return Math.floor(Math.random() * (max - min + 1) + min); //The maximum is inclusive and the minimum is inclusive
	}

	it("random tests", () => {
		for (let i = 0; i < 100; i++) {
			const n = randInt(1, 3999);
			const romanString = solution.toRoman(n);
			assert.strictEqual(
				RomanNumerals.toRoman(n),
				romanString
			);
			assert.strictEqual(
				RomanNumerals.fromRoman(romanString),
				n
			);
		}
	});
});

describe("Alphametics", () => {
	const example_tests = [
		["SEND + MORE = MONEY", "9567 + 1085 = 10652"],
		["ZEROES + ONES = BINARY", "698392 + 3192 = 701584"],
		["COUPLE + COUPLE = QUARTET", "653924 + 653924 = 1307848"],
		["DO + YOU + FEEL = LUCKY", "57 + 870 + 9441 = 10368"],
		[
			"ELEVEN + NINE + FIVE + FIVE = THIRTY",
			"797275 + 5057 + 4027 + 4027 = 810386",
		],
	];
	example_tests.forEach(([s, user]) =>
    it(`should return ${user} for ${s}`, () => {
		expect(alphametics(s)).toEqual(user)  })
	);
});