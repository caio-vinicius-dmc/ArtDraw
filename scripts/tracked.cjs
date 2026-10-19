const fs=require('fs'),path=require('path');
const root=path.resolve(__dirname,'..'),memory=path.join(root,'ARCHITECTURE_MEMORY.md');
function put(file,content,description){let m=fs.readFileSync(memory,'utf8');fs.writeFileSync(memory,m+`\n- EM EXECUÇÃO: ${file} — ${description}.\n`);const destination=path.join(root,file);fs.mkdirSync(path.dirname(destination),{recursive:true});fs.writeFileSync(destination,content);m=fs.readFileSync(memory,'utf8');fs.writeFileSync(memory,m+`- CONCLUÍDO: ${file}.\n`);}
function read(file){return fs.readFileSync(path.join(root,file),'utf8').replace(/\r\n/g,'\n');}
function replace(source,oldText,newText){if(!source.includes(oldText))throw new Error('Trecho ausente: '+oldText.slice(0,100));return source.replace(oldText,newText);}
function edit(file,operation,description){put(file,operation(read(file)),description);}
module.exports={put,read,replace,edit,root};