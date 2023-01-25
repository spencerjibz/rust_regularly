import { RomanNumerals } from "./jsVers.js";
import assert from "assert";
import { createRequire } from "module";
const require = createRequire(import.meta.url);
let {toRoman,fromRoman}  = require('./index.node');


let Roman = {toRoman,fromRoman};


 let JsVer = function() {
/*
    assert.strictEqual(RomanNumerals.toRoman(1000), "M");
    assert.strictEqual(RomanNumerals.toRoman(4), "IV");
    assert.strictEqual(RomanNumerals.toRoman(1), "I");
    assert.strictEqual(RomanNumerals.toRoman(1990), "MCMXC");
    assert.strictEqual(RomanNumerals.toRoman(2008), "MMVIII");
    */

    assert.strictEqual(RomanNumerals.fromRoman("XXI"), 21);
    assert.strictEqual(RomanNumerals.fromRoman("I"), 1);
    assert.strictEqual(RomanNumerals.fromRoman("IV"), 4);
    assert.strictEqual(RomanNumerals.fromRoman("MMVIII"), 2008);
    assert.strictEqual(RomanNumerals.fromRoman("MDCLXVI"), 1666);
    assert.strictEqual(RomanNumerals.fromRoman("MCMXC"), 1990);
    
 }

 let RustVer = function() {
    /* assert.strictEqual(Roman.toRoman(1000), "M");
    assert.strictEqual(Roman.toRoman(4), "IV");
    assert.strictEqual(Roman.toRoman(1), "I");
    assert.strictEqual(Roman.toRoman(1990), "MCMXC");
    assert.strictEqual(Roman.toRoman(2008), "MMVIII");
*/
    assert.strictEqual(Roman.fromRoman("XXI"), 21);
    assert.strictEqual(Roman.fromRoman("I"), 1);
    assert.strictEqual(Roman.fromRoman("IV"), 4);
    assert.strictEqual(Roman.fromRoman("MMVIII"), 2008);
    assert.strictEqual(Roman.fromRoman("MDCLXVI"), 1666);
  assert.strictEqual(Roman.fromRoman("MCMXC"), 1990);
  
 }
// time both versions;
 console.time('js');
  for (let i = 0; i < 100000; i++) {
    JsVer();
  }
 
 console.timeEnd('js');

 console.time('rust-addon');
 for (let i = 0; i < 100000; i++) {
    RustVer();
  }
 console.timeEnd('rust-addon');