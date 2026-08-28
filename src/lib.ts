import type { Product, QuoteRequest } from './data';

export function money(cents:number|null|undefined,currency:string):string {
  if (cents == null) return 'POA';
  return new Intl.NumberFormat('en',{style:'currency',currency}).format(cents/100);
}

export function parseCsv(text:string):Product[] {
  const rows:string[][]=[]; let row:string[]=[]; let cell=''; let quoted=false;
  for(let i=0;i<text.length;i++){ const c=text[i];
    if(c==='"' && quoted && text[i+1]==='"'){cell+='"';i++;}
    else if(c==='"'){quoted=!quoted;}
    else if(c===',' && !quoted){row.push(cell.trim());cell='';}
    else if((c==='\n'||c==='\r')&&!quoted){if(c==='\r'&&text[i+1]==='\n')i++;row.push(cell.trim());if(row.some(Boolean))rows.push(row);row=[];cell='';}
    else cell+=c;
  }
  row.push(cell.trim()); if(row.some(Boolean))rows.push(row);
  if(rows.length<2) throw new Error('The CSV needs a header and at least one product row.');
  const headers=rows[0].map(h=>h.toLowerCase().replace(/\s+/g,'_'));
  for(const needed of ['sku','name']) if(!headers.includes(needed)) throw new Error(`The CSV needs a ${needed} column.`);
  const val=(r:string[],key:string)=>r[headers.indexOf(key)]??'';
  return rows.slice(1).map((r,i)=>{
    const raw=val(r,'price'); const price=raw===''?null:Number(raw);
    if(!val(r,'sku')||!val(r,'name')||(price!==null&&(!Number.isFinite(price)||price<0))) throw new Error(`Row ${i+2} needs a SKU, name, and a valid price or blank POA price.`);
    return {id:crypto.randomUUID(),sku:val(r,'sku'),name:val(r,'name'),description:val(r,'description'),category:val(r,'category')||'Products',price_cents:price===null?null:Math.round(price*100),stock_note:val(r,'stock_note')||'Ask about availability'};
  });
}

export function requestsCsv(requests:QuoteRequest[]):string {
  const q=(v:unknown)=>`"${String(v??'').replaceAll('"','""')}"`;
  const lines=['Request,Date,Company,Client,Email,PO,SKU,Product,Quantity,Unit price'];
  for(const r of requests) for(const l of r.lines) lines.push([r.id,r.created_at,r.company,r.client_name,r.email,r.po_number,l.sku,l.name,l.quantity,l.price_cents==null?'POA':(l.price_cents/100).toFixed(2)].map(q).join(','));
  return lines.join('\n');
}
