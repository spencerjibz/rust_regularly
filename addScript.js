 let {readdirSync,lstatSync,readFileSync} = require("fs");
 let {execSync,exec} = require('child_process');

 let args = process.argv;
 let cmd = args[2]

 // get a list of subdirectories  excluding .git
 let subDir = readdirSync(process.cwd())
 .filter(v=> lstatSync(v).isDirectory()&!v.includes('git'))

// get all these directories to the module tree
let count =0;

subDir.forEach(dir=> { 

    // check if the dir exists in .gitmodules
 let exists = readFileSync('.gitmodules',{encoding:'utf8'}).includes(dir)
 if (!exists) { 
     // enter directory -V 
    
 process.chdir('./'+dir);

  exec(cmd,(err,stdout,stderr)=> { 


 if (err||stderr) { 
     //  remove and submodules associate with filename 
     console.log('failed' +'' + err&& err!==null?err.message:stderr);
 }
 else { 
   count++
   console.log(`${count} of ${subDir.length} added`)

 }
  })

  process.chdir('../')
  
  
 }






})
