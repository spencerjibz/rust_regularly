

import assert from 'assert';
import {createRequire} from 'module';
import {encodeRailFenceCipher2,listSquaredJs,crc32Js,crc3Js,longAddition as longAddition2} from './jsVers.js';
import Benchmark  from 'benchmark';
const require = createRequire(import.meta.url);
 let  {encodeRailFenceCipher,listSquared,crc32,crc3,longAddition} =   require('./index.node');
  let suite = new Benchmark.Suite('node vs rust-addon')
 /* console.time('js')
 console.log(listSquaredJs(5000,10000))
 console.timeEnd('js')

 console.time('addon')
  console.log(listSquared(5000,10000))
 console.timeEnd('addon')

  
  //console.log(crc32Js('hello'))
  */
suite
 .add('Rust Version',()=> { 
     //listSquaredJs(5000,10000)
     //crc3('My name is spencer and I love you')
    // crc32('my name is spencer and I love you and I make money easily')
    let add = longAddition2;
    for(let i = 0; i < 100; i++) {
      let  a =  Math.round(Math.random() * 1000000);
      let b = Math.round(Math.random() * 1000000);
           
      add( a.toString(), b.toString());

    }



 }) 
 
 .add('JS Version ',()=> {
 // crc3Js('my name is spencer and I love you and I make money easily')
 let add = longAddition2;
 for(let i = 0; i < 100; i++) {
  let  a =  Math.round(Math.random() * 1000000);
  let b = Math.round(Math.random() * 1000000);
       
 add( a.toString(), b.toString()); 
}
 })
 .on('cycle',(event) => { 
 console.log(String(event.target))
 })
 
 .on('complete', function() {
    console.log('Fastest is ' + this.filter('fastest').map('name'));
  })
  // run async
  .run({ 'async': true });
 