import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

type Claim = { id:string; test:string };

const root=process.cwd();
const claims=JSON.parse(readFileSync(resolve(root,'.factory/claims.json'),'utf8')) as Claim[];
const testSources=[
  'tests/e2e/claims.spec.ts',
  'tests/lib.test.ts',
  'tests/dockerfile.test.ts',
].map(file=>readFileSync(resolve(root,file),'utf8')).join('\n');
const rustSources=['src/main.rs','src/api.rs'].map(file=>readFileSync(resolve(root,file),'utf8')).join('\n');

describe('claims manifest',()=>{
  it('has unique IDs and a real test for every declared claim',()=>{
    expect(new Set(claims.map(claim=>claim.id)).size).toBe(claims.length);
    for(const claim of claims){
      expect(claim.test,claim.id).toBeTruthy();
      if(claim.test.includes(`@claim:${claim.id}`)){
        const occurrences=[...testSources.matchAll(new RegExp(`@claim:${claim.id}(?![a-z0-9-])`,'g'))];
        expect(occurrences,`${claim.id} must tag exactly one test`).toHaveLength(1);
      }else if(claim.test.startsWith('cargo test ')){
        const name=claim.test.slice('cargo test '.length).trim();
        const occurrences=[...rustSources.matchAll(new RegExp(`fn\\s+${name}\\s*\\(`,'g'))];
        expect(occurrences,`${claim.id} must name exactly one Rust test`).toHaveLength(1);
      }else{
        throw new Error(`${claim.id} does not point to a supported claim test`);
      }
    }
  });

  it('does not contain undeclared browser or unit claim tags',()=>{
    const declared=new Set(claims.map(claim=>claim.id));
    const tagged=[...testSources.matchAll(/@claim:([a-z0-9-]+)/g)].map(match=>match[1]);
    expect(tagged.filter(id=>!declared.has(id))).toEqual([]);
  });
});
