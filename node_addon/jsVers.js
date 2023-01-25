 export function encodeRailFenceCipher2(string, numberRails) {
    if (!string || !numberRails) {
      console.log('invalid input');
      return '';
    }
    var arr = string.split(""),
        result = [];
    for (var i = 0; i < numberRails; i++) {
      result[i] = [];
      for (var j = 0; j < arr.length; j++) {
        var k = j * 2 * (numberRails - 1) + i;
        k < arr.length ? result[i].push(k) : 1;
        if (i !== 0 && i !== numberRails) {
          var k2 = j * 2 * (numberRails - 1) - i;
          k2 < arr.length && k2 > 0 ? result[i].push(k2) : 1;
        }
      }
    }
    function uniqueSort(arr) {
      arr = Array.from(new Set(arr));
      return arr.sort(function (a, b) {
        return a - b;
      });
    }
  
    result = result.map(function (arr) {
      return uniqueSort(arr);
    }).reduce(function (a, b) {
      return a.concat(b);
    }).map(function (i) {
      return arr[i];
    }).join("");
    return result;
  }
  
   export function decodeRailFenceCipher2(string, numberRails) {
    if (!string || !numberRails) {
      console.log('invalid input');
      return '';
    }
    var div = 2 * (numberRails - 2) + 2,
        stringArr = string.split(""),
        len = parseInt(stringArr.length / div),
        remainder = stringArr.length % div,
        splitArr = [],
        tempArr = [],
        result = [];
    for (var i = 0; i < numberRails; i++) {
      splitArr.push(i == 0 || i == numberRails - 1 ? len : 2 * len);
    }
    if (remainder > numberRails) {
      splitArr = splitArr.map(function (num) {
        return num + 1;
      });
      remainder = remainder - numberRails;
      for (var j = numberRails - 2; j >= numberRails - remainder - 1; j--) {
        splitArr[j]++;
      }
    } else {
      for (var j = 0; j < remainder; j++) {
        splitArr[j]++;
      }
    }
  
    tempArr = splitArr.map(function (len) {
      var ans = stringArr.splice(0, len);
      return ans;
    });
    var float = 0,
        k = 0;
  
    function lineUp(isAdd) {
      if (k == string.length) {
        return;
      }
      result.push(tempArr[float].shift());
      k++;
      isAdd ? float++ : float--;
      if (float == numberRails) {
        float = float - 2;
        isAdd = false;
      }
      if (float == 0) {
        isAdd = true;
      }
      lineUp(isAdd);
    }
  
    lineUp(true);
    return result.join("");
  }

  
  Number.prototype[Symbol.iterator] = function * () { 
    let sqrt = Math.sqrt(this);
    for (let i=1; i<=sqrt; i++)  { 
       if ((this%i)===0)  { 
         // check for the divisors  that are equal 
             if (i === this/i) { 
                yield i 
             }
             else {
              yield [i,this/i];
             }
             
          }
       }
    }
  Array.prototype.flat = function () { 
   return this.reduce((acc, val) => acc.concat(val), []);
  }
 
  export function listSquaredJs(m, n) { // O(N^2)
    // your code
   
  
    let result = []; // O(N)
  
      while (m<=n) { 
   let factors = [...m].flat();
   let squareDivs= factors .map(v=> Math.pow(v,2)); //0(N)
  
   let sum = squareDivs.reduce((c,nex)=> c+nex,0); //On
    
   let root = Math.sqrt(sum)
  
     if(Number.isInteger(root)) { 
  
        result.push([m,sum])
     }
  m++
      }
     return result
  }


  const URICR32_TABLE = [
    0x00000000, 0x77073096, 0xEE0E612C,
      0x990951BA, 0x076DC419, 0x706AF48F,
      0xE963A535, 0x9E6495A3, 0x0EDB8832,
      0x79DCB8A4, 0xE0D5E91E, 0x97D2D988,
      0x09B64C2B, 0x7EB17CBD, 0xE7B82D07,
      0x90BF1D91, 0x1DB71064, 0x6AB020F2,
      0xF3B97148, 0x84BE41DE, 0x1ADAD47D,
      0x6DDDE4EB, 0xF4D4B551, 0x83D385C7,
      0x136C9856, 0x646BA8C0, 0xFD62F97A,
      0x8A65C9EC, 0x14015C4F, 0x63066CD9,
      0xFA0F3D63, 0x8D080DF5, 0x3B6E20C8,
      0x4C69105E, 0xD56041E4, 0xA2677172,
      0x3C03E4D1, 0x4B04D447, 0xD20D85FD,
      0xA50AB56B, 0x35B5A8FA, 0x42B2986C,
      0xDBBBC9D6, 0xACBCF940, 0x32D86CE3,
      0x45DF5C75, 0xDCD60DCF, 0xABD13D59,
      0x26D930AC, 0x51DE003A, 0xC8D75180,
      0xBFD06116, 0x21B4F4B5, 0x56B3C423,
      0xCFBA9599, 0xB8BDA50F, 0x2802B89E,
      0x5F058808, 0xC60CD9B2, 0xB10BE924,
      0x2F6F7C87, 0x58684C11, 0xC1611DAB,
      0xB6662D3D, 0x76DC4190, 0x01DB7106,
      0x98D220BC, 0xEFD5102A, 0x71B18589,
      0x06B6B51F, 0x9FBFE4A5, 0xE8B8D433,
      0x7807C9A2, 0x0F00F934, 0x9609A88E,
      0xE10E9818, 0x7F6A0DBB, 0x086D3D2D,
      0x91646C97, 0xE6635C01, 0x6B6B51F4,
      0x1C6C6162, 0x856530D8, 0xF262004E,
      0x6C0695ED, 0x1B01A57B, 0x8208F4C1,
      0xF50FC457, 0x65B0D9C6, 0x12B7E950,
      0x8BBEB8EA, 0xFCB9887C, 0x62DD1DDF,
      0x15DA2D49, 0x8CD37CF3, 0xFBD44C65,
      0x4DB26158, 0x3AB551CE, 0xA3BC0074,
      0xD4BB30E2, 0x4ADFA541, 0x3DD895D7,
      0xA4D1C46D, 0xD3D6F4FB, 0x4369E96A,
      0x346ED9FC, 0xAD678846, 0xDA60B8D0,
      0x44042D73, 0x33031DE5, 0xAA0A4C5F,
      0xDD0D7CC9, 0x5005713C, 0x270241AA,
      0xBE0B1010, 0xC90C2086, 0x5768B525,
      0x206F85B3, 0xB966D409, 0xCE61E49F,
      0x5EDEF90E, 0x29D9C998, 0xB0D09822,
      0xC7D7A8B4, 0x59B33D17, 0x2EB40D81,
      0xB7BD5C3B, 0xC0BA6CAD, 0xEDB88320,
      0x9ABFB3B6, 0x03B6E20C, 0x74B1D29A,
      0xEAD54739, 0x9DD277AF, 0x04DB2615,
      0x73DC1683, 0xE3630B12, 0x94643B84,
      0x0D6D6A3E, 0x7A6A5AA8, 0xE40ECF0B,
      0x9309FF9D, 0x0A00AE27, 0x7D079EB1,
      0xF00F9344, 0x8708A3D2, 0x1E01F268,
      0x6906C2FE, 0xF762575D, 0x806567CB,
      0x196C3671, 0x6E6B06E7, 0xFED41B76,
      0x89D32BE0, 0x10DA7A5A, 0x67DD4ACC,
      0xF9B9DF6F, 0x8EBEEFF9, 0x17B7BE43,
      0x60B08ED5, 0xD6D6A3E8, 0xA1D1937E,
      0x38D8C2C4, 0x4FDFF252, 0xD1BB67F1,
      0xA6BC5767, 0x3FB506DD, 0x48B2364B,
      0xD80D2BDA, 0xAF0A1B4C, 0x36034AF6,
      0x41047A60, 0xDF60EFC3, 0xA867DF55,
      0x316E8EEF, 0x4669BE79, 0xCB61B38C,
      0xBC66831A, 0x256FD2A0, 0x5268E236,
      0xCC0C7795, 0xBB0B4703, 0x220216B9,
      0x5505262F, 0xC5BA3BBE, 0xB2BD0B28,
      0x2BB45A92, 0x5CB36A04, 0xC2D7FFA7,
      0xB5D0CF31, 0x2CD99E8B, 0x5BDEAE1D,
      0x9B64C2B0, 0xEC63F226, 0x756AA39C,
      0x026D930A, 0x9C0906A9, 0xEB0E363F,
      0x72076785, 0x05005713, 0x95BF4A82,
      0xE2B87A14, 0x7BB12BAE, 0x0CB61B38,
      0x92D28E9B, 0xE5D5BE0D, 0x7CDCEFB7,
      0x0BDBDF21, 0x86D3D2D4, 0xF1D4E242,
      0x68DDB3F8, 0x1FDA836E, 0x81BE16CD,
      0xF6B9265B, 0x6FB077E1, 0x18B74777,
      0x88085AE6, 0xFF0F6A70, 0x66063BCA,
      0x11010B5C, 0x8F659EFF, 0xF862AE69,
      0x616BFFD3, 0x166CCF45, 0xA00AE278,
      0xD70DD2EE, 0x4E048354, 0x3903B3C2,
      0xA7672661, 0xD06016F7, 0x4969474D,
      0x3E6E77DB, 0xAED16A4A, 0xD9D65ADC,
      0x40DF0B66, 0x37D83BF0, 0xA9BCAE53,
      0xDEBB9EC5, 0x47B2CF7F, 0x30B5FFE9,
      0xBDBDF21C, 0xCABAC28A, 0x53B39330,
      0x24B4A3A6, 0xBAD03605, 0xCDD70693,
      0x54DE5729, 0x23D967BF, 0xB3667A2E,
      0xC4614AB8, 0x5D681B02, 0x2A6F2B94,
      0xB40BBE37, 0xC30C8EA1, 0x5A05DF1B,
      0x2D02EF8D
    
    ];
    

     export function crc32Js(str) { 
       let string = str;
       str = [...str];
      let crc = 0xFFFFFFFF;
      let done;
    //let table = make_crc_table();
    str.forEach((ch,ind)=>  { 
        
        let   code = string.charCodeAt(ind) 
        // chartcode for
		let index = ((crc^code) & 0xFF) 
		 done = URICR32_TABLE[ index];
	  //dbg!(done,table.nth(index));
           crc = (((crc >>8) &  0x00FFFFFF) ^done)>>>0; // convert to Uint32
       
          // console.log(ch,code,'shift:',crc, 'index:',index) 

		   
    });
      //console.log(URICR32_TABLE)                                                               
     
     return crc^0xFFFFFFFF
    } 

    function make_crc_table(n)  { 
 
     
        
       
      let   c = n;
          
       for (let i= 0;i<8;i++)  { 
           if ( c&1 ===1)
            { 
         
              c= (0xEDB88320) ^ (c >>> 1)
            }
             else { 
              c = c >>> 1 ;
             }
           
           
       }
        let   result= c>>0;
      
         return result
      
     
     }
     export function crc3Js(str) { 
      let string = str;
      str = [...str];
     let crc = 0xFFFFFFFF;
     let done;
   //let table = make_crc_table();
   str.forEach((ch,ind)=>  { 
       
       let   code = string.charCodeAt(ind) 
       // chartcode for
   let index = ((crc^code) & 0xFF) 
    done = make_crc_table(index);
   //dbg!(done,table.nth(index));

          crc = (((crc >>8) &  0x00FFFFFF) ^done)>>>0; // convert to Uint32
     
         // console.log(ch,code,'shift:',crc, 'index:',index) 

      
   });
   
     //console.log(URICR32_TABLE)
    return crc^0xFFFFFFFF
   } 

    export function longAddition(right, left) { 
    // make array of each
    
    let [rightArr,leftArr] = [right.split(''),left.split('')]
    
    // find the  longest
    let longest = right.length> left.length? rightArr: right.length===left.length?rightArr:leftArr; 
    // use the longest to  fill the shortest
    
    let shortest = right.length < left.length? rightArr:right.length===left.length?leftArr: leftArr; 
    // difference between shortest and  longest 
     let diff =   longest.length - shortest.length;
    
    // make short one the same size
     if (longest.length=== shortest.length) { 
       longest.unshift('0');   
       shortest.unshift('0');
    
     } else { 
    
    
     shortest.splice(0,0,...'0'.repeat(diff))  // O(n) worst case , O(1) best case
    // longest addition implemention 
     }
    
     //console.log(longest,shortest)
     longest= longest.map(v=> parseInt(v)).reverse(),shortest = shortest.map(v=> parseInt(v)).reverse();
    
     // if both left and right are equal, expand both by 0
    
    //console.log(longest,shortest)
    
    let result = new Array(longest.length);
    let remainder =0;
     for ( let i  in longest) { 
     // 
      
    let sum = ( remainder + longest[i] + shortest[i]).toString();
    
     remainder =  sum.length>1? Number(sum[0]):0;
    
      if (sum.length>1) { 
          result[i] =  sum[1]
      }
       else { 
          result[i] = sum[0]
       }
         
     }
    /// 
    
     result = result.reverse();
    
     // if length of result is less than the length of the longest, prefill with the first digits of  longests
       if (remainder >0) { 
         result.unshift(remainder);
       }
       
       // remove the useless zeros
      if (result[0]==='0') { 
    
        result.shift()
      }
    
       
    
      return result.join('')
    
     }
     // f
     
    // add the big integer module
 var bigInt=function(undefined){"use strict";var BASE=1e7,LOG_BASE=7,MAX_INT=9007199254740992,MAX_INT_ARR=smallToArray(MAX_INT),DEFAULT_ALPHABET="0123456789abcdefghijklmnopqrstuvwxyz";var supportsNativeBigInt=typeof BigInt==="function";function Integer(v,radix,alphabet,caseSensitive){if(typeof v==="undefined")return Integer[0];if(typeof radix!=="undefined")return+radix===10&&!alphabet?parseValue(v):parseBase(v,radix,alphabet,caseSensitive);return parseValue(v)}function BigInteger(value,sign){this.value=value;this.sign=sign;this.isSmall=false}BigInteger.prototype=Object.create(Integer.prototype);function SmallInteger(value){this.value=value;this.sign=value<0;this.isSmall=true}SmallInteger.prototype=Object.create(Integer.prototype);function NativeBigInt(value){this.value=value}NativeBigInt.prototype=Object.create(Integer.prototype);function isPrecise(n){return-MAX_INT<n&&n<MAX_INT}function smallToArray(n){if(n<1e7)return[n];if(n<1e14)return[n%1e7,Math.floor(n/1e7)];return[n%1e7,Math.floor(n/1e7)%1e7,Math.floor(n/1e14)]}function arrayToSmall(arr){trim(arr);var length=arr.length;if(length<4&&compareAbs(arr,MAX_INT_ARR)<0){switch(length){case 0:return 0;case 1:return arr[0];case 2:return arr[0]+arr[1]*BASE;default:return arr[0]+(arr[1]+arr[2]*BASE)*BASE}}return arr}function trim(v){var i=v.length;while(v[--i]===0);v.length=i+1}function createArray(length){var x=new Array(length);var i=-1;while(++i<length){x[i]=0}return x}function truncate(n){if(n>0)return Math.floor(n);return Math.ceil(n)}function add(a,b){var l_a=a.length,l_b=b.length,r=new Array(l_a),carry=0,base=BASE,sum,i;for(i=0;i<l_b;i++){sum=a[i]+b[i]+carry;carry=sum>=base?1:0;r[i]=sum-carry*base}while(i<l_a){sum=a[i]+carry;carry=sum===base?1:0;r[i++]=sum-carry*base}if(carry>0)r.push(carry);return r}function addAny(a,b){if(a.length>=b.length)return add(a,b);return add(b,a)}function addSmall(a,carry){var l=a.length,r=new Array(l),base=BASE,sum,i;for(i=0;i<l;i++){sum=a[i]-base+carry;carry=Math.floor(sum/base);r[i]=sum-carry*base;carry+=1}while(carry>0){r[i++]=carry%base;carry=Math.floor(carry/base)}return r}BigInteger.prototype.add=function(v){var n=parseValue(v);if(this.sign!==n.sign){return this.subtract(n.negate())}var a=this.value,b=n.value;if(n.isSmall){return new BigInteger(addSmall(a,Math.abs(b)),this.sign)}return new BigInteger(addAny(a,b),this.sign)};BigInteger.prototype.plus=BigInteger.prototype.add;SmallInteger.prototype.add=function(v){var n=parseValue(v);var a=this.value;if(a<0!==n.sign){return this.subtract(n.negate())}var b=n.value;if(n.isSmall){if(isPrecise(a+b))return new SmallInteger(a+b);b=smallToArray(Math.abs(b))}return new BigInteger(addSmall(b,Math.abs(a)),a<0)};SmallInteger.prototype.plus=SmallInteger.prototype.add;NativeBigInt.prototype.add=function(v){return new NativeBigInt(this.value+parseValue(v).value)};NativeBigInt.prototype.plus=NativeBigInt.prototype.add;function subtract(a,b){var a_l=a.length,b_l=b.length,r=new Array(a_l),borrow=0,base=BASE,i,difference;for(i=0;i<b_l;i++){difference=a[i]-borrow-b[i];if(difference<0){difference+=base;borrow=1}else borrow=0;r[i]=difference}for(i=b_l;i<a_l;i++){difference=a[i]-borrow;if(difference<0)difference+=base;else{r[i++]=difference;break}r[i]=difference}for(;i<a_l;i++){r[i]=a[i]}trim(r);return r}function subtractAny(a,b,sign){var value;if(compareAbs(a,b)>=0){value=subtract(a,b)}else{value=subtract(b,a);sign=!sign}value=arrayToSmall(value);if(typeof value==="number"){if(sign)value=-value;return new SmallInteger(value)}return new BigInteger(value,sign)}function subtractSmall(a,b,sign){var l=a.length,r=new Array(l),carry=-b,base=BASE,i,difference;for(i=0;i<l;i++){difference=a[i]+carry;carry=Math.floor(difference/base);difference%=base;r[i]=difference<0?difference+base:difference}r=arrayToSmall(r);if(typeof r==="number"){if(sign)r=-r;return new SmallInteger(r)}return new BigInteger(r,sign)}BigInteger.prototype.subtract=function(v){var n=parseValue(v);if(this.sign!==n.sign){return this.add(n.negate())}var a=this.value,b=n.value;if(n.isSmall)return subtractSmall(a,Math.abs(b),this.sign);return subtractAny(a,b,this.sign)};BigInteger.prototype.minus=BigInteger.prototype.subtract;SmallInteger.prototype.subtract=function(v){var n=parseValue(v);var a=this.value;if(a<0!==n.sign){return this.add(n.negate())}var b=n.value;if(n.isSmall){return new SmallInteger(a-b)}return subtractSmall(b,Math.abs(a),a>=0)};SmallInteger.prototype.minus=SmallInteger.prototype.subtract;NativeBigInt.prototype.subtract=function(v){return new NativeBigInt(this.value-parseValue(v).value)};NativeBigInt.prototype.minus=NativeBigInt.prototype.subtract;BigInteger.prototype.negate=function(){return new BigInteger(this.value,!this.sign)};SmallInteger.prototype.negate=function(){var sign=this.sign;var small=new SmallInteger(-this.value);small.sign=!sign;return small};NativeBigInt.prototype.negate=function(){return new NativeBigInt(-this.value)};BigInteger.prototype.abs=function(){return new BigInteger(this.value,false)};SmallInteger.prototype.abs=function(){return new SmallInteger(Math.abs(this.value))};NativeBigInt.prototype.abs=function(){return new NativeBigInt(this.value>=0?this.value:-this.value)};function multiplyLong(a,b){var a_l=a.length,b_l=b.length,l=a_l+b_l,r=createArray(l),base=BASE,product,carry,i,a_i,b_j;for(i=0;i<a_l;++i){a_i=a[i];for(var j=0;j<b_l;++j){b_j=b[j];product=a_i*b_j+r[i+j];carry=Math.floor(product/base);r[i+j]=product-carry*base;r[i+j+1]+=carry}}trim(r);return r}function multiplySmall(a,b){var l=a.length,r=new Array(l),base=BASE,carry=0,product,i;for(i=0;i<l;i++){product=a[i]*b+carry;carry=Math.floor(product/base);r[i]=product-carry*base}while(carry>0){r[i++]=carry%base;carry=Math.floor(carry/base)}return r}function shiftLeft(x,n){var r=[];while(n-- >0)r.push(0);return r.concat(x)}function multiplyKaratsuba(x,y){var n=Math.max(x.length,y.length);if(n<=30)return multiplyLong(x,y);n=Math.ceil(n/2);var b=x.slice(n),a=x.slice(0,n),d=y.slice(n),c=y.slice(0,n);var ac=multiplyKaratsuba(a,c),bd=multiplyKaratsuba(b,d),abcd=multiplyKaratsuba(addAny(a,b),addAny(c,d));var product=addAny(addAny(ac,shiftLeft(subtract(subtract(abcd,ac),bd),n)),shiftLeft(bd,2*n));trim(product);return product}function useKaratsuba(l1,l2){return-.012*l1-.012*l2+15e-6*l1*l2>0}BigInteger.prototype.multiply=function(v){var n=parseValue(v),a=this.value,b=n.value,sign=this.sign!==n.sign,abs;if(n.isSmall){if(b===0)return Integer[0];if(b===1)return this;if(b===-1)return this.negate();abs=Math.abs(b);if(abs<BASE){return new BigInteger(multiplySmall(a,abs),sign)}b=smallToArray(abs)}if(useKaratsuba(a.length,b.length))return new BigInteger(multiplyKaratsuba(a,b),sign);return new BigInteger(multiplyLong(a,b),sign)};BigInteger.prototype.times=BigInteger.prototype.multiply;function multiplySmallAndArray(a,b,sign){if(a<BASE){return new BigInteger(multiplySmall(b,a),sign)}return new BigInteger(multiplyLong(b,smallToArray(a)),sign)}SmallInteger.prototype._multiplyBySmall=function(a){if(isPrecise(a.value*this.value)){return new SmallInteger(a.value*this.value)}return multiplySmallAndArray(Math.abs(a.value),smallToArray(Math.abs(this.value)),this.sign!==a.sign)};BigInteger.prototype._multiplyBySmall=function(a){if(a.value===0)return Integer[0];if(a.value===1)return this;if(a.value===-1)return this.negate();return multiplySmallAndArray(Math.abs(a.value),this.value,this.sign!==a.sign)};SmallInteger.prototype.multiply=function(v){return parseValue(v)._multiplyBySmall(this)};SmallInteger.prototype.times=SmallInteger.prototype.multiply;NativeBigInt.prototype.multiply=function(v){return new NativeBigInt(this.value*parseValue(v).value)};NativeBigInt.prototype.times=NativeBigInt.prototype.multiply;function square(a){var l=a.length,r=createArray(l+l),base=BASE,product,carry,i,a_i,a_j;for(i=0;i<l;i++){a_i=a[i];carry=0-a_i*a_i;for(var j=i;j<l;j++){a_j=a[j];product=2*(a_i*a_j)+r[i+j]+carry;carry=Math.floor(product/base);r[i+j]=product-carry*base}r[i+l]=carry}trim(r);return r}BigInteger.prototype.square=function(){return new BigInteger(square(this.value),false)};SmallInteger.prototype.square=function(){var value=this.value*this.value;if(isPrecise(value))return new SmallInteger(value);return new BigInteger(square(smallToArray(Math.abs(this.value))),false)};NativeBigInt.prototype.square=function(v){return new NativeBigInt(this.value*this.value)};function divMod1(a,b){var a_l=a.length,b_l=b.length,base=BASE,result=createArray(b.length),divisorMostSignificantDigit=b[b_l-1],lambda=Math.ceil(base/(2*divisorMostSignificantDigit)),remainder=multiplySmall(a,lambda),divisor=multiplySmall(b,lambda),quotientDigit,shift,carry,borrow,i,l,q;if(remainder.length<=a_l)remainder.push(0);divisor.push(0);divisorMostSignificantDigit=divisor[b_l-1];for(shift=a_l-b_l;shift>=0;shift--){quotientDigit=base-1;if(remainder[shift+b_l]!==divisorMostSignificantDigit){quotientDigit=Math.floor((remainder[shift+b_l]*base+remainder[shift+b_l-1])/divisorMostSignificantDigit)}carry=0;borrow=0;l=divisor.length;for(i=0;i<l;i++){carry+=quotientDigit*divisor[i];q=Math.floor(carry/base);borrow+=remainder[shift+i]-(carry-q*base);carry=q;if(borrow<0){remainder[shift+i]=borrow+base;borrow=-1}else{remainder[shift+i]=borrow;borrow=0}}while(borrow!==0){quotientDigit-=1;carry=0;for(i=0;i<l;i++){carry+=remainder[shift+i]-base+divisor[i];if(carry<0){remainder[shift+i]=carry+base;carry=0}else{remainder[shift+i]=carry;carry=1}}borrow+=carry}result[shift]=quotientDigit}remainder=divModSmall(remainder,lambda)[0];return[arrayToSmall(result),arrayToSmall(remainder)]}function divMod2(a,b){var a_l=a.length,b_l=b.length,result=[],part=[],base=BASE,guess,xlen,highx,highy,check;while(a_l){part.unshift(a[--a_l]);trim(part);if(compareAbs(part,b)<0){result.push(0);continue}xlen=part.length;highx=part[xlen-1]*base+part[xlen-2];highy=b[b_l-1]*base+b[b_l-2];if(xlen>b_l){highx=(highx+1)*base}guess=Math.ceil(highx/highy);do{check=multiplySmall(b,guess);if(compareAbs(check,part)<=0)break;guess--}while(guess);result.push(guess);part=subtract(part,check)}result.reverse();return[arrayToSmall(result),arrayToSmall(part)]}function divModSmall(value,lambda){var length=value.length,quotient=createArray(length),base=BASE,i,q,remainder,divisor;remainder=0;for(i=length-1;i>=0;--i){divisor=remainder*base+value[i];q=truncate(divisor/lambda);remainder=divisor-q*lambda;quotient[i]=q|0}return[quotient,remainder|0]}function divModAny(self,v){var value,n=parseValue(v);if(supportsNativeBigInt){return[new NativeBigInt(self.value/n.value),new NativeBigInt(self.value%n.value)]}var a=self.value,b=n.value;var quotient;if(b===0)throw new Error("Cannot divide by zero");if(self.isSmall){if(n.isSmall){return[new SmallInteger(truncate(a/b)),new SmallInteger(a%b)]}return[Integer[0],self]}if(n.isSmall){if(b===1)return[self,Integer[0]];if(b==-1)return[self.negate(),Integer[0]];var abs=Math.abs(b);if(abs<BASE){value=divModSmall(a,abs);quotient=arrayToSmall(value[0]);var remainder=value[1];if(self.sign)remainder=-remainder;if(typeof quotient==="number"){if(self.sign!==n.sign)quotient=-quotient;return[new SmallInteger(quotient),new SmallInteger(remainder)]}return[new BigInteger(quotient,self.sign!==n.sign),new SmallInteger(remainder)]}b=smallToArray(abs)}var comparison=compareAbs(a,b);if(comparison===-1)return[Integer[0],self];if(comparison===0)return[Integer[self.sign===n.sign?1:-1],Integer[0]];if(a.length+b.length<=200)value=divMod1(a,b);else value=divMod2(a,b);quotient=value[0];var qSign=self.sign!==n.sign,mod=value[1],mSign=self.sign;if(typeof quotient==="number"){if(qSign)quotient=-quotient;quotient=new SmallInteger(quotient)}else quotient=new BigInteger(quotient,qSign);if(typeof mod==="number"){if(mSign)mod=-mod;mod=new SmallInteger(mod)}else mod=new BigInteger(mod,mSign);return[quotient,mod]}BigInteger.prototype.divmod=function(v){var result=divModAny(this,v);return{quotient:result[0],remainder:result[1]}};NativeBigInt.prototype.divmod=SmallInteger.prototype.divmod=BigInteger.prototype.divmod;BigInteger.prototype.divide=function(v){return divModAny(this,v)[0]};NativeBigInt.prototype.over=NativeBigInt.prototype.divide=function(v){return new NativeBigInt(this.value/parseValue(v).value)};SmallInteger.prototype.over=SmallInteger.prototype.divide=BigInteger.prototype.over=BigInteger.prototype.divide;BigInteger.prototype.mod=function(v){return divModAny(this,v)[1]};NativeBigInt.prototype.mod=NativeBigInt.prototype.remainder=function(v){return new NativeBigInt(this.value%parseValue(v).value)};SmallInteger.prototype.remainder=SmallInteger.prototype.mod=BigInteger.prototype.remainder=BigInteger.prototype.mod;BigInteger.prototype.pow=function(v){var n=parseValue(v),a=this.value,b=n.value,value,x,y;if(b===0)return Integer[1];if(a===0)return Integer[0];if(a===1)return Integer[1];if(a===-1)return n.isEven()?Integer[1]:Integer[-1];if(n.sign){return Integer[0]}if(!n.isSmall)throw new Error("The exponent "+n.toString()+" is too large.");if(this.isSmall){if(isPrecise(value=Math.pow(a,b)))return new SmallInteger(truncate(value))}x=this;y=Integer[1];while(true){if(b&1===1){y=y.times(x);--b}if(b===0)break;b/=2;x=x.square()}return y};SmallInteger.prototype.pow=BigInteger.prototype.pow;NativeBigInt.prototype.pow=function(v){var n=parseValue(v);var a=this.value,b=n.value;var _0=BigInt(0),_1=BigInt(1),_2=BigInt(2);if(b===_0)return Integer[1];if(a===_0)return Integer[0];if(a===_1)return Integer[1];if(a===BigInt(-1))return n.isEven()?Integer[1]:Integer[-1];if(n.isNegative())return new NativeBigInt(_0);var x=this;var y=Integer[1];while(true){if((b&_1)===_1){y=y.times(x);--b}if(b===_0)break;b/=_2;x=x.square()}return y};BigInteger.prototype.modPow=function(exp,mod){exp=parseValue(exp);mod=parseValue(mod);if(mod.isZero())throw new Error("Cannot take modPow with modulus 0");var r=Integer[1],base=this.mod(mod);while(exp.isPositive()){if(base.isZero())return Integer[0];if(exp.isOdd())r=r.multiply(base).mod(mod);exp=exp.divide(2);base=base.square().mod(mod)}return r};NativeBigInt.prototype.modPow=SmallInteger.prototype.modPow=BigInteger.prototype.modPow;function compareAbs(a,b){if(a.length!==b.length){return a.length>b.length?1:-1}for(var i=a.length-1;i>=0;i--){if(a[i]!==b[i])return a[i]>b[i]?1:-1}return 0}BigInteger.prototype.compareAbs=function(v){var n=parseValue(v),a=this.value,b=n.value;if(n.isSmall)return 1;return compareAbs(a,b)};SmallInteger.prototype.compareAbs=function(v){var n=parseValue(v),a=Math.abs(this.value),b=n.value;if(n.isSmall){b=Math.abs(b);return a===b?0:a>b?1:-1}return-1};NativeBigInt.prototype.compareAbs=function(v){var a=this.value;var b=parseValue(v).value;a=a>=0?a:-a;b=b>=0?b:-b;return a===b?0:a>b?1:-1};BigInteger.prototype.compare=function(v){if(v===Infinity){return-1}if(v===-Infinity){return 1}var n=parseValue(v),a=this.value,b=n.value;if(this.sign!==n.sign){return n.sign?1:-1}if(n.isSmall){return this.sign?-1:1}return compareAbs(a,b)*(this.sign?-1:1)};BigInteger.prototype.compareTo=BigInteger.prototype.compare;SmallInteger.prototype.compare=function(v){if(v===Infinity){return-1}if(v===-Infinity){return 1}var n=parseValue(v),a=this.value,b=n.value;if(n.isSmall){return a==b?0:a>b?1:-1}if(a<0!==n.sign){return a<0?-1:1}return a<0?1:-1};SmallInteger.prototype.compareTo=SmallInteger.prototype.compare;NativeBigInt.prototype.compare=function(v){if(v===Infinity){return-1}if(v===-Infinity){return 1}var a=this.value;var b=parseValue(v).value;return a===b?0:a>b?1:-1};NativeBigInt.prototype.compareTo=NativeBigInt.prototype.compare;BigInteger.prototype.equals=function(v){return this.compare(v)===0};NativeBigInt.prototype.eq=NativeBigInt.prototype.equals=SmallInteger.prototype.eq=SmallInteger.prototype.equals=BigInteger.prototype.eq=BigInteger.prototype.equals;BigInteger.prototype.notEquals=function(v){return this.compare(v)!==0};NativeBigInt.prototype.neq=NativeBigInt.prototype.notEquals=SmallInteger.prototype.neq=SmallInteger.prototype.notEquals=BigInteger.prototype.neq=BigInteger.prototype.notEquals;BigInteger.prototype.greater=function(v){return this.compare(v)>0};NativeBigInt.prototype.gt=NativeBigInt.prototype.greater=SmallInteger.prototype.gt=SmallInteger.prototype.greater=BigInteger.prototype.gt=BigInteger.prototype.greater;BigInteger.prototype.lesser=function(v){return this.compare(v)<0};NativeBigInt.prototype.lt=NativeBigInt.prototype.lesser=SmallInteger.prototype.lt=SmallInteger.prototype.lesser=BigInteger.prototype.lt=BigInteger.prototype.lesser;BigInteger.prototype.greaterOrEquals=function(v){return this.compare(v)>=0};NativeBigInt.prototype.geq=NativeBigInt.prototype.greaterOrEquals=SmallInteger.prototype.geq=SmallInteger.prototype.greaterOrEquals=BigInteger.prototype.geq=BigInteger.prototype.greaterOrEquals;BigInteger.prototype.lesserOrEquals=function(v){return this.compare(v)<=0};NativeBigInt.prototype.leq=NativeBigInt.prototype.lesserOrEquals=SmallInteger.prototype.leq=SmallInteger.prototype.lesserOrEquals=BigInteger.prototype.leq=BigInteger.prototype.lesserOrEquals;BigInteger.prototype.isEven=function(){return(this.value[0]&1)===0};SmallInteger.prototype.isEven=function(){return(this.value&1)===0};NativeBigInt.prototype.isEven=function(){return(this.value&BigInt(1))===BigInt(0)};BigInteger.prototype.isOdd=function(){return(this.value[0]&1)===1};SmallInteger.prototype.isOdd=function(){return(this.value&1)===1};NativeBigInt.prototype.isOdd=function(){return(this.value&BigInt(1))===BigInt(1)};BigInteger.prototype.isPositive=function(){return!this.sign};SmallInteger.prototype.isPositive=function(){return this.value>0};NativeBigInt.prototype.isPositive=SmallInteger.prototype.isPositive;BigInteger.prototype.isNegative=function(){return this.sign};SmallInteger.prototype.isNegative=function(){return this.value<0};NativeBigInt.prototype.isNegative=SmallInteger.prototype.isNegative;BigInteger.prototype.isUnit=function(){return false};SmallInteger.prototype.isUnit=function(){return Math.abs(this.value)===1};NativeBigInt.prototype.isUnit=function(){return this.abs().value===BigInt(1)};BigInteger.prototype.isZero=function(){return false};SmallInteger.prototype.isZero=function(){return this.value===0};NativeBigInt.prototype.isZero=function(){return this.value===BigInt(0)};BigInteger.prototype.isDivisibleBy=function(v){var n=parseValue(v);if(n.isZero())return false;if(n.isUnit())return true;if(n.compareAbs(2)===0)return this.isEven();return this.mod(n).isZero()};NativeBigInt.prototype.isDivisibleBy=SmallInteger.prototype.isDivisibleBy=BigInteger.prototype.isDivisibleBy;function isBasicPrime(v){var n=v.abs();if(n.isUnit())return false;if(n.equals(2)||n.equals(3)||n.equals(5))return true;if(n.isEven()||n.isDivisibleBy(3)||n.isDivisibleBy(5))return false;if(n.lesser(49))return true}function millerRabinTest(n,a){var nPrev=n.prev(),b=nPrev,r=0,d,t,i,x;while(b.isEven())b=b.divide(2),r++;next:for(i=0;i<a.length;i++){if(n.lesser(a[i]))continue;x=bigInt(a[i]).modPow(b,n);if(x.isUnit()||x.equals(nPrev))continue;for(d=r-1;d!=0;d--){x=x.square().mod(n);if(x.isUnit())return false;if(x.equals(nPrev))continue next}return false}return true}BigInteger.prototype.isPrime=function(strict){var isPrime=isBasicPrime(this);if(isPrime!==undefined)return isPrime;var n=this.abs();var bits=n.bitLength();if(bits<=64)return millerRabinTest(n,[2,3,5,7,11,13,17,19,23,29,31,37]);var logN=Math.log(2)*bits.toJSNumber();var t=Math.ceil(strict===true?2*Math.pow(logN,2):logN);for(var a=[],i=0;i<t;i++){a.push(bigInt(i+2))}return millerRabinTest(n,a)};NativeBigInt.prototype.isPrime=SmallInteger.prototype.isPrime=BigInteger.prototype.isPrime;BigInteger.prototype.isProbablePrime=function(iterations){var isPrime=isBasicPrime(this);if(isPrime!==undefined)return isPrime;var n=this.abs();var t=iterations===undefined?5:iterations;for(var a=[],i=0;i<t;i++){a.push(bigInt.randBetween(2,n.minus(2)))}return millerRabinTest(n,a)};NativeBigInt.prototype.isProbablePrime=SmallInteger.prototype.isProbablePrime=BigInteger.prototype.isProbablePrime;BigInteger.prototype.modInv=function(n){var t=bigInt.zero,newT=bigInt.one,r=parseValue(n),newR=this.abs(),q,lastT,lastR;while(!newR.isZero()){q=r.divide(newR);lastT=t;lastR=r;t=newT;r=newR;newT=lastT.subtract(q.multiply(newT));newR=lastR.subtract(q.multiply(newR))}if(!r.isUnit())throw new Error(this.toString()+" and "+n.toString()+" are not co-prime");if(t.compare(0)===-1){t=t.add(n)}if(this.isNegative()){return t.negate()}return t};NativeBigInt.prototype.modInv=SmallInteger.prototype.modInv=BigInteger.prototype.modInv;BigInteger.prototype.next=function(){var value=this.value;if(this.sign){return subtractSmall(value,1,this.sign)}return new BigInteger(addSmall(value,1),this.sign)};SmallInteger.prototype.next=function(){var value=this.value;if(value+1<MAX_INT)return new SmallInteger(value+1);return new BigInteger(MAX_INT_ARR,false)};NativeBigInt.prototype.next=function(){return new NativeBigInt(this.value+BigInt(1))};BigInteger.prototype.prev=function(){var value=this.value;if(this.sign){return new BigInteger(addSmall(value,1),true)}return subtractSmall(value,1,this.sign)};SmallInteger.prototype.prev=function(){var value=this.value;if(value-1>-MAX_INT)return new SmallInteger(value-1);return new BigInteger(MAX_INT_ARR,true)};NativeBigInt.prototype.prev=function(){return new NativeBigInt(this.value-BigInt(1))};var powersOfTwo=[1];while(2*powersOfTwo[powersOfTwo.length-1]<=BASE)powersOfTwo.push(2*powersOfTwo[powersOfTwo.length-1]);var powers2Length=powersOfTwo.length,highestPower2=powersOfTwo[powers2Length-1];function shift_isSmall(n){return Math.abs(n)<=BASE}BigInteger.prototype.shiftLeft=function(v){var n=parseValue(v).toJSNumber();if(!shift_isSmall(n)){throw new Error(String(n)+" is too large for shifting.")}if(n<0)return this.shiftRight(-n);var result=this;if(result.isZero())return result;while(n>=powers2Length){result=result.multiply(highestPower2);n-=powers2Length-1}return result.multiply(powersOfTwo[n])};NativeBigInt.prototype.shiftLeft=SmallInteger.prototype.shiftLeft=BigInteger.prototype.shiftLeft;BigInteger.prototype.shiftRight=function(v){var remQuo;var n=parseValue(v).toJSNumber();if(!shift_isSmall(n)){throw new Error(String(n)+" is too large for shifting.")}if(n<0)return this.shiftLeft(-n);var result=this;while(n>=powers2Length){if(result.isZero()||result.isNegative()&&result.isUnit())return result;remQuo=divModAny(result,highestPower2);result=remQuo[1].isNegative()?remQuo[0].prev():remQuo[0];n-=powers2Length-1}remQuo=divModAny(result,powersOfTwo[n]);return remQuo[1].isNegative()?remQuo[0].prev():remQuo[0]};NativeBigInt.prototype.shiftRight=SmallInteger.prototype.shiftRight=BigInteger.prototype.shiftRight;function bitwise(x,y,fn){y=parseValue(y);var xSign=x.isNegative(),ySign=y.isNegative();var xRem=xSign?x.not():x,yRem=ySign?y.not():y;var xDigit=0,yDigit=0;var xDivMod=null,yDivMod=null;var result=[];while(!xRem.isZero()||!yRem.isZero()){xDivMod=divModAny(xRem,highestPower2);xDigit=xDivMod[1].toJSNumber();if(xSign){xDigit=highestPower2-1-xDigit}yDivMod=divModAny(yRem,highestPower2);yDigit=yDivMod[1].toJSNumber();if(ySign){yDigit=highestPower2-1-yDigit}xRem=xDivMod[0];yRem=yDivMod[0];result.push(fn(xDigit,yDigit))}var sum=fn(xSign?1:0,ySign?1:0)!==0?bigInt(-1):bigInt(0);for(var i=result.length-1;i>=0;i-=1){sum=sum.multiply(highestPower2).add(bigInt(result[i]))}return sum}BigInteger.prototype.not=function(){return this.negate().prev()};NativeBigInt.prototype.not=SmallInteger.prototype.not=BigInteger.prototype.not;BigInteger.prototype.and=function(n){return bitwise(this,n,function(a,b){return a&b})};NativeBigInt.prototype.and=SmallInteger.prototype.and=BigInteger.prototype.and;BigInteger.prototype.or=function(n){return bitwise(this,n,function(a,b){return a|b})};NativeBigInt.prototype.or=SmallInteger.prototype.or=BigInteger.prototype.or;BigInteger.prototype.xor=function(n){return bitwise(this,n,function(a,b){return a^b})};NativeBigInt.prototype.xor=SmallInteger.prototype.xor=BigInteger.prototype.xor;var LOBMASK_I=1<<30,LOBMASK_BI=(BASE&-BASE)*(BASE&-BASE)|LOBMASK_I;function roughLOB(n){var v=n.value,x=typeof v==="number"?v|LOBMASK_I:typeof v==="bigint"?v|BigInt(LOBMASK_I):v[0]+v[1]*BASE|LOBMASK_BI;return x&-x}function integerLogarithm(value,base){if(base.compareTo(value)<=0){var tmp=integerLogarithm(value,base.square(base));var p=tmp.p;var e=tmp.e;var t=p.multiply(base);return t.compareTo(value)<=0?{p:t,e:e*2+1}:{p:p,e:e*2}}return{p:bigInt(1),e:0}}BigInteger.prototype.bitLength=function(){var n=this;if(n.compareTo(bigInt(0))<0){n=n.negate().subtract(bigInt(1))}if(n.compareTo(bigInt(0))===0){return bigInt(0)}return bigInt(integerLogarithm(n,bigInt(2)).e).add(bigInt(1))};NativeBigInt.prototype.bitLength=SmallInteger.prototype.bitLength=BigInteger.prototype.bitLength;function max(a,b){a=parseValue(a);b=parseValue(b);return a.greater(b)?a:b}function min(a,b){a=parseValue(a);b=parseValue(b);return a.lesser(b)?a:b}function gcd(a,b){a=parseValue(a).abs();b=parseValue(b).abs();if(a.equals(b))return a;if(a.isZero())return b;if(b.isZero())return a;var c=Integer[1],d,t;while(a.isEven()&&b.isEven()){d=min(roughLOB(a),roughLOB(b));a=a.divide(d);b=b.divide(d);c=c.multiply(d)}while(a.isEven()){a=a.divide(roughLOB(a))}do{while(b.isEven()){b=b.divide(roughLOB(b))}if(a.greater(b)){t=b;b=a;a=t}b=b.subtract(a)}while(!b.isZero());return c.isUnit()?a:a.multiply(c)}function lcm(a,b){a=parseValue(a).abs();b=parseValue(b).abs();return a.divide(gcd(a,b)).multiply(b)}function randBetween(a,b){a=parseValue(a);b=parseValue(b);var low=min(a,b),high=max(a,b);var range=high.subtract(low).add(1);if(range.isSmall)return low.add(Math.floor(Math.random()*range));var digits=toBase(range,BASE).value;var result=[],restricted=true;for(var i=0;i<digits.length;i++){var top=restricted?digits[i]:BASE;var digit=truncate(Math.random()*top);result.push(digit);if(digit<top)restricted=false}return low.add(Integer.fromArray(result,BASE,false))}var parseBase=function(text,base,alphabet,caseSensitive){alphabet=alphabet||DEFAULT_ALPHABET;text=String(text);if(!caseSensitive){text=text.toLowerCase();alphabet=alphabet.toLowerCase()}var length=text.length;var i;var absBase=Math.abs(base);var alphabetValues={};for(i=0;i<alphabet.length;i++){alphabetValues[alphabet[i]]=i}for(i=0;i<length;i++){var c=text[i];if(c==="-")continue;if(c in alphabetValues){if(alphabetValues[c]>=absBase){if(c==="1"&&absBase===1)continue;throw new Error(c+" is not a valid digit in base "+base+".")}}}base=parseValue(base);var digits=[];var isNegative=text[0]==="-";for(i=isNegative?1:0;i<text.length;i++){var c=text[i];if(c in alphabetValues)digits.push(parseValue(alphabetValues[c]));else if(c==="<"){var start=i;do{i++}while(text[i]!==">"&&i<text.length);digits.push(parseValue(text.slice(start+1,i)))}else throw new Error(c+" is not a valid character")}return parseBaseFromArray(digits,base,isNegative)};function parseBaseFromArray(digits,base,isNegative){var val=Integer[0],pow=Integer[1],i;for(i=digits.length-1;i>=0;i--){val=val.add(digits[i].times(pow));pow=pow.times(base)}return isNegative?val.negate():val}function stringify(digit,alphabet){alphabet=alphabet||DEFAULT_ALPHABET;if(digit<alphabet.length){return alphabet[digit]}return"<"+digit+">"}function toBase(n,base){base=bigInt(base);if(base.isZero()){if(n.isZero())return{value:[0],isNegative:false};throw new Error("Cannot convert nonzero numbers to base 0.")}if(base.equals(-1)){if(n.isZero())return{value:[0],isNegative:false};if(n.isNegative())return{value:[].concat.apply([],Array.apply(null,Array(-n.toJSNumber())).map(Array.prototype.valueOf,[1,0])),isNegative:false};var arr=Array.apply(null,Array(n.toJSNumber()-1)).map(Array.prototype.valueOf,[0,1]);arr.unshift([1]);return{value:[].concat.apply([],arr),isNegative:false}}var neg=false;if(n.isNegative()&&base.isPositive()){neg=true;n=n.abs()}if(base.isUnit()){if(n.isZero())return{value:[0],isNegative:false};return{value:Array.apply(null,Array(n.toJSNumber())).map(Number.prototype.valueOf,1),isNegative:neg}}var out=[];var left=n,divmod;while(left.isNegative()||left.compareAbs(base)>=0){divmod=left.divmod(base);left=divmod.quotient;var digit=divmod.remainder;if(digit.isNegative()){digit=base.minus(digit).abs();left=left.next()}out.push(digit.toJSNumber())}out.push(left.toJSNumber());return{value:out.reverse(),isNegative:neg}}function toBaseString(n,base,alphabet){var arr=toBase(n,base);return(arr.isNegative?"-":"")+arr.value.map(function(x){return stringify(x,alphabet)}).join("")}BigInteger.prototype.toArray=function(radix){return toBase(this,radix)};SmallInteger.prototype.toArray=function(radix){return toBase(this,radix)};NativeBigInt.prototype.toArray=function(radix){return toBase(this,radix)};BigInteger.prototype.toString=function(radix,alphabet){if(radix===undefined)radix=10;if(radix!==10)return toBaseString(this,radix,alphabet);var v=this.value,l=v.length,str=String(v[--l]),zeros="0000000",digit;while(--l>=0){digit=String(v[l]);str+=zeros.slice(digit.length)+digit}var sign=this.sign?"-":"";return sign+str};SmallInteger.prototype.toString=function(radix,alphabet){if(radix===undefined)radix=10;if(radix!=10)return toBaseString(this,radix,alphabet);return String(this.value)};NativeBigInt.prototype.toString=SmallInteger.prototype.toString;NativeBigInt.prototype.toJSON=BigInteger.prototype.toJSON=SmallInteger.prototype.toJSON=function(){return this.toString()};BigInteger.prototype.valueOf=function(){return parseInt(this.toString(),10)};BigInteger.prototype.toJSNumber=BigInteger.prototype.valueOf;SmallInteger.prototype.valueOf=function(){return this.value};SmallInteger.prototype.toJSNumber=SmallInteger.prototype.valueOf;NativeBigInt.prototype.valueOf=NativeBigInt.prototype.toJSNumber=function(){return parseInt(this.toString(),10)};function parseStringValue(v){if(isPrecise(+v)){var x=+v;if(x===truncate(x))return supportsNativeBigInt?new NativeBigInt(BigInt(x)):new SmallInteger(x);throw new Error("Invalid integer: "+v)}var sign=v[0]==="-";if(sign)v=v.slice(1);var split=v.split(/e/i);if(split.length>2)throw new Error("Invalid integer: "+split.join("e"));if(split.length===2){var exp=split[1];if(exp[0]==="+")exp=exp.slice(1);exp=+exp;if(exp!==truncate(exp)||!isPrecise(exp))throw new Error("Invalid integer: "+exp+" is not a valid exponent.");var text=split[0];var decimalPlace=text.indexOf(".");if(decimalPlace>=0){exp-=text.length-decimalPlace-1;text=text.slice(0,decimalPlace)+text.slice(decimalPlace+1)}if(exp<0)throw new Error("Cannot include negative exponent part for integers");text+=new Array(exp+1).join("0");v=text}var isValid=/^([0-9][0-9]*)$/.test(v);if(!isValid)throw new Error("Invalid integer: "+v);if(supportsNativeBigInt){return new NativeBigInt(BigInt(sign?"-"+v:v))}var r=[],max=v.length,l=LOG_BASE,min=max-l;while(max>0){r.push(+v.slice(min,max));min-=l;if(min<0)min=0;max-=l}trim(r);return new BigInteger(r,sign)}function parseNumberValue(v){if(supportsNativeBigInt){return new NativeBigInt(BigInt(v))}if(isPrecise(v)){if(v!==truncate(v))throw new Error(v+" is not an integer.");return new SmallInteger(v)}return parseStringValue(v.toString())}function parseValue(v){if(typeof v==="number"){return parseNumberValue(v)}if(typeof v==="string"){return parseStringValue(v)}if(typeof v==="bigint"){return new NativeBigInt(v)}return v}for(var i=0;i<1e3;i++){Integer[i]=parseValue(i);if(i>0)Integer[-i]=parseValue(-i)}Integer.one=Integer[1];Integer.zero=Integer[0];Integer.minusOne=Integer[-1];Integer.max=max;Integer.min=min;Integer.gcd=gcd;Integer.lcm=lcm;Integer.isInstance=function(x){return x instanceof BigInteger||x instanceof SmallInteger||x instanceof NativeBigInt};Integer.randBetween=randBetween;Integer.fromArray=function(digits,base,isNegative){return parseBaseFromArray(digits.map(parseValue),parseValue(base||10),isNegative)};return Integer}();if(typeof module!=="undefined"&&module.hasOwnProperty("exports")){module.exports=bigInt}if(typeof define==="function"&&define.amd){define("big-integer",[],function(){return bigInt})}
  export function add(a, b) {
   return `${bigInt(a).add(bigInt(b))}`; // Fix me!
 }

 export function add2 (a, b) {
  var res = '', c = 0
  a = a.split('')
  b = b.split('')
  while (a.length || b.length || c) {
    c += ~~a.pop() + ~~b.pop()
  
    res = c % 10 + res
   
    c = c > 9
    
  }
  
  return res
}

var DP=20,RM=1,MAX_DP=1e6,MAX_POWER=1e6,NE=-7,PE=21,NAME="[big.js] ",INVALID=NAME+"Invalid ",INVALID_DP=INVALID+"decimal places",INVALID_RM=INVALID+"rounding mode",DIV_BY_ZERO=NAME+"Division by zero",P={},UNDEFINED=void 0,NUMERIC=/^-?(\d+(\.\d*)?|\.\d+)(e[+-]?\d+)?$/i;function _Big_(){function Big(n){var x=this;if(!(x instanceof Big))return n===UNDEFINED?_Big_():new Big(n);if(n instanceof Big){x.s=n.s;x.e=n.e;x.c=n.c.slice()}else{parse(x,n)}x.constructor=Big}Big.prototype=P;Big.DP=DP;Big.RM=RM;Big.NE=NE;Big.PE=PE;Big.version="5.2.2";return Big}function parse(x,n){var e,i,nl;if(n===0&&1/n<0)n="-0";else if(!NUMERIC.test(n+=""))throw Error(INVALID+"number");x.s=n.charAt(0)=="-"?(n=n.slice(1),-1):1;if((e=n.indexOf("."))>-1)n=n.replace(".","");if((i=n.search(/e/i))>0){if(e<0)e=i;e+=+n.slice(i+1);n=n.substring(0,i)}else if(e<0){e=n.length}nl=n.length;for(i=0;i<nl&&n.charAt(i)=="0";)++i;if(i==nl){x.c=[x.e=0]}else{for(;nl>0&&n.charAt(--nl)=="0";);x.e=e-i-1;x.c=[];for(e=0;i<=nl;)x.c[e++]=+n.charAt(i++)}return x}function round(x,dp,rm,more){var xc=x.c,i=x.e+dp+1;if(i<xc.length){if(rm===1){more=xc[i]>=5}else if(rm===2){more=xc[i]>5||xc[i]==5&&(more||i<0||xc[i+1]!==UNDEFINED||xc[i-1]&1)}else if(rm===3){more=more||!!xc[0]}else{more=!1;if(rm!==0)throw Error(INVALID_RM)}if(i<1){xc.length=1;if(more){x.e=-dp;xc[0]=1}else{xc[0]=x.e=0}}else{xc.length=i--;if(more){for(;++xc[i]>9;){xc[i]=0;if(!i--){++x.e;xc.unshift(1)}}}for(i=xc.length;!xc[--i];)xc.pop()}}else if(rm<0||rm>3||rm!==~~rm){throw Error(INVALID_RM)}return x}function stringify(x,id,n,k){var e,s,Big=x.constructor,z=!x.c[0];if(n!==UNDEFINED){if(n!==~~n||n<(id==3)||n>MAX_DP){throw Error(id==3?INVALID+"precision":INVALID_DP)}x=new Big(x);n=k-x.e;if(x.c.length>++k)round(x,n,Big.RM);if(id==2)k=x.e+n+1;for(;x.c.length<k;)x.c.push(0)}e=x.e;s=x.c.join("");n=s.length;if(id!=2&&(id==1||id==3&&k<=e||e<=Big.NE||e>=Big.PE)){s=s.charAt(0)+(n>1?"."+s.slice(1):"")+(e<0?"e":"e+")+e}else if(e<0){for(;++e;)s="0"+s;s="0."+s}else if(e>0){if(++e>n)for(e-=n;e--;)s+="0";else if(e<n)s=s.slice(0,e)+"."+s.slice(e)}else if(n>1){s=s.charAt(0)+"."+s.slice(1)}return x.s<0&&(!z||id==4)?"-"+s:s}P.abs=function(){var x=new this.constructor(this);x.s=1;return x};P.cmp=function(y){var isneg,x=this,xc=x.c,yc=(y=new x.constructor(y)).c,i=x.s,j=y.s,k=x.e,l=y.e;if(!xc[0]||!yc[0])return!xc[0]?!yc[0]?0:-j:i;if(i!=j)return i;isneg=i<0;if(k!=l)return k>l^isneg?1:-1;j=(k=xc.length)<(l=yc.length)?k:l;for(i=-1;++i<j;){if(xc[i]!=yc[i])return xc[i]>yc[i]^isneg?1:-1}return k==l?0:k>l^isneg?1:-1};P.div=function(y){var x=this,Big=x.constructor,a=x.c,b=(y=new Big(y)).c,k=x.s==y.s?1:-1,dp=Big.DP;if(dp!==~~dp||dp<0||dp>MAX_DP)throw Error(INVALID_DP);if(!b[0])throw Error(DIV_BY_ZERO);if(!a[0])return new Big(k*0);var bl,bt,n,cmp,ri,bz=b.slice(),ai=bl=b.length,al=a.length,r=a.slice(0,bl),rl=r.length,q=y,qc=q.c=[],qi=0,d=dp+(q.e=x.e-y.e)+1;q.s=k;k=d<0?0:d;bz.unshift(0);for(;rl++<bl;)r.push(0);do{for(n=0;n<10;n++){if(bl!=(rl=r.length)){cmp=bl>rl?1:-1}else{for(ri=-1,cmp=0;++ri<bl;){if(b[ri]!=r[ri]){cmp=b[ri]>r[ri]?1:-1;break}}}if(cmp<0){for(bt=rl==bl?b:bz;rl;){if(r[--rl]<bt[rl]){ri=rl;for(;ri&&!r[--ri];)r[ri]=9;--r[ri];r[rl]+=10}r[rl]-=bt[rl]}for(;!r[0];)r.shift()}else{break}}qc[qi++]=cmp?n:++n;if(r[0]&&cmp)r[rl]=a[ai]||0;else r=[a[ai]]}while((ai++<al||r[0]!==UNDEFINED)&&k--);if(!qc[0]&&qi!=1){qc.shift();q.e--}if(qi>d)round(q,dp,Big.RM,r[0]!==UNDEFINED);return q};P.eq=function(y){return!this.cmp(y)};P.gt=function(y){return this.cmp(y)>0};P.gte=function(y){return this.cmp(y)>-1};P.lt=function(y){return this.cmp(y)<0};P.lte=function(y){return this.cmp(y)<1};P.minus=P.sub=function(y){var i,j,t,xlty,x=this,Big=x.constructor,a=x.s,b=(y=new Big(y)).s;if(a!=b){y.s=-b;return x.plus(y)}var xc=x.c.slice(),xe=x.e,yc=y.c,ye=y.e;if(!xc[0]||!yc[0]){return yc[0]?(y.s=-b,y):new Big(xc[0]?x:0)}if(a=xe-ye){if(xlty=a<0){a=-a;t=xc}else{ye=xe;t=yc}t.reverse();for(b=a;b--;)t.push(0);t.reverse()}else{j=((xlty=xc.length<yc.length)?xc:yc).length;for(a=b=0;b<j;b++){if(xc[b]!=yc[b]){xlty=xc[b]<yc[b];break}}}if(xlty){t=xc;xc=yc;yc=t;y.s=-y.s}if((b=(j=yc.length)-(i=xc.length))>0)for(;b--;)xc[i++]=0;for(b=i;j>a;){if(xc[--j]<yc[j]){for(i=j;i&&!xc[--i];)xc[i]=9;--xc[i];xc[j]+=10}xc[j]-=yc[j]}for(;xc[--b]===0;)xc.pop();for(;xc[0]===0;){xc.shift();--ye}if(!xc[0]){y.s=1;xc=[ye=0]}y.c=xc;y.e=ye;return y};P.mod=function(y){var ygtx,x=this,Big=x.constructor,a=x.s,b=(y=new Big(y)).s;if(!y.c[0])throw Error(DIV_BY_ZERO);x.s=y.s=1;ygtx=y.cmp(x)==1;x.s=a;y.s=b;if(ygtx)return new Big(x);a=Big.DP;b=Big.RM;Big.DP=Big.RM=0;x=x.div(y);Big.DP=a;Big.RM=b;return this.minus(x.times(y))};P.plus=P.add=function(y){var t,x=this,Big=x.constructor,a=x.s,b=(y=new Big(y)).s;if(a!=b){y.s=-b;return x.minus(y)}var xe=x.e,xc=x.c,ye=y.e,yc=y.c;if(!xc[0]||!yc[0])return yc[0]?y:new Big(xc[0]?x:a*0);xc=xc.slice();if(a=xe-ye){if(a>0){ye=xe;t=yc}else{a=-a;t=xc}t.reverse();for(;a--;)t.push(0);t.reverse()}if(xc.length-yc.length<0){t=yc;yc=xc;xc=t}a=yc.length;for(b=0;a;xc[a]%=10)b=(xc[--a]=xc[a]+yc[a]+b)/10|0;if(b){xc.unshift(b);++ye}for(a=xc.length;xc[--a]===0;)xc.pop();y.c=xc;y.e=ye;return y};P.pow=function(n){var x=this,one=new x.constructor(1),y=one,isneg=n<0;if(n!==~~n||n<-MAX_POWER||n>MAX_POWER)throw Error(INVALID+"exponent");if(isneg)n=-n;while(!0){if(n&1)y=y.times(x);n>>=1;if(!n)break;x=x.times(x)}return isneg?one.div(y):y};P.round=function(dp,rm){var Big=this.constructor;if(dp===UNDEFINED)dp=0;else if(dp!==~~dp||dp<-MAX_DP||dp>MAX_DP)throw Error(INVALID_DP);return round(new Big(this),dp,rm===UNDEFINED?Big.RM:rm)};P.sqrt=function(){var r,c,t,x=this,Big=x.constructor,s=x.s,e=x.e,half=new Big(.5);if(!x.c[0])return new Big(x);if(s<0)throw Error(NAME+"No square root");s=Math.sqrt(x+"");if(s===0||s===1/0){c=x.c.join("");if(!(c.length+e&1))c+="0";s=Math.sqrt(c);e=((e+1)/2|0)-(e<0||e&1);r=new Big((s==1/0?"1e":(s=s.toExponential()).slice(0,s.indexOf("e")+1))+e)}else{r=new Big(s)}e=r.e+(Big.DP+=4);do{t=r;r=half.times(t.plus(x.div(t)))}while(t.c.slice(0,e).join("")!==r.c.slice(0,e).join(""));return round(r,Big.DP-=4,Big.RM)};P.times=P.mul=function(y){var c,x=this,Big=x.constructor,xc=x.c,yc=(y=new Big(y)).c,a=xc.length,b=yc.length,i=x.e,j=y.e;y.s=x.s==y.s?1:-1;if(!xc[0]||!yc[0])return new Big(y.s*0);y.e=i+j;if(a<b){c=xc;xc=yc;yc=c;j=a;a=b;b=j}for(c=new Array(j=a+b);j--;)c[j]=0;for(i=b;i--;){b=0;for(j=a+i;j>i;){b=c[j]+yc[i]*xc[j-i-1]+b;c[j--]=b%10;b=b/10|0}c[j]=(c[j]+b)%10}if(b)++y.e;else c.shift();for(i=c.length;!c[--i];)c.pop();y.c=c;return y};P.toExponential=function(dp){return stringify(this,1,dp,dp)};P.toFixed=function(dp){return stringify(this,2,dp,this.e+dp)};P.toPrecision=function(sd){return stringify(this,3,sd,sd-1)};P.toString=function(){return stringify(this)};P.valueOf=P.toJSON=function(){return stringify(this,4)};
let big =  _Big_();
function multiply(a, b) {
  // handle floats 
  console.log(big(a).times(big(b)).toString());
 if (a.includes('.')||b.includes('.')) { 
      // if both are floats 
      let arr = big(a).times(big(b))
      let sign = arr.s.toString()[0]
      let val = arr.toString();
      let result = arr.c;
      if (arr.c.length<2) { // 
        result = arr.toString();

      } 
 // if there is exponential 
     if (val.includes('e')) {
        
         result.splice(arr.e+1,0,'.');
      
     } 
      if (sign === '-' ) { 
  result.unshift('-');
      }
      return result.join('')
 }
 else {
 return doMath(a,b)
 }
  
  //
}
function doMath(a,b) { 
 // check for a negative sign
  let isOneNegative  = a.includes('-')|| b.includes('-');
  let AreBothNegative = a.includes('-')&& b.includes('-');
  // create two array from string a and b 
  let [right,left] = [a.split('').filter(v=> (/\d/g).test(v)), b.split('').filter(v=> (/\d/g).test(v))];

  // longest and shortest common
  let longest = left.length >= right.length ? left : right;
  let shortest = left.length >= right.length ? right : left;


  //create a result array
  let result = [];
   let count =0;
   longest = longest.reverse();
  // loop through the shortest array  reversely 
   for (let i = shortest.length;i>=0;i--) {
   
      // 
      let rem = 0;
       let mult = longest.map(ele=> { 
     if (ele&& shortest[i]) {
            let n =  (rem+ (parseInt(ele) * parseInt(shortest[i]))).toString()
             rem = n.length>1 ? parseInt(n[0]):0;
             let res  = n.length>1?parseInt(n[1]): parseInt(n[0])
              //console.log(`${shortest[i]} * ${ele} = ${n} remainder ${rem}`)
              if (n.length>0)  { return    res} 

     }

       }).reverse().join('');

         if (rem>0) { 
            mult = rem.toString() + mult
         }
       //   loop through the longest array
       let output =  count >1?'0'.repeat(count-1):''; 
       let final  =   mult+output
        if (final.length>0) {
          result.push(final);
        } 
        else { }
     
      count++;

  }
   let  f = result.reduce((c,n) => longAddition(c,n),'');
   return  isOneNegative && !AreBothNegative? '-'+f :  f;

}
function multiply2(a, b) {
  // handle floats 
  console.log(parseInt(a))

 if(!Number.isInteger(a)||!Number.isInteger(b)) { 
      // if both are floats 
      let arr = big(a).times(big(b))
      let sign = arr.s.toString()[0]
      let val = arr.toString();
      let result =arr.c;
      
      if (arr.c.length<2 && arr.c[0]===0) { // 
        result = arr.c
    return result.toString()
      } 
 // if there is exponential 
     if (val.includes('e')) {
       if (arr.e+1>result.length) {
         result.splice(arr.e+1,0,'.');
       }
         if (sign === '-' ) { 
          console.log( typeof result)
    result.unshift('-');
        }
        return result.join('')
     } 

     else { 
       return val
     }
      
 }
 else {
 return  big(a).times(big(b)).toString()
 }
  
  //
}

export const RomanNumerals = {};

RomanNumerals.toRoman = function(n) { 

let res = '';
let roman = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
let arabic = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
 for(let i=0;i<13;i++) { 

    while (n >= arabic[i]) {
        res += roman[i];
        n -= arabic[i];
    }
 }

return res

} 


RomanNumerals.fromRoman = function(s) {

let a = [];
 for (let c of s) { 
    switch (c) {
    case "I":
      a.push(1);
      break;
    case "V":
      a.push(5);
      break;
    case "X":
      a.push(10);
      break;
    case "L":
      a.push(50);
      break;
    case "C":
      a.push(100);
      break;
    case "D":
      a.push(500);
      break;
    case "M":
      a.push(1000);
      break;
    }
 }
    let res = a[a.length - 1],size = a.length;
for (let i =0;i<size-1;i++) {
 if (a[i]>=a[i+1]) {
    res += a[i];
 }
    else {
        res -= a[i];
    }

 }
 return res;

}



 //console.log(multiply2("86425673780839342594739977518938854481470531040072592413690623405174242", "1206870133414937016358360133741542591435196083747442088"));
 
function* Numbers (n) {
    for (let i = 0; i <n; i++) {
        yield i;
    }
}
 function solve(input) { 
    let values = parseStr(input);
    //console.log('before',values)
  values.sort((a,b)=>  a[0].localeCompare(b[0]));
  
    let ans = new Array(values.length).fill(0);
   
    return find(0,ans,[...Numbers(10)],values)
 }
 function parseStr(str) {
    let ans = {}// hashmap
    let leading  = new Set() // hashset
    let prev =''
    let value = -1;
    for ( let char of [...str].reverse()) { 
        // test for A..=Z 
        if (char.charCodeAt(0) >= 65 && char.charCodeAt(0) <= 90) {
           // console.log(ans[char])
      ans[char]? ans[char] : ans[char] = 0;
      ans[char] += value;
        value *= 10;
        prev = char;
        } else {
           
            value =1;
            leading.add(prev);
        }

  
       
 }
       leading.add(prev);
       return Object.entries(ans).map(([k, v])=> [k,v,leading.has(k)])

}

// calc function 
function calc(ans,values) {
 let  total =0;

 for (let i=0; i<ans.length; i++) {
    total += ans[i]*values[i][1];
  
 }
  return total ===0


}

// find function 
function find(count,ans,digits,values) {  
    //console.log('now',count)
    let sol;   
   if (count === values.length)  {
    //console.log(calc(ans,values))
         if (calc(ans,values)) {
           let result = {};
              for (let i=0; i<ans.length; i++) {
                result[values[i][0]] = ans[i];
                }
              return result
         }
        else {
         return null;
        }
   }

    for (let i=0; i<digits.length; i++) {
             //console.log(digits[i], values[count][2]) 
  if (digits[i] == 0 && values[count][2]) continue;
   
    ans[count] = digits[i];// set the value

    let digits2 = digits.map(v=>v)

    // remove element at i from digits2 
      digits2.splice(i,1);
      //console.log(digits2)

      sol = find(count+1,ans,digits2,values);
      
   
     if (sol) { 
// built an object;

  return sol;
     }
     
    }
    return null
 
}


 export function alphametics(str) {
	//your code goes here. you can do it!
	let result = solve(str);
   
	return str.split('').map(e=> {
        if (e.charCodeAt(0) >= 65 && e.charCodeAt(0) <= 90) {
            return result[e]
        } else {
            return e
        }
    }).join('')
}

