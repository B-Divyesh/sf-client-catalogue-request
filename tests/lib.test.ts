import { describe, expect, it } from 'vitest';
import { money, parseCsv, requestsCsv } from '../src/lib';
import { sampleRequests } from '../src/data';

describe('catalogue files',()=>{
  it('parses quoted fields and blank POA prices',()=>{
    const rows=parseCsv('sku,name,description,category,price,stock_note\nA-1,"Cup, blue",Glazed,Cups,,Made to order');
    expect(rows[0]).toMatchObject({sku:'A-1',name:'Cup, blue',price_cents:null,stock_note:'Made to order'});
    expect(money(rows[0].price_cents,'GBP')).toBe('POA');
  });
  it('rejects missing required headings',()=>expect(()=>parseCsv('title,price\nCup,2')).toThrow(/sku column/));
  it('writes one CSV row per request line',()=>{
    const csv=requestsCsv(sampleRequests);
    expect(csv.split('\n')).toHaveLength(3);
    expect(csv).toContain('NW-101');
    expect(csv).toContain('POA');
  });
});
