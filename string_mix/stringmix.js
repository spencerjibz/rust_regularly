

function mix(s1,s2) {
// get all letters in both;
let combined = (s1+s2).replace(/\W|[A-Z]/g,''); // consider only non-capital letters
let noDups = new Set(combined)

// find the frequencies in both s1 and s2;
let freq = {}// O(N)
  for (const ele of noDups.values()) {
    freq[ele] = {};
  }
 
// use 1 loop for both s1 and s2; avoid 0(n) - time 
 for (const ele  of noDups.values()) {
     let s1Match = s1.match( new RegExp(ele,'g'));
     let s2Match = s2.match( new RegExp(ele,'g'));

     freq[ele] = {matches:[{ ele:'1',count: s1Match?s1Match.length:0},{ ele:'2',count: s2Match?s2Match.length:0}]}
      freq[ele].matches.sort((a,b)=> b.count-a.count)
      // remove elements with no repeats;
       if (freq[ele].matches.every((e)=> e.count<=1)) {
         delete freq[ele]
       }
       // if which is better;
        if ( freq[ele] &&freq[ele].matches[0].count===freq[ele].matches[1].count) {
             freq[ele].result = `=:${ele.repeat(freq[ele].matches[0].count)}`
        }
        else {
             if (freq[ele]) {
        freq[ele].result = `${freq[ele].matches[0].ele}:${ele.repeat(
		freq[ele].matches[0].count
        )}`;
             }
        }
     //
  }
   let result = Object.values(freq).map(v=> v.result)
  
   //  remove elements with a freq less than 2;
   // sort by length first ;
   result.sort((a,b) => { let re = b.length-a.length;
    // both a and b should not contain numbers;
    let reg =/\d/;
    let noNumbs = !(reg.test(a) || reg.test(b))
 if (re===0) {
  //a = a.replace(reg, '');
  //b = b.replace(reg, '');
    return a.localeCompare(b)
 }

 return re
})
 return result.join('/')

}
console.time('start')
 console.log(mix("looping is fun but dangerous", "less dangerous than coding"));
console.timeEnd('start')