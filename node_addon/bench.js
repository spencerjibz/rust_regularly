

import assert from 'assert';
import {createRequire} from 'module';
import {encodeRailFenceCipher2,listSquaredJs,crc32Js,crc3Js,longAddition as longAddition2,add,add2,RomanNumerals,alphametics as alphametics2} from './jsVers.js';
import Benchmark  from 'benchmark';
const require = createRequire(import.meta.url);
 let {
		encodeRailFenceCipher,
		listSquared,
		crc32,
		crc3,
		longAddition,
		decomp,
		longAdd,
		toRoman,
		fromRoman,
		alphametics,
 } = require("./index.node");
 
 let Roman = {toRoman,fromRoman};
  let suite = new Benchmark.Suite("Huffman Encoding algorithms");
 console.time('JS')
longAddition2(
	"987429134712934876249385134781395873198472398562384958739845234859234758913759182357398457398474598237459823745928347538",
	"835743829547328954732895474893754893753281957319857432958432548937859483265893274891378593187431583942678439217431924789"
)
console.timeEnd('JS')
console.time('Rust')
longAddition(
	"987429134712934876249385134781395873198472398562384958739845234859234758913759182357398457398474598237459823745928347538",
	"835743829547328954732895474893754893753281957319857432958432548937859483265893274891378593187431583942678439217431924789"
)
console.timeEnd('Rust')