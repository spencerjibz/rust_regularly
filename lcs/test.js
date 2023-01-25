 function solution (n) {
    let gcd = (a,b) => b ===0?a : gcd(b,a%b);
    return n.reduce((c,n) => gcd(c,n))*n.length;

 }

   console.log(solution([3,13,23,7,28]))