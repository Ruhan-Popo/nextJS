module.exports = "\"use strict\";var h=Object.defineProperty,A=Object.getOwnPropertyDescriptor,D=Object.getOwnPropertyNames,N=Object.prototype.hasOwnProperty,p=(e,n)=>h(e,\"name\",{value:n,configurable:!0}),J=(e,n)=>{for(var s in n)h(e,s,{get:n[s],enumerable:!0})},Y=(e,n,s,i)=>{if(n&&typeof n==\"object\"||typeof n==\"function\")for(let t of D(n))!N.call(e,t)&&t!==s&&h(e,t,{get:()=>n[t],enumerable:!(i=A(n,t))||i.enumerable});return e},x=e=>Y(h({},\"__esModule\",{value:!0}),e),R={};J(R,{structuredClone:()=>B});module.exports=x(R);var z=-1,l=0,_=1,d=2,m=3,S=4,P=5,E=6,C=7,M=8,I=typeof self==\"object\"?self:globalThis,G=p((e,n)=>{let s=p((t,u)=>(e.set(u,t),t),\"as\"),i=p(t=>{if(e.has(t))return e.get(t);let[u,r]=n[t];switch(u){case l:case z:return s(r,t);case _:{let o=s([],t);for(let c of r)o.push(i(c));return o}case d:{let o=s({},t);for(let[c,b]of r)o[i(c)]=i(b);return o}case m:return s(new Date(r),t);case S:{let{source:o,flags:c}=r;return s(new RegExp(o,c),t)}case P:{let o=s(new Map,t);for(let[c,b]of r)o.set(i(c),i(b));return o}case E:{let o=s(new Set,t);for(let c of r)o.add(i(c));return o}case C:{let{name:o,message:c}=r;return s(new I[o](c),t)}case M:return s(BigInt(r),t);case\"BigInt\":return s(Object(BigInt(r)),t)}return s(new I[u](r),t)},\"unpair\");return i},\"deserializer\"),T=p(e=>G(new Map,e)(0),\"deserialize\"),g=\"\",{toString:V}={},{keys:v}=Object,w=p(e=>{let n=typeof e;if(n!==\"object\"||!e)return[l,n];let s=V.call(e).slice(8,-1);switch(s){case\"Array\":return[_,g];case\"Object\":return[d,g];case\"Date\":return[m,g];case\"RegExp\":return[S,g];case\"Map\":return[P,g];case\"Set\":return[E,g]}return s.includes(\"Array\")?[_,s]:s.includes(\"Error\")?[C,s]:[d,s]},\"typeOf\"),O=p(([e,n])=>e===l&&(n===\"function\"||n===\"symbol\"),\"shouldSkip\"),X=p((e,n,s,i)=>{let t=p((r,o)=>{let c=i.push(r)-1;return s.set(o,c),c},\"as\"),u=p(r=>{if(s.has(r))return s.get(r);let[o,c]=w(r);switch(o){case l:{let a=r;switch(c){case\"bigint\":o=M,a=r.toString();break;case\"function\":case\"symbol\":if(e)throw new TypeError(\"unable to serialize \"+c);a=null;break;case\"undefined\":return t([z],r)}return t([o,a],r)}case _:{if(c)return t([c,[...r]],r);let a=[],y=t([o,a],r);for(let f of r)a.push(u(f));return y}case d:{if(c)switch(c){case\"BigInt\":return t([c,r.toString()],r);case\"Boolean\":case\"Number\":case\"String\":return t([c,r.valueOf()],r)}if(n&&\"toJSON\"in r)return u(r.toJSON());let a=[],y=t([o,a],r);for(let f of v(r))(e||!O(w(r[f])))&&a.push([u(f),u(r[f])]);return y}case m:return t([o,r.toISOString()],r);case S:{let{source:a,flags:y}=r;return t([o,{source:a,flags:y}],r)}case P:{let a=[],y=t([o,a],r);for(let[f,j]of r)(e||!(O(w(f))||O(w(j))))&&a.push([u(f),u(j)]);return y}case E:{let a=[],y=t([o,a],r);for(let f of r)(e||!O(w(f)))&&a.push(u(f));return y}}let{message:b}=r;return t([o,{name:c,message:b}],r)},\"pair\");return u},\"serializer\"),k=p((e,{json:n,lossy:s}={})=>{let i=[];return X(!(n||s),!!n,new Map,i)(e),i},\"serialize\"),q=typeof structuredClone==\"function\"?(e,n)=>n&&(\"json\"in n||\"lossy\"in n)?T(k(e,n)):structuredClone(e):(e,n)=>T(k(e,n));function B(e,n){if(e instanceof ReadableStream){let s=new TransformStream({});return e.pipeTo(s.writable),s.readable}return q(e,n)}p(B,\"structuredClone\");\n"