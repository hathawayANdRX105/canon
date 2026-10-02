(()=>{var op=Object.defineProperty;var ip=(e)=>e;function ap(e,t){this[e]=ip.bind(null,t)}var sp=(e,t)=>{for(var n in t)op(e,n,{get:t[n],enumerable:!0,configurable:!0,set:ap.bind(t,n)})};var Se=(e,t,n)=>()=>{if(e)try{t=e(e=0)}catch(r){n=[r]}if(n)throw n[0];return t};function Ki(){let e=globalThis;if(!e[Kr]){let t=!1,n={providers:new Set,listeners:new Set,attributes:new WeakMap,notify(){if(t)return;t=!0,queueMicrotask(()=>{t=!1;for(let r of n.listeners)r()})}};e[Kr]=n}return e[Kr]}var Gt,Kr;var Wi=Se(()=>{Gt=Object.freeze({generation:0,variantId:"original"}),Kr=Symbol.for("ainotation.ui-variants.v1")});function Kn(e){let t=Object.values(e).filter((r)=>typeof r==="number");return Object.entries(e).filter(([r,o])=>t.indexOf(+r)===-1).map(([r,o])=>o)}function Gr(e,t="|"){return e.map((n)=>Qr(n)).join(t)}function mn(e,t){if(typeof t==="bigint")return t.toString();return t}function gn(e){return{get value(){{let n=e();return Object.defineProperty(this,"value",{value:n}),n}throw Error("cached value already set")}}}function Gi(e){return e===null||e===void 0}function Wn(e){let t=e.startsWith("^")?1:0,n=e.endsWith("$")?e.length-1:e.length;return e.slice(t,n)}function Xi(e,t){let n=e/t,r=Math.round(n),o=4*Number.EPSILON*Math.max(Math.abs(n),1);if(Math.abs(n-r)<o)return 0;return n-r}function Ue(e,t,n){Object.defineProperty(e,t,{value:n,writable:!0,enumerable:!0,configurable:!0})}function xt(...e){let t={};for(let n of e){let r=Object.getOwnPropertyDescriptors(n);Object.assign(t,r)}return Object.defineProperties({},t)}function Yi(e){return JSON.stringify(e)}function Qi(e){return e.toLowerCase().trim().replace(/[^\w\s-]/g,"").replace(/[\s_-]+/g,"-").replace(/^-+|-+$/g,"")}function Xt(e){return typeof e==="object"&&e!==null&&!Array.isArray(e)}function At(e){if(Xt(e)===!1)return!1;let t=e.constructor;if(t===void 0)return!0;if(typeof t!=="function")return!0;let n=t.prototype;if(Xt(n)===!1)return!1;if(Object.prototype.hasOwnProperty.call(n,"isPrototypeOf")===!1)return!1;return!0}function Yr(e){if(At(e))return{...e};if(Array.isArray(e))return[...e];if(e instanceof Map)return new Map(e);if(e instanceof Set)return new Set(e);return e}function _t(e){return e.replace(/[.*+?^${}()|[\]\\]/g,"\\$&")}function lt(e,t,n){let r=new e._zod.constr(t??e._zod.def);if(!t||n?.parent)r._zod.parent=e;return r}function ae(e){let t=e;if(!t)return{};if(typeof t==="string")return{error:()=>t};if(t?.message!==void 0){if(t?.error!==void 0)throw Error("Cannot specify both `message` and `error` params");t.error=t.message}if(delete t.message,typeof t.error==="string")return{...t,error:()=>t.error};return t}function Qr(e){if(typeof e==="bigint")return e.toString()+"n";if(typeof e==="string")return`"${e}"`;return`${e}`}function na(e){return Object.keys(e).filter((t)=>e[t]._zod.optin!==void 0&&e[t]._zod.optout==="optional")}function cp(e,t){let n=e._zod.def,r=n.checks;if(r&&r.length>0)throw Error(".pick() cannot be used on object schemas containing refinements");let i=xt(e._zod.def,{get shape(){let a={};for(let s of Reflect.ownKeys(t)){if(!Object.prototype.hasOwnProperty.call(n.shape,s))throw Error(`Unrecognized key: "${String(s)}"`);if(!t[s])continue;Ue(a,s,n.shape[s])}return Ue(this,"shape",a),a},checks:[]});return lt(e,i)}function lp(e,t){let n=e._zod.def,r=n.checks;if(r&&r.length>0)throw Error(".omit() cannot be used on object schemas containing refinements");let i=xt(e._zod.def,{get shape(){let a={...e._zod.def.shape};for(let s of Reflect.ownKeys(t)){if(!Object.prototype.hasOwnProperty.call(n.shape,s))throw Error(`Unrecognized key: "${String(s)}"`);if(!t[s])continue;delete a[s]}return Ue(this,"shape",a),a},checks:[]});return lt(e,i)}function up(e,t){if(!At(t))throw Error("Invalid input to extend: expected a plain object");let n=e._zod.def.checks;if(n&&n.length>0){let i=e._zod.def.shape;for(let a of Reflect.ownKeys(t))if(Object.getOwnPropertyDescriptor(i,a)!==void 0)throw Error("Cannot overwrite keys on object schemas containing refinements. Use `.safeExtend()` instead.")}let o=xt(e._zod.def,{get shape(){let i={...e._zod.def.shape,...t};return Ue(this,"shape",i),i}});return lt(e,o)}function dp(e,t){if(!At(t))throw Error("Invalid input to safeExtend: expected a plain object");let n=xt(e._zod.def,{get shape(){let r={...e._zod.def.shape,...t};return Ue(this,"shape",r),r}});return lt(e,n)}function pp(e,t){if(!t?._zod?.def)throw Error("Invalid input to merge: expected an object schema. To merge a plain shape, use `.extend()`.");if(e._zod.def.checks?.length)throw Error(".merge() cannot be used on object schemas containing refinements. Use .safeExtend() instead.");let n=xt(e._zod.def,{get shape(){let r={...e._zod.def.shape,...t._zod.def.shape};return Ue(this,"shape",r),r},get catchall(){return t._zod.def.catchall},checks:t._zod.def.checks??[]});return lt(e,n)}function oa(e,t,n,r="partial"){let i=t._zod.def.checks;if(i&&i.length>0)throw Error(`.${r}() cannot be used on object schemas containing refinements`);let s=xt(t._zod.def,{get shape(){let c=t._zod.def.shape,l={...c};if(n)for(let p of Reflect.ownKeys(n)){if(!Object.prototype.hasOwnProperty.call(c,p))throw Error(`Unrecognized key: "${String(p)}"`);if(!n[p])continue;l[p]=e?new e({type:"optional",innerType:c[p]}):c[p]}else for(let p of Reflect.ownKeys(c))l[p]=e?new e({type:"optional",innerType:c[p]}):c[p];return Ue(this,"shape",l),l},checks:[]});return lt(t,s)}function fp(e,t,n){let r=xt(t._zod.def,{get shape(){let o=t._zod.def.shape,i={...o};if(n)for(let a of Reflect.ownKeys(n)){if(!Object.prototype.hasOwnProperty.call(i,a))throw Error(`Unrecognized key: "${String(a)}"`);if(!n[a])continue;i[a]=new e({type:"nonoptional",innerType:o[a]})}else for(let a of Reflect.ownKeys(o))i[a]=new e({type:"nonoptional",innerType:o[a]});return Ue(this,"shape",i),i}});return lt(t,r)}function Et(e,t=0){if(e.aborted===!0)return!0;for(let n=t;n<e.issues.length;n++)if(e.issues[n]?.continue!==!0)return!0;return!1}function ia(e,t=0){if(e.aborted===!0)return!0;for(let n=t;n<e.issues.length;n++)if(e.issues[n]?.continue===!1)return!0;return!1}function kt(e,t){return t.map((n)=>{var r;return(r=n).path??(r.path=[]),n.path.unshift(e),n})}function hn(e){return typeof e==="string"?e:e?.message}function eo(e,t,n){var r;for(let o=t;o<e.length;o++)(r=e[o]).schema??(r.schema=n)}function ut(e,t,n){var r;let o=e.inst?._zod?.traits;if(o?.has("$ZodType"))if(o.has("$ZodCheck"))(r=e).schema??(r.schema=e.inst);else e.schema=e.inst;let i=e.schema!==e.inst?e.schema?._zod.def?.error:void 0,a=e.message?e.message:hn(e.inst?._zod.def?.error?.(e))??hn(i?.(e))??hn(t?.error?.(e))??hn(n.customError?.(e))??hn(n.localeError?.(e))??"Invalid input",{inst:s,schema:c,continue:l,input:p,...h}=e;if(h.path??(h.path=[]),h.message=a,t?.reportInput)h.input=p;return h}function Gn(e){let t=e.length;if(!hp.test(e))return t;let n=t;for(let r=0;r<t-1;r++)if((e.charCodeAt(r)&64512)===55296&&(e.charCodeAt(r+1)&64512)===56320)n--,r++;return n}function Xn(e){if(Array.isArray(e))return"array";if(typeof e==="string")return"string";return"unknown"}function aa(e){let t=typeof e;switch(t){case"number":return Number.isNaN(e)?"nan":"number";case"object":{if(e===null)return"null";if(Array.isArray(e))return"array";let n=e;if(n&&Object.getPrototypeOf(n)!==Object.prototype&&"constructor"in n&&n.constructor)return n.constructor.name}}return t}function Tt(...e){let[t,n,r]=e;if(typeof t==="string")return{message:t,code:"custom",input:n,inst:r};return{...t}}function sa(e,t){for(let n in t){let r=Object.getOwnPropertyDescriptor(t,n);if(r.get)Object.defineProperty(e,n,{...r,enumerable:!1});else mp(e,n,r.value)}}function Pt(e,t,n,r=!0){return Object.defineProperty(e,t,{configurable:!0,writable:!0,enumerable:r,value:n}),n}function to(e,t,n){return Pt(e,t,n,!1)}function mp(e,t,n){Object.defineProperty(e,t,{configurable:!0,get(){return this==null?n:Pt(this,t,n.bind(this))},set(r){Pt(this,t,r)}})}function gp(e,t){let n=Object.getPrototypeOf(e);return t in n?void 0:n}function Ae(e,t,n){let r=Object.getPrototypeOf(e._zod);if(t in r&&Wr!==e._zod){Wr=void 0;return}Wr=e._zod,Object.defineProperty(r,t,{configurable:!0,get(){Object.defineProperty(this,t,yp);let o=$t;$t=!1;try{let i=n(this);if($t)delete this[t];else Object.defineProperty(this,t,{configurable:!0,writable:!0,value:i});return $t=$t||o,i}catch(i){throw delete this[t],$t=$t||o,i}},set(o){Object.defineProperty(this,t,{configurable:!0,writable:!0,value:o})}})}function bp(e,t,n,r){let o=gp(e,t);if(!o)return;Object.defineProperty(o,t,{configurable:!0,get(){let i={configurable:!0,writable:!0,enumerable:r,value:void 0};return Object.defineProperty(this,t,i),i.value=n(this),Object.defineProperty(this,t,i),i.value},set(i){Object.defineProperty(this,t,{configurable:!0,writable:!0,enumerable:r,value:i})}})}function ca(e){let t=()=>e;return t[vp]=!0,t}var Xr,ea,ta,ra,hp,Wr,$t=!1,yp,vp="~constantCatch";var et=Se(()=>{Ot();Xr="captureStackTrace"in Error?Error.captureStackTrace:(...e)=>{};ea=gn(()=>{if(Xe.jitless)return!1;if(typeof navigator<"u"&&navigator?.userAgent?.includes("Cloudflare"))return!1;try{return new Function(""),!0}catch(e){return!1}});ta=new Set(["string","number","symbol"]);ra=(()=>({safeint:[Number.MIN_SAFE_INTEGER,Number.MAX_SAFE_INTEGER],int32:[-2147483648,2147483647],uint32:[0,4294967295],float32:[-340282346638528860000000000000000000000,340282346638528860000000000000000000000],float64:[-Number.MAX_VALUE,Number.MAX_VALUE]}))();hp=/[\uD800-\uDBFF]/;yp={configurable:!0,get(){$t=!0;return}}});function wp(e){let t=ua;if(t){let n=t.stackTraceLimit;if(typeof n==="number"){try{t.stackTraceLimit=0}catch{return ua=null,new e}try{return new e}finally{t.stackTraceLimit=n}}}return new e}function O(e,t,n,r){let o={};function i(d){this.def=d,this.constr=h,this.traits=new Set}i.prototype=o;let a=n,s=a&&new WeakSet;function c(d,u){if(!d._zod){no.value=new i(u);try{Object.defineProperty(d,"_zod",no)}finally{no.value=void 0}}if(d._zod.traits.has(e))return;if(d._zod.traits.add(e),t(d,u),s){let x=Object.getPrototypeOf(d),k=d._zod.constr.prototype,w=x;while(w&&w!==k)w=Object.getPrototypeOf(w);let P=w??x;if(!s.has(P))s.add(P),sa(P,a)}let f=h.prototype;for(let x in f){if(!Object.prototype.hasOwnProperty.call(f,x))continue;if(!(x in d))d[x]=f[x].bind(d)}}let l=r?.Parent??Object;class p extends l{}Object.defineProperty(p,"name",{value:e});function h(d){let u=r?.Parent?wp(p):this;c(u,d);let f=u._zod.deferred;if(f){for(let k of f)k();u._zod.deferred=void 0}let x=globalThis.__zod_globalConfig?.postProcessor;if(x)x(u);return u}return Object.defineProperty(h,"init",{value:c}),Object.defineProperty(h,Symbol.hasInstance,{value:(d)=>{if(r?.Parent&&d instanceof r.Parent)return!0;return d?._zod?.traits?.has(e)}}),Object.defineProperty(h,"name",{value:e}),h}function tt(e){if(e)Object.assign(Xe,e);return Xe}var la,no,ua,gt,yn,Xe;var Ot=Se(()=>{et();no={value:void 0,enumerable:!1},ua="captureStackTrace"in Error?Error:null;gt=class gt extends Error{constructor(){super("Encountered Promise during synchronous parse. Use .parseAsync() instead.")}};yn=class yn extends Error{constructor(e){super(`Encountered unidirectional transform during encode: ${e}`);this.name="ZodEncodeError"}};(la=globalThis).__zod_globalConfig??(la.__zod_globalConfig={});Xe=globalThis.__zod_globalConfig});function $p(){let e=this._zod;return e.message??(e.message=JSON.stringify(e.def,mn,2)),e.message}function xp(e){this._zod.message=e}function kp(e,t,n){if(!Object.prototype.hasOwnProperty.call(e,t))if(t==="__proto__")Object.defineProperty(e,t,{value:n(),writable:!0,enumerable:!0,configurable:!0});else e[t]=n();return e[t]}function fa(e,t=(n)=>n.message){let n={},r=[];for(let o of e.issues)if(o.path.length>0)kp(n,o.path[0],()=>[]).push(t(o));else r.push(t(o));return{formErrors:r,fieldErrors:n}}function ha(e,t=(n)=>n.message){let n={_errors:[]},r=(o,i=[])=>{for(let a of o.issues)if(a.code==="invalid_union"&&a.errors.length)a.errors.map((s)=>r({issues:s},[...i,...a.path]));else if(a.code==="invalid_key")r({issues:a.issues},[...i,...a.path]);else if(a.code==="invalid_element")r({issues:a.issues},[...i,...a.path]);else{let s=[...i,...a.path];if(s.length===0)n._errors.push(t(a));else{let c=n,l=0;while(l<s.length){let p=s[l],h=l===s.length-1;if(p==="_errors"){if(h)c._errors.push(t(a));l++;continue}if(!Object.prototype.hasOwnProperty.call(c,p))Object.defineProperty(c,p,{value:{_errors:[]},enumerable:!0,writable:!0,configurable:!0});let d=c[p];if(h)d._errors.push(t(a));c=d,l++}}}};return r(e),n}var _p,oo,io,da,pa=(e,t)=>{e.name="$ZodError",oo.value=e._zod,Object.defineProperty(e,"_zod",oo),io.value=t,Object.defineProperty(e,"issues",io),oo.value=void 0,io.value=void 0,Object.defineProperty(e,"message",_p);let n=Object.getPrototypeOf(e);if(!da.has(n))da.add(n),Object.defineProperty(n,"toString",{configurable:!0,enumerable:!1,get(){let r=()=>this.message;return Object.defineProperty(this,"toString",{value:r,configurable:!0,writable:!0}),r},set(r){Object.defineProperty(this,"toString",{value:r,configurable:!0,writable:!0})}})},Yn,ao;var so=Se(()=>{Ot();et();_p={get:$p,set:xp,enumerable:!0,configurable:!0},oo={value:void 0,enumerable:!1},io={value:void 0,enumerable:!1},da=new WeakSet([Object.prototype,Error.prototype]),Yn=O("$ZodError",pa),ao=O("$ZodError",pa,void 0,{Parent:Error})});function Qn(e,t){return{callee:t?.callee??e,Err:t?.Err}}var er=(e)=>{let t=(n,r,o,i)=>{let a=o?{...o,async:!1}:{async:!1},s=n._zod.run({value:r,issues:[]},a);if(s instanceof Promise)throw new gt;if(s.issues.length){let c=new(i?.Err??e)(s.issues.map((l)=>ut(l,a,tt())));throw Xr(c,i?.callee??t),c}return s.value};return t},tr=(e)=>{let t=async(n,r,o,i)=>{let a=o?{...o,async:!0}:{async:!0},s=n._zod.run({value:r,issues:[]},a);if(s instanceof Promise)s=await s;if(s.issues.length){let c=new(i?.Err??e)(s.issues.map((l)=>ut(l,a,tt())));throw Xr(c,i?.callee??t),c}return s.value};return t},bn=(e)=>(t,n,r)=>{let o=r?{...r,async:!1}:{async:!1},i=t._zod.run({value:n,issues:[]},o);if(i instanceof Promise)throw new gt;return i.issues.length?{success:!1,error:new(e??Yn)(i.issues.map((a)=>ut(a,o,tt())))}:{success:!0,data:i.value}},ma,vn=(e)=>async(t,n,r)=>{let o=r?{...r,async:!0}:{async:!0},i=t._zod.run({value:n,issues:[]},o);if(i instanceof Promise)i=await i;return i.issues.length?{success:!1,error:new e(i.issues.map((a)=>ut(a,o,tt())))}:{success:!0,data:i.value}},ga,ya=(e)=>{let t=er(e),n=(r,o,i,a)=>{let s=i?{...i,direction:"backward"}:{direction:"backward"};return t(r,o,s,Qn(n,a))};return n},ba=(e)=>{let t=er(e),n=(r,o,i,a)=>t(r,o,i,Qn(n,a));return n},va=(e)=>{let t=tr(e),n=async(r,o,i,a)=>{let s=i?{...i,direction:"backward"}:{direction:"backward"};return await t(r,o,s,Qn(n,a))};return n},wa=(e)=>{let t=tr(e),n=async(r,o,i,a)=>await t(r,o,i,Qn(n,a));return n},$a=(e)=>(t,n,r)=>{let o=r?{...r,direction:"backward"}:{direction:"backward"};return bn(e)(t,n,o)},xa=(e)=>(t,n,r)=>bn(e)(t,n,r),_a=(e)=>async(t,n,r)=>{let o=r?{...r,direction:"backward"}:{direction:"backward"};return vn(e)(t,n,o)},ka=(e)=>async(t,n,r)=>vn(e)(t,n,r);var co=Se(()=>{Ot();so();et();ma=bn(ao),ga=vn(ao)});function Ea(e){return new RegExp(`^[a-zA-Z0-9_-]{${e}}$`)}function Na(){return new RegExp(Ip,"u")}function Cp(e){return new RegExp(`^${e}$`)}function lo(e){return typeof e.precision==="number"?e.precision===-1?"(?:[01]\\d|2[0-3]):[0-5]\\d":e.precision===0?"(?:[01]\\d|2[0-3]):[0-5]\\d:[0-5]\\d":`(?:[01]\\d|2[0-3]):[0-5]\\d:[0-5]\\d\\.\\d{${e.precision}}`:e.seconds?"(?:[01]\\d|2[0-3]):[0-5]\\d:[0-5]\\d(?:\\.\\d+)?":"(?:[01]\\d|2[0-3]):[0-5]\\d(?::[0-5]\\d(?:\\.\\d+)?)?"}function Ha(e){return new RegExp(`^${lo(e)}$`)}function Ja(e){let t=["Z"];if(e.offset)t.push("([+-](?:[01]\\d|2[0-3]):[0-5]\\d)");let n=`${lo({precision:e.precision,seconds:!0})}(?:${t.join("|")})`,r=e.local?`${n}|${lo({precision:e.precision})}`:n;return new RegExp(`^${Ua}T(?:${r})$`)}var Sa,Ia,Ca,za,Pa,Aa,Ta,Oa,uo=(e)=>{if(!e)return/^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[1-8][0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}|00000000-0000-0000-0000-000000000000|ffffffff-ffff-ffff-ffff-ffffffffffff)$/;return new RegExp(`^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-${e}[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12})$`)},Ma,Ip="^[\\p{Extended_Pictographic}\\p{Emoji_Component}]+$",Da,La,Ra,Za,ja,po,Va,Fa,Ua="(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))",Ba,qa=(e)=>{let t=e?`[\\s\\S]{${e?.minimum??0},${e?.maximum??""}}`:"[\\s\\S]*";return new RegExp(`^${t}$`)},nr,wn,Ka,Wa,Ga;var xn=Se(()=>{Sa=/^[cC][0-9a-z]{6,}$/,Ia=/^[0-9a-z]+$/,Ca=/^[0-7][0-9A-HJKMNP-TV-Za-hjkmnp-tv-z]{25}$/,za=/^[0-9a-vA-V]{20}$/,Pa=/^[A-Za-z0-9]{27}$/,Aa=/^[a-zA-Z0-9_-]{21}$/;Ta=/^P(?:(\d+W)|(?!.*W)(?=\d|T\d)(\d+Y)?(\d+M)?(\d+D)?(T(?=\d)(\d+H)?(\d+M)?(\d+([.,]\d+)?S)?)?)$/,Oa=/^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})$/,Ma=/^(?!\.)(?!.*\.\.)([A-Za-z0-9_'+\-\.]*)[A-Za-z0-9_+-]@([A-Za-z0-9][A-Za-z0-9\-]*\.)+[A-Za-z]{2,}$/;Da=/^(?:(?:25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9][0-9]|[0-9])\.){3}(?:25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9][0-9]|[0-9])$/,La=/^(([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:))$/,Ra=/^((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9][0-9]|[0-9])\.){3}(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9][0-9]|[0-9])\/([0-9]|[1-2][0-9]|3[0-2])$/,Za=/^(([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:))\/(12[0-8]|1[01][0-9]|[1-9]?[0-9])$/,ja=/^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$/,po=/^[A-Za-z0-9_-]*$/,Va=/^https?$/,Fa=/^\+[1-9]\d{6,14}$/;Ba=Cp(Ua);nr=/^-?\d+$/,wn=/^-?\d+(?:\.\d+)?$/,Ka=/^(?:true|false)$/i,Wa=/^[^A-Z]*$/,Ga=/^[^a-z]*$/});var Je,fo=(e)=>{let t=e.value;return!Gi(t)&&t.length!==void 0},rr,ho,mo,Xa,Ya,Qa,es,ts,_n,ns,rs,os,is,as,ss,cs;var or=Se(()=>{Ot();xn();et();Je=O("$ZodCheck",(e,t)=>{var n;e._zod??(e._zod={}),e._zod.def=t,(n=e._zod).onattach??(n.onattach=[])}),rr={number:"number",bigint:"bigint",object:"date"},ho=O("$ZodCheckLessThan",(e,t)=>{Je.init(e,t);let n=rr[typeof t.value];e._zod.onattach.push((r)=>{let o=r._zod.bag,i=(t.inclusive?o.maximum:o.exclusiveMaximum)??Number.POSITIVE_INFINITY;if(t.value<i)if(t.inclusive)o.maximum=t.value;else o.exclusiveMaximum=t.value}),e._zod.check=(r)=>{if(t.inclusive?r.value<=t.value:r.value<t.value)return;r.issues.push({origin:rr[typeof r.value]??n,code:"too_big",maximum:typeof t.value==="object"?t.value.getTime():t.value,input:r.value,inclusive:t.inclusive,inst:e,continue:!t.abort})}}),mo=O("$ZodCheckGreaterThan",(e,t)=>{Je.init(e,t);let n=rr[typeof t.value];e._zod.onattach.push((r)=>{let o=r._zod.bag,i=(t.inclusive?o.minimum:o.exclusiveMinimum)??Number.NEGATIVE_INFINITY;if(t.value>i)if(t.inclusive)o.minimum=t.value;else o.exclusiveMinimum=t.value}),e._zod.check=(r)=>{if(t.inclusive?r.value>=t.value:r.value>t.value)return;r.issues.push({origin:rr[typeof r.value]??n,code:"too_small",minimum:typeof t.value==="object"?t.value.getTime():t.value,input:r.value,inclusive:t.inclusive,inst:e,continue:!t.abort})}}),Xa=O("$ZodCheckMultipleOf",(e,t)=>{Je.init(e,t),e._zod.onattach.push((n)=>{var r;(r=n._zod.bag).multipleOf??(r.multipleOf=t.value)}),e._zod.check=(n)=>{if(typeof n.value!==typeof t.value)throw Error("Cannot mix number and bigint in multiple_of check.");if(typeof n.value==="bigint"?t.value!==BigInt(0)&&n.value%t.value===BigInt(0):Xi(n.value,t.value)===0)return;n.issues.push({origin:typeof n.value,code:"not_multiple_of",divisor:t.value,input:n.value,inst:e,continue:!t.abort})}}),Ya=O("$ZodCheckNumberFormat",(e,t)=>{Je.init(e,t),t.format=t.format||"float64";let n=t.format?.includes("int"),r=n?"int":"number",[o,i]=ra[t.format];e._zod.onattach.push((a)=>{let s=a._zod.bag;if(s.format=t.format,s.minimum=o,s.maximum=i,n)s.pattern=nr}),e._zod.check=(a)=>{let s=a.value;if(n){if(!Number.isInteger(s)){a.issues.push({expected:r,format:t.format,code:"invalid_type",continue:!1,input:s,inst:e});return}if(!Number.isSafeInteger(s)){if(s>0)a.issues.push({input:s,code:"too_big",maximum:Number.MAX_SAFE_INTEGER,note:"Integers must be within the safe integer range.",inst:e,origin:r,inclusive:!0,continue:!t.abort});else a.issues.push({input:s,code:"too_small",minimum:Number.MIN_SAFE_INTEGER,note:"Integers must be within the safe integer range.",inst:e,origin:r,inclusive:!0,continue:!t.abort});return}}if(s<o)a.issues.push({origin:"number",input:s,code:"too_small",minimum:o,inclusive:!0,inst:e,continue:!t.abort});if(s>i)a.issues.push({origin:"number",input:s,code:"too_big",maximum:i,inclusive:!0,inst:e,continue:!t.abort})}}),Qa=O("$ZodCheckMaxLength",(e,t)=>{var n;Je.init(e,t),(n=e._zod.def).when??(n.when=fo),e._zod.onattach.push((r)=>{let o=r._zod.bag.maximum??Number.POSITIVE_INFINITY;if(t.maximum<o)r._zod.bag.maximum=t.maximum}),e._zod.check=(r)=>{let o=r.value,i=o.length;if((typeof o==="string"&&i>t.maximum?Gn(o):i)<=t.maximum)return;let s=Xn(o);r.issues.push({origin:s,code:"too_big",maximum:t.maximum,inclusive:!0,input:o,inst:e,continue:!t.abort})}}),es=O("$ZodCheckMinLength",(e,t)=>{var n;Je.init(e,t),(n=e._zod.def).when??(n.when=fo),e._zod.onattach.push((r)=>{let o=r._zod.bag.minimum??Number.NEGATIVE_INFINITY;if(t.minimum>o)r._zod.bag.minimum=t.minimum}),e._zod.check=(r)=>{let o=r.value,i=o.length;if((typeof o==="string"&&i>=t.minimum&&i<t.minimum*2?Gn(o):i)>=t.minimum)return;let s=Xn(o);r.issues.push({origin:s,code:"too_small",minimum:t.minimum,inclusive:!0,input:o,inst:e,continue:!t.abort})}}),ts=O("$ZodCheckLengthEquals",(e,t)=>{var n;Je.init(e,t),(n=e._zod.def).when??(n.when=fo),e._zod.onattach.push((r)=>{let o=r._zod.bag;o.minimum=t.length,o.maximum=t.length,o.length=t.length}),e._zod.check=(r)=>{let o=r.value,i=o.length,a=typeof o==="string"&&i>=t.length&&i<=t.length*2?Gn(o):i;if(a===t.length)return;let s=Xn(o),c=a>t.length;r.issues.push({origin:s,...c?{code:"too_big",maximum:t.length}:{code:"too_small",minimum:t.length},inclusive:!0,exact:!0,input:r.value,inst:e,continue:!t.abort})}}),_n=O("$ZodCheckStringFormat",(e,t)=>{var n,r;if(Je.init(e,t),e._zod.onattach.push((o)=>{let i=o._zod.bag;if(i.format=t.format,t.pattern)i.patterns??(i.patterns=new Set),i.patterns.add(t.pattern)}),t.pattern)(n=e._zod).check??(n.check=(o)=>{if(t.pattern.lastIndex=0,t.pattern.test(o.value))return;o.issues.push({origin:"string",code:"invalid_format",format:t.format,input:o.value,...t.pattern?{pattern:t.pattern.toString()}:{},inst:e,continue:!t.abort})});else(r=e._zod).check??(r.check=()=>{})}),ns=O("$ZodCheckRegex",(e,t)=>{_n.init(e,t),e._zod.check=(n)=>{if(t.pattern.lastIndex=0,t.pattern.test(n.value))return;n.issues.push({origin:"string",code:"invalid_format",format:"regex",input:n.value,pattern:t.pattern.toString(),inst:e,continue:!t.abort})}}),rs=O("$ZodCheckLowerCase",(e,t)=>{t.pattern??(t.pattern=Wa),_n.init(e,t)}),os=O("$ZodCheckUpperCase",(e,t)=>{t.pattern??(t.pattern=Ga),_n.init(e,t)}),is=O("$ZodCheckIncludes",(e,t)=>{Je.init(e,t);let n=_t(t.includes),r=new RegExp(typeof t.position==="number"?`^.{${t.position},}${n}`:n);t.pattern=r,e._zod.onattach.push((o)=>{let i=o._zod.bag;i.patterns??(i.patterns=new Set),i.patterns.add(r)}),e._zod.check=(o)=>{if(o.value.includes(t.includes,t.position))return;o.issues.push({origin:"string",code:"invalid_format",format:"includes",includes:t.includes,input:o.value,inst:e,continue:!t.abort})}}),as=O("$ZodCheckStartsWith",(e,t)=>{Je.init(e,t);let n=new RegExp(`^${_t(t.prefix)}.*`);t.pattern??(t.pattern=n),e._zod.onattach.push((r)=>{let o=r._zod.bag;o.patterns??(o.patterns=new Set),o.patterns.add(n)}),e._zod.check=(r)=>{if(r.value.startsWith(t.prefix))return;r.issues.push({origin:"string",code:"invalid_format",format:"starts_with",prefix:t.prefix,input:r.value,inst:e,continue:!t.abort})}}),ss=O("$ZodCheckEndsWith",(e,t)=>{Je.init(e,t);let n=new RegExp(`.*${_t(t.suffix)}$`);t.pattern??(t.pattern=n),e._zod.onattach.push((r)=>{let o=r._zod.bag;o.patterns??(o.patterns=new Set),o.patterns.add(n)}),e._zod.check=(r)=>{if(r.value.endsWith(t.suffix))return;r.issues.push({origin:"string",code:"invalid_format",format:"ends_with",suffix:t.suffix,input:r.value,inst:e,continue:!t.abort})}}),cs=O("$ZodCheckOverwrite",(e,t)=>{Je.init(e,t),e._zod.check=(n)=>{n.value=t.tx(n.value)}})});class go{constructor(e=[],t={}){this.content=[],this.indent=0,this.args=e,this.closed=t}indented(e){this.indent+=1,e(this),this.indent-=1}write(e){if(typeof e==="function"){e(this,{execution:"sync"}),e(this,{execution:"async"});return}let n=e.split(`
`).filter((i)=>i),r=Math.min(...n.map((i)=>i.length-i.trimStart().length)),o=n.map((i)=>i.slice(r)).map((i)=>" ".repeat(this.indent*2)+i);for(let i of o)this.content.push(i)}compile(){let e=Function,t=this?.content??[""];return new e(...Object.keys(this.closed),`return function (${this.args.join(", ")}) {
${t.join(`
`)}
};`)(...Object.values(this.closed))}}var us;var yo=Se(()=>{us={major:4,minor:5,patch:4}});function wo(e){return{validate:(t)=>{try{return ds(ma(e,t))}catch(n){return ga(e,t).then(ds)}},vendor:"zod",version:1}}function Pp(e,t){if(!t.normalize&&t.protocol?.source===Va.source&&!/^https?:\/\//i.test(e))return ks;try{return new URL(e)}catch{return Ss}}function Ep(e){return e.replace(Ap,"")}function Tp(e,t){return t.lastIndex=0,t.test(e.hostname)}function Op(e,t){return t.lastIndex=0,t.test(e.protocol.endsWith(":")?e.protocol.slice(0,-1):e.protocol)}function Zs(e){if(!Mp.test(e))return!1;try{return new URL(`http://[${e}]`),!0}catch{return!1}}function Np(e){let t=e.split("/");if(t.length!==2)return!1;let[n,r]=t;if(!r)return!1;let o=Number(r);if(`${o}`!==r)return!1;if(o<0||o>128)return!1;return Zs(n)}function Us(e){if(e==="")return!0;if(/\s/.test(e))return!1;if(e.length%4!==0)return!1;try{return atob(e),!0}catch{return!1}}function Dp(e){if(!po.test(e))return!1;let t=e.replace(/[-_]/g,(r)=>r==="-"?"+":"/"),n=t.padEnd(Math.ceil(t.length/4)*4,"=");return Us(n)}function Lp(e,t=null){try{let n=e.split(".");if(n.length!==3)return!1;let[r]=n;if(!r)return!1;let o=JSON.parse(atob(r));if("typ"in o&&o?.typ!=="JWT")return!1;if(!o.alg)return!1;if(t&&(!("alg"in o)||o.alg!==t))return!1;return!0}catch{return!1}}function ps(e,t,n){if(e.issues.length)t.issues.push(...kt(n,e.issues));t.value[n]=e.value}function ar(e,t,n,r,o,i){let a=n in r,s=i==="optional";if(!a&&s&&o==="optional")return;if(e.issues.length){if(o!==void 0&&s&&!a)return;t.issues.push(...kt(n,e.issues))}if(!a&&o===void 0){if(!e.issues.length)t.issues.push({code:"invalid_type",expected:"nonoptional",input:void 0,path:[n]});return}if(e.value===void 0){if(a)t.value[n]=void 0}else t.value[n]=e.value}function Qs(e){let t=Object.keys(e.shape),n=Object.getOwnPropertySymbols(e.shape),r=n.length?n:Rp,o=r.length?[...t,...r]:t;for(let a of o)if(!e.shape?.[a]?._zod?.traits?.has("$ZodType"))throw Error(`Invalid element at key "${String(a)}": expected a Zod schema`);let i=na(e.shape);return{...e,allKeys:o,symbolKeys:r,keySet:new Set(t),numKeys:t.length,optionalKeys:new Set(i)}}function ec(e,t,n,r,o,i){let a=[],s=o.keySet,c=o.catchall._zod,l=c.def.type,{optin:p,optout:h}=c;for(let d in t){if(s.has(d))continue;if(d==="__proto__"){if(l==="never")a.push(d);continue}if(l==="never"){a.push(d);continue}let u=c.run({value:t[d],issues:[]},r);if(u instanceof Promise)e.push(u.then((f)=>ar(f,n,d,t,p,h)));else ar(u,n,d,t,p,h)}if(a.length)n.issues.push({code:"unrecognized_keys",keys:a,input:t,inst:i,continue:!0});if(!e.length)return n;return Promise.all(e).then(()=>n)}function fs(e,t,n,r){for(let i of e)if(i.issues.length===0)return t.value=i.value,t;let o=e.filter((i)=>!Et(i));if(o.length===1)return t.value=o[0].value,o[0];return t.issues.push({code:"invalid_union",input:t.value,inst:n,errors:e.map((i)=>i.issues.map((a)=>ut(a,r,tt())))}),t}function vo(e,t){if(e===t)return{valid:!0,data:e};if(e instanceof Date&&t instanceof Date&&+e===+t)return{valid:!0,data:e};if(At(e)&&At(t)){let n=Object.keys(t),r=Object.keys(e).filter((i)=>n.indexOf(i)!==-1),o={...e,...t};if(Object.prototype.hasOwnProperty.call(o,"__proto__"))delete o.__proto__;for(let i of r){if(i==="__proto__")continue;let a=vo(e[i],t[i]);if(!a.valid)return{valid:!1,mergeErrorPath:[i,...a.mergeErrorPath]};o[i]=a.data}return{valid:!0,data:o}}if(Array.isArray(e)&&Array.isArray(t)){if(e.length!==t.length)return{valid:!1,mergeErrorPath:[]};let n=[];for(let r=0;r<e.length;r++){let o=e[r],i=t[r],a=vo(o,i);if(!a.valid)return{valid:!1,mergeErrorPath:[r,...a.mergeErrorPath]};n.push(a.data)}return{valid:!0,data:n}}return{valid:!1,mergeErrorPath:[]}}function hs(e,t,n){let r=new Map,o,i=new Map,a=(l,p)=>{let h;if(l.code==="unrecognized_keys"&&!l.path?.length)o??(o=l),h=l.keys;else if(l.code==="invalid_key"&&l.origin==="record"&&l.path?.length===1){let d=String(l.path[0]);if(!i.has(d))i.set(d,l);h=[d]}else return!1;for(let d of h){if(!r.has(d))r.set(d,{});r.get(d)[p]=!0}return!0};for(let l of t.issues)if(!a(l,"l"))e.issues.push(l);for(let l of n.issues)if(!a(l,"r"))e.issues.push(l);let s=[...r].filter(([,l])=>l.l&&l.r).map(([l])=>l);if(s.length){let l=o?s.filter((p)=>o.keys.includes(p)):[];if(l.length)e.issues.push({...o,keys:l});for(let p of s)if(!l.includes(p)&&i.has(p))e.issues.push(i.get(p))}let c=vo(t.value,n.value);if(!c.valid){if(Et(e))return e;throw Error(`Unmergable intersection. Error path: ${JSON.stringify(c.mergeErrorPath)}`)}return e.value=c.data,e}function ms(e,t){return e.value=t.issues.length?void 0:t.value,e}function gs(e,t){if(e.value===void 0)e.value=t.defaultValue;return e}function ys(e,t){if(!e.issues.length&&e.value===void 0)e.issues.push({code:"invalid_type",expected:"nonoptional",input:e.value,inst:t});return e}function bs(e,t,n,r){if(!t.issues.length){if(e.value=t.value,t.memo)e.memo=!0;return e}return e.value=n.catchValue({...t,value:e.value,error:{issues:t.issues.map((o)=>ut(o,r,tt()))},input:e.value}),e}function ir(e,t,n){if(e.issues.some((r)=>r.code!=="unrecognized_keys"))return e.aborted=!0,e;return t._zod.run({value:e.value,issues:e.issues},n)}function vs(e){if(!e.memo)e.value=Object.freeze(e.value);return e}function ws(e,t,n,r){if(!e){let o={code:"custom",input:n,inst:r,path:[...r._zod.def.path??[]],continue:!r._zod.def.abort};if(r._zod.def.params)o.params=r._zod.def.params;t.issues.push(Tt(o))}}var De,ds=(e)=>e.success?{value:e.data}:{issues:e.error?.issues},sr,Me,$s,xs,_s,ks=1,Ss=2,Ap,Is,Cs,zs,Ps,As,Es,Ts,Os,Ms,Ns,Ds,Ls,Rs,Mp,js,Vs,Fs,Bs,Hs,Js,qs,$o,Ks,Ws,Gs,Xs,Ys,Rp,bo,Zp,tc,xo,nc,rc,oc,ic,ac,sc,_o,cc,lc,uc,dc,pc,fc,hc,mc,gc;var yc=Se(()=>{or();Ot();co();xn();et();yo();et();De=O("$ZodType",(e,t)=>{var n;e??(e={}),e._zod.def=t,e._zod.bag=e._zod.bag||{},e._zod.version=us;let r=e._zod.def.checks,o=e._zod.traits.has("$ZodCheck")?[e,...r??[]]:r?.length?[...r]:[];for(let i of o)for(let a of i._zod.onattach)a(e);if(o.length===0)(n=e._zod).deferred??(n.deferred=[]),e._zod.deferred?.push(()=>{e._zod.run=e._zod.parse});else{let i=(s,c,l)=>{if(s.memo)return s;let p=Et(s),h;for(let d of c){if(d._zod.def.when){if(ia(s))continue;if(!d._zod.def.when(s))continue}else if(p)continue;let u=s.issues.length,f=d._zod.check(s);if(f instanceof Promise&&l?.async===!1)throw new gt;if(h||f instanceof Promise)h=(h??Promise.resolve()).then(async()=>{if(await f,s.issues.length===u)return;if(eo(s.issues,u,e),!p)p=Et(s,u)});else{if(s.issues.length===u)continue;if(eo(s.issues,u,e),!p)p=Et(s,u)}}if(h)return h.then(()=>s);return s},a=(s,c,l)=>{if(Et(s))return s.aborted=!0,s;let p=i(c,o,l);if(p instanceof Promise){if(l.async===!1)throw new gt;return p.then((h)=>e._zod.parse(h,l))}return e._zod.parse(p,l)};e._zod.run=(s,c)=>{if(c.skipChecks)return e._zod.parse(s,c);if(c.direction==="backward"){let p=e._zod.parse({value:s.value,issues:[]},{...c,skipChecks:!0});if(p instanceof Promise)return p.then((h)=>a(h,s,c));return a(p,s,c)}let l=e._zod.parse(s,c);if(l instanceof Promise){if(c.async===!1)throw new gt;return l.then((p)=>i(p,o,c))}return i(l,o,c)}}},{get "~standard"(){return to(this,"~standard",wo(this))},set "~standard"(e){Pt(this,"~standard",e)}});sr=O("$ZodString",(e,t)=>{De.init(e,t),e._zod.pattern=[...e?._zod.bag?.patterns??[]].pop()??qa(e._zod.bag),e._zod.parse=(n,r)=>{if(t.coerce)try{n.value=String(n.value)}catch(o){}if(typeof n.value==="string")return n;return n.issues.push({expected:"string",code:"invalid_type",input:n.value,inst:e}),n}}),Me=O("$ZodStringFormat",(e,t)=>{_n.init(e,t),sr.init(e,t)}),$s=O("$ZodGUID",(e,t)=>{t.pattern??(t.pattern=Oa),Me.init(e,t)}),xs=O("$ZodUUID",(e,t)=>{if(t.version){let r={v1:1,v2:2,v3:3,v4:4,v5:5,v6:6,v7:7,v8:8}[t.version];if(r===void 0)throw Error(`Invalid UUID version: "${t.version}"`);t.pattern??(t.pattern=uo(r))}else t.pattern??(t.pattern=uo());Me.init(e,t)}),_s=O("$ZodEmail",(e,t)=>{t.pattern??(t.pattern=Ma),Me.init(e,t)});Ap=/[\t\n\r]/g;Is=O("$ZodURL",(e,t)=>{Me.init(e,t),e._zod.check=(n)=>{try{let r=n.value.trim(),o=Pp(r,t);if(o===ks){n.issues.push({code:"invalid_format",format:"url",note:"Invalid URL format",input:n.value,inst:e,continue:!t.abort});return}if(o===Ss){n.issues.push({code:"invalid_format",format:"url",input:n.value,inst:e,continue:!t.abort});return}if(t.hostname&&!Tp(o,t.hostname))n.issues.push({code:"invalid_format",format:"url",note:"Invalid hostname",pattern:t.hostname.source,input:n.value,inst:e,continue:!t.abort});if(t.protocol&&!Op(o,t.protocol))n.issues.push({code:"invalid_format",format:"url",note:"Invalid protocol",pattern:t.protocol.source,input:n.value,inst:e,continue:!t.abort});n.value=t.normalize?o.href:Ep(r);return}catch(r){n.issues.push({code:"invalid_format",format:"url",input:n.value,inst:e,continue:!t.abort})}}}),Cs=O("$ZodEmoji",(e,t)=>{t.pattern??(t.pattern=Na()),Me.init(e,t)}),zs=O("$ZodNanoID",(e,t)=>{if(t.length!==void 0&&(!Number.isInteger(t.length)||t.length<1))throw Error(`Invalid nanoid length: ${t.length}`);t.pattern??(t.pattern=t.length===void 0?Aa:Ea(t.length)),Me.init(e,t)}),Ps=O("$ZodCUID",(e,t)=>{t.pattern??(t.pattern=Sa),Me.init(e,t)}),As=O("$ZodCUID2",(e,t)=>{t.pattern??(t.pattern=Ia),Me.init(e,t)}),Es=O("$ZodULID",(e,t)=>{t.pattern??(t.pattern=Ca),Me.init(e,t)}),Ts=O("$ZodXID",(e,t)=>{t.pattern??(t.pattern=za),Me.init(e,t)}),Os=O("$ZodKSUID",(e,t)=>{t.pattern??(t.pattern=Pa),Me.init(e,t)}),Ms=O("$ZodISODateTime",(e,t)=>{if(t.pattern??(t.pattern=Ja(t)),Me.init(e,t),t.local||t.precision===-1)e._zod.bag.laxFormat=!0,e._zod.onattach.push((n)=>{n._zod.bag.laxFormat=!0})}),Ns=O("$ZodISODate",(e,t)=>{t.pattern??(t.pattern=Ba),Me.init(e,t)}),Ds=O("$ZodISOTime",(e,t)=>{t.pattern??(t.pattern=Ha(t)),Me.init(e,t)}),Ls=O("$ZodISODuration",(e,t)=>{t.pattern??(t.pattern=Ta),Me.init(e,t)}),Rs=O("$ZodIPv4",(e,t)=>{t.pattern??(t.pattern=Da),Me.init(e,t),e._zod.bag.format="ipv4"}),Mp=/^[0-9a-fA-F:.]+$/;js=O("$ZodIPv6",(e,t)=>{t.pattern??(t.pattern=La),Me.init(e,t),e._zod.bag.format="ipv6",e._zod.check=(n)=>{if(!Zs(n.value))n.issues.push({code:"invalid_format",format:"ipv6",input:n.value,inst:e,continue:!t.abort})}}),Vs=O("$ZodCIDRv4",(e,t)=>{t.pattern??(t.pattern=Ra),Me.init(e,t)});Fs=O("$ZodCIDRv6",(e,t)=>{t.pattern??(t.pattern=Za),Me.init(e,t),e._zod.check=(n)=>{if(!Np(n.value))n.issues.push({code:"invalid_format",format:"cidrv6",input:n.value,inst:e,continue:!t.abort})}});Bs=O("$ZodBase64",(e,t)=>{t.pattern??(t.pattern=ja),Me.init(e,t),e._zod.bag.contentEncoding="base64",e._zod.check=(n)=>{if(Us(n.value))return;n.issues.push({code:"invalid_format",format:"base64",input:n.value,inst:e,continue:!t.abort})}});Hs=O("$ZodBase64URL",(e,t)=>{t.pattern??(t.pattern=po),Me.init(e,t),e._zod.bag.contentEncoding="base64url",e._zod.check=(n)=>{if(Dp(n.value))return;n.issues.push({code:"invalid_format",format:"base64url",input:n.value,inst:e,continue:!t.abort})}}),Js=O("$ZodE164",(e,t)=>{t.pattern??(t.pattern=Fa),Me.init(e,t)});qs=O("$ZodJWT",(e,t)=>{Me.init(e,t),e._zod.check=(n)=>{if(Lp(n.value,t.alg))return;n.issues.push({code:"invalid_format",format:"jwt",input:n.value,inst:e,continue:!t.abort})}}),$o=O("$ZodNumber",(e,t)=>{De.init(e,t),e._zod.pattern=e._zod.bag.pattern??wn,e._zod.parse=(n,r)=>{if(t.coerce)try{n.value=Number(n.value)}catch(a){}let o=n.value;if(typeof o==="number"&&!Number.isNaN(o)&&Number.isFinite(o))return n;let i=typeof o==="number"?Number.isNaN(o)?"NaN":!Number.isFinite(o)?String(o):void 0:void 0;return n.issues.push({expected:"number",code:"invalid_type",input:o,inst:e,...i?{received:i}:{}}),n}}),Ks=O("$ZodNumberFormat",(e,t)=>{Ya.init(e,t),$o.init(e,t)}),Ws=O("$ZodBoolean",(e,t)=>{De.init(e,t),e._zod.pattern=Ka,e._zod.parse=(n,r)=>{if(t.coerce)try{n.value=Boolean(n.value)}catch(i){}let o=n.value;if(typeof o==="boolean")return n;return n.issues.push({expected:"boolean",code:"invalid_type",input:o,inst:e}),n}}),Gs=O("$ZodUnknown",(e,t)=>{De.init(e,t),e._zod.parse=(n)=>n}),Xs=O("$ZodNever",(e,t)=>{De.init(e,t),e._zod.parse=(n,r)=>(n.issues.push({expected:"never",code:"invalid_type",input:n.value,inst:e}),n)});Ys=O("$ZodArray",(e,t)=>{De.init(e,t);let n=Xe.memoizer;n?.attach(e),e._zod.parse=(r,o)=>{let i=r.value;if(!Array.isArray(i))return r.issues.push({expected:"array",code:"invalid_type",input:i,inst:e}),r;r.value=n?n.alloc(e,r,Array(i.length),o):Array(i.length);let a=[];for(let s=0;s<i.length;s++){let c=i[s],l=t.element._zod.run({value:c,issues:[]},o);if(l instanceof Promise)a.push(l.then((p)=>ps(p,r,s)));else ps(l,r,s)}if(a.length)return Promise.all(a).then(()=>r);return r}});Rp=[];bo=new WeakMap,Zp=O("$ZodObject",(e,t)=>{if(De.init(e,t),!Object.getOwnPropertyDescriptor(t,"shape")?.get){let c=t.shape;bo.set(t,c),Object.defineProperty(t,"shape",{get:()=>{let l={...c};return Object.defineProperty(t,"shape",{value:l}),bo.set(t,l),l}})}let r=gn(()=>Qs(t));Ae(e,"propValues",(c)=>{let l=c.def.shape,p={};for(let h in l){let d=l[h]._zod;if(d.values){if(!Object.prototype.hasOwnProperty.call(p,h))Ue(p,h,new Set);for(let u of d.values)p[h].add(u);if(d.optin!==void 0)p[h].add(void 0)}}return p});let o=Xt,i=t.catchall,a,s=Xe.memoizer;s?.attach(e),e._zod.parse=(c,l)=>{a??(a=r.value);let p=c.value;if(!o(p))return c.issues.push({expected:"object",code:"invalid_type",input:p,inst:e}),c;c.value=s?s.alloc(e,c,{},l):{};let h=[],d=a.shape;for(let u of a.allKeys){if(u==="__proto__")continue;let f=d[u],x=f._zod.optin,k=f._zod.optout,w=f._zod.run({value:p[u],issues:[]},l);if(w instanceof Promise)h.push(w.then((P)=>ar(P,c,u,p,x,k)));else ar(w,c,u,p,x,k)}if(!i)return h.length?Promise.all(h).then(()=>c):c;return ec(h,p,c,l,r.value,e)}}),tc=O("$ZodObjectJIT",(e,t)=>{Zp.init(e,t);let n=e._zod.parse,r=gn(()=>Qs(t)),o=Xe.memoizer,i=(u)=>{let f=r.value,x=f.symbolKeys,k=new go(["payload","ctx"],{shape:u,inst:e,memo:o,syms:x}),w=(Z)=>`shape[${Z}]._zod.run({ value: input[${Z}], issues: [] }, ctx)`,P=(Z,z)=>`
          for (let i = 0; i < ${Z}.issues.length; i++) {
            const iss = ${Z}.issues[i];
            iss.path = iss.path ? [${z}, ...iss.path] : [${z}];
            payload.issues.push(iss);
          }`;k.write("const input = payload.value;");let D=Object.create(null),T=0;for(let Z of f.allKeys)D[Z]=`key_${T++}`;k.write(o?"const newResult = memo.alloc(inst, payload, {}, ctx);":"const newResult = {};");for(let Z of f.allKeys){if(Z==="__proto__")continue;let z=D[Z],W=typeof Z==="symbol"?`syms[${x.indexOf(Z)}]`:Yi(Z),j=`${W} in input`,B=u[Z],ue=B?._zod?.optin,ve=ue!==void 0,H=B?._zod?.optout==="optional";if(k.write(`const ${z} = ${w(W)};`),ve&&H){let Y=ue==="optional"?`${z}_present`:`${z}.value !== undefined || ${z}_present`;k.write(`
        const ${z}_present = ${j};
        if (!${z}.issues.length || ${z}_present) {
          if (${z}.issues.length) {${P(z,W)}
          }

          if (${Y}) {
            newResult[${W}] = ${z}.value;
          }
        }

      `)}else if(!ve)k.write(`
        const ${z}_present = ${j};
        if (${z}.issues.length) {${P(z,W)}
        }
        if (!${z}_present && !${z}.issues.length) {
          payload.issues.push({
            code: "invalid_type",
            expected: "nonoptional",
            input: undefined,
            path: [${W}]
          });
        }

        if (${z}_present) {
          newResult[${W}] = ${z}.value;
        }

      `);else k.write(`
        if (${z}.issues.length) {${P(z,W)}
        }
        
        if (${z}.value === undefined) {
          if (${j}) {
            newResult[${W}] = undefined;
          }
        } else {
          newResult[${W}] = ${z}.value;
        }

      `)}return k.write("payload.value = newResult;"),k.write("return payload;"),k.compile()},a,s=Xt,c=!Xe.jitless,p=c&&ea.value,h=t.catchall,d;e._zod.parse=(u,f)=>{d??(d=r.value);let x=u.value;if(!s(x))return u.issues.push({expected:"object",code:"invalid_type",input:x,inst:e}),u;if(c&&p&&f?.async===!1&&f.jitless!==!0){if(!a)a=i(t.shape);if(u=a(u,f),!h)return u;return ec([],x,u,f,d,e)}return n(u,f)}});xo=O("$ZodUnion",(e,t)=>{De.init(e,t),Ae(e,"optin",(r)=>r.def.options.some((o)=>o._zod.optin==="defaulted")?"defaulted":r.def.options.some((o)=>o._zod.optin!==void 0)?"optional":void 0),Ae(e,"optout",(r)=>r.def.options.some((o)=>o._zod.optout==="optional")?"optional":void 0),Ae(e,"values",(r)=>{if(r.def.options.every((o)=>o._zod.values))return new Set(r.def.options.flatMap((o)=>Array.from(o._zod.values)));return}),Ae(e,"pattern",(r)=>{if(r.def.options.every((o)=>o._zod.pattern)){let o=r.def.options.map((i)=>i._zod.pattern);return new RegExp(`^(${o.map((i)=>Wn(i.source)).join("|")})$`)}return});let n=t.options.length===1?t.options[0]._zod.run:null;e._zod.parse=(r,o)=>{if(n)return n(r,o);let i=!1,a=[];for(let s of t.options){let c=s._zod.run({value:r.value,issues:[]},o);if(c instanceof Promise)a.push(c),i=!0;else{if(c.issues.length===0)return c;a.push(c)}}if(!i)return fs(a,r,e,o);return Promise.all(a).then((s)=>fs(s,r,e,o))}}),nc=O("$ZodDiscriminatedUnion",(e,t)=>{t.inclusive=!1,xo.init(e,t);let n=e._zod.parse;Ae(e,"propValues",(o)=>{let i={};for(let a of o.def.options){let s=a._zod.propValues;if(!s||Object.keys(s).length===0)throw Error(`Invalid discriminated union option at index "${o.def.options.indexOf(a)}"`);for(let[c,l]of Object.entries(s)){if(!Object.prototype.hasOwnProperty.call(i,c))Ue(i,c,new Set);for(let p of l)i[c].add(p)}}return i}),t.options.forEach((o,i)=>{let a=bo.get(o._zod.def);if(a&&!Object.prototype.hasOwnProperty.call(a,t.discriminator))throw Error(`Invalid discriminated union option at index "${i}"`)});let r=gn(()=>{let o=t.options,i=new Map;for(let a of o){let s=a._zod.propValues?.[t.discriminator];if(!s||s.size===0)throw Error(`Invalid discriminated union option at index "${t.options.indexOf(a)}"`);for(let c of s){if(i.has(c))throw Error(`Duplicate discriminator value "${String(c)}"`);i.set(c,a)}}return i});e._zod.parse=(o,i)=>{let a=o.value;if(!Xt(a))return o.issues.push({code:"invalid_type",expected:"object",input:a,inst:e}),o;let s=r.value.get(a?.[t.discriminator]);if(s)return s._zod.run(o,i);if(t.unionFallback||i.direction==="backward")return n(o,i);return o.issues.push({code:"invalid_union",errors:[],note:"No matching discriminator",discriminator:t.discriminator,options:Array.from(r.value.keys()),input:a,path:[t.discriminator],inst:e}),o}}),rc=O("$ZodIntersection",(e,t)=>{De.init(e,t),e._zod.parse=(n,r)=>{let o=n.value,i=t.left._zod.run({value:o,issues:[]},r),a=t.right._zod.run({value:o,issues:[]},r);if(i instanceof Promise||a instanceof Promise)return Promise.all([i,a]).then(([c,l])=>hs(n,c,l));return hs(n,i,a)}});oc=O("$ZodRecord",(e,t)=>{De.init(e,t);let n=Xe.memoizer;n?.attach(e),e._zod.parse=(r,o)=>{let i=r.value;if(!At(i))return r.issues.push({expected:"record",code:"invalid_type",input:i,inst:e}),r;let a=[],s=t.keyType._zod.values;if(s&&!t.partial){r.value=n?n.alloc(e,r,{},o):{};let c=new Set;for(let p of s)if(typeof p==="string"||typeof p==="number"||typeof p==="symbol"){if(c.add(typeof p==="number"?p.toString():p),p==="__proto__")continue;let h=t.keyType._zod.run({value:p,issues:[]},o);if(h instanceof Promise)throw Error("Async schemas not supported in object keys currently");if(h.issues.length){r.issues.push({code:"invalid_key",origin:"record",issues:h.issues.map((f)=>ut(f,o,tt())),input:p,path:[p],inst:e});continue}let d=h.value;if(d==="__proto__")continue;let u=t.valueType._zod.run({value:i[p],issues:[]},o);if(u instanceof Promise)a.push(u.then((f)=>{if(f.issues.length)r.issues.push(...kt(p,f.issues));r.value[d]=f.value}));else{if(u.issues.length)r.issues.push(...kt(p,u.issues));r.value[d]=u.value}}let l;for(let p in i)if(!c.has(p))if(t.mode==="loose"){if(p==="__proto__")continue;r.value[p]=i[p]}else l=l??[],l.push(p);if(l&&l.length>0)r.issues.push({code:"unrecognized_keys",input:i,inst:e,keys:l,continue:!0})}else{r.value=n?n.alloc(e,r,{},o):{};let c;for(let l of Reflect.ownKeys(i)){if(l==="__proto__")continue;if(!Object.prototype.propertyIsEnumerable.call(i,l))continue;let p=t.keyType._zod.run({value:l,issues:[]},o);if(p instanceof Promise)throw Error("Async schemas not supported in object keys currently");if(typeof l==="string"&&wn.test(l)&&p.issues.length){let f=t.keyType._zod.run({value:Number(l),issues:[]},o);if(f instanceof Promise)throw Error("Async schemas not supported in object keys currently");if(f.issues.length===0)p=f}if(p.issues.length){if(t.mode==="loose")r.value[l]=i[l];else if(s)c=c??[],c.push(l);else r.issues.push({code:"invalid_key",origin:"record",issues:p.issues.map((f)=>ut(f,o,tt())),input:l,path:[l],inst:e});continue}let d=p.value;if(d==="__proto__")continue;let u=t.valueType._zod.run({value:i[l],issues:[]},o);if(u instanceof Promise)a.push(u.then((f)=>{if(f.issues.length)r.issues.push(...kt(l,f.issues));r.value[d]=f.value}));else{if(u.issues.length)r.issues.push(...kt(l,u.issues));r.value[d]=u.value}}if(c&&c.length>0)r.issues.push({code:"unrecognized_keys",input:i,inst:e,keys:c,continue:!0})}if(a.length)return Promise.all(a).then(()=>r);return r}}),ic=O("$ZodEnum",(e,t)=>{De.init(e,t);let n=Kn(t.entries),r=new Set(n);e._zod.values=r;let o=n.filter((i)=>ta.has(typeof i));e._zod.pattern=new RegExp(o.length?`^(${o.map((i)=>_t(i.toString())).join("|")})$`:"^[^\\s\\S]$"),e._zod.parse=(i,a)=>{let s=i.value;if(r.has(s))return i;return i.issues.push({code:"invalid_value",values:n,input:s,inst:e}),i}}),ac=O("$ZodLiteral",(e,t)=>{De.init(e,t);let n=new Set(t.values);e._zod.values=n,e._zod.pattern=new RegExp(t.values.length?`^(${t.values.map((r)=>typeof r==="string"?_t(r):r?_t(r.toString()):String(r)).join("|")})$`:"^[^\\s\\S]$"),e._zod.parse=(r,o)=>{let i=r.value;if(n.has(i))return r;return r.issues.push({code:"invalid_value",values:t.values,input:i,inst:e}),r}}),sc=O("$ZodTransform",(e,t)=>{De.init(e,t),e._zod.optin="optional",Xe.memoizer?.guard(e),e._zod.parse=(n,r)=>{if(r.direction==="backward")throw new yn(e.constructor.name);let o=t.transform(n.value,n);if(r.async)return(o instanceof Promise?o:Promise.resolve(o)).then((a)=>(n.value=a,n));if(o instanceof Promise)throw new gt;return n.value=o,n}});_o=O("$ZodOptional",(e,t)=>{De.init(e,t),Ae(e,"optin",(n)=>n.def.innerType._zod.optin==="defaulted"?"defaulted":"optional"),e._zod.optout="optional",Ae(e,"values",(n)=>{let r=n.def.innerType._zod.values;return r?new Set([...r,void 0]):void 0}),Ae(e,"pattern",(n)=>{let r=n.def.innerType._zod.pattern;return r?new RegExp(`^(${Wn(r.source)})?$`):void 0}),e._zod.parse=(n,r)=>{if(n.value===void 0){if(t.innerType._zod.optin!=="defaulted")return n;let o=t.innerType._zod.run({value:n.value,issues:[]},r);if(o instanceof Promise)return o.then((i)=>ms(n,i));return ms(n,o)}return t.innerType._zod.run(n,r)}}),cc=O("$ZodExactOptional",(e,t)=>{_o.init(e,t),Ae(e,"values",(n)=>n.def.innerType._zod.values),Ae(e,"pattern",(n)=>n.def.innerType._zod.pattern),e._zod.parse=(n,r)=>t.innerType._zod.run(n,r)}),lc=O("$ZodNullable",(e,t)=>{De.init(e,t),Ae(e,"optin",(n)=>n.def.innerType._zod.optin),Ae(e,"optout",(n)=>n.def.innerType._zod.optout),Ae(e,"pattern",(n)=>{let r=n.def.innerType._zod.pattern;return r?new RegExp(`^(${Wn(r.source)}|null)$`):void 0}),Ae(e,"values",(n)=>n.def.innerType._zod.values?new Set([...n.def.innerType._zod.values,null]):void 0),e._zod.parse=(n,r)=>{if(n.value===null)return n;return t.innerType._zod.run(n,r)}}),uc=O("$ZodDefault",(e,t)=>{De.init(e,t),e._zod.optin="defaulted",Ae(e,"values",(n)=>n.def.innerType._zod.values),e._zod.parse=(n,r)=>{if(r.direction==="backward")return t.innerType._zod.run(n,r);if(n.value===void 0)return n.value=t.defaultValue,n;let o=t.innerType._zod.run(n,r);if(o instanceof Promise)return o.then((i)=>gs(i,t));return gs(o,t)}});dc=O("$ZodPrefault",(e,t)=>{De.init(e,t),e._zod.optin="defaulted",Ae(e,"values",(n)=>n.def.innerType._zod.values),e._zod.parse=(n,r)=>{if(r.direction==="backward")return t.innerType._zod.run(n,r);if(n.value===void 0)n.value=t.defaultValue;return t.innerType._zod.run(n,r)}}),pc=O("$ZodNonOptional",(e,t)=>{De.init(e,t),Ae(e,"values",(n)=>{let r=n.def.innerType._zod.values;return r?new Set([...r].filter((o)=>o!==void 0)):void 0}),e._zod.parse=(n,r)=>{let o=t.innerType._zod.run(n,r);if(o instanceof Promise)return o.then((i)=>ys(i,e));return ys(o,e)}});fc=O("$ZodCatch",(e,t)=>{De.init(e,t),Ae(e,"optin",(n)=>n.def.innerType._zod.optin==="defaulted"?"defaulted":"optional"),Ae(e,"optout",(n)=>n.def.innerType._zod.optout),Ae(e,"values",(n)=>n.def.innerType._zod.values),e._zod.parse=(n,r)=>{if(r.direction==="backward")return t.innerType._zod.run(n,r);let o=t.innerType._zod.run({value:n.value,issues:[]},r);if(o instanceof Promise)return o.then((i)=>bs(n,i,t,r));return bs(n,o,t,r)}}),hc=O("$ZodPipe",(e,t)=>{De.init(e,t),Ae(e,"values",(n)=>n.def.in._zod.values),Ae(e,"optin",(n)=>n.def.in._zod.optin),Ae(e,"optout",(n)=>n.def.out._zod.optout),Ae(e,"propValues",(n)=>n.def.in._zod.propValues),e._zod.parse=(n,r)=>{if(r.direction==="backward"){let i=t.out._zod.run(n,r);if(i instanceof Promise)return i.then((a)=>ir(a,t.in,r));return ir(i,t.in,r)}let o=t.in._zod.run(n,r);if(o instanceof Promise)return o.then((i)=>ir(i,t.out,r));return ir(o,t.out,r)}});mc=O("$ZodReadonly",(e,t)=>{De.init(e,t),Ae(e,"propValues",(n)=>n.def.innerType._zod.propValues),Ae(e,"values",(n)=>n.def.innerType._zod.values),Ae(e,"optin",(n)=>n.def.innerType?._zod?.optin),Ae(e,"optout",(n)=>n.def.innerType?._zod?.optout),e._zod.parse=(n,r)=>{if(r.direction==="backward")return t.innerType._zod.run(n,r);let o=t.innerType._zod.run(n,r);if(o instanceof Promise)return o.then(vs);return vs(o)}});gc=O("$ZodCustom",(e,t)=>{Je.init(e,t),De.init(e,t),e._zod.parse=(n,r)=>n,e._zod.check=(n)=>{let r=n.value,o=t.fn(r);if(o instanceof Promise)return o.then((i)=>ws(i,n,r,e));ws(o,n,r,e);return}})});function ko(e){return e.map((t)=>t.path?{...t,path:t.path.slice()}:{...t})}function $c(e,t){let n=vc.get(e);if(n!==void 0)return n;if(t.has(e))return!0;t.add(e);let r=!1,o=(s)=>{if(!r&&s?._zod&&$c(s,t))r=!0},i=e._zod.def,a=i.type;switch(a){case"object":{for(let s of Reflect.ownKeys(i.shape))o(i.shape[s]);o(i.catchall);break}case"array":o(i.element);break;case"tuple":for(let s of i.items)o(s);o(i.rest);break;case"record":case"map":o(i.keyType),o(i.valueType);break;case"set":o(i.valueType);break;case"union":for(let s of i.options)o(s);break;case"intersection":o(i.left),o(i.right);break;case"optional":case"nullable":case"default":case"prefault":case"catch":case"readonly":case"nonoptional":case"promise":case"success":o(i.innerType);break;case"pipe":o(i.in),o(i.out);break;case"function":o(i.input),o(i.output);break;case"lazy":o(e._zod.innerType);break;case"template_literal":case"string":case"number":case"int":case"boolean":case"bigint":case"symbol":case"undefined":case"null":case"void":case"never":case"any":case"unknown":case"date":case"nan":case"enum":case"literal":case"file":case"transform":case"custom":break;default:for(let s in i){let c=Object.getOwnPropertyDescriptor(i,s);if(!c||c.get)continue;let l=c.value;if(!l||typeof l!=="object")continue;if(l._zod)o(l);else if(Array.isArray(l))for(let p of l)o(p)}}return t.delete(e),vc.set(e,r),r}function jp(e,t){let n=e.buckets.get(t);if(!n)n=new Map,e.buckets.set(t,n);return n}function xc(){return Vp}function Fp(e,t){let n=e[So]?.backEdges;return n!==void 0&&t!==null&&typeof t==="object"&&n.has(t)}var wc,So="~memo",bc,vc,cr,lr,Vp;var _c=Se(()=>{wc=class wc extends Error{constructor(){super("Cannot parse a reference cycle that closes through a transform");this.name="ZodCyclicError"}};bc=[];vc=new WeakMap;lr=[],Vp={alloc(e,t,n){let r=cr;if(!r)return n;cr=void 0;let o={value:n,issues:null};return r.set(t.value,o),lr.push(o),n},guard(e){var t;(t=e._zod).deferred??(t.deferred=[]),e._zod.deferred.push(()=>{let n=e._zod.parse,r=(o,i)=>{if(i.direction!=="backward"&&Fp(i,o.value))throw new wc;return n(o,i)};if(e._zod.parse=r,e._zod.run===n)e._zod.run=r})},attach(e){var t;let n,r,o;(t=e._zod).deferred??(t.deferred=[]),e._zod.deferred.push(()=>{let i=e._zod.parse,a=(s,c)=>{if(n===void 0){if(n=$c(e,new Set),!n){if(e._zod.parse=i,e._zod.run===a)e._zod.run=i;return i(s,c)}}let l=s.value;if(l===null||typeof l!=="object")return i(s,c);let p=c[So];if(!p)p={buckets:new Map,backEdges:void 0},c[So]=p;let h;if(r===c)h=o;else h=jp(p,e),r=c,o=h;let d=h.get(l);if(d){if(s.value=d.value,d.issues){if(d.issues.length)s.issues.push(...ko(d.issues))}else s.memo=!0,p.backEdges??(p.backEdges=new Set),p.backEdges.add(d.value);return s}cr=h;let u=lr.length,f=i(s,c);cr=void 0;let x=lr.length>u?lr.pop():void 0;if(f instanceof Promise)return f.then((k)=>{if(x)x.issues=k.issues.length?ko(k.issues):bc;return k});if(x)x.issues=f.issues.length?ko(f.issues):bc;return f};if(e._zod.parse=a,e._zod.run===i)e._zod.run=a})}}});function Io(){return{localeError:Up()}}var Up=()=>{let e={string:{unit:"characters",verb:"to have"},file:{unit:"bytes",verb:"to have"},array:{unit:"items",verb:"to have"},set:{unit:"items",verb:"to have"},map:{unit:"entries",verb:"to have"}};function t(i){return e[i]??null}let n={regex:"input",email:"email address",url:"URL",emoji:"emoji",uuid:"UUID",uuidv4:"UUIDv4",uuidv6:"UUIDv6",nanoid:"nanoid",guid:"GUID",cuid:"cuid",cuid2:"cuid2",ulid:"ULID",xid:"XID",ksuid:"KSUID",datetime:"ISO datetime",date:"ISO date",time:"ISO time",duration:"ISO duration",ipv4:"IPv4 address",ipv6:"IPv6 address",mac:"MAC address",cidrv4:"IPv4 range",cidrv6:"IPv6 range",base64:"base64-encoded string",base64url:"base64url-encoded string",json_string:"JSON string",e164:"E.164 number",credit_card:"credit card number",jwt:"JWT",template_literal:"input"},r={nan:"NaN"};function o(i,a){if(i==="number"&&typeof a==="number"&&!Number.isFinite(a))return String(a);return r[i]??i}return(i)=>{switch(i.code){case"invalid_type":{let a=o(i.expected),s=aa(i.input),c=o(s,i.input);return`Invalid input: expected ${a}, received ${c}`}case"invalid_value":if(i.values.length===1)return`Invalid input: expected ${Qr(i.values[0])}`;return`Invalid option: expected one of ${Gr(i.values,"|")}`;case"too_big":{let a=i.exact?"exactly ":i.inclusive?"<=":"<",s=t(i.origin);if(s)return`Too big: expected ${i.origin??"value"} to have ${a}${i.maximum.toString()} ${s.unit??"elements"}`;return`Too big: expected ${i.origin??"value"} to be ${a}${i.maximum.toString()}`}case"too_small":{let a=i.exact?"exactly ":i.inclusive?">=":">",s=t(i.origin);if(s)return`Too small: expected ${i.origin} to have ${a}${i.minimum.toString()} ${s.unit}`;return`Too small: expected ${i.origin} to be ${a}${i.minimum.toString()}`}case"invalid_format":{let a=i;if(a.format==="starts_with")return`Invalid string: must start with "${a.prefix}"`;if(a.format==="ends_with")return`Invalid string: must end with "${a.suffix}"`;if(a.format==="includes")return`Invalid string: must include "${a.includes}"`;if(a.format==="regex")return`Invalid string: must match pattern ${a.pattern}`;return`Invalid ${n[a.format]??i.format}`}case"not_multiple_of":return`Invalid number: must be a multiple of ${i.divisor}`;case"unrecognized_keys":return`Unrecognized key${i.keys.length>1?"s":""}: ${Gr(i.keys,", ")}`;case"invalid_key":return`Invalid key in ${i.origin}`;case"invalid_union":if(i.options&&Array.isArray(i.options)&&i.options.length>0)return`Invalid discriminator value. Expected ${i.options.map((s)=>`'${s}'`).join(" | ")}`;if(i.inclusive===!1)return"Invalid input: more than one option matched";return"Invalid input";case"invalid_element":return`Invalid value in ${i.origin}`;default:return"Invalid input"}}};var kc=Se(()=>{et()});var Co=()=>{};class Cc{constructor(){this._map=new WeakMap,this._idmap=new Map}add(e,...t){let n=t[0];if(this._map.set(e,n),n&&typeof n==="object"&&"id"in n)this._idmap.set(n.id,e);return this}clear(){return this._map=new WeakMap,this._idmap=new Map,this}remove(e){let t=this._map.get(e);if(t&&typeof t==="object"&&"id"in t)this._idmap.delete(t.id);return this._map.delete(e),this}get(e){let t=e._zod.parent;if(t){let n={...this.get(t)??{}};delete n.id;let r={...n,...this._map.get(e)};return Object.keys(r).length?r:void 0}return this._map.get(e)}has(e){return this._map.has(e)}}function Bp(){return new Cc}var Ic,Mt;var zo=Se(()=>{(Ic=globalThis).__zod_globalRegistry??(Ic.__zod_globalRegistry=Bp());Mt=globalThis.__zod_globalRegistry});var zc=()=>{};function Pc(e,t){return new e({type:"string",...ae(t)})}function Ac(e,t){return new e({type:"string",format:"email",check:"string_format",abort:!1,...ae(t)})}function Ec(e,t){return new e({type:"string",format:"guid",check:"string_format",abort:!1,...ae(t)})}function Po(e,t){return new e({type:"string",format:"uuid",check:"string_format",abort:!1,...ae(t)})}function Tc(e,t){return new e({type:"string",format:"uuid",check:"string_format",abort:!1,version:"v4",...ae(t)})}function Oc(e,t){return new e({type:"string",format:"uuid",check:"string_format",abort:!1,version:"v6",...ae(t)})}function Mc(e,t){return new e({type:"string",format:"uuid",check:"string_format",abort:!1,version:"v7",...ae(t)})}function Ao(e,t){return new e({type:"string",format:"url",check:"string_format",abort:!1,...ae(t)})}function Nc(e,t){return new e({type:"string",format:"emoji",check:"string_format",abort:!1,...ae(t)})}function Dc(e,t){return new e({type:"string",format:"nanoid",check:"string_format",abort:!1,...ae(t)})}function Lc(e,t){return new e({type:"string",format:"cuid",check:"string_format",abort:!1,...ae(t)})}function Rc(e,t){return new e({type:"string",format:"cuid2",check:"string_format",abort:!1,...ae(t)})}function Zc(e,t){return new e({type:"string",format:"ulid",check:"string_format",abort:!1,...ae(t)})}function jc(e,t){return new e({type:"string",format:"xid",check:"string_format",abort:!1,...ae(t)})}function Vc(e,t){return new e({type:"string",format:"ksuid",check:"string_format",abort:!1,...ae(t)})}function Fc(e,t){return new e({type:"string",format:"ipv4",check:"string_format",abort:!1,...ae(t)})}function Uc(e,t){return new e({type:"string",format:"ipv6",check:"string_format",abort:!1,...ae(t)})}function Bc(e,t){return new e({type:"string",format:"cidrv4",check:"string_format",abort:!1,...ae(t)})}function Hc(e,t){return new e({type:"string",format:"cidrv6",check:"string_format",abort:!1,...ae(t)})}function Jc(e,t){return new e({type:"string",format:"base64",check:"string_format",abort:!1,...ae(t)})}function qc(e,t){return new e({type:"string",format:"base64url",check:"string_format",abort:!1,...ae(t)})}function Kc(e,t){return new e({type:"string",format:"e164",check:"string_format",abort:!1,...ae(t)})}function Wc(e,t){return new e({type:"string",format:"jwt",check:"string_format",abort:!1,...ae(t)})}function ur(e,t){return new e({type:"string",format:"datetime",check:"string_format",offset:!1,local:!1,precision:null,...ae(t)})}function dr(e,t){return new e({type:"string",format:"date",check:"string_format",...ae(t)})}function pr(e,t){return new e({type:"string",format:"time",check:"string_format",precision:null,...ae(t)})}function fr(e,t){return new e({type:"string",format:"duration",check:"string_format",...ae(t)})}function Gc(e,t){return new e({type:"number",checks:[],...ae(t)})}function Xc(e,t){return new e({type:"number",check:"number_format",abort:!1,format:"safeint",...ae(t)})}function Yc(e,t){return new e({type:"boolean",...ae(t)})}function Qc(e){return new e({type:"unknown"})}function el(e,t){return new e({type:"never",...ae(t)})}function hr(e,t){return new ho({check:"less_than",...ae(t),value:e,inclusive:!1})}function kn(e,t){return new ho({check:"less_than",...ae(t),value:e,inclusive:!0})}function mr(e,t){return new mo({check:"greater_than",...ae(t),value:e,inclusive:!1})}function Sn(e,t){return new mo({check:"greater_than",...ae(t),value:e,inclusive:!0})}function gr(e,t){return new Xa({check:"multiple_of",...ae(t),value:e})}function yr(e,t){return new Qa({check:"max_length",...ae(t),maximum:e})}function Yt(e,t){return new es({check:"min_length",...ae(t),minimum:e})}function br(e,t){return new ts({check:"length_equals",...ae(t),length:e})}function Eo(e,t){return new ns({check:"string_format",format:"regex",...ae(t),pattern:e})}function To(e){return new rs({check:"string_format",format:"lowercase",...ae(e)})}function Oo(e){return new os({check:"string_format",format:"uppercase",...ae(e)})}function Mo(e,t){return new is({check:"string_format",format:"includes",...ae(t),includes:e})}function No(e,t){return new as({check:"string_format",format:"starts_with",...ae(t),prefix:e})}function Do(e,t){return new ss({check:"string_format",format:"ends_with",...ae(t),suffix:e})}function St(e){return new cs({check:"overwrite",tx:e})}function Lo(e){return St((t)=>t.normalize(e))}function Ro(){return St((e)=>e.trim())}function Zo(){return St((e)=>e.toLowerCase())}function jo(){return St((e)=>e.toUpperCase())}function Vo(){return St((e)=>Qi(e))}function tl(e,t,n){return new e({type:"array",element:t,...ae(n)})}function nl(e,t,n){return new e({type:"custom",check:"custom",fn:t,...ae(n)})}function rl(e,t){let n=Hp((r)=>(r.addIssue=(o)=>{if(typeof o==="string")r.issues.push(Tt(o,r.value,n._zod.def));else{let i=o;if(i.fatal)i.continue=!1;if(i.code??(i.code="custom"),!("input"in i))i.input=r.value;i.inst??(i.inst=n),i.continue??(i.continue=!n._zod.def.abort),r.issues.push(Tt(i))}},e(r.value,r)),t);return n}function Hp(e,t){let n=new Je({check:"custom",...ae(t)});return n._zod.check=e,n}var ol=Se(()=>{or();et()});function In(e,...t){for(let n of t)for(let r of Reflect.ownKeys(n))if(Object.prototype.propertyIsEnumerable.call(n,r))Ue(e,r,n[r]);return e}function Uo(e){let t=e?.target??"draft-2020-12";if(t==="draft-4")t="draft-04";if(t==="draft-7")t="draft-07";return{processors:e.processors??{},metadataRegistry:e?.metadata??Mt,target:t,unrepresentable:e?.unrepresentable??"throw",override:e?.override??(()=>{}),io:e?.io??"output",counter:0,seen:new Map,sharedDefsExtractedFor:void 0,sharedEmitDoneFor:void 0,cycles:e?.cycles??"ref",reused:e?.reused??"inline",intersections:[],deferred:[],external:e?.external??void 0}}function yt(e,t,n,r,o){let i=typeof t.unrepresentable==="function"?t.unrepresentable({zodSchema:e,path:r.path,message:o}):t.unrepresentable;if(i==="any")return!1;if(i===void 0||i==="throw")throw Error(o);return Object.assign(n,i),!0}function Ve(e,t,n={path:[],schemaPath:[]}){var r;let o=e._zod.def,i=t.seen.get(e);if(i){if(i.count++,n.schemaPath.includes(e))i.cycle=n.path;return i.schema}let a={schema:{},count:1,cycle:void 0,path:n.path};t.seen.set(e,a),t.sharedDefsExtractedFor=void 0,t.sharedEmitDoneFor=void 0;let s=e._zod.toJSONSchema?.();if(s)a.schema=s;else{let p={...n,schemaPath:[...n.schemaPath,e],path:n.path};if(e._zod.processJSONSchema)e._zod.processJSONSchema(t,a.schema,p);else{let d=a.schema,u=t.processors[o.type];if(!u)throw Error(`[toJSONSchema]: Non-representable type encountered: ${o.type}`);u(e,t,d,p)}let h=e._zod.parent;if(h){if(!a.ref)a.ref=h;Ve(h,t,p),t.seen.get(h).isParent=!0}}let c=t.metadataRegistry.get(e);if(c)In(a.schema,c);if(t.io==="input"&&Ke(e))delete a.schema.examples,delete a.schema.default;if(t.io==="input"&&"_prefault"in a.schema)(r=a.schema).default??(r.default=a.schema._prefault);return delete a.schema._prefault,t.seen.get(e).schema}function il(e){return e.replace(/~/g,"~0").replace(/\//g,"~1")}function Bo(e,t){let n=e.seen.get(t);if(!n)throw Error("Unprocessed schema. This is a bug in Zod.");if(e.external&&e.sharedDefsExtractedFor===e.external)return;let r=new Map;for(let a of e.seen.entries()){let s=e.metadataRegistry.get(a[0])?.id;if(s){let c=r.get(s);if(c&&c!==a[0])throw Error(`Duplicate schema id "${s}" detected during JSON Schema conversion. Two different schemas cannot share the same id when converted together.`);r.set(s,a[0])}}let o=(a)=>{let s=e.target==="draft-2020-12"?"$defs":"definitions";if(e.external){let h=e.external.registry.get(a[0])?.id,d=e.external.uri??((f)=>f);if(h)return{ref:d(h)};let u=a[1].defId??a[1].schema.id??`schema${e.counter++}`;return a[1].defId=u,{defId:u,ref:`${d("__shared")}#/${s}/${il(u)}`}}let c="#",l=`${c}/${s}/`;if(a[1]===n&&!a[1].schema.id)return{ref:c};let p=a[1].schema.id??`__schema${e.counter++}`;return{defId:p,ref:l+il(p)}},i=(a)=>{if(a[1].schema.$ref)return;let s=a[1],{ref:c,defId:l}=o(a);if(s.def={...s.schema},l)s.defId=l;let p=s.schema;for(let h in p)delete p[h];p.$ref=c};if(e.cycles==="throw")for(let a of e.seen.entries()){let s=a[1];if(s.cycle)throw Error(`Cycle detected: #/${s.cycle?.join("/")}/<root>

Set the \`cycles\` parameter to \`"ref"\` to resolve cyclical schemas with defs.`)}for(let a of e.seen.entries()){let s=a[1];if(t===a[0]){i(a);continue}if(e.external){let l=e.external.registry.get(a[0])?.id;if(t!==a[0]&&l){i(a);continue}}if(e.metadataRegistry.get(a[0])?.id){i(a);continue}if(s.cycle){i(a);continue}if(s.count>1){if(e.reused==="ref"){i(a);continue}}}if(e.external)e.sharedDefsExtractedFor=e.external}function cl(e){let t=e.anyOf;if(!Array.isArray(t)||t.length===0||e.type!==void 0)return;let n=[];for(let r of t){if(!r||typeof r!=="object")return;cl(r);let o=Object.keys(r);if(o.length!==1||o[0]!=="type")return;let i=r.type;for(let a of Array.isArray(i)?i:[i]){if(typeof a!=="string")return;if(!n.includes(a))n.push(a)}}delete e.anyOf,e.type=n.length===1?n[0]:n}function sl(e){let t=e.additionalProperties;if(t===void 0||t===!1||typeof t!=="object"||t===null)return null;return Object.keys(t).length?t:null}function Fo(e){let t=[];for(let i of e){if(typeof i!=="object"||i.type!=="object")return null;for(let a in i)if(!ll.has(a))return null;t.push(i)}let n={},r=new Set;for(let i of t){for(let a in i.properties){if(Object.prototype.hasOwnProperty.call(n,a))continue;let s=[];for(let l of t){let p=l.properties?.[a]??sl(l);if(p===null||p===void 0)continue;if(!s.some((h)=>JSON.stringify(h)===JSON.stringify(p)))s.push(p)}let c=s.length===1?s[0]:Fo(s)??{allOf:s};Ue(n,a,c)}for(let a of i.required??[])r.add(a)}let o={type:"object",properties:n};if(r.size)o.required=[...r];if(t.every((i)=>i.additionalProperties===!1))o.additionalProperties=!1;else{let i=[];for(let a of t){let s=sl(a);if(s&&!i.some((c)=>JSON.stringify(c)===JSON.stringify(s)))i.push(s)}if(i.length===1)o.additionalProperties=i[0];else if(i.length>1)o.additionalProperties={allOf:i}}return o}function Jp(e){let t=e.allOf;if(!Array.isArray(t)||t.length<2)return;for(let o of ll)if(o in e)return;let n=t.filter((o)=>al.some((i)=>Array.isArray(o[i]))),r=null;if(!n.length)r=Fo(t);else{let o=n[0],i=al.find((c)=>Array.isArray(o[c]));if(Object.keys(o).length!==1)return;let a=t.filter((c)=>c!==o),s=o[i].map((c)=>Fo([...a,c]));if(s.some((c)=>!c))return;r={[i]:s}}if(!r)return;delete e.allOf,In(e,r)}function Ho(e,t){let n=e.seen.get(t);if(!n)throw Error("Unprocessed schema. This is a bug in Zod.");let r=(s)=>{let c=e.seen.get(s);if(c.ref===null)return;let l=c.def??c.schema,p={...l},h=c.ref;if(c.ref=null,h){r(h);let u=e.seen.get(h),f=u.schema;if(f.$ref&&(e.target==="draft-07"||e.target==="draft-04"||e.target==="openapi-3.0"))l.allOf=l.allOf??[],l.allOf.push(f);else In(l,f);if(In(l,p),s._zod.parent===h)for(let k in l){if(k==="$ref"||k==="allOf")continue;if(!(k in p))delete l[k]}if(f.$ref&&u.def)for(let k in l){if(k==="$ref"||k==="allOf")continue;if(k in u.def&&JSON.stringify(l[k])===JSON.stringify(u.def[k]))delete l[k]}}let d=s._zod.parent;if(d&&d!==h){r(d);let u=e.seen.get(d);if(u?.schema.$ref){if(l.$ref=u.schema.$ref,u.def)for(let f in l){if(f==="$ref"||f==="allOf")continue;if(f in u.def&&JSON.stringify(l[f])===JSON.stringify(u.def[f]))delete l[f]}}}e.override({zodSchema:s,jsonSchema:l,path:c.path??[]})};if(!e.external||e.sharedEmitDoneFor!==e.external){for(let s of[...e.seen.entries()].reverse())r(s[0]);if(e.target!=="openapi-3.0")for(let s of e.seen.entries())cl(s[1].def??s[1].schema);for(let s of e.deferred)s();if(e.intersections.length){let s=new Map;for(let c of e.seen.values())for(let l of[c.schema,c.def]){let p=l?.allOf;if(!Array.isArray(p))continue;let h=s.get(p);if(h)h.push(l);else s.set(p,[l])}for(let c of e.intersections)for(let l of s.get(c)??[])Jp(l)}}let o={};if(e.target==="draft-2020-12")o.$schema="https://json-schema.org/draft/2020-12/schema";else if(e.target==="draft-07")o.$schema="http://json-schema.org/draft-07/schema#";else if(e.target==="draft-04")o.$schema="http://json-schema.org/draft-04/schema#";else if(e.target==="openapi-3.0");if(e.external?.uri){let s=e.external.registry.get(t)?.id;if(!s)throw Error("Schema is missing an `id` property");o.$id=e.external.uri(s)}In(o,n.defId?n.schema:n.def??n.schema);let i=e.metadataRegistry.get(t)?.id;if(i!==void 0&&o.id===i)delete o.id;let a=e.external?.defs??{};if(!e.external||e.sharedEmitDoneFor!==e.external)for(let s of e.seen.entries()){let c=s[1];if(c.def&&c.defId){if(c.def.id===c.defId)delete c.def.id;Ue(a,c.defId,c.def)}}if(e.external)e.sharedEmitDoneFor=e.external;if(e.external);else if(Object.keys(a).length>0)if(e.target==="draft-2020-12")o.$defs=a;else o.definitions=a;try{let s=JSON.parse(JSON.stringify(o));return Object.defineProperty(s,"~standard",{value:{...t["~standard"],jsonSchema:{input:Cn(t,"input",e.processors),output:Cn(t,"output",e.processors)}},enumerable:!1,writable:!1}),s}catch(s){throw Error("Error converting schema to JSON.")}}function Ke(e,t){let n=t??{seen:new Set};if(n.seen.has(e))return!1;n.seen.add(e);let r=e._zod.def;if(r.type==="transform")return!0;if(r.type==="array")return Ke(r.element,n);if(r.type==="set")return Ke(r.valueType,n);if(r.type==="lazy")return Ke(r.getter(),n);if(r.type==="promise"||r.type==="optional"||r.type==="nonoptional"||r.type==="nullable"||r.type==="readonly"||r.type==="default"||r.type==="prefault"||r.type==="catch")return Ke(r.innerType,n);if(r.type==="intersection")return Ke(r.left,n)||Ke(r.right,n);if(r.type==="record"||r.type==="map")return Ke(r.keyType,n)||Ke(r.valueType,n);if(r.type==="pipe"){if(e._zod.traits.has("$ZodCodec"))return!0;return Ke(r.in,n)||Ke(r.out,n)}if(r.type==="object"){for(let o in r.shape)if(Ke(r.shape[o],n))return!0;return!1}if(r.type==="union"){for(let o of r.options)if(Ke(o,n))return!0;return!1}if(r.type==="tuple"){for(let o of r.items)if(Ke(o,n))return!0;if(r.rest&&Ke(r.rest,n))return!0;return!1}return!1}var ll,al,ul=(e,t={})=>(n)=>{let r=Uo({...n,processors:t});return Ve(e,r),Bo(r,e),Ho(r,e)},Cn=(e,t,n={})=>(r)=>{let{libraryOptions:o,target:i}=r??{},a=Uo({...o??{},target:i,io:t,processors:n});return Ve(e,a),Bo(a,e),Ho(a,e)};var vr=Se(()=>{zo();et();ll=new Set(["type","properties","required","additionalProperties"]),al=["oneOf","anyOf"]});function wr(e){let t=e._zod.def;if(t.type==="pipe"&&t.in._zod.traits.has("$ZodTransform"))return wr(t.out);if(t.type==="catch")return wr(t.innerType);return e._zod.optin}function Jo(e,t,n){if(t.$ref){if(n.has(t))return t;n.add(t);let f=e.get(t)?.def;if(!f)return t;let x=Jo(e,f,n);return x===f?t:x}for(let f of["anyOf","oneOf"]){let x=t[f];if(!Array.isArray(x))continue;let k=x.map((w)=>Jo(e,w,n));if(k.some((w,P)=>w!==x[P]))t={...t,[f]:k}}let r=Array.isArray(t.type)?t.type:[t.type],o=!r.includes("string")&&r.some((f)=>f==="number"||f==="integer"),i=t.enum??(t.const!==void 0?[t.const]:void 0);if(!o&&!i?.some((f)=>typeof f==="number"))return t;let{minimum:a,maximum:s,exclusiveMinimum:c,exclusiveMaximum:l,multipleOf:p,format:h,id:d,...u}=t;if(u.enum)u.enum=u.enum.map((f)=>typeof f==="number"?String(f):f);else if(typeof u.const==="number")u.const=String(u.const);if(!o)return u;if(u.type="string",!i)u.pattern=(r.includes("number")?wn:nr).source;return u}function Kp(e){let t=new Map;for(let r of e.seen.values())if(r.def&&!t.has(r.schema))t.set(r.schema,r);let n=new Map;for(let r of qo.get(e)??[]){let o=e.seen.get(r),i=(o?.def??o?.schema)?.propertyNames;if(!i||i===!0||n.has(i))continue;let a=Jo(t,i,new Set);if(a!==i)n.set(i,a)}if(!n.size)return;for(let r of e.seen.values())for(let o of[r.schema,r.def]){let i=o&&n.get(o.propertyNames);if(i)o.propertyNames=i}}function Cl(e,t,n,r,o){let i=!1,a=JSON.stringify(e,(s,c)=>{if(typeof c!=="bigint")return c;return i=!0,null});if(!i)return JSON.parse(a);return yt(t,n,r,o,"BigInt defaults cannot be represented in JSON Schema"),Ko}var qp,dl=(e,t,n,r)=>{let o=n;o.type="string";let{minimum:i,maximum:a,format:s,patterns:c,contentEncoding:l,laxFormat:p}=e._zod.bag;if(typeof i==="number")o.minLength=i;if(typeof a==="number")o.maxLength=a;if(s){if(o.format=qp[s]??s,o.format==="")delete o.format;if(s==="time"||p)delete o.format}if(l)o.contentEncoding=l;if(c&&c.size>0){let h=[...c];if(h.length===1)o.pattern=h[0].source;else if(h.length>1)o.allOf=[...h.map((d)=>({...t.target==="draft-07"||t.target==="draft-04"||t.target==="openapi-3.0"?{type:"string"}:{},pattern:d.source}))]}},pl=(e,t,n,r)=>{let o=n,{minimum:i,maximum:a,format:s,multipleOf:c,exclusiveMaximum:l,exclusiveMinimum:p}=e._zod.bag;if(typeof s==="string"&&s.includes("int"))o.type="integer";else o.type="number";let h=typeof p==="number"&&p>=(i??Number.NEGATIVE_INFINITY),d=typeof l==="number"&&l<=(a??Number.POSITIVE_INFINITY),u=t.target==="draft-04"||t.target==="openapi-3.0";if(h)if(u)o.minimum=p,o.exclusiveMinimum=!0;else o.exclusiveMinimum=p;else if(typeof i==="number")o.minimum=i;if(d)if(u)o.maximum=l,o.exclusiveMaximum=!0;else o.exclusiveMaximum=l;else if(typeof a==="number")o.maximum=a;if(typeof c==="number")if(Number.isFinite(c)&&c!==0)o.multipleOf=Math.abs(c);else yt(e,t,o,r,`A multipleOf divisor of ${c} cannot be represented in JSON Schema`)},fl=(e,t,n,r)=>{n.type="boolean"},hl=(e,t,n,r)=>{n.not={}},ml=(e,t,n,r)=>{},gl=(e,t,n,r)=>{let o=e._zod.def,i=Kn(o.entries);if(i.length===0){n.not={};return}if(i.every((a)=>typeof a==="number"))n.type="number";if(i.every((a)=>typeof a==="string"))n.type="string";n.enum=i},yl=(e,t,n,r)=>{let o=e._zod.def;if(o.values.length===0){n.not={};return}let i=[];for(let a of o.values)if(a===void 0){if(yt(e,t,n,r,"Literal `undefined` cannot be represented in JSON Schema"))return}else if(typeof a==="bigint"){if(yt(e,t,n,r,"BigInt literals cannot be represented in JSON Schema"))return;i.push(Number(a))}else i.push(a);if(i.length===0);else if(i.length===1){let a=i[0];if(n.type=a===null?"null":typeof a,t.target==="draft-04"||t.target==="openapi-3.0")n.enum=[a];else n.const=a}else{if(i.every((a)=>typeof a==="number"))n.type="number";if(i.every((a)=>typeof a==="string"))n.type="string";if(i.every((a)=>typeof a==="boolean"))n.type="boolean";if(i.every((a)=>a===null))n.type="null";n.enum=i}},bl=(e,t,n,r)=>{yt(e,t,n,r,"Custom types cannot be represented in JSON Schema")},vl=(e,t,n,r)=>{yt(e,t,n,r,"Transforms cannot be represented in JSON Schema")},wl=(e,t,n,r)=>{let o=n,i=e._zod.def,{minimum:a,maximum:s}=e._zod.bag;if(typeof a==="number")o.minItems=a;if(typeof s==="number")o.maxItems=s;o.type="array",o.items=Ve(i.element,t,{...r,path:[...r.path,"items"]})},$l=(e,t,n,r)=>{let o=n,i=e._zod.def,a=i.shape;if(Object.getOwnPropertySymbols(a).length&&yt(e,t,o,r,"Symbol keys cannot be represented in JSON Schema"))return;o.type="object",o.properties={};for(let p in a)Ue(o.properties,p,Ve(a[p],t,{...r,path:[...r.path,"properties",p]}));let c=new Set(Object.keys(a)),l=new Set([...c].filter((p)=>{let h=i.shape[p];if(t.io==="input")return wr(h)===void 0;else return h._zod.optout===void 0}));if(l.size>0)o.required=Array.from(l);if(i.catchall?._zod.def.type==="never")o.additionalProperties=!1;else if(!i.catchall){if(t.io==="output")o.additionalProperties=!1}else if(i.catchall)o.additionalProperties=Ve(i.catchall,t,{...r,path:[...r.path,"additionalProperties"]})},xl=(e,t,n,r)=>{let o=e._zod.def,i=o.inclusive===!1,a=o.options.map((s,c)=>Ve(s,t,{...r,path:[...r.path,i?"oneOf":"anyOf",c]}));if(i)n.oneOf=a;else n.anyOf=a},_l=(e,t,n,r)=>{let o=e._zod.def,i=Ve(o.left,t,{...r,path:[...r.path,"allOf",0]}),a=Ve(o.right,t,{...r,path:[...r.path,"allOf",1]}),s=(l)=>("allOf"in l)&&Object.keys(l).length===1,c=[...s(i)?i.allOf:[i],...s(a)?a.allOf:[a]];n.allOf=c,t.intersections.push(c)},qo,kl=(e,t,n,r)=>{let o=n,i=e._zod.def;o.type="object";let a=i.keyType,c=a._zod.bag?.patterns;if(i.mode==="loose"&&c&&c.size>0){let h=Ve(i.valueType,t,{...r,path:[...r.path,"patternProperties","*"]});o.patternProperties={};for(let d of c)Ue(o.patternProperties,d.source,h)}else{if(t.target==="draft-07"||t.target==="draft-2020-12"){o.propertyNames=Ve(i.keyType,t,{...r,path:[...r.path,"propertyNames"]});let h=qo.get(t);if(!h)h=[],qo.set(t,h),t.deferred.push(()=>Kp(t));h.push(e)}o.additionalProperties=Ve(i.valueType,t,{...r,path:[...r.path,"additionalProperties"]})}let l=a._zod.values,p=t.io==="input"&&wr(i.valueType)!==void 0;if(l&&!i.partial&&!p){let h=[...l].filter((d)=>typeof d==="string"||typeof d==="number");if(h.length>0)o.required=h.map(String)}},Sl=(e,t,n,r)=>{let o=e._zod.def,i=Ve(o.innerType,t,r),a=t.seen.get(e);if(t.target==="openapi-3.0")a.ref=o.innerType,n.nullable=!0;else n.anyOf=[i,{type:"null"}]},Il=(e,t,n,r)=>{let o=e._zod.def;Ve(o.innerType,t,r);let i=t.seen.get(e);i.ref=o.innerType},Ko,zl=(e,t,n,r)=>{let o=e._zod.def;Ve(o.innerType,t,r);let i=t.seen.get(e);i.ref=o.innerType;let a=Cl(o.defaultValue,e,t,n,r);if(a!==Ko)n.default=a},Pl=(e,t,n,r)=>{let o=e._zod.def;Ve(o.innerType,t,r);let i=t.seen.get(e);if(i.ref=o.innerType,t.io!=="input")return;let a=Cl(o.defaultValue,e,t,n,r);if(a!==Ko)n._prefault=a},Al=(e,t,n,r)=>{let o=e._zod.def;Ve(o.innerType,t,r);let i=t.seen.get(e);i.ref=o.innerType;let a;try{a=o.catchValue(void 0)}catch{yt(e,t,n,r,"Dynamic catch values are not supported in JSON Schema");return}n.default=a},El=(e,t,n,r)=>{let o=e._zod.def,i=o.in._zod.traits.has("$ZodTransform"),a=t.io==="input"?i?o.out:o.in:o.out;Ve(a,t,r);let s=t.seen.get(e);s.ref=a},Tl=(e,t,n,r)=>{let o=e._zod.def;Ve(o.innerType,t,r);let i=t.seen.get(e);i.ref=o.innerType,n.readOnly=!0},Wo=(e,t,n,r)=>{let o=e._zod.def;Ve(o.innerType,t,r);let i=t.seen.get(e);i.ref=o.innerType};var Ol=Se(()=>{xn();vr();et();qp={guid:"uuid",url:"uri",datetime:"date-time",json_string:"json-string",regex:""};qo=new WeakMap;Ko=Symbol()});var Ml=()=>{};var bt=Se(()=>{et();xn();Co();Ml();Ot();co();so();yc();_c();or();yo();zo();zc();ol();vr()});var Go=Se(()=>{bt()});function $r(e,t,n){Object.defineProperty(e,t,{configurable:!0,enumerable:!1,get(){let r=n(this);return Object.defineProperty(this,t,{value:r,configurable:!0,writable:!0}),r},set(r){Object.defineProperty(this,t,{value:r,configurable:!0,writable:!0})}})}var Nl,tf=(e,t)=>{Yn.init(e,t),e.name="ZodError";let n=Object.getPrototypeOf(e);if(Nl.has(n))return;Nl.add(n),$r(n,"format",(r)=>(o)=>ha(r,o)),$r(n,"flatten",(r)=>(o)=>fa(r,o)),$r(n,"addIssue",(r)=>(o)=>{r.issues.push(o),r.message=JSON.stringify(r.issues,mn,2)}),$r(n,"addIssues",(r)=>(o)=>{r.issues.push(...o),r.message=JSON.stringify(r.issues,mn,2)}),Object.defineProperty(n,"isEmpty",{configurable:!0,enumerable:!1,get(){return this.issues.length===0}})},nt;var Xo=Se(()=>{bt();bt();et();Nl=new WeakSet([Object.prototype,Error.prototype]);nt=O("ZodError",tf,void 0,{Parent:Error})});var Dl,Ll,Rl,Zl,jl,Vl,Fl,Ul,Bl,Hl,Jl,ql;var Yo=Se(()=>{bt();Xo();Dl=er(nt),Ll=tr(nt),Rl=bn(nt),Zl=vn(nt),jl=ya(nt),Vl=ba(nt),Fl=va(nt),Ul=wa(nt),Bl=$a(nt),Hl=xa(nt),Jl=_a(nt),ql=ka(nt)});function rf(){if(!Xe.localeError)tt(Io())}function xr(){if(!Xe.memoizer)tt({memoizer:xc()})}function xe(e){return Pc(of,e)}function ze(e){return Po(Pn,e)}function nu(e){return Ao(tu,e)}function Le(e){return Gc(ru,e)}function Kl(e){return Xc(_f,e)}function Qt(e){return Yc(kf,e)}function Wl(){return Qc(Sf)}function Cf(e){return el(If,e)}function Be(e,t){return tl(zf,e,t)}function _e(e,t){let n={type:"object",shape:e??{},...ae(t)};return new Pf(n)}function Af(e,t){return new ou({type:"union",options:e,...ae(t)})}function iu(e,t,n){return new Ef({type:"union",options:t,discriminator:e,...ae(n)})}function Of(e,t){return new Tf({type:"intersection",left:e,right:t})}function Mn(e,t,n){if(!t||!t._zod)return new Gl({type:"record",keyType:xe(),valueType:e,...ae(t)});return new Gl({type:"record",keyType:e,valueType:t,...ae(n)})}function at(e,t){let n=Array.isArray(e)?Object.fromEntries(e.map((r)=>[r,r])):e;return new Qo({type:"enum",entries:n,...ae(t)})}function je(e,t){return new Mf({type:"literal",values:Array.isArray(e)?e:[e],...ae(t)})}function Df(e){return new Nf({type:"transform",transform:e})}function Xl(e){return new au({type:"optional",innerType:e})}function Lf(e){return new su({type:"optional",innerType:e})}function Yl(e){return new Rf({type:"nullable",innerType:e})}function jf(e,t){return new Zf({type:"default",innerType:e,get defaultValue(){return typeof t==="function"?t():Yr(t)}})}function Ff(e,t){return new Vf({type:"prefault",innerType:e,get defaultValue(){return typeof t==="function"?t():Yr(t)}})}function Uf(e,t){return new cu({type:"nonoptional",innerType:e,...ae(t)})}function Hf(e,t){return new Bf({type:"catch",innerType:e,catchValue:typeof t==="function"?t:ca(t)})}function Ql(e,t){return new Jf({type:"pipe",in:e,out:t})}function Kf(e){return new qf({type:"readonly",innerType:e})}function Gf(e,t={}){return nl(Wf,e,t)}function Xf(e,t){return rl(e,t)}var Re,eu,of,Ze,An,En,Tn,On,af,sf,Pn,tu,cf,lf,uf,df,pf,ff,hf,mf,gf,yf,bf,vf,wf,$f,xf,ru,_f,kf,Sf,If,zf,Pf,ou,Ef,Tf,Gl,Qo,Mf,Nf,au,su,Rf,Zf,Vf,cu,Bf,Jf,qf,Wf;var _r=Se(()=>{bt();bt();Ol();vr();kc();Go();Yo();Re=O("ZodType",(e,t)=>(rf(),De.init(e,t),e.def=t,e.type=t.type,e),{check(...e){let t=this.def;return this.clone(xt(t,{checks:[...t.checks??[],...e.map((n)=>typeof n==="function"?{_zod:{check:n,def:{check:"custom"},onattach:[]}}:n)]}),{parent:!0})},with(...e){return this.check(...e)},clone(e,t){return lt(this,e,t)},brand(){return this},register(e,t){return e.add(this,t),this},refine(e,t){return this.check(Gf(e,t))},superRefine(e,t){return this.check(Xf(e,t))},overwrite(e){return this.check(St(e))},optional(){return Xl(this)},exactOptional(){return Lf(this)},nullable(){return Yl(this)},nullish(){return Xl(Yl(this))},nonoptional(e){return Uf(this,e)},array(){return Be(this)},or(e){return Af([this,e])},and(e){return Of(this,e)},transform(e){return Ql(this,Df(e))},default(e){return jf(this,e)},prefault(e){return Ff(this,e)},catch(e){return Hf(this,e)},pipe(e){return Ql(this,e)},readonly(){return Kf(this)},describe(e){let t=this.clone();return Mt.add(t,{description:e}),t},meta(...e){if(e.length===0)return Mt.get(this);let t=this.clone();return Mt.add(t,e[0]),t},isOptional(){return this.safeParse(void 0).success},isNullable(){return this.safeParse(null).success},apply(e,...t){return t.length===0?e(this):e(this,...t)},get "~standard"(){return to(this,"~standard",{...wo(this),jsonSchema:{input:Cn(this,"input"),output:Cn(this,"output")}})},set "~standard"(e){Pt(this,"~standard",e)},parse:function e(t,n){return Dl(this,t,n,{callee:e})},parseAsync:async function e(t,n){return await Ll(this,t,n,{callee:e})},safeParse(e,t){return Rl(this,e,t)},async safeParseAsync(e,t){return Zl(this,e,t)},get spa(){return this?.safeParseAsync},set spa(e){Pt(this,"spa",e)},encode:function e(t,n){return jl(this,t,n,{callee:e})},decode:function e(t,n){return Vl(this,t,n,{callee:e})},encodeAsync:async function e(t,n){return await Fl(this,t,n,{callee:e})},decodeAsync:async function e(t,n){return await Ul(this,t,n,{callee:e})},safeEncode(e,t){return Bl(this,e,t)},safeDecode(e,t){return Hl(this,e,t)},async safeEncodeAsync(e,t){return Jl(this,e,t)},async safeDecodeAsync(e,t){return ql(this,e,t)},toJSONSchema(e){return ul(this,{})(e)},get description(){return Mt.get(this)?.description},get _def(){return this._zod.def}}),eu=O("_ZodString",(e,t)=>{sr.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(r,o,i)=>dl(e,r,o,i);let n=e._zod.bag;e.format=n.format??null,e.minLength=n.minimum??null,e.maxLength=n.maximum??null},{regex(...e){return this.check(Eo(...e))},includes(...e){return this.check(Mo(...e))},startsWith(...e){return this.check(No(...e))},endsWith(...e){return this.check(Do(...e))},min(...e){return this.check(Yt(...e))},max(...e){return this.check(yr(...e))},length(...e){return this.check(br(...e))},nonempty(...e){return this.check(Yt(1,...e))},lowercase(e){return this.check(To(e))},uppercase(e){return this.check(Oo(e))},trim(){return this.check(Ro())},normalize(...e){return this.check(Lo(...e))},toLowerCase(){return this.check(Zo())},toUpperCase(){return this.check(jo())},slugify(){return this.check(Vo())}}),of=O("ZodString",(e,t)=>{sr.init(e,t),eu.init(e,t)},{email(e){return this.check(Ac(af,e))},url(e){return this.check(Ao(tu,e))},jwt(e){return this.check(Wc(xf,e))},emoji(e){return this.check(Nc(cf,e))},guid(e){return this.check(Ec(sf,e))},uuid(e){return this.check(Po(Pn,e))},uuidv4(e){return this.check(Tc(Pn,e))},uuidv6(e){return this.check(Oc(Pn,e))},uuidv7(e){return this.check(Mc(Pn,e))},nanoid(e){return this.check(Dc(lf,e))},cuid(e){return this.check(Lc(uf,e))},cuid2(e){return this.check(Rc(df,e))},ulid(e){return this.check(Zc(pf,e))},base64(e){return this.check(Jc(vf,e))},base64url(e){return this.check(qc(wf,e))},xid(e){return this.check(jc(ff,e))},ksuid(e){return this.check(Vc(hf,e))},ipv4(e){return this.check(Fc(mf,e))},ipv6(e){return this.check(Uc(gf,e))},cidrv4(e){return this.check(Bc(yf,e))},cidrv6(e){return this.check(Hc(bf,e))},e164(e){return this.check(Kc($f,e))},datetime(e){return this.check(ur(An,e))},date(e){return this.check(dr(En,e))},time(e){return this.check(pr(Tn,e))},duration(e){return this.check(fr(On,e))}});Ze=O("ZodStringFormat",(e,t)=>{Me.init(e,t),eu.init(e,t)}),An=O("ZodISODateTime",(e,t)=>{Ms.init(e,t),Ze.init(e,t)}),En=O("ZodISODate",(e,t)=>{Ns.init(e,t),Ze.init(e,t)}),Tn=O("ZodISOTime",(e,t)=>{Ds.init(e,t),Ze.init(e,t)}),On=O("ZodISODuration",(e,t)=>{Ls.init(e,t),Ze.init(e,t)}),af=O("ZodEmail",(e,t)=>{_s.init(e,t),Ze.init(e,t)}),sf=O("ZodGUID",(e,t)=>{$s.init(e,t),Ze.init(e,t)}),Pn=O("ZodUUID",(e,t)=>{xs.init(e,t),Ze.init(e,t)});tu=O("ZodURL",(e,t)=>{Is.init(e,t),Ze.init(e,t)});cf=O("ZodEmoji",(e,t)=>{Cs.init(e,t),Ze.init(e,t)}),lf=O("ZodNanoID",(e,t)=>{zs.init(e,t),Ze.init(e,t)}),uf=O("ZodCUID",(e,t)=>{Ps.init(e,t),Ze.init(e,t)}),df=O("ZodCUID2",(e,t)=>{As.init(e,t),Ze.init(e,t)}),pf=O("ZodULID",(e,t)=>{Es.init(e,t),Ze.init(e,t)}),ff=O("ZodXID",(e,t)=>{Ts.init(e,t),Ze.init(e,t)}),hf=O("ZodKSUID",(e,t)=>{Os.init(e,t),Ze.init(e,t)}),mf=O("ZodIPv4",(e,t)=>{Rs.init(e,t),Ze.init(e,t)}),gf=O("ZodIPv6",(e,t)=>{js.init(e,t),Ze.init(e,t)}),yf=O("ZodCIDRv4",(e,t)=>{Vs.init(e,t),Ze.init(e,t)}),bf=O("ZodCIDRv6",(e,t)=>{Fs.init(e,t),Ze.init(e,t)}),vf=O("ZodBase64",(e,t)=>{Bs.init(e,t),Ze.init(e,t)}),wf=O("ZodBase64URL",(e,t)=>{Hs.init(e,t),Ze.init(e,t)}),$f=O("ZodE164",(e,t)=>{Js.init(e,t),Ze.init(e,t)}),xf=O("ZodJWT",(e,t)=>{qs.init(e,t),Ze.init(e,t)}),ru=O("ZodNumber",(e,t)=>{$o.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(r,o,i)=>pl(e,r,o,i);let n=e._zod.bag;e.minValue=Math.max(n.minimum??Number.NEGATIVE_INFINITY,n.exclusiveMinimum??Number.NEGATIVE_INFINITY)??null,e.maxValue=Math.min(n.maximum??Number.POSITIVE_INFINITY,n.exclusiveMaximum??Number.POSITIVE_INFINITY)??null,e.isInt=(n.format??"").includes("int")||Number.isSafeInteger(n.multipleOf??0.5),e.isFinite=!0,e.format=n.format??null},{gt(e,t){return this.check(mr(e,t))},gte(e,t){return this.check(Sn(e,t))},min(e,t){return this.check(Sn(e,t))},lt(e,t){return this.check(hr(e,t))},lte(e,t){return this.check(kn(e,t))},max(e,t){return this.check(kn(e,t))},int(e){return this.check(Kl(e))},safe(e){return this.check(Kl(e))},positive(e){return this.check(mr(0,e))},nonnegative(e){return this.check(Sn(0,e))},negative(e){return this.check(hr(0,e))},nonpositive(e){return this.check(kn(0,e))},multipleOf(e,t){return this.check(gr(e,t))},step(e,t){return this.check(gr(e,t))},finite(){return this}});_f=O("ZodNumberFormat",(e,t)=>{Ks.init(e,t),ru.init(e,t)});kf=O("ZodBoolean",(e,t)=>{Ws.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(n,r,o)=>fl(e,n,r,o)});Sf=O("ZodUnknown",(e,t)=>{Gs.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(n,r,o)=>ml(e,n,r,o)});If=O("ZodNever",(e,t)=>{Xs.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(n,r,o)=>hl(e,n,r,o)});zf=O("ZodArray",(e,t)=>{xr(),Ys.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(n,r,o)=>wl(e,n,r,o),e.element=t.element},{min(e,t){return this.check(Yt(e,t))},nonempty(e){return this.check(Yt(1,e))},max(e,t){return this.check(yr(e,t))},length(e,t){return this.check(br(e,t))},unwrap(){return this.element}});Pf=O("ZodObject",(e,t)=>{xr(),tc.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(n,r,o)=>$l(e,n,r,o),bp(e,"shape",(n)=>n._zod.def.shape,!1)},{keyof(){return at(Object.keys(this._zod.def.shape))},catchall(e){return this.clone({...this._zod.def,catchall:e})},passthrough(){return this.clone({...this._zod.def,catchall:Wl()})},loose(){return this.clone({...this._zod.def,catchall:Wl()})},strict(){return this.clone({...this._zod.def,catchall:Cf()})},strip(){return this.clone({...this._zod.def,catchall:void 0})},extend(e){return up(this,e)},safeExtend(e){return dp(this,e)},merge(e){return pp(this,e)},pick(e){return cp(this,e)},omit(e){return lp(this,e)},partial(...e){return oa(au,this,e[0])},exactPartial(...e){return oa(su,this,e[0],"exactPartial")},required(...e){return fp(cu,this,e[0])}});ou=O("ZodUnion",(e,t)=>{xo.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(n,r,o)=>xl(e,n,r,o),e.options=t.options});Ef=O("ZodDiscriminatedUnion",(e,t)=>{ou.init(e,t),nc.init(e,t)});Tf=O("ZodIntersection",(e,t)=>{rc.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(n,r,o)=>_l(e,n,r,o)});Gl=O("ZodRecord",(e,t)=>{xr(),oc.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(n,r,o)=>kl(e,n,r,o),e.keyType=t.keyType,e.valueType=t.valueType});Qo=O("ZodEnum",(e,t)=>{ic.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(r,o,i)=>gl(e,r,o,i),e.enum=t.entries,e.options=Object.values(t.entries);let n=new Set(Object.keys(t.entries));e.extract=(r,o)=>{let i={};for(let a of r)if(n.has(a))i[a]=t.entries[a];else throw Error(`Key ${a} not found in enum`);return new Qo({...t,checks:[],...ae(o),entries:i})},e.exclude=(r,o)=>{let i={...t.entries};for(let a of r)if(n.has(a))delete i[a];else throw Error(`Key ${a} not found in enum`);return new Qo({...t,checks:[],...ae(o),entries:i})}});Mf=O("ZodLiteral",(e,t)=>{ac.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(n,r,o)=>yl(e,n,r,o),e.values=new Set(t.values),Object.defineProperty(e,"value",{get(){if(t.values.length>1)throw Error("This schema contains multiple valid literal values. Use `.values` instead.");return t.values[0]}})});Nf=O("ZodTransform",(e,t)=>{xr(),sc.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(n,r,o)=>vl(e,n,r,o),e._zod.parse=(n,r)=>{if(r.direction==="backward")throw new yn(e.constructor.name);n.addIssue=(i)=>{if(typeof i==="string")n.issues.push(Tt(i,n.value,t));else{let a=i;if(a.fatal)a.continue=!1;if(a.code??(a.code="custom"),!("input"in a))a.input=n.value;a.inst??(a.inst=e),n.issues.push(Tt(a))}};let o=t.transform(n.value,n);if(o instanceof Promise)return o.then((i)=>(n.value=i,n));return n.value=o,n}});au=O("ZodOptional",(e,t)=>{_o.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(n,r,o)=>Wo(e,n,r,o),e.unwrap=()=>e._zod.def.innerType});su=O("ZodExactOptional",(e,t)=>{cc.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(n,r,o)=>Wo(e,n,r,o),e.unwrap=()=>e._zod.def.innerType});Rf=O("ZodNullable",(e,t)=>{lc.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(n,r,o)=>Sl(e,n,r,o),e.unwrap=()=>e._zod.def.innerType});Zf=O("ZodDefault",(e,t)=>{uc.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(n,r,o)=>zl(e,n,r,o),e.unwrap=()=>e._zod.def.innerType,e.removeDefault=e.unwrap});Vf=O("ZodPrefault",(e,t)=>{dc.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(n,r,o)=>Pl(e,n,r,o),e.unwrap=()=>e._zod.def.innerType});cu=O("ZodNonOptional",(e,t)=>{pc.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(n,r,o)=>Il(e,n,r,o),e.unwrap=()=>e._zod.def.innerType});Bf=O("ZodCatch",(e,t)=>{fc.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(n,r,o)=>Al(e,n,r,o),e.unwrap=()=>e._zod.def.innerType,e.removeCatch=e.unwrap});Jf=O("ZodPipe",(e,t)=>{hc.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(n,r,o)=>El(e,n,r,o),e.in=t.in,e.out=t.out});qf=O("ZodReadonly",(e,t)=>{mc.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(n,r,o)=>Tl(e,n,r,o),e.unwrap=()=>e._zod.def.innerType});Wf=O("ZodCustom",(e,t)=>{gc.init(e,t),Re.init(e,t),e._zod.processJSONSchema=(n,r,o)=>bl(e,n,r,o)})});var lu;var uu=Se(()=>{(function(e){})(lu||(lu={}))});var pt={};sp(pt,{ZodISODate:()=>En,ZodISODateTime:()=>An,ZodISODuration:()=>On,ZodISOTime:()=>Tn,date:()=>Qf,datetime:()=>Yf,duration:()=>th,time:()=>eh});function Yf(e){return ur(An,e)}function Qf(e){return dr(En,e)}function eh(e){return pr(Tn,e)}function th(e){return fr(On,e)}var du=Se(()=>{bt();_r();_r()});var pu=()=>{};var ei=Se(()=>{bt();Co();du();pu();_r();Go();Xo();Yo();uu()});var hu=Se(()=>{ei();ei()});function kr(e){return`${e.id}.png`}function ch(e,t,n){return!e||e.status==="completed"&&e.id!==t&&n?.status==="completed"&&n.id===e.id&&n.revision===e.revision}function lh(e,t){return Cr.parse({protocolVersion:1,id:e,revision:1,generation:1,status:"requested",targetIds:t,defaultCount:3})}function mu(e,t){let{action:n}=t;if(e.id!==t.explorationId||e.revision!==t.revision||e.generation!==t.generation)return null;let r=structuredClone(e);if(n.type==="report"){if(!["requested","published"].includes(e.status)||n.report.generation!==e.generation)return null;if(n.report.status==="ready"&&(e.status!=="published"||e.manifest?.generation!==e.generation))return null;return r.report=structuredClone(n.report),r}if(n.type==="publish"){if(e.status!=="requested")return null;r.manifest={generation:e.generation,choices:structuredClone(n.choices)},r.status="published",delete r.report}else if(n.type==="complete"){if(!["accepted","cancelled"].includes(e.status)||e.decision?.id!==n.decisionId)return null;r.status="completed",r.completion={decisionId:n.decisionId,summary:n.summary}}else{if(n.type==="accept"){if(e.status!=="published"||e.manifest?.generation!==e.generation||!["original",...e.manifest.choices.map((o)=>o.id)].includes(n.variantId))return null;r.status="accepted"}else if(n.type==="cancel"){if(!["requested","published","accepted"].includes(e.status))return null;r.status="cancelled"}else{if(!["published","accepted"].includes(e.status)||e.generation>=1e4)return null;r.generation++,r.status="requested",delete r.report}r.decision={id:t.id,generation:e.generation,kind:n.type,...n.type==="accept"?{variantId:n.variantId}:{},feedback:n.feedback,createdAt:new Date().toISOString()}}return r.revision++,Cr.parse(r)}function gu(e){if(e.status==="completed")return`UI Variants exploration ${e.id}, generation ${e.generation}, revision ${e.revision}. Status: completed. Cleanup has already been reported complete. No further source changes or completion calls are requested for this exploration.`;return`UI Variants exploration ${e.id}, generation ${e.generation}, revision ${e.revision}. Status: ${e.status}. Read ainotation_get_variants_guide before implementation. Generate ${e.defaultCount} candidates unless the user requests another count (1–6); original is separate. Target IDs are immutable slot IDs: ${e.targetIds.join(", ")}. Use the host framework to render one branch per slot. Register candidates through ainotation_publish_variants; registration is not proof of browser readiness. Read the latest state with ainotation_get_variants. The user decides when to ask you to continue. Do not wait, poll indefinitely, or auto-resume. After an accept/cancel decision, apply or restore the implementation, remove temporary variant integration, verify the page, then call ainotation_complete_variants with the exact decision ID.`}function ft(e){return[...e.annotations,...e.variantCleanups??[]]}function ci(e){return ph.parse(nn(e))}function nn(e,t=[],n={}){let r=structuredClone(e),o=structuredClone(r.targetStyles??{});if(!r.targetStyles){for(let c of[...r.annotations].sort((l,p)=>l.updatedAt.localeCompare(p.updatedAt)))for(let l of c.targets)if(l.styleChanges?.length){let p=Ye(l);o[p]=[...new Map([...o[p]??[],...structuredClone(l.styleChanges)].map((h)=>[h.property,h])).values()]}}let i=new Map([...Object.entries(n),...t.map((c)=>[c.id,Ye(c)])]);for(let[c,l]of i)if(o[c]&&!o[l])o[l]=o[c];for(let c of t){let l=Ye(c);if(l!==c.id&&o[c.id]&&!o[l])o[l]=o[c.id];if(c.styleChanges!==void 0)o[l]=structuredClone(c.styleChanges)}let a=new Set;for(let c of r.annotations)for(let l of c.targets){let p=i.get(l.id);if(p&&p!==l.id)l.styleTargetId=p;else if(p)delete l.styleTargetId;let h=Ye(l);if(a.add(h),delete l.styleChanges,o[h]?.length)l.styleChanges=structuredClone(o[h])}let s=Object.fromEntries(Object.entries(o).filter(([c,l])=>a.has(c)&&l.length));if(Object.keys(s).length)r.targetStyles=s;else delete r.targetStyles;return r}function zr(e,t){let n=structuredClone(e),r=t.kind==="upsert"?t.annotation.id:t.annotationId,o=n.annotations.findIndex((a)=>a.id===r),i=n.annotations[o];if(t.kind==="upsert"){if(n.variantCleanups?.some((c)=>c.id===r))return n;if(i)n.annotations[o]={...t.annotation,status:i.status,replies:i.replies,variants:i.variants};else n.annotations.push(structuredClone(t.annotation));let a=n.annotations.find((c)=>c.id===r);if(i?.variants)a.targets=structuredClone(i.targets);let s=t.variantRequest&&ch(a.variants,t.variantRequest,t.annotation.variants);if(s&&ft(n).some((c)=>c.id!==r&&qe(c.variants)))throw Error("Another annotation already owns UI Variants on this page.");if(s)a.variants=lh(t.variantRequest,a.targets.map((c)=>c.id));if(a.variants===void 0)delete a.variants}else if(t.kind==="delete"){if(i?.variants&&qe(i.variants)){let a=i.variants,s=a.status==="cancelled"?a:mu(a,{id:t.id,kind:"variants",annotationId:r,explorationId:a.id,generation:a.generation,revision:a.revision,action:{type:"cancel",feedback:a.decision?.feedback??""}});n.variantCleanups=[...(n.variantCleanups??[]).filter((c)=>c.id!==r),{id:r,comment:i.comment,createdAt:i.createdAt,updatedAt:i.updatedAt,page:structuredClone(i.page),targets:structuredClone(i.targets),variants:s,deletedAt:new Date().toISOString(),reason:"annotation-deleted"}]}n.annotations=n.annotations.filter((a)=>a.id!==r)}else if(i&&t.kind==="reply"&&!i.replies.some((a)=>a.id===t.reply.id))i.replies.push(structuredClone(t.reply)),i.updatedAt=t.reply.createdAt;else if(i&&t.kind==="reopen")i.status="pending";else if(t.kind==="variants"){let a=i??n.variantCleanups?.find((c)=>c.id===r),s=a?.variants&&mu(a.variants,t);if(s)a.variants=s}return nn(n,t.kind==="upsert"?t.annotation.targets:[],t.kind==="upsert"?t.styleLinks:void 0)}function di(e,t={}){e=nn(e);let n=t.detail??"standard",r=n==="detailed"||n==="forensic",o=n==="forensic",i=(h)=>h.replace(/\r/g,"").split(`
`).map((d)=>`> ${d}`).join(`
`),a=(h)=>{let d=h.textSelection?.exact,u=d?` — selected ${JSON.stringify(d.slice(0,30))}${d.length>30?"…":""}`:"";return`${JSON.stringify(h.label||h.tagName)} (${JSON.stringify(h.selector)})${u}`},s=(h)=>[...h.ancestors??[],h].map((d)=>{let u=d.attributes.id?`#${d.attributes.id}`:d.attributes.class?`.${d.attributes.class.split(/\s+/).filter(Boolean).join(".")}`:"",f="shadowHost"in d&&d.shadowHost?" ::shadow":"";return`${d.tagName}${u}${f}`}).join(" > "),c=new Set,l=(h)=>h.variants?`
UI Variants: ${gu(h.variants)}
${JSON.stringify(h.variants)}`:"",p=(h)=>{if(!h.styleChanges?.length)return"";let d=Ye(h);if(c.has(d))return`Style suggestions: shared target ${d} (see above).`;return c.add(d),`Style suggestions for ${JSON.stringify(h.selector)} (shared target ${d}; previewed at the recorded viewport; implement using the project's existing styles):
${h.styleChanges.map((u)=>`- ${u.property}: ${JSON.stringify(u.before)} → ${JSON.stringify(u.value)}`).join(`
`)}`};return["# Page feedback",`Page: ${e.url}`,...o?[`Session: ${e.id}`]:[],...e.annotations.map((h,d)=>n==="compact"?`${d+1}. ${h.targets.map(a).join("; ")}
${i(h.comment)}${l(h)}${h.images?.length?`
Images: ${h.images.map(kr).join(", ")}`:""}${h.targets.some((u)=>u.styleChanges?.length)?`
${h.targets.map(p).filter(Boolean).join(`
`)}`:""}`:[`## ${d+1}. ${t.includeConversation?h.status:"Annotation"} (${h.id})`,`Page: ${h.page.url}`,...o&&h.marker?[`Marker anchor: ${JSON.stringify(h.marker)}`]:[],`Viewport: ${h.page.viewport.width} x ${h.page.viewport.height}; DPR ${h.page.viewport.devicePixelRatio}; scroll ${h.page.viewport.scrollX}, ${h.page.viewport.scrollY}`,...o?[`Page title: ${JSON.stringify(h.page.title)}`,...h.page.userAgent?[`User Agent: ${JSON.stringify(h.page.userAgent)}`]:[],...h.page.capturedAt?[`Captured at: ${h.page.capturedAt}`]:[]]:[],i(h.comment),...h.variants?[l(h)]:[],...h.images?.map((u)=>`Image: ${kr(u)} (${u.width} × ${u.height}); attachment ID: ${u.id}`)??[],...h.targets.map((u,f)=>[`### Target ${f+1}`,`Selector: ${JSON.stringify(u.selector)}`,...u.styleChanges?.length?[p(u)]:[],...u.shadowHosts.length?[`Shadow hosts: ${JSON.stringify(u.shadowHosts)}`]:[],`Element: ${JSON.stringify(u.label||u.tagName)}`,...u.textSelection?[`Selected text: ${JSON.stringify(u.textSelection.exact)}${u.textSelection.truncated?" (truncated)":""}`]:[],...r?[`Classes: ${JSON.stringify(u.attributes.class||"")}`,`Bounds (viewport px): ${JSON.stringify(u.rect)}`,...u.textSelection?[`Selection context: ${JSON.stringify({prefix:u.textSelection.prefix,suffix:u.textSelection.suffix})}`]:[`Text: ${JSON.stringify(u.text)}`,...u.nearbyText?[`Nearby text: ${JSON.stringify(u.nearbyText)}`]:[]],...u.states?[`States at selection: ${JSON.stringify(u.states)}`]:[]]:[],...o?[...u.ancestors?[`DOM path${u.ancestryTruncated?" (truncated)":""}: ${JSON.stringify(s(u))}`]:[],`Attributes: ${JSON.stringify(u.attributes)}`,`Styles at annotation: ${JSON.stringify(u.styles)}`,...u.accessibility?[`Accessibility: ${JSON.stringify(u.accessibility)}`]:[],...u.nearbyElements?[`Nearby elements (${u.siblingCount??u.nearbyElements.length} siblings): ${JSON.stringify(u.nearbyElements)}`]:[],...u.textSelection?[`Selection bounds (viewport px): ${JSON.stringify(u.textSelection.rects)}`]:[]]:[]].join(`
`)),...t.includeConversation?h.replies.map((u)=>`${u.role} (${u.createdAt}):
${i(u.message)}`):[]].join(`

`)),...(e.variantCleanups??[]).filter((h)=>qe(h.variants)).map((h)=>`## Pending UI Variants cleanup (${h.id})

Annotation deleted at ${h.deletedAt}. Restore the original implementation and remove generated candidates, CSS and temporary integration.

${i(h.comment)}

${JSON.stringify(h,null,2)}

${gu(h.variants)}`)].join(`

`)}function Iu(e){return It.parse({schemaVersion:1,id:crypto.randomUUID(),url:e,createdAt:new Date().toISOString(),annotations:[]})}var tn=8388608,Sr=16777216,vu=8,ti,ni,ri,Nt,rh,oi,Ir,wu,ii,$u,oh,xu,ih,Cr,ah,sh,qe=(e)=>!!e&&e.status!=="completed",ai,yu,bu,rt,si,_u,uh,Nn,en,It,dh,ph,Ye=(e)=>e.styleTargetId??e.id,li,ku,fh,Su,ui;var pi=Se(()=>{hu();ti=_e({id:ze(),mimeType:je("image/png"),width:Le().int().positive().max(16384),height:Le().int().positive().max(16384),size:Le().int().positive().max(tn),sha256:xe().regex(/^[a-f0-9]{64}$/),source:at(["screen","import"])}).refine((e)=>e.width*e.height<=Sr,"Image exceeds the pixel limit"),ni=Be(ti).max(8).refine((e)=>new Set(e.map((t)=>t.id)).size===e.length,"Duplicate image attachment ID");ri=_e({styleSuggestions:je(!0),sharedStyles:je(!0)}),Nt=["width","height","padding-top","padding-right","padding-bottom","padding-left","margin-top","margin-right","margin-bottom","margin-left","font-size","line-height","font-weight","text-align","color","background-color","border-width","border-color","border-radius","opacity","display","gap","flex-direction","align-items","justify-content"],rh=at(Nt),oi=xe().trim().min(1).max(200).refine((e)=>{for(let t of e)if(t.charCodeAt(0)<32)return!1;return!/[;{}<>\\@"']|\/\*|(?:url|image|image-set|paint|var|attr|env|expression)\s*\(/i.test(e)},"Use a literal CSS property value without external resources."),Ir=_e({property:rh,before:xe().max(1000),value:oi}),wu=Be(Ir).max(Nt.length).refine((e)=>new Set(e.map((t)=>t.property)).size===e.length,"Each CSS property may appear only once per target."),ii=xe().regex(/^[a-z][a-z0-9-]{0,47}$/),$u=Be(_e({id:ii,label:xe().trim().min(1).max(80),description:xe().max(500).optional()})).min(1).max(6).refine((e)=>e.every((t)=>t.id!=="original")&&new Set(e.map((t)=>t.id)).size===e.length,"Candidates must have distinct IDs other than original."),oh=_e({generation:Le().int().min(1).max(1e4),choices:$u}),xu=_e({clientId:ze(),generation:Le().int().positive(),status:at(["waiting","ready","error"]),detail:xe().max(1000),observedAt:pt.datetime()}),ih=_e({id:ze(),generation:Le().int().positive(),kind:at(["accept","regenerate","cancel"]),variantId:ii.optional(),feedback:xe().trim().max(1e4),createdAt:pt.datetime()}),Cr=_e({protocolVersion:je(1),id:ze(),revision:Le().int().positive(),generation:Le().int().min(1).max(1e4),status:at(["requested","published","accepted","cancelled","completed"]),targetIds:Be(ze()).min(1).max(20).refine((e)=>new Set(e).size===e.length),defaultCount:je(3),manifest:oh.optional(),decision:ih.optional(),report:xu.optional(),completion:_e({decisionId:ze(),summary:xe().trim().min(1).max(2000)}).optional()}).superRefine((e,t)=>{let n=(r)=>t.addIssue({code:"custom",message:r});if(e.manifest&&e.manifest.generation>e.generation)n("A manifest cannot belong to a future generation.");if(["published","accepted"].includes(e.status)&&e.manifest?.generation!==e.generation)n("Published explorations require a current manifest.");if(e.status==="accepted"&&(e.decision?.kind!=="accept"||e.decision.generation!==e.generation||!["original",...e.manifest?.choices.map((r)=>r.id)??[]].includes(e.decision.variantId??"")))n("Acceptance requires a decision for a current candidate.");if(e.status==="cancelled"&&(e.decision?.kind!=="cancel"||e.decision.generation!==e.generation))n("Cancellation requires a current user decision.");if(e.report&&e.report.generation!==e.generation)n("A preview report must belong to the current generation.");if(e.status==="completed"&&(!e.decision||e.decision.kind==="regenerate"||e.completion?.decisionId!==e.decision.id))n("Completion requires the exact accepted/cancelled decision.");if(e.status!=="completed"&&e.completion)n("Only completed explorations can have a completion report.")}),ah=iu("type",[_e({type:je("publish"),choices:$u}),_e({type:je("accept"),variantId:ii,feedback:xe().trim().max(1e4).default("")}),_e({type:je("regenerate"),feedback:xe().trim().max(1e4).default("")}),_e({type:je("cancel"),feedback:xe().trim().max(1e4).default("")}),_e({type:je("report"),report:xu}),_e({type:je("complete"),decisionId:ze(),summary:xe().trim().min(1).max(2000)})]),sh=_e({id:ze(),kind:je("variants"),annotationId:ze(),explorationId:ze(),revision:Le().int().positive(),generation:Le().int().positive(),action:ah});ai=at(["compact","standard","detailed","forensic"]),yu=_e({tagName:xe().min(1).max(100),attributes:Mn(xe().max(100),xe().max(1000)),text:xe().max(160).optional(),shadowHost:Qt().optional()}),bu=_e({x:Le().finite(),y:Le().finite(),width:Le().nonnegative(),height:Le().nonnegative()}),rt=_e({id:ze(),selector:xe().min(1).max(4000),shadowHosts:Be(xe().min(1).max(4000)).max(20),tagName:xe().min(1).max(100),text:xe().max(500),attributes:Mn(xe().max(100),xe().max(1000)),rect:bu,styles:Mn(xe().max(100),xe().max(1000)),styleChanges:wu.optional(),styleTargetId:ze().optional(),states:_e({focused:Qt(),focusWithin:Qt()}).optional(),label:xe().max(160).optional(),ancestors:Be(yu).max(32).optional(),ancestryTruncated:Qt().optional(),nearbyText:_e({before:xe().max(160),after:xe().max(160)}).optional(),nearbyElements:Be(yu).max(4).optional(),siblingCount:Le().int().nonnegative().optional(),accessibility:_e({focusable:Qt()}).optional(),textSelection:_e({exact:xe().min(1).max(1000),prefix:xe().max(64),suffix:xe().max(64),truncated:Qt(),rects:Be(bu).max(32)}).optional()}),si=_e({url:nu().max(8000),title:xe().max(1000),userAgent:xe().max(1000).optional(),capturedAt:pt.datetime().optional(),viewport:_e({width:Le().positive(),height:Le().positive(),devicePixelRatio:Le().positive(),scrollX:Le().finite(),scrollY:Le().finite()})}),_u=_e({id:ze(),role:at(["human","agent"]),message:xe().trim().min(1).max(1e4),createdAt:pt.datetime()}),uh=at(["pending","acknowledged","resolved","dismissed"]),Nn=_e({x:Le().finite(),y:Le().finite(),space:at(["document","viewport"]),targetId:ze().optional(),ratioX:Le().finite().optional(),ratioY:Le().finite().optional()}),en=_e({id:ze(),comment:xe().trim().min(1).max(1e4),createdAt:pt.datetime(),updatedAt:pt.datetime(),page:si,targets:Be(rt).min(1).max(20),marker:Nn.optional(),images:ni.optional(),variants:Cr.optional(),status:uh,replies:Be(_u).max(500)}),It=_e({schemaVersion:je(1),id:ze(),url:nu().max(8000),createdAt:pt.datetime(),annotations:Be(en).max(1000),targetStyles:Mn(ze(),wu).optional(),variantCleanups:Be(en.omit({status:!0,replies:!0,images:!0,marker:!0}).extend({variants:Cr.refine((e)=>["cancelled","completed"].includes(e.status)&&e.decision?.kind==="cancel","Deleted explorations must remain cancelled until cleanup completes."),deletedAt:pt.datetime(),reason:je("annotation-deleted")})).max(1000).optional()}),dh=en.omit({status:!0,replies:!0}),ph=It.extend({annotations:Be(dh).max(1000)});li=iu("kind",[_e({id:ze(),kind:je("upsert"),annotation:en,styleLinks:Mn(ze(),ze()).optional(),variantRequest:ze().optional()}),_e({id:ze(),kind:je("delete"),annotationId:ze()}),_e({id:ze(),kind:je("reply"),annotationId:ze(),reply:_u.extend({role:je("human")})}),_e({id:ze(),kind:je("reopen"),annotationId:ze()}),sh]),ku=_e({document:It,operations:Be(li).max(1000),storageEpoch:ze().optional(),recovery:_e({epoch:ze(),revision:xe().regex(/^[a-f0-9]{64}$/),source:at(["browser","server"])}).optional()}),fh=at(["recovery-stale","storage-conflict","storage-unavailable","storage-damaged","service-ownership","variants-busy"]),Su=_e({code:fh.optional()}),ui=_e({document:It,acknowledged:Be(ze()),styleSuggestions:je(!0).optional(),sharedStyles:je(!0).optional(),uiVariants:je(1).optional(),uiVariantsCleanup:je(1).optional(),variantConflicts:Be(ze()).max(1000).optional(),storageEpoch:ze().optional(),missingImages:Be(ze()).optional(),recovery:_e({revision:xe().regex(/^[a-f0-9]{64}$/)}).optional()}).refine((e)=>!e.recovery||!!e.storageEpoch,"Recovery responses require a storage epoch.")});function Uu(e,t){if(!$i(e)||!e.hasOwnProperty("raw"))throw Error("invalid template strings array");return Tu!==void 0?Tu.createHTML(t):t}function Zt(e,t,n=e,r){if(t===Ct)return t;let o=r!==void 0?n._$Co?.[r]:n._$Cl,i=Zn(t)?void 0:t._$litDirective$;return o?.constructor!==i&&(o?._$AO?.(!1),i===void 0?o=void 0:(o=new i(e),o._$AT(e,n,r)),r!==void 0?(n._$Co??=[])[r]=o:n._$Cl=o),o!==void 0&&(t=Zt(e,o._$AS(e,t.values),o,r)),t}function Ut(e){return typeof e==="string"&&_i.includes(e)}function st(e){return Ut(e)?Fh[e]:ht}function ki(e=navigator.languages){for(let t of e){let n;try{n=new Intl.Locale(t)}catch{continue}if(n.language==="zh")return n.maximize().script==="Hant"?"zh-Hant":"zh-Hans";if(n.language==="en"||n.language==="ja"||n.language==="ko")return n.language}return"en"}function we(e,...t){return{key:e,args:t}}function mt(e,t){if(typeof t==="string")return t;let n=st(e)[t.key];return typeof n==="string"?n:n(...t.args)}function le(e,...t){return new Dr(we(e,...t))}function sn(e,t="operationFailed"){if(e instanceof Dr)return e.description;return{key:t,args:[]}}function Lr(e="en"){let t=e,n=new Set;return{get locale(){return t},get messages(){return st(t)},setLocale(r){if(!Ut(r)||r===t)return;t=r;for(let o of n)o()},subscribe(r,o){if(o.aborted)return;n.add(r),o.addEventListener("abort",()=>n.delete(r),{once:!0})}}}function cn(){let e=window.visualViewport;return{left:e?.offsetLeft??0,top:e?.offsetTop??0,width:e?.width??document.documentElement.clientWidth,height:e?.height??window.innerHeight}}function Qu(e,t,n){return{left:Math.max(n.left,Math.min(e.left,n.left+n.width-t.width)),top:Math.max(n.top,Math.min(e.top,n.top+n.height-t.height))}}function Si(e,t,n){return{left:e.left+(t&&n?e.width-48:0),top:e.top+(t?e.height-48:0),opensLeft:n}}function ed(e,t,n){return{left:e.left-(n&&e.opensLeft?t.width-48:0),top:e.top-(n?t.height-48:0)}}function td(e,t,n){let r=Math.min(294,Math.max(0,n.width-32)),o=e.left+e.width-r>=n.left,i=e.left+r<=n.left+n.width;if(t&&!o)t=!1;else if(!t&&!i)t=!0;return{point:{left:t?e.left+e.width-r:e.left,top:e.top+e.height-52},opensLeft:t}}function Qe(e){let t=e.getRootNode();return e.parentElement??(t instanceof ShadowRoot?t.host:null)}function Ii(e,t){for(let n=e;n;n=Qe(n))if(n.matches(t))return n;return null}function Rr(e,t){for(let n=e;n;n=Qe(n))if(n.hasAttribute("data-ainotation-ui")||t?.(n))return!0;return!1}function Bt(e){let t=e instanceof Element?e:e instanceof ShadowRoot?e.host:e.parentElement;return!!t&&Rr(t)}function Zr(e,t){let n=[],r=document.elementFromPoint(e,t);while(r?.shadowRoot){n.push(r.shadowRoot);let o=r.shadowRoot.elementFromPoint(e,t);if(!o||o===r)break;r=o}return{element:r,shadowRoots:n}}function jr(e){return e.some((t,n)=>e.some((r,o)=>n!==o&&Bh(t,r)))}function nd(e){let t=Ki(),n=crypto.randomUUID(),r,o=Gt,i=Gt,a=Ar(),s=!1,c=!1,l,p="",h="",d=null,u=new Set,f=[],x=new MutationObserver((j)=>{if(j.some((B)=>!(B.target instanceof Element?B.target:B.target.parentElement)?.closest("[data-ainotation-ui], ainotation-inspector-shell")))T()}),k=new ResizeObserver(T),w=new Set,P=new AbortController,D=()=>[...t.providers].filter((j)=>j.options.explorationId===r?.id);function T(){if(c||s)return;c=!0,queueMicrotask(()=>{if(c=!1,!s)W()})}function Z(){for(let j of u)j.set(Gt);u.clear(),f=[],clearTimeout(l),l=void 0}function z(j){a=j;let B=new Set(D().flatMap((H)=>[...H.bindings].filter((Y)=>Y.element.isConnected).map((Y)=>Y.element)));for(let H of w)if(!B.has(H))k.unobserve(H),w.delete(H);for(let H of B){if(!w.has(H))k.observe(H),w.add(H);let Y=H.getRootNode();while(Y instanceof ShadowRoot)x.observe(Y,{subtree:!0,childList:!0,attributes:!0}),Y=Y.host.getRootNode()}let ue=f.map(({element:H})=>{let Y=H.getBoundingClientRect();return[Y.x,Y.y,Y.width,Y.height]}),ve=JSON.stringify([j,ue,r?.id,r?.revision]);if(p!==ve)p=ve,e.onChange();if(r&&["requested","published"].includes(r.status)&&o.generation===r.generation){let H={clientId:n,generation:r.generation,status:j.status==="ready"?"ready":j.status==="error"?"error":"waiting",detail:j.problem??"",observedAt:new Date().toISOString()},Y=JSON.stringify([r.id,r.generation,H.status,H.detail]);if(h!==Y){if(e.onReport(H)!==!1)h=Y}}}function W(){if(x.disconnect(),f=[],!r){z(Ar());return}x.observe(document,{subtree:!0,childList:!0,attributes:!0});let j=D();if(r.status==="completed"){Z(),z({...Ar(),status:j.length?"error":"completed",problem:j.length?"cleanup":null});return}let B=r.manifest,ue=null,ve=j[0];if(!B||!ve)ue="missing";else if(j.length!==1)ue="duplicate";else{let H=ve.options.generations.find((Y)=>Y.generation===B.generation);if(!H||H.variants.length!==B.choices.length||B.choices.some((Y)=>!H.variants.includes(Y.id))||ve.options.targetIds.length!==r.targetIds.length||r.targetIds.some((Y)=>!ve.options.targetIds.includes(Y)))ue="mismatch";else{if(u.add(ve),ve.set(o),ve.failed)ue="render";for(let Y of r.targetIds){let fe=[...ve.bindings].filter((ne)=>ne.targetId===Y&&ne.element.isConnected);if(fe.length!==1){ue??=fe.length?"duplicate":"missing";continue}let y=fe[0];if(!Uh(y.snapshot,o)){ue??="mismatch";continue}if(y.element.getAttribute("data-ainotation-exploration")!==r.id||y.element.getAttribute("data-ainotation-generation")!==String(o.generation)||y.element.getAttribute("data-ainotation-slot")!==Y||y.element.getAttribute("data-ainotation-variant")!==o.variantId){ue??="mismatch";continue}let N=y.element.getBoundingClientRect();if(!N.width||!N.height||getComputedStyle(y.element).visibility==="hidden"){ue??="missing";continue}f.push(y)}if(jr(f.map((Y)=>Y.element)))ue="overlap"}}if(ue)z({...o,status:l?"switching":d||["duplicate","overlap","render"].includes(ue)?"error":"waiting",problem:d??ue});else clearTimeout(l),l=void 0,i=o,z({...o,status:d?"error":"ready",problem:d})}return t.listeners.add(T),window.addEventListener("resize",T,{signal:P.signal}),window.addEventListener("scroll",T,{signal:P.signal,capture:!0}),{state:()=>({...a}),rect(j){let B=f.find((ue)=>ue.targetId===j)?.element;return B?.isConnected?B.getBoundingClientRect():null},elements:()=>D().flatMap((j)=>[...j.bindings].filter((B)=>B.element.isConnected).map((B)=>B.element)),sync(j){let B=r;if(r=j,B?.id!==j?.id)Z(),d=null,h="",o=Gt,i=Gt;if(j?.status==="cancelled"&&B?.status!=="cancelled")Z();if(j?.status==="cancelled"||j?.manifest&&o.generation!==j.manifest.generation)o=Object.freeze({generation:j.manifest?.generation??0,variantId:"original"}),i=o,d=null;if(j?.status==="accepted"&&j.decision?.variantId&&B?.revision!==j.revision)o=Object.freeze({generation:j.generation,variantId:j.decision.variantId});T()},select(j){let B=r?.manifest;if(!B||!["requested","published"].includes(r.status)||!["original",...B.choices.map((ue)=>ue.id)].includes(j))return!1;return d=null,o=Object.freeze({generation:B.generation,variantId:j}),clearTimeout(l),l=setTimeout(()=>{l=void 0,d=a.problem??"render",o=i,T()},2500),T(),!0},destroy(){s=!0,Z(),t.listeners.delete(T),x.disconnect(),k.disconnect(),w.clear(),P.abort()}}}function Vr(){return{document:null,selected:[],targetNavigation:{},targetsAdjusted:!1,availability:{},picking:!1,passthrough:!1,draft:"",editingId:null,editorOpen:!1,marker:null,saving:!1,message:"",storage:"loading",connection:"offline",endpoint:"http://127.0.0.1:4748",syncing:!1,outputDetail:"standard",theme:"light",locale:"en",managedConnection:!1,localOnly:!1,recoveryNeeded:!1,recoveringProject:!1,recoveryPages:[],hasRecoveryCopy:!1,projectName:"",images:[],imageUrls:{},editorTab:"feedback",editorSessionId:"",editorPosition:null,styleTargetId:"",styleTargets:[],variantsRequested:!1,variantsSupported:!1,variantsComparing:!1,variantAnnotationId:"",variantPreview:Ar(),variantFeedback:"",variantPosition:null,variantMinimized:!1,variantSaving:!1,variantStyleBlocked:!1,variantConflict:!1,styleEditor:{preview:!1,values:{},changes:[],count:0,dirty:!1,canUndo:!1,canRedo:!1,problem:null,globalPreview:!0,globalCount:0,previewMixed:!1,scopeCount:0,current:{},mixed:[],mixedOriginal:[],stepProperties:[],sharedMarkers:0}}}var Pr,gi,yi,Cu,Ru=class{constructor(e,t,n){if(this._$cssResult$=!0,n!==yi)throw Error("CSSResult is not constructable. Use `unsafeCSS` or `css` instead.");this.cssText=e,this.t=t}get styleSheet(){let e=this.o,t=this.t;if(gi&&e===void 0){let n=t!==void 0&&t.length===1;n&&(e=Cu.get(t)),e===void 0&&((this.o=e=new CSSStyleSheet).replaceSync(this.cssText),n&&Cu.set(t,e))}return e}toString(){return this.cssText}},hh=(e)=>new Ru(typeof e=="string"?e:e+"",void 0,yi),wt=(e,...t)=>new Ru(e.length===1?e[0]:t.reduce((n,r,o)=>n+((i)=>{if(i._$cssResult$===!0)return i.cssText;if(typeof i=="number")return i;throw Error("Value passed to 'css' function must be a 'css' function result: "+i+". Use 'unsafeCSS' to pass non-literal values, but take care to ensure page security.")})(r)+e[o+1],e[0]),e,yi),mh=(e,t)=>{if(gi)e.adoptedStyleSheets=t.map((n)=>n instanceof CSSStyleSheet?n:n.styleSheet);else for(let n of t){let r=document.createElement("style"),o=Pr.litNonce;o!==void 0&&r.setAttribute("nonce",o),r.textContent=n.cssText,e.appendChild(r)}},zu,gh,yh,bh,vh,wh,$h,Tr,Pu,xh,_h,Ln=(e,t)=>e,hi,Zu=(e,t)=>!gh(e,t),Au,rn,bi,Eu=(e)=>e,Er,Tu,vi="$lit$",vt,wi,kh,Rt,Rn=()=>Rt.createComment(""),Zn=(e)=>e===null||typeof e!="object"&&typeof e!="function",$i,ju=(e)=>$i(e)||typeof e?.[Symbol.iterator]=="function",fi=`[ 	
\f\r]`,Dn,Ou,Mu,Dt,Nu,Du,Vu,Fu=(e)=>(t,...n)=>({_$litType$:e,strings:t,values:n}),J,We,Ct,K,Lu,Lt,Bu=(e,t)=>{let n=e.length-1,r=[],o,i=t===2?"<svg>":t===3?"<math>":"",a=Dn;for(let s=0;s<n;s++){let c=e[s],l,p,h=-1,d=0;for(;d<c.length&&(a.lastIndex=d,p=a.exec(c),p!==null);)d=a.lastIndex,a===Dn?p[1]==="!--"?a=Ou:p[1]!==void 0?a=Mu:p[2]!==void 0?(Vu.test(p[2])&&(o=RegExp("</"+p[2],"g")),a=Dt):p[3]!==void 0&&(a=Dt):a===Dt?p[0]===">"?(a=o??Dn,h=-1):p[1]===void 0?h=-2:(h=a.lastIndex-p[2].length,l=p[1],a=p[3]===void 0?Dt:p[3]==='"'?Du:Nu):a===Du||a===Nu?a=Dt:a===Ou||a===Mu?a=Dn:(a=Dt,o=void 0);let u=a===Dt&&e[s+1].startsWith("/>")?" ":"";i+=a===Dn?c+kh:h>=0?(r.push(l),c.slice(0,h)+vi+c.slice(h)+vt+u):c+vt+(h===-2?s:u)}return[Uu(e,i+(e[n]||"<?>")+(t===2?"</svg>":t===3?"</math>":"")),r]},mi=class e{constructor({strings:t,_$litType$:n},r){let o;this.parts=[];let i=0,a=0,s=t.length-1,c=this.parts,[l,p]=Bu(t,n);if(this.el=e.createElement(l,r),Lt.currentNode=this.el.content,n===2||n===3){let h=this.el.content.firstChild;h.replaceWith(...h.childNodes)}for(;(o=Lt.nextNode())!==null&&c.length<s;){if(o.nodeType===1){if(o.hasAttributes())for(let h of o.getAttributeNames())if(h.endsWith(vi)){let d=p[a++],u=o.getAttribute(h).split(vt),f=/([.?@])?(.*)/.exec(d);c.push({type:1,index:i,name:f[2],strings:u,ctor:f[1]==="."?Ju:f[1]==="?"?qu:f[1]==="@"?Ku:jn}),o.removeAttribute(h)}else h.startsWith(vt)&&(c.push({type:6,index:i}),o.removeAttribute(h));if(Vu.test(o.tagName)){let h=o.textContent.split(vt),d=h.length-1;if(d>0){o.textContent=Er?Er.emptyScript:"";for(let u=0;u<d;u++)o.append(h[u],Rn()),Lt.nextNode(),c.push({type:2,index:++i});o.append(h[d],Rn())}}}else if(o.nodeType===8)if(o.data===wi)c.push({type:2,index:i});else{let h=-1;for(;(h=o.data.indexOf(vt,h+1))!==-1;)c.push({type:7,index:i}),h+=vt.length-1}i++}}static createElement(t,n){let r=Rt.createElement("template");return r.innerHTML=t,r}},Hu=class{constructor(e,t){this._$AV=[],this._$AN=void 0,this._$AD=e,this._$AM=t}get parentNode(){return this._$AM.parentNode}get _$AU(){return this._$AM._$AU}u(e){let{el:{content:t},parts:n}=this._$AD,r=(e?.creationScope??Rt).importNode(t,!0);Lt.currentNode=r;let o=Lt.nextNode(),i=0,a=0,s=n[0];for(;s!==void 0;){if(i===s.index){let c;s.type===2?c=new Or(o,o.nextSibling,this,e):s.type===1?c=new s.ctor(o,s.name,s.strings,this,e):s.type===6&&(c=new Wu(o,this,e)),this._$AV.push(c),s=n[++a]}i!==s?.index&&(o=Lt.nextNode(),i++)}return Lt.currentNode=Rt,r}p(e){let t=0;for(let n of this._$AV)n!==void 0&&(n.strings!==void 0?(n._$AI(e,n,t),t+=n.strings.length-2):n._$AI(e[t])),t++}},Or=class e{get _$AU(){return this._$AM?._$AU??this._$Cv}constructor(t,n,r,o){this.type=2,this._$AH=K,this._$AN=void 0,this._$AA=t,this._$AB=n,this._$AM=r,this.options=o,this._$Cv=o?.isConnected??!0}get parentNode(){let t=this._$AA.parentNode,n=this._$AM;return n!==void 0&&t?.nodeType===11&&(t=n.parentNode),t}get startNode(){return this._$AA}get endNode(){return this._$AB}_$AI(t,n=this){t=Zt(this,t,n),Zn(t)?t===K||t==null||t===""?(this._$AH!==K&&this._$AR(),this._$AH=K):t!==this._$AH&&t!==Ct&&this._(t):t._$litType$!==void 0?this.$(t):t.nodeType!==void 0?this.T(t):ju(t)?this.k(t):this._(t)}O(t){return this._$AA.parentNode.insertBefore(t,this._$AB)}T(t){this._$AH!==t&&(this._$AR(),this._$AH=this.O(t))}_(t){this._$AH!==K&&Zn(this._$AH)?this._$AA.nextSibling.data=t:this.T(Rt.createTextNode(t)),this._$AH=t}$(t){let{values:n,_$litType$:r}=t,o=typeof r=="number"?this._$AC(t):(r.el===void 0&&(r.el=mi.createElement(Uu(r.h,r.h[0]),this.options)),r);if(this._$AH?._$AD===o)this._$AH.p(n);else{let i=new Hu(o,this),a=i.u(this.options);i.p(n),this.T(a),this._$AH=i}}_$AC(t){let n=Lu.get(t.strings);return n===void 0&&Lu.set(t.strings,n=new mi(t)),n}k(t){$i(this._$AH)||(this._$AH=[],this._$AR());let n=this._$AH,r,o=0;for(let i of t)o===n.length?n.push(r=new e(this.O(Rn()),this.O(Rn()),this,this.options)):r=n[o],r._$AI(i),o++;o<n.length&&(this._$AR(r&&r._$AB.nextSibling,o),n.length=o)}_$AR(t=this._$AA.nextSibling,n){for(this._$AP?.(!1,!0,n);t!==this._$AB;){let r=Eu(t).nextSibling;Eu(t).remove(),t=r}}setConnected(t){this._$AM===void 0&&(this._$Cv=t,this._$AP?.(t))}},jn=class{get tagName(){return this.element.tagName}get _$AU(){return this._$AM._$AU}constructor(e,t,n,r,o){this.type=1,this._$AH=K,this._$AN=void 0,this.element=e,this.name=t,this._$AM=r,this.options=o,n.length>2||n[0]!==""||n[1]!==""?(this._$AH=Array(n.length-1).fill(new String),this.strings=n):this._$AH=K}_$AI(e,t=this,n,r){let o=this.strings,i=!1;if(o===void 0)e=Zt(this,e,t,0),i=!Zn(e)||e!==this._$AH&&e!==Ct,i&&(this._$AH=e);else{let a=e,s,c;for(e=o[0],s=0;s<o.length-1;s++)c=Zt(this,a[n+s],t,s),c===Ct&&(c=this._$AH[s]),i||=!Zn(c)||c!==this._$AH[s],c===K?e=K:e!==K&&(e+=(c??"")+o[s+1]),this._$AH[s]=c}i&&!r&&this.j(e)}j(e){e===K?this.element.removeAttribute(this.name):this.element.setAttribute(this.name,e??"")}},Ju,qu,Ku,Wu=class{constructor(e,t,n){this.element=e,this.type=6,this._$AN=void 0,this._$AM=t,this.options=n}get _$AU(){return this._$AM._$AU}_$AI(e){Zt(this,e)}},Gu,Sh,jt=(e,t,n)=>{let r=n?.renderBefore??t,o=r._$litPart$;if(o===void 0){let i=n?.renderBefore??null;r._$litPart$=o=new Or(t.insertBefore(Rn(),i),i,void 0,n??{})}return o._$AI(e),o},xi,on,Ih,Ch,Xu=([e,t,n])=>{let r=document.createElementNS("http://www.w3.org/2000/svg",e);if(Object.keys(t).forEach((o)=>{r.setAttribute(o,String(t[o]))}),n?.length)n.forEach((o)=>{let i=Xu(o);r.appendChild(i)});return r},Fe=(e,t={})=>{let r={...Ch,...t};return Xu(["svg",r,e])},Mr,Nr,Vt,an,Ft,Vn=(e)=>(t)=>e[Nt.indexOf(t)]??t,zh,Ph,Ah,Eh,Th,Oh,Mh,Nh,Dh,Lh,ht,Rh,Zh,jh,Vh,_i,Yu,Fh,Dr,Ar=()=>({status:"waiting",generation:0,variantId:"original",problem:null}),Uh=(e,t)=>e.generation===t.generation&&e.variantId===t.variantId,Bh=(e,t)=>{for(let n=t;n;n=Qe(n))if(n===e)return!0;return!1},ln;var Fn=Se(()=>{Wi();pi();Pr=globalThis,gi=Pr.ShadowRoot&&(Pr.ShadyCSS===void 0||Pr.ShadyCSS.nativeShadow)&&"adoptedStyleSheets"in Document.prototype&&"replace"in CSSStyleSheet.prototype,yi=Symbol(),Cu=new WeakMap,zu=gi?(e)=>e:(e)=>e instanceof CSSStyleSheet?((t)=>{let n="";for(let r of t.cssRules)n+=r.cssText;return hh(n)})(e):e,{is:gh,defineProperty:yh,getOwnPropertyDescriptor:bh,getOwnPropertyNames:vh,getOwnPropertySymbols:wh,getPrototypeOf:$h}=Object,Tr=globalThis,Pu=Tr.trustedTypes,xh=Pu?Pu.emptyScript:"",_h=Tr.reactiveElementPolyfillSupport,hi={toAttribute(e,t){switch(t){case Boolean:e=e?xh:null;break;case Object:case Array:e=e==null?e:JSON.stringify(e)}return e},fromAttribute(e,t){let n=e;switch(t){case Boolean:n=e!==null;break;case Number:n=e===null?null:Number(e);break;case Object:case Array:try{n=JSON.parse(e)}catch(r){n=null}}return n}},Au={attribute:!0,type:String,converter:hi,reflect:!1,useDefault:!1,hasChanged:Zu};Symbol.metadata??=Symbol("metadata"),Tr.litPropertyMetadata??=new WeakMap;rn=class extends HTMLElement{static addInitializer(e){this._$Ei(),(this.l??=[]).push(e)}static get observedAttributes(){return this.finalize(),this._$Eh&&[...this._$Eh.keys()]}static createProperty(e,t=Au){if(t.state&&(t.attribute=!1),this._$Ei(),this.prototype.hasOwnProperty(e)&&((t=Object.create(t)).wrapped=!0),this.elementProperties.set(e,t),!t.noAccessor){let n=Symbol(),r=this.getPropertyDescriptor(e,n,t);r!==void 0&&yh(this.prototype,e,r)}}static getPropertyDescriptor(e,t,n){let{get:r,set:o}=bh(this.prototype,e)??{get(){return this[t]},set(i){this[t]=i}};return{get:r,set(i){let a=r?.call(this);o?.call(this,i),this.requestUpdate(e,a,n)},configurable:!0,enumerable:!0}}static getPropertyOptions(e){return this.elementProperties.get(e)??Au}static _$Ei(){if(this.hasOwnProperty(Ln("elementProperties")))return;let e=$h(this);e.finalize(),e.l!==void 0&&(this.l=[...e.l]),this.elementProperties=new Map(e.elementProperties)}static finalize(){if(this.hasOwnProperty(Ln("finalized")))return;if(this.finalized=!0,this._$Ei(),this.hasOwnProperty(Ln("properties"))){let t=this.properties,n=[...vh(t),...wh(t)];for(let r of n)this.createProperty(r,t[r])}let e=this[Symbol.metadata];if(e!==null){let t=litPropertyMetadata.get(e);if(t!==void 0)for(let[n,r]of t)this.elementProperties.set(n,r)}this._$Eh=new Map;for(let[t,n]of this.elementProperties){let r=this._$Eu(t,n);r!==void 0&&this._$Eh.set(r,t)}this.elementStyles=this.finalizeStyles(this.styles)}static finalizeStyles(e){let t=[];if(Array.isArray(e)){let n=new Set(e.flat(1/0).reverse());for(let r of n)t.unshift(zu(r))}else e!==void 0&&t.push(zu(e));return t}static _$Eu(e,t){let n=t.attribute;return n===!1?void 0:typeof n=="string"?n:typeof e=="string"?e.toLowerCase():void 0}constructor(){super(),this._$Ep=void 0,this.isUpdatePending=!1,this.hasUpdated=!1,this._$Em=null,this._$Ev()}_$Ev(){this._$ES=new Promise((e)=>this.enableUpdating=e),this._$AL=new Map,this._$E_(),this.requestUpdate(),this.constructor.l?.forEach((e)=>e(this))}addController(e){(this._$EO??=new Set).add(e),this.renderRoot!==void 0&&this.isConnected&&e.hostConnected?.()}removeController(e){this._$EO?.delete(e)}_$E_(){let e=new Map,t=this.constructor.elementProperties;for(let n of t.keys())this.hasOwnProperty(n)&&(e.set(n,this[n]),delete this[n]);e.size>0&&(this._$Ep=e)}createRenderRoot(){let e=this.shadowRoot??this.attachShadow(this.constructor.shadowRootOptions);return mh(e,this.constructor.elementStyles),e}connectedCallback(){this.renderRoot??=this.createRenderRoot(),this.enableUpdating(!0),this._$EO?.forEach((e)=>e.hostConnected?.())}enableUpdating(e){}disconnectedCallback(){this._$EO?.forEach((e)=>e.hostDisconnected?.())}attributeChangedCallback(e,t,n){this._$AK(e,n)}_$ET(e,t){let n=this.constructor.elementProperties.get(e),r=this.constructor._$Eu(e,n);if(r!==void 0&&n.reflect===!0){let o=(n.converter?.toAttribute!==void 0?n.converter:hi).toAttribute(t,n.type);this._$Em=e,o==null?this.removeAttribute(r):this.setAttribute(r,o),this._$Em=null}}_$AK(e,t){let n=this.constructor,r=n._$Eh.get(e);if(r!==void 0&&this._$Em!==r){let o=n.getPropertyOptions(r),i=typeof o.converter=="function"?{fromAttribute:o.converter}:o.converter?.fromAttribute!==void 0?o.converter:hi;this._$Em=r;let a=i.fromAttribute(t,o.type);this[r]=a??this._$Ej?.get(r)??a,this._$Em=null}}requestUpdate(e,t,n,r=!1,o){if(e!==void 0){let i=this.constructor;if(r===!1&&(o=this[e]),n??=i.getPropertyOptions(e),!((n.hasChanged??Zu)(o,t)||n.useDefault&&n.reflect&&o===this._$Ej?.get(e)&&!this.hasAttribute(i._$Eu(e,n))))return;this.C(e,t,n)}this.isUpdatePending===!1&&(this._$ES=this._$EP())}C(e,t,{useDefault:n,reflect:r,wrapped:o},i){n&&!(this._$Ej??=new Map).has(e)&&(this._$Ej.set(e,i??t??this[e]),o!==!0||i!==void 0)||(this._$AL.has(e)||(this.hasUpdated||n||(t=void 0),this._$AL.set(e,t)),r===!0&&this._$Em!==e&&(this._$Eq??=new Set).add(e))}async _$EP(){this.isUpdatePending=!0;try{await this._$ES}catch(t){Promise.reject(t)}let e=this.scheduleUpdate();return e!=null&&await e,!this.isUpdatePending}scheduleUpdate(){return this.performUpdate()}performUpdate(){if(!this.isUpdatePending)return;if(!this.hasUpdated){if(this.renderRoot??=this.createRenderRoot(),this._$Ep){for(let[r,o]of this._$Ep)this[r]=o;this._$Ep=void 0}let n=this.constructor.elementProperties;if(n.size>0)for(let[r,o]of n){let{wrapped:i}=o,a=this[r];i!==!0||this._$AL.has(r)||a===void 0||this.C(r,void 0,o,a)}}let e=!1,t=this._$AL;try{e=this.shouldUpdate(t),e?(this.willUpdate(t),this._$EO?.forEach((n)=>n.hostUpdate?.()),this.update(t)):this._$EM()}catch(n){throw e=!1,this._$EM(),n}e&&this._$AE(t)}willUpdate(e){}_$AE(e){this._$EO?.forEach((t)=>t.hostUpdated?.()),this.hasUpdated||(this.hasUpdated=!0,this.firstUpdated(e)),this.updated(e)}_$EM(){this._$AL=new Map,this.isUpdatePending=!1}get updateComplete(){return this.getUpdateComplete()}getUpdateComplete(){return this._$ES}shouldUpdate(e){return!0}update(e){this._$Eq&&=this._$Eq.forEach((t)=>this._$ET(t,this[t])),this._$EM()}updated(e){}firstUpdated(e){}};rn.elementStyles=[],rn.shadowRootOptions={mode:"open"},rn[Ln("elementProperties")]=new Map,rn[Ln("finalized")]=new Map,_h?.({ReactiveElement:rn}),(Tr.reactiveElementVersions??=[]).push("2.1.2");bi=globalThis,Er=bi.trustedTypes,Tu=Er?Er.createPolicy("lit-html",{createHTML:(e)=>e}):void 0,vt=`lit$${Math.random().toFixed(9).slice(2)}$`,wi="?"+vt,kh=`<${wi}>`,Rt=document,$i=Array.isArray,Dn=/<(?:(!--|\/[^a-zA-Z])|(\/?[a-zA-Z][^>\s]*)|(\/?$))/g,Ou=/-->/g,Mu=/>/g,Dt=RegExp(`>|${fi}(?:([^\\s"'>=/]+)(${fi}*=${fi}*(?:[^ 	
\f\r"'\`<>=]|("|')|))|$)`,"g"),Nu=/'/g,Du=/"/g,Vu=/^(?:script|style|textarea|title)$/i,J=Fu(1),We=Fu(2),Ct=Symbol.for("lit-noChange"),K=Symbol.for("lit-nothing"),Lu=new WeakMap,Lt=Rt.createTreeWalker(Rt,129);Ju=class extends jn{constructor(){super(...arguments),this.type=3}j(e){this.element[this.name]=e===K?void 0:e}},qu=class extends jn{constructor(){super(...arguments),this.type=4}j(e){this.element.toggleAttribute(this.name,!!e&&e!==K)}},Ku=class extends jn{constructor(e,t,n,r,o){super(e,t,n,r,o),this.type=5}_$AI(e,t=this){if((e=Zt(this,e,t,0)??K)===Ct)return;let n=this._$AH,r=e===K&&n!==K||e.capture!==n.capture||e.once!==n.once||e.passive!==n.passive,o=e!==K&&(n===K||r);r&&this.element.removeEventListener(this.name,this,n),o&&this.element.addEventListener(this.name,this,e),this._$AH=e}handleEvent(e){typeof this._$AH=="function"?this._$AH.call(this.options?.host??this.element,e):this._$AH.handleEvent(e)}},Gu={M:vi,P:vt,A:wi,C:1,L:Bu,R:Hu,D:ju,V:Zt,I:Or,H:jn,N:qu,U:Ku,B:Ju,F:Wu},Sh=bi.litHtmlPolyfillSupport;Sh?.(mi,Or),(bi.litHtmlVersions??=[]).push("3.3.3");xi=globalThis,on=class extends rn{constructor(){super(...arguments),this.renderOptions={host:this},this._$Do=void 0}createRenderRoot(){let e=super.createRenderRoot();return this.renderOptions.renderBefore??=e.firstChild,e}update(e){let t=this.render();this.hasUpdated||(this.renderOptions.isConnected=this.isConnected),super.update(e),this._$Do=jt(t,this.renderRoot,this.renderOptions)}connectedCallback(){super.connectedCallback(),this._$Do?.setConnected(!0)}disconnectedCallback(){super.disconnectedCallback(),this._$Do?.setConnected(!1)}render(){return Ct}};on._$litElement$=!0,on.finalized=!0,xi.litElementHydrateSupport?.({LitElement:on});Ih=xi.litElementPolyfillSupport;Ih?.({LitElement:on});(xi.litElementVersions??=[]).push("4.2.2");Ch={xmlns:"http://www.w3.org/2000/svg",width:24,height:24,viewBox:"0 0 24 24",fill:"none",stroke:"currentColor","stroke-width":2,"stroke-linecap":"round","stroke-linejoin":"round"},Mr=[["rect",{width:"14",height:"14",x:"8",y:"8",rx:"2",ry:"2"}],["path",{d:"M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"}]],Nr=[["path",{d:"M12 15V3"}],["path",{d:"M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"}],["path",{d:"m7 10 5 5 5-5"}]],Vt=[["path",{d:"M10 11v6"}],["path",{d:"M14 11v6"}],["path",{d:"M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"}],["path",{d:"M3 6h18"}],["path",{d:"M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"}]],an=[["path",{d:"M18 6 6 18"}],["path",{d:"m6 6 12 12"}]],Ft=wt`
  /* Happy Hues #5: mint surfaces, forest text and golden primary actions.
     Derived tones provide readable interaction and danger states. */
  :host {
    --ain-scheme: light;
    --ain-surface: #f2f7f5;
    --ain-surface-muted: #e2ece7;
    --ain-field: #fffffe;
    --ain-text: #00473e;
    --ain-brand-mark: #00473e;
    --ain-brand-container: #00473e;
    --ain-on-brand: #faae2b;
    --ain-muted: #475d5b;
    --ain-border: #b9cdc4;
    --ain-field-border: #77948a;
    --ain-hover: #e1ece6;
    --ain-selected: #fff0d2;
    --ain-accent: #faae2b;
    --ain-accent-hover: #eaa022;
    --ain-on-accent: #00473e;
    --ain-focus: #00665a;
    --ain-message: #00665a;
    --ain-quote: #fae4eb;
    --ain-quote-border: #ac5474;
    --ain-idle: #77948a;
    --ain-success: #00665a;
    --ain-error: #b8352c;
    --ain-error-surface: #fbe3de;
    --ain-shadow: #00332c26;
    --ain-tooltip: #00332c;
    --ain-on-tooltip: #fffffe;
    --ain-guide: #6b8279;
    --ain-guide-focus: #00665a;
    --ain-guide-fill: #00665a14;
    --ain-handle: #fffffe;
    --ain-crop-edge: #fffffe;
    --ain-crop-shade: #00332c52;
  }
  /* Happy Hues #10: teal surfaces, mint secondary text and warm gold. */
  :host([data-theme='dark']) {
    --ain-scheme: dark;
    --ain-surface: #004643;
    --ain-surface-muted: #001e1d;
    --ain-field: #003532;
    --ain-text: #fffffe;
    --ain-brand-mark: #f9bc60;
    --ain-brand-container: #e2ece7;
    --ain-on-brand: #00473e;
    --ain-muted: #abd1c6;
    --ain-border: #376f68;
    --ain-field-border: #7baba0;
    --ain-hover: #0f5650;
    --ain-selected: #34594b;
    --ain-accent: #f9bc60;
    --ain-accent-hover: #ffcd83;
    --ain-on-accent: #001e1d;
    --ain-focus: #f9bc60;
    --ain-message: #abd1c6;
    --ain-quote: #0f3433;
    --ain-quote-border: #abd1c6;
    --ain-idle: #7baba0;
    --ain-success: #abd1c6;
    --ain-error: #ffa19a;
    --ain-error-surface: #593d3d;
    --ain-shadow: #001e1d80;
    --ain-tooltip: #e8e4e6;
    --ain-on-tooltip: #001e1d;
    --ain-guide: #abd1c6;
    --ain-guide-focus: #f9bc60;
    --ain-guide-fill: #f9bc601c;
    --ain-handle: #001e1d;
    --ain-crop-edge: #fffffe;
    --ain-crop-shade: #001e1d8c;
  }
`,zh={styleMargin:"Margin",styleSpacingMode:"Spacing controls",styleSpacingAxes:"Horizontal / vertical",styleSpacingSides:"Individual sides",styleSpacingHorizontal:"Horizontal",styleSpacingVertical:"Vertical",styleSpacingToggleHint:"Click again to link all sides",styleSelectedTargets:(e)=>`Selected ${e} elements`,styleGlobalPreview:"Preview all changes",styleGlobalHint:(e)=>`Preview all page changes (${e} style suggestions)`,styleGlobalOff:"Global preview is off. Enable it in the toolbar to preview this selection.",styleAllTargets:(e)=>`All ${e} elements`,stylePreviewMany:(e)=>`Preview these ${e} elements`,styleMixed:"Mixed",styleShared:(e)=>`Shared by ${e} annotations`,styleCloseEditor:"Close editor",styleSyncUnsupported:"Update and restart the MCP service to sync style suggestions. Local feedback is retained.",styleFeedbackTab:"Feedback",styleStylesTab:"Styles",stylePreview:"Preview changes",styleResetAll:"Restore all",styleReset:"Restore original value",styleTarget:"Editing target",styleLinked:"Linked sides",styleSeparate:"Separate sides",stylePadding:"Padding",styleNumericHint:"Focus and scroll, or use ↑ ↓ to adjust. Shift: 10× step.",styleInvalid:"Enter a valid literal CSS value.",styleMissing:"The target is unavailable. Suggestions are kept.",styleChanged:"The page styles changed. Preview is paused to preserve the current page.",styleForcePreview:"Preview over current styles",styleRestoreOnSave:"Style suggestions are shared by target. Saving keeps the page preview.",styleOnlyFeedback:"Apply the attached style suggestions.",styleTargetLimit:"An annotation can include at most 20 targets. Restore changes on a target before selecting another.",styleMovePanel:"Move panel",styleSize:"Size and spacing",styleText:"Typography",styleAppearance:"Appearance",styleLayout:"Layout",styleBefore:(e)=>`Original: ${e}`,styleCount:(e)=>`${e} style suggestions`,styleProperty:Vn(["Width","Height","Top padding","Right padding","Bottom padding","Left padding","Top margin","Right margin","Bottom margin","Left margin","Font size","Line height","Font weight","Text alignment","Text color","Background color","Border width","Border color","Border radius","Opacity","Display","Gap","Direction","Cross-axis alignment","Main-axis alignment"])},Ph={styleMargin:"外边距",styleSpacingMode:"间距编辑模式",styleSpacingAxes:"水平 / 垂直联动",styleSpacingSides:"四边独立",styleSpacingHorizontal:"水平（左 / 右）",styleSpacingVertical:"垂直（上 / 下）",styleSpacingToggleHint:"再次点击返回四边联动",styleSelectedTargets:(e)=>`当前选中的 ${e} 个元素`,styleGlobalPreview:"预览全部修改",styleGlobalHint:(e)=>`预览当前页面的全部修改（${e} 项样式建议）`,styleGlobalOff:"全局预览已关闭，请在工具栏开启后预览当前选择。",styleAllTargets:(e)=>`全部 ${e} 个元素`,stylePreviewMany:(e)=>`预览这 ${e} 个元素`,styleMixed:"混合值",styleShared:(e)=>`由 ${e} 条标注共享`,styleCloseEditor:"关闭编辑器",styleSyncUnsupported:"请更新并重启 MCP 服务以同步样式建议。本地反馈已保留。",styleFeedbackTab:"反馈",styleStylesTab:"样式",stylePreview:"预览修改",styleResetAll:"全部恢复",styleReset:"恢复原值",styleTarget:"当前编辑目标",styleLinked:"四边联动",styleSeparate:"独立编辑",stylePadding:"内边距",styleNumericHint:"聚焦后用滚轮或 ↑ ↓ 微调，Shift 为 10 倍步长。",styleInvalid:"请输入有效的 CSS 属性值。",styleMissing:"目标当前不可用，样式建议已保留。",styleChanged:"页面样式已变化，预览已暂停，以保留页面当前状态。",styleForcePreview:"在当前样式上预览",styleRestoreOnSave:"样式建议按目标共享，保存后保留页面预览。",styleOnlyFeedback:"请应用附带的样式建议。",styleTargetLimit:"一条标注最多包含 20 个目标。请先恢复某个目标的修改，再选择其他目标。",styleMovePanel:"移动面板",styleSize:"尺寸与间距",styleText:"排版",styleAppearance:"外观",styleLayout:"布局",styleBefore:(e)=>`原值：${e}`,styleCount:(e)=>`${e} 项样式建议`,styleProperty:Vn(["宽度","高度","上内边距","右内边距","下内边距","左内边距","上外边距","右外边距","下外边距","左外边距","字号","行高","字重","文字对齐","文字颜色","背景颜色","边框宽度","边框颜色","圆角","透明度","显示方式","间隙","方向","交叉轴对齐","主轴对齐"])},Ah={styleMargin:"外距",styleSpacingMode:"間距編輯模式",styleSpacingAxes:"水平 / 垂直連動",styleSpacingSides:"四邊獨立",styleSpacingHorizontal:"水平（左 / 右）",styleSpacingVertical:"垂直（上 / 下）",styleSpacingToggleHint:"再次點擊返回四邊連動",styleSelectedTargets:(e)=>`目前選取的 ${e} 個元素`,styleGlobalPreview:"預覽全部修改",styleGlobalHint:(e)=>`預覽目前頁面的全部修改（${e} 項樣式建議）`,styleGlobalOff:"全域預覽已關閉，請在工具列開啟後預覽目前選取範圍。",styleAllTargets:(e)=>`全部 ${e} 個元素`,stylePreviewMany:(e)=>`預覽這 ${e} 個元素`,styleMixed:"混合值",styleShared:(e)=>`由 ${e} 則標註共用`,styleCloseEditor:"關閉編輯器",styleSyncUnsupported:"請更新並重新啟動 MCP 服務以同步樣式建議。本機回饋已保留。",styleFeedbackTab:"回饋",styleStylesTab:"樣式",stylePreview:"預覽修改",styleResetAll:"全部還原",styleReset:"還原原值",styleTarget:"目前編輯目標",styleLinked:"四邊連動",styleSeparate:"獨立編輯",stylePadding:"內距",styleNumericHint:"聚焦後用滾輪或 ↑ ↓ 微調，Shift 為 10 倍步長。",styleInvalid:"請輸入有效的 CSS 屬性值。",styleMissing:"目標目前無法使用，樣式建議已保留。",styleChanged:"頁面樣式已變更，預覽已暫停，以保留頁面目前狀態。",styleForcePreview:"在目前樣式上預覽",styleRestoreOnSave:"樣式建議按目標共用，儲存後保留頁面預覽。",styleOnlyFeedback:"請套用附帶的樣式建議。",styleTargetLimit:"一則標註最多包含 20 個目標。請先還原某個目標的修改，再選擇其他目標。",styleMovePanel:"移動面板",styleSize:"尺寸與間距",styleText:"排版",styleAppearance:"外觀",styleLayout:"佈局",styleBefore:(e)=>`原值：${e}`,styleCount:(e)=>`${e} 項樣式建議`,styleProperty:Vn(["寬度","高度","上內距","右內距","下內距","左內距","上外距","右外距","下外距","左外距","字級","行高","字重","文字對齊","文字顏色","背景顏色","邊框寬度","邊框顏色","圓角","透明度","顯示方式","間距","方向","交叉軸對齊","主軸對齊"])},Eh={styleMargin:"外側の余白",styleSpacingMode:"余白の編集方法",styleSpacingAxes:"水平 / 垂直",styleSpacingSides:"四辺を個別に編集",styleSpacingHorizontal:"水平（左右）",styleSpacingVertical:"垂直（上下）",styleSpacingToggleHint:"再クリックで四辺を連動",styleSelectedTargets:(e)=>`選択した ${e} 個の要素`,styleGlobalPreview:"すべての変更をプレビュー",styleGlobalHint:(e)=>`ページ全体の変更をプレビュー（${e} 件）`,styleGlobalOff:"全体のプレビューはオフです。ツールバーでオンにしてください。",styleAllTargets:(e)=>`${e} 個すべての要素`,stylePreviewMany:(e)=>`この ${e} 個の要素をプレビュー`,styleMixed:"混在",styleShared:(e)=>`${e} 件の注釈で共有`,styleCloseEditor:"エディターを閉じる",styleSyncUnsupported:"スタイル提案を同期するには MCP サービスを更新して再起動してください。ローカルのフィードバックは保持されています。",styleFeedbackTab:"フィードバック",styleStylesTab:"スタイル",stylePreview:"変更をプレビュー",styleResetAll:"すべて戻す",styleReset:"元の値に戻す",styleTarget:"編集中の対象",styleLinked:"四辺を連動",styleSeparate:"個別に編集",stylePadding:"内側の余白",styleNumericHint:"フォーカス中にホイールまたは ↑ ↓ で調整。Shift で10倍。",styleInvalid:"有効な CSS の値を入力してください。",styleMissing:"対象を利用できません。スタイルの提案は保持されています。",styleChanged:"ページのスタイルが変わったため、プレビューを一時停止しました。",styleForcePreview:"現在のスタイルに重ねてプレビュー",styleRestoreOnSave:"スタイル提案は対象ごとに共有され、保存後もプレビューを保持します。",styleOnlyFeedback:"添付のスタイル提案を適用してください。",styleTargetLimit:"1件の注釈には最大20個の対象を含められます。変更を戻してから別の対象を選択してください。",styleMovePanel:"パネルを移動",styleSize:"サイズと余白",styleText:"文字",styleAppearance:"外観",styleLayout:"レイアウト",styleBefore:(e)=>`元の値：${e}`,styleCount:(e)=>`${e} 件のスタイル提案`,styleProperty:Vn(["幅","高さ","上の内余白","右の内余白","下の内余白","左の内余白","上の外余白","右の外余白","下の外余白","左の外余白","文字サイズ","行高","文字の太さ","文字揃え","文字色","背景色","枠線の幅","枠線の色","角丸","不透明度","表示","間隔","方向","交差軸の配置","主軸の配置"])},Th={styleMargin:"바깥 여백",styleSpacingMode:"여백 편집 모드",styleSpacingAxes:"가로 / 세로",styleSpacingSides:"네 방향 개별 편집",styleSpacingHorizontal:"가로 (좌 / 우)",styleSpacingVertical:"세로 (위 / 아래)",styleSpacingToggleHint:"다시 클릭하면 네 방향 연동",styleSelectedTargets:(e)=>`선택한 요소 ${e}개`,styleGlobalPreview:"모든 변경 미리보기",styleGlobalHint:(e)=>`페이지 전체 변경 미리보기 (스타일 제안 ${e}개)`,styleGlobalOff:"전체 미리보기가 꺼져 있습니다. 도구 모음에서 켜 주세요.",styleAllTargets:(e)=>`전체 요소 ${e}개`,stylePreviewMany:(e)=>`이 요소 ${e}개 미리보기`,styleMixed:"혼합 값",styleShared:(e)=>`주석 ${e}개에서 공유`,styleCloseEditor:"편집기 닫기",styleSyncUnsupported:"스타일 제안을 동기화하려면 MCP 서비스를 업데이트하고 다시 시작하세요. 로컬 피드백은 유지됩니다.",styleFeedbackTab:"피드백",styleStylesTab:"스타일",stylePreview:"변경 미리보기",styleResetAll:"모두 복원",styleReset:"원래 값 복원",styleTarget:"편집 대상",styleLinked:"네 방향 연동",styleSeparate:"개별 편집",stylePadding:"안쪽 여백",styleNumericHint:"포커스 후 휠 또는 ↑ ↓로 조정합니다. Shift는 10배 단위입니다.",styleInvalid:"유효한 CSS 값을 입력하세요.",styleMissing:"대상을 사용할 수 없습니다. 스타일 제안은 유지됩니다.",styleChanged:"페이지 스타일이 변경되어 현재 상태를 보존하도록 미리보기를 중지했습니다.",styleForcePreview:"현재 스타일 위에 미리보기",styleRestoreOnSave:"스타일 제안은 대상별로 공유되며 저장 후에도 미리보기를 유지합니다.",styleOnlyFeedback:"첨부된 스타일 제안을 적용해 주세요.",styleTargetLimit:"주석 하나에 최대 20개의 대상을 포함할 수 있습니다. 변경을 복원한 후 다른 대상을 선택하세요.",styleMovePanel:"패널 이동",styleSize:"크기와 여백",styleText:"글꼴",styleAppearance:"모양",styleLayout:"레이아웃",styleBefore:(e)=>`원래 값: ${e}`,styleCount:(e)=>`스타일 제안 ${e}개`,styleProperty:Vn(["너비","높이","위 안쪽 여백","오른쪽 안쪽 여백","아래 안쪽 여백","왼쪽 안쪽 여백","위 바깥 여백","오른쪽 바깥 여백","아래 바깥 여백","왼쪽 바깥 여백","글자 크기","줄 높이","글자 굵기","텍스트 정렬","글자색","배경색","테두리 너비","테두리 색","모서리 반경","불투명도","표시","간격","방향","교차축 정렬","주축 정렬"])},Oh={variantsTitle:"UI Variants",variantsMinimize:"Minimize panel",variantsExpand:"Expand panel",variantsSyncPending:"Changes are saved locally and will sync when the MCP connection is ready.",variantsHint:"Ask your agent for design candidates. Compare here, then tell the agent when to continue.",variantsNeedsConnection:"Connect a compatible MCP service to start an exploration.",variantsUnsupported:"The connected service does not support UI Variants. Local changes are retained. Update and restart the MCP service and development server, then retry.",variantsBusy:"Another annotation is exploring designs on this page.",variantsInvalidTargets:"Select visible, distinct target roots without parent/child overlap.",variantsRequested:"Waiting for your agent to generate candidates",variantsWaiting:"Waiting for the host to register all targets",variantsSwitching:"Switching all targets…",variantsPrevious:"Previous design",variantsNext:"Next design",variantsPage:"Design",variantsAccepted:"Selection recorded. Ask your agent to apply it.",variantsCancelled:"Cancellation recorded. Ask your agent to restore and clean up.",variantsDeleted:"Annotation deleted. Ask your agent to restore Original and remove generated variants and temporary integration. Cleanup is still pending.",variantsCompleted:"Agent reported completion",variantsCleanup:"Temporary integration is still present. Ask your agent to finish cleanup.",variantsBindingError:"The host could not render every target. Ask your agent to check the integration.",variantsOriginal:"Original",variantsAccept:"I want this",variantsRegenerate:"Regenerate",variantsCancel:"Cancel",variantsCancelTitle:"End this exploration?",variantsCancelDescription:"The preview will return to Original. Ask your agent to restore the source and remove the temporary integration afterwards.",variantsKeepComparing:"Keep comparing",variantsFeedback:"Feedback for the next step (optional)",variantsDecisionSaved:"Decision saved. Tell your agent when you are ready to continue.",variantsConflict:"This exploration changed elsewhere. Review its latest state before continuing.",variantsStylesPaused:"Style overrides on these targets are paused during exploration. Your edits are retained.",variantsOpen:"View annotation",variantsEmptyComment:"Explore alternative designs for these elements.",variantsGeneration:"Round"},Mh={variantsMinimize:"最小化面板",variantsExpand:"展开面板",variantsSyncPending:"修改已保存在本地，MCP 连接就绪后会同步。",variantsTitle:"UI Variants",variantsHint:"让 Agent 生成候选设计，在这里比较，再由你决定何时让 Agent 继续。",variantsNeedsConnection:"连接支持 UI Variants 的 MCP 服务后即可开始探索。",variantsUnsupported:"已连接的服务不支持 UI Variants。本地修改已保留，请更新并重启 MCP 服务和开发服务器后重试。",variantsBusy:"当前页面已有另一个标注正在探索设计。",variantsInvalidTargets:"请选择可见、独立且没有父子重叠的目标根元素。",variantsRequested:"等待 Agent 生成候选方案",variantsWaiting:"等待宿主登记全部目标",variantsSwitching:"正在切换全部目标…",variantsPrevious:"上一个方案",variantsNext:"下一个方案",variantsPage:"方案",variantsAccepted:"已记录选择，等待你告知 Agent 应用方案。",variantsCancelled:"已记录取消，等待你告知 Agent 恢复并清理。",variantsDeleted:"标注已删除，源码清理尚未完成。请告知 Agent 恢复原版，并移除生成的候选方案和临时接入。",variantsCompleted:"Agent 已报告完成",variantsCleanup:"临时接入仍然存在，请让 Agent 完成清理。",variantsBindingError:"宿主未能渲染全部目标，请让 Agent 检查接入。",variantsOriginal:"原版",variantsAccept:"就要这个",variantsRegenerate:"重新生成",variantsCancel:"结束探索",variantsCancelTitle:"结束这次探索？",variantsCancelDescription:"预览将恢复为原版。之后请告知 Agent 恢复源码并清理临时接入。",variantsKeepComparing:"继续比较",variantsFeedback:"下一步反馈（可选）",variantsDecisionSaved:"决定已保存。准备好后，请告知 Agent 继续。",variantsConflict:"探索已在其他会话中变更，请检查最新状态后继续。",variantsStylesPaused:"探索期间暂停这些目标的样式覆盖，已有修改仍然保留。",variantsOpen:"查看标注",variantsEmptyComment:"为这些元素探索更多设计方案。",variantsGeneration:"轮次"},Nh={variantsMinimize:"最小化面板",variantsExpand:"展開面板",variantsSyncPending:"修改已儲存在本機，MCP 連線就緒後會同步。",variantsTitle:"UI Variants",variantsHint:"讓 Agent 產生候選設計，在這裡比較，再由你決定何時讓 Agent 繼續。",variantsNeedsConnection:"連線至支援 UI Variants 的 MCP 服務後即可開始探索。",variantsUnsupported:"已連線的服務不支援 UI Variants。本機修改已保留，請更新並重新啟動 MCP 服務與開發伺服器後重試。",variantsBusy:"目前頁面已有另一個標註正在探索設計。",variantsInvalidTargets:"請選擇可見、獨立且沒有父子重疊的目標根元素。",variantsRequested:"等待 Agent 產生候選方案",variantsWaiting:"等待宿主登記全部目標",variantsSwitching:"正在切換全部目標…",variantsPrevious:"上一個方案",variantsNext:"下一個方案",variantsPage:"方案",variantsAccepted:"已記錄選擇，等待你告知 Agent 套用方案。",variantsCancelled:"已記錄取消，等待你告知 Agent 還原並清理。",variantsDeleted:"標註已刪除，原始碼尚未清理。請告知 Agent 還原原版，並移除產生的候選方案和臨時接入。",variantsCompleted:"Agent 已回報完成",variantsCleanup:"臨時接入仍然存在，請讓 Agent 完成清理。",variantsBindingError:"宿主未能呈現全部目標，請讓 Agent 檢查接入。",variantsOriginal:"原版",variantsAccept:"就要這個",variantsRegenerate:"重新產生",variantsCancel:"結束探索",variantsCancelTitle:"結束這次探索？",variantsCancelDescription:"預覽將還原為原版。之後請告知 Agent 還原原始碼並清理臨時接入。",variantsKeepComparing:"繼續比較",variantsFeedback:"下一步回饋（選填）",variantsDecisionSaved:"決定已儲存。準備好後，請告知 Agent 繼續。",variantsConflict:"探索已在其他工作階段中變更，請檢查最新狀態後繼續。",variantsStylesPaused:"探索期間暫停這些目標的樣式覆寫，現有修改仍會保留。",variantsOpen:"查看標註",variantsEmptyComment:"為這些元素探索更多設計方案。",variantsGeneration:"輪次"},Dh={variantsMinimize:"パネルを最小化",variantsExpand:"パネルを展開",variantsSyncPending:"変更はローカルに保存されています。MCP の接続後に同期されます。",variantsTitle:"UI Variants",variantsHint:"Agent に候補を生成してもらい、ここで比較した後、続行するタイミングを伝えてください。",variantsNeedsConnection:"対応する MCP サービスに接続して探索を開始してください。",variantsUnsupported:"接続中のサービスは UI Variants に対応していません。変更は保持されています。MCP サービスと開発サーバーを更新・再起動して再試行してください。",variantsBusy:"このページでは別の注釈がデザインを探索しています。",variantsInvalidTargets:"表示されている独立した要素を選択してください。親子の重複は利用できません。",variantsRequested:"Agent による候補生成を待っています",variantsWaiting:"ホストによる全対象の登録を待っています",variantsSwitching:"すべての対象を切り替え中…",variantsPrevious:"前のデザイン",variantsNext:"次のデザイン",variantsPage:"デザイン",variantsAccepted:"選択を記録しました。Agent に適用を依頼してください。",variantsCancelled:"キャンセルを記録しました。Agent に復元と後片付けを依頼してください。",variantsDeleted:"注釈を削除しました。ソースの後片付けは未完了です。Agent に元のデザインへの復元と候補・一時的な連携の削除を依頼してください。",variantsCompleted:"Agent が完了を報告しました",variantsCleanup:"一時的な連携が残っています。Agent に削除を依頼してください。",variantsBindingError:"一部の対象を描画できません。Agent に連携の確認を依頼してください。",variantsOriginal:"元のデザイン",variantsAccept:"このデザインを採用",variantsRegenerate:"再生成",variantsCancel:"探索をキャンセル",variantsCancelTitle:"この探索を終了しますか？",variantsCancelDescription:"プレビューは元のデザインに戻ります。その後、Agent にソースの復元と一時的な連携の削除を依頼してください。",variantsKeepComparing:"比較を続ける",variantsFeedback:"次のステップへのフィードバック（任意）",variantsDecisionSaved:"選択を保存しました。準備ができたら Agent に続行を伝えてください。",variantsConflict:"別のセッションで探索が変更されました。最新の状態を確認してください。",variantsStylesPaused:"探索中は対象のスタイル上書きを一時停止します。編集内容は保持されます。",variantsOpen:"注釈を見る",variantsEmptyComment:"これらの要素の別のデザインを探索してください。",variantsGeneration:"ラウンド"},Lh={variantsMinimize:"패널 최소화",variantsExpand:"패널 펼치기",variantsSyncPending:"변경은 로컬에 저장되며 MCP 연결이 준비되면 동기화됩니다.",variantsTitle:"UI Variants",variantsHint:"Agent가 만든 디자인을 여기서 비교한 뒤, 계속할 시점을 직접 알려주세요.",variantsNeedsConnection:"지원되는 MCP 서비스에 연결해 탐색을 시작하세요.",variantsUnsupported:"연결된 서비스는 UI Variants를 지원하지 않습니다. 로컬 변경은 유지됩니다. MCP 서비스와 개발 서버를 업데이트하고 재시작한 뒤 다시 시도하세요.",variantsBusy:"이 페이지의 다른 주석에서 디자인을 탐색하고 있습니다.",variantsInvalidTargets:"표시된 독립 요소를 선택하세요. 부모와 자식이 겹치면 안 됩니다.",variantsRequested:"Agent의 후보 생성을 기다리는 중",variantsWaiting:"호스트가 모든 대상을 등록하기를 기다리는 중",variantsSwitching:"모든 대상 전환 중…",variantsPrevious:"이전 디자인",variantsNext:"다음 디자인",variantsPage:"디자인",variantsAccepted:"선택이 기록되었습니다. Agent에게 적용을 요청하세요.",variantsCancelled:"취소가 기록되었습니다. Agent에게 복원과 정리를 요청하세요.",variantsDeleted:"주석이 삭제되었지만 소스 정리가 남아 있습니다. Agent에게 원본 복원과 생성된 후보 및 임시 연동 제거를 요청하세요.",variantsCompleted:"Agent가 완료를 보고했습니다",variantsCleanup:"임시 연동이 남아 있습니다. Agent에게 정리를 요청하세요.",variantsBindingError:"호스트가 모든 대상을 표시하지 못했습니다. Agent에게 연동 확인을 요청하세요.",variantsOriginal:"원본",variantsAccept:"이 디자인 사용",variantsRegenerate:"다시 생성",variantsCancel:"탐색 취소",variantsCancelTitle:"이 탐색을 종료할까요?",variantsCancelDescription:"미리보기가 원본으로 돌아갑니다. 이후 Agent에게 소스 복원과 임시 연동 정리를 요청하세요.",variantsKeepComparing:"계속 비교",variantsFeedback:"다음 단계 피드백 (선택)",variantsDecisionSaved:"결정이 저장되었습니다. 준비되면 Agent에게 계속하라고 알려주세요.",variantsConflict:"다른 세션에서 탐색이 변경되었습니다. 최신 상태를 확인하세요.",variantsStylesPaused:"탐색 중 대상의 스타일 덮어쓰기가 일시 중지됩니다. 기존 수정은 유지됩니다.",variantsOpen:"주석 보기",variantsEmptyComment:"이 요소들의 다른 디자인을 탐색하세요.",variantsGeneration:"라운드"},ht={...Oh,...zh,settings:"Settings",language:"Language",theme:"Theme",light:"Light",dark:"Dark",save:"Save",add:"Add",cancel:"Cancel",delete:"Delete",openInspector:"Open inspector",closeInspector:"Close inspector",inspector:"Ainotation inspector",moveInspector:"Move inspector",inspectorSettings:"Inspector settings",copy:"Copy feedback",copyHint:"Copy feedback from all pages in this project",exportJson:"Export JSON",exportImages:"Export feedback and images",exportJsonHint:"Export JSON for this page",exportImagesHint:"Export this page with image attachments",clear:"Clear all annotations",clearHint:"Clear all annotations on this page",offline:"Offline",connecting:"Connecting",connected:"Connected",error:"Error",loadingLocal:"Loading local feedback...",syncing:"Syncing feedback...",storageNotice:"Local storage unavailable. Changes are not saved locally.",detail:"Output Detail",compact:"Compact",standard:"Standard",detailed:"Detailed",forensic:"Everything",compactDescription:"Short notes with selectors and brief text quotes.",standardDescription:"Element locations, viewport and selected text.",detailedDescription:"Adds classes, bounds, nearby text and captured states.",forensicDescription:"Adds DOM ancestry, styles, accessibility and environment.",copiedMarkdown:"Applies to copied Markdown.",switchTo:(e)=>`Switch to ${e}`,labeled:(e,t)=>`${e}: ${t}`,shortcut:(e,t)=>`${e} (${t})`,switchTheme:(e)=>`Switch to ${e.toLowerCase()} mode`,connection:"MCP connection",localMode:"Local only",syncStorageFailed:"Saved locally. Service storage needs repair; run ainotation-mcp doctor, then retry.",syncRecoveryNeeded:"Saved locally. The service recovered a different version; synchronization is paused.",syncImagesMissing:"Feedback is saved, but some image files are missing. Open the browser that holds those images to upload them again.",retrySync:"Retry connection",recoverProject:"Restore project from this browser",exportRecovery:"Export previous local version",recoveryLocalPreview:(e)=>`Browser version (${e} annotations)`,recoveryServerPreview:(e)=>`Recovered server version (${e} annotations)`,syncProjectProgress:(e,t)=>`Checked ${e} of ${t} saved pages.`,syncProjectReview:"Some pages need attention. Open them to review versions or missing images.",recoveryHelp:"Export a copy first. Choose which version to keep for this page; the service preserves both feedback snapshots.",recoveryBrowser:"Use browser version",recoveryServer:"Use recovered server version",localModeDescription:"Feedback stays in this browser. Copy or export it to share.",automaticConnected:"Connected through the development server.",automaticConnection:"Automatic connection through the development server.",manualConnected:"Feedback sync is connected to the local MCP server.",manualConnection:"Local feedback is available. Connect a local MCP server to share it with your agent.",endpoint:"Endpoint",token:"Token",connect:"Connect",disconnect:"Disconnect",newAnnotation:"New annotation",newFeedback:"New feedback",editFeedback:"Edit feedback",targetUnavailable:"Target unavailable",selectedElements:"Selected elements",copySelector:"Copy selector",parentTarget:"Select parent element",previousTarget:"Return to previous element",selectorCopied:"Selector copied",locatorDetails:"Locator details",selectedText:"Selected text",feedbackContent:"Feedback content",attachedImages:"Attached images",editAnnotation:(e)=>`Edit annotation ${e}`,editImage:(e)=>`Edit image ${e}`,attachedImage:(e)=>`Attached image ${e}`,image:(e)=>`Image ${e}`,downloadImage:"Download image",removeImage:"Remove image",downloadImageNumber:(e)=>`Download image ${e}`,removeImageNumber:(e)=>`Remove image ${e}`,screenshot:"Screenshot",screenshotHint:"Draw on this page and capture a screenshot",chooseImage:"Choose image",pasteImage:"Paste or drop an image here",imageEditor:"Image annotation editor",drawingSurface:"Drawing surface",drawingTools:"Drawing tools",imageToAnnotate:"Image to annotate",select:"Select",selectMove:"Select / move",arrow:"Arrow",rectangle:"Rectangle",ellipse:"Ellipse",pen:"Pen",freeDraw:"Free draw",crop:"Crop",color:"Color",colorValue:(e)=>`Color ${e}`,colorCycle:(e)=>`Color: ${e} (C to cycle)`,red:"Red",amber:"Amber",green:"Green",blue:"Blue",white:"White",black:"Black",colorPalette:"Color palette",lineWidth:"Line width",lineWidths:"Line widths",widthCycle:"Line width (S to cycle)",undo:"Undo",redo:"Redo",deleteShape:"Delete shape",deleteShapes:(e)=>`Delete ${e} ${e===1?"shape":"shapes"}`,clearCrop:"Clear crop",cancelDrawing:"Cancel drawing",captureAttach:"Capture and attach",attachImage:"Attach image",captureHint:"Capture and attach (Command / Ctrl + Enter; hold Option / Alt to interact with the page)",rotateHint:"Drag outside the square to rotate",arrowTailHint:"Drag the tail to change length and direction",resizeHint:"Drag to resize",operationFailed:"The operation failed. Feedback was not cleared.",storageUnavailable:"Local storage unavailable. Export feedback before reloading.",feedbackLoading:"Feedback is still loading.",pageLoading:"Loading feedback for this page",localOnly:"Local only",copied:"Feedback copied",exported:"Feedback exported",saved:"Feedback saved",cleared:"All annotations on this page cleared",remoteDeleted:"An edited annotation was deleted remotely. Its text is kept for a new selection.",annotationDeleted:"The annotation was deleted. Unsaved text is kept for a new selection.",originalDeleted:"The original feedback was deleted. Your draft is kept as a new annotation draft.",missingAnnotation:"This feedback no longer exists.",changedWhileDrawing:"The annotation changed while drawing.",imageLimit:"Up to 8 images can be attached to one annotation.",imagePending:"Image is not available locally yet. Wait for synchronization.",imageAttached:"Image attached. Save the feedback to share it.",imageAttachFailed:"Could not attach image.",shapesLimit:"Up to 200 shapes per image.",captureUnavailable:"Screen capture is unavailable. Paste, drop or choose an image instead.",captureStopped:"Screen sharing has ended.",captureEnded:"Screen sharing ended. Cancel and start a new screenshot, or import an image.",captureTimeout:"Screen capture stopped or timed out.",frameWaiting:"Capture stopped or no new frame arrived.",frameFailed:"Could not read the capture frame. Try capturing again.",cropNeedsTab:"Choose the current browser tab for live cropping. For a window or screen, capture the full image and crop its attachment.",invalidDimensions:(e,t)=>`Image frame has invalid dimensions (${e} × ${t}). Try capturing again.`,imageTooLarge:(e,t)=>`Image is too large (${e} × ${t}). Use an image up to 16 megapixels.`,encodeFailed:(e,t)=>`Could not encode the image (${e} × ${t}). Try capturing again.`,imageBytesLimit:"Image exceeds 8 MB. Use a smaller image.",imageFitFailed:"Could not fit the screenshot within the 8 MiB attachment limit.",drawingUnavailable:"Image drawing is unavailable.",imageType:"Choose a PNG, JPEG or WebP image.",imageValidation:"Image attachment failed validation.",composeTimeout:"Image composition stopped or timed out.",zipLimit:"This export exceeds the ZIP size limit. Download images separately.",cropOutside:"The crop is outside the visible area. Move it back into view or clear it.",cropInvalid:"Invalid crop dimensions.",cropOutsideImage:"The crop is outside the image.",invalidEndpoint:"MCP endpoint must be a loopback HTTP(S) origin.",invalidToken:"Enter a valid pairing token.",bridgeUnavailable:"Development bridge is unavailable.",sessionMismatch:"MCP returned another session.",clipboardFailed:"Clipboard access failed. Use Export JSON; feedback is still saved.",exportImagesPending:"Some images are not available locally yet. Wait for synchronization before exporting.",preferencesUnsaved:"The setting changed for this session, but could not be saved.",connectionRestoreFailed:"MCP settings could not be restored. Connect manually.",syncConnecting:"Connecting to MCP server",syncConnected:"MCP server connected",syncFailed:"MCP sync failed; local changes are retained for retry.",syncUnavailable:"MCP unavailable. Local feedback is retained; retrying."},Rh={...Mh,...Ph,settings:"设置",language:"语言",theme:"主题",light:"浅色",dark:"深色",save:"保存",add:"添加",cancel:"取消",delete:"删除",openInspector:"打开检查器",closeInspector:"关闭检查器",inspector:"Ainotation 检查器",moveInspector:"移动检查器",inspectorSettings:"检查器设置",copy:"复制反馈",copyHint:"复制当前项目所有页面的反馈",exportJson:"导出 JSON",exportImages:"导出反馈和图片",exportJsonHint:"导出当前页面的 JSON",exportImagesHint:"导出当前页面及图片附件",clear:"清除全部标注",clearHint:"清除当前页面的全部标注",offline:"未连接",connecting:"连接中",connected:"已连接",error:"错误",loadingLocal:"正在加载本地反馈…",syncing:"正在同步反馈…",storageNotice:"本地存储不可用，修改不会保存到本地。",detail:"输出详情",compact:"精简",standard:"标准",detailed:"详细",forensic:"全部",compactDescription:"简短说明、选择器和文本引用。",standardDescription:"元素位置、视口和选中文本。",detailedDescription:"增加类名、边界、相邻文本和捕获状态。",forensicDescription:"增加 DOM 祖先、样式、无障碍和环境信息。",copiedMarkdown:"适用于复制的 Markdown。",switchTo:(e)=>`切换为${e}`,labeled:(e,t)=>`${e}：${t}`,shortcut:(e,t)=>`${e}（${t}）`,switchTheme:(e)=>`切换到${e}主题`,connection:"MCP 连接",localMode:"仅本地",syncStorageFailed:"已保存在本地。服务存储需要修复；请运行 ainotation-mcp doctor，然后重试。",syncRecoveryNeeded:"已保存在本地。服务恢复了不同版本，同步已暂停。",syncImagesMissing:"反馈已保存，但部分图片文件缺失。请打开仍保存这些图片的浏览器重新上传。",retrySync:"重试连接",recoverProject:"从当前浏览器恢复项目",exportRecovery:"导出之前的本地版本",recoveryLocalPreview:(e)=>`浏览器版本（${e} 条标注）`,recoveryServerPreview:(e)=>`服务恢复的版本（${e} 条标注）`,syncProjectProgress:(e,t)=>`已检查 ${e} / ${t} 个已保存页面。`,syncProjectReview:"部分页面需要处理。请打开这些页面检查版本或缺失图片。",recoveryHelp:"请先导出副本，再选择此页面要保留的版本；服务会保留双方的反馈快照。",recoveryBrowser:"使用浏览器版本",recoveryServer:"使用服务恢复的版本",localModeDescription:"反馈保存在当前浏览器中，可通过复制或导出分享。",automaticConnected:"已通过开发服务器连接。",automaticConnection:"通过开发服务器自动连接。",manualConnected:"反馈已连接到本地 MCP 服务器进行同步。",manualConnection:"本地反馈可用。连接本地 MCP 服务器即可与 Agent 共享。",endpoint:"服务地址",token:"配对令牌",connect:"连接",disconnect:"断开连接",newAnnotation:"新建标注",newFeedback:"新建反馈",editFeedback:"编辑反馈",targetUnavailable:"目标不可用",selectedElements:"选中的元素",copySelector:"复制选择符",parentTarget:"选择父元素",previousTarget:"返回刚才的下层元素",selectorCopied:"选择符已复制",locatorDetails:"定位详情",selectedText:"选中的文本",feedbackContent:"反馈内容",attachedImages:"图片附件",editAnnotation:(e)=>`编辑标注 ${e}`,editImage:(e)=>`编辑图片 ${e}`,attachedImage:(e)=>`图片附件 ${e}`,image:(e)=>`图片 ${e}`,downloadImage:"下载图片",removeImage:"移除图片",downloadImageNumber:(e)=>`下载图片 ${e}`,removeImageNumber:(e)=>`移除图片 ${e}`,screenshot:"截图",screenshotHint:"在页面上绘制并截图",chooseImage:"选择图片",pasteImage:"在此粘贴或拖入图片",imageEditor:"图片标注编辑器",drawingSurface:"绘图区域",drawingTools:"绘图工具",imageToAnnotate:"待标注图片",select:"选择",selectMove:"选择／移动",arrow:"箭头",rectangle:"矩形",ellipse:"椭圆",pen:"画笔",freeDraw:"自由绘制",crop:"裁剪",color:"颜色",colorValue:(e)=>`颜色 ${e}`,colorCycle:(e)=>`颜色：${e}（C 循环切换）`,red:"红色",amber:"琥珀色",green:"绿色",blue:"蓝色",white:"白色",black:"黑色",colorPalette:"颜色选择",lineWidth:"线条粗细",lineWidths:"线条粗细选项",widthCycle:"线条粗细（S 循环切换）",undo:"撤销",redo:"重做",deleteShape:"删除图形",deleteShapes:(e)=>`删除 ${e} 个图形`,clearCrop:"清除裁剪",cancelDrawing:"取消绘图",captureAttach:"截图并添加附件",attachImage:"添加图片附件",captureHint:"截图并添加附件（Command / Ctrl + Enter；按住 Option / Alt 与页面交互）",rotateHint:"在方块外侧拖动以旋转",arrowTailHint:"拖动尾部调整长度和方向",resizeHint:"拖动调整大小",operationFailed:"操作失败，反馈未被清除。",storageUnavailable:"本地存储不可用，请在刷新前导出反馈。",feedbackLoading:"反馈仍在加载中。",pageLoading:"正在加载当前页面的反馈",localOnly:"仅本地",copied:"反馈已复制",exported:"反馈已导出",saved:"反馈已保存",cleared:"已清除当前页面的全部标注",remoteDeleted:"正在编辑的标注已被远程删除，文本已保留，可重新选择目标。",annotationDeleted:"标注已删除，未保存的文本已保留，可重新选择目标。",originalDeleted:"原反馈已删除，当前内容已保留为新标注草稿。",missingAnnotation:"此反馈已不存在。",changedWhileDrawing:"绘图期间标注发生了变化。",imageLimit:"每条标注最多添加 8 张图片。",imagePending:"图片暂未保存到本地，请等待同步。",imageAttached:"图片已添加，保存反馈后即可共享。",imageAttachFailed:"无法添加图片。",shapesLimit:"每张图片最多绘制 200 个图形。",captureUnavailable:"无法使用屏幕捕获，请粘贴、拖入或选择图片。",captureStopped:"屏幕共享已结束。",captureEnded:"屏幕共享已结束，请取消后重新截图，或导入图片。",captureTimeout:"屏幕捕获已停止或超时。",frameWaiting:"捕获已停止或没有新的画面。",frameFailed:"无法读取捕获画面，请重试。",cropNeedsTab:"实时裁剪请选择当前浏览器标签页。窗口或整屏可先截图，再裁剪图片附件。",invalidDimensions:(e,t)=>`画面尺寸无效（${e} × ${t}），请重试截图。`,imageTooLarge:(e,t)=>`图片过大（${e} × ${t}），请使用不超过 1600 万像素的图片。`,encodeFailed:(e,t)=>`无法编码图片（${e} × ${t}），请重试截图。`,imageBytesLimit:"图片超过 8 MB，请使用更小的图片。",imageFitFailed:"无法将截图缩小到 8 MiB 的附件限制以内。",drawingUnavailable:"图片绘制功能不可用。",imageType:"请选择 PNG、JPEG 或 WebP 图片。",imageValidation:"图片附件校验失败。",composeTimeout:"图片合成已停止或超时。",zipLimit:"导出内容超过 ZIP 大小限制，请单独下载图片。",cropOutside:"裁剪区域位于可见范围外，请移回视口或清除裁剪。",cropInvalid:"裁剪尺寸无效。",cropOutsideImage:"裁剪区域位于图片之外。",invalidEndpoint:"MCP 地址必须是本机回环 HTTP(S) 地址。",invalidToken:"请输入有效的配对令牌。",bridgeUnavailable:"开发环境连接不可用。",sessionMismatch:"MCP 返回了其他会话。",clipboardFailed:"无法访问剪贴板，请使用导出；反馈仍已保存。",exportImagesPending:"部分图片尚未保存到本地，请同步完成后再导出。",preferencesUnsaved:"设置已在本次会话生效，但未能保存。",connectionRestoreFailed:"无法恢复 MCP 设置，请手动连接。",syncConnecting:"正在连接 MCP 服务器",syncConnected:"已连接 MCP 服务器",syncFailed:"MCP 同步失败，本地修改已保留，稍后重试。",syncUnavailable:"MCP 不可用，本地反馈已保留，正在重试。"},Zh={...Nh,...Ah,settings:"設定",language:"語言",theme:"主題",light:"淺色",dark:"深色",save:"儲存",add:"新增",cancel:"取消",delete:"刪除",openInspector:"開啟檢查器",closeInspector:"關閉檢查器",inspector:"Ainotation 檢查器",moveInspector:"移動檢查器",inspectorSettings:"檢查器設定",copy:"複製回饋",copyHint:"複製目前專案所有頁面的回饋",exportJson:"匯出 JSON",exportImages:"匯出回饋與圖片",exportJsonHint:"匯出目前頁面的 JSON",exportImagesHint:"匯出目前頁面與圖片附件",clear:"清除全部標註",clearHint:"清除目前頁面的全部標註",offline:"未連線",connecting:"連線中",connected:"已連線",error:"錯誤",loadingLocal:"正在載入本機回饋…",syncing:"正在同步回饋…",storageNotice:"本機儲存空間無法使用，變更不會儲存至本機。",detail:"輸出詳細程度",compact:"精簡",standard:"標準",detailed:"詳細",forensic:"全部",compactDescription:"簡短說明、選擇器與文字引用。",standardDescription:"元素位置、檢視區與選取文字。",detailedDescription:"加入類別名稱、邊界、鄰近文字與擷取狀態。",forensicDescription:"加入 DOM 祖先、樣式、無障礙與環境資訊。",copiedMarkdown:"套用至複製的 Markdown。",switchTo:(e)=>`切換為${e}`,labeled:(e,t)=>`${e}：${t}`,shortcut:(e,t)=>`${e}（${t}）`,switchTheme:(e)=>`切換至${e}主題`,connection:"MCP 連線",localMode:"僅本機",syncStorageFailed:"已儲存在本機。服務儲存需要修復；請執行 ainotation-mcp doctor，然後重試。",syncRecoveryNeeded:"已儲存在本機。服務還原了不同版本，同步已暫停。",syncImagesMissing:"回饋已儲存，但部分圖片檔案遺失。請開啟仍儲存這些圖片的瀏覽器重新上傳。",retrySync:"重試連線",recoverProject:"從目前瀏覽器還原專案",exportRecovery:"匯出先前的本機版本",recoveryLocalPreview:(e)=>`瀏覽器版本（${e} 則標註）`,recoveryServerPreview:(e)=>`服務還原的版本（${e} 則標註）`,syncProjectProgress:(e,t)=>`已檢查 ${e} / ${t} 個已儲存頁面。`,syncProjectReview:"部分頁面需要處理。請開啟這些頁面檢查版本或遺失的圖片。",recoveryHelp:"請先匯出副本，再選擇此頁面要保留的版本；服務會保留雙方的回饋快照。",recoveryBrowser:"使用瀏覽器版本",recoveryServer:"使用服務還原的版本",localModeDescription:"回饋儲存在目前的瀏覽器中，可透過複製或匯出分享。",automaticConnected:"已透過開發伺服器連線。",automaticConnection:"透過開發伺服器自動連線。",manualConnected:"回饋已連接至本機 MCP 伺服器進行同步。",manualConnection:"本機回饋可用。連接本機 MCP 伺服器即可與 Agent 共用。",endpoint:"服務位址",token:"配對權杖",connect:"連線",disconnect:"中斷連線",newAnnotation:"新增標註",newFeedback:"新增回饋",editFeedback:"編輯回饋",targetUnavailable:"目標無法使用",selectedElements:"選取的元素",copySelector:"複製選擇器",parentTarget:"選取父元素",previousTarget:"返回先前的下層元素",selectorCopied:"已複製選擇器",locatorDetails:"定位詳細資訊",selectedText:"選取的文字",feedbackContent:"回饋內容",attachedImages:"圖片附件",editAnnotation:(e)=>`編輯標註 ${e}`,editImage:(e)=>`編輯圖片 ${e}`,attachedImage:(e)=>`圖片附件 ${e}`,image:(e)=>`圖片 ${e}`,downloadImage:"下載圖片",removeImage:"移除圖片",downloadImageNumber:(e)=>`下載圖片 ${e}`,removeImageNumber:(e)=>`移除圖片 ${e}`,screenshot:"截圖",screenshotHint:"在頁面上繪圖並擷取畫面",chooseImage:"選擇圖片",pasteImage:"在此貼上或拖入圖片",imageEditor:"圖片標註編輯器",drawingSurface:"繪圖區域",drawingTools:"繪圖工具",imageToAnnotate:"待標註圖片",select:"選取",selectMove:"選取／移動",arrow:"箭頭",rectangle:"矩形",ellipse:"橢圓",pen:"畫筆",freeDraw:"自由繪圖",crop:"裁切",color:"顏色",colorValue:(e)=>`顏色 ${e}`,colorCycle:(e)=>`顏色：${e}（C 循環切換）`,red:"紅色",amber:"琥珀色",green:"綠色",blue:"藍色",white:"白色",black:"黑色",colorPalette:"顏色選擇",lineWidth:"線條粗細",lineWidths:"線條粗細選項",widthCycle:"線條粗細（S 循環切換）",undo:"復原",redo:"重做",deleteShape:"刪除圖形",deleteShapes:(e)=>`刪除 ${e} 個圖形`,clearCrop:"清除裁切",cancelDrawing:"取消繪圖",captureAttach:"截圖並加入附件",attachImage:"加入圖片附件",captureHint:"截圖並加入附件（Command / Ctrl + Enter；按住 Option / Alt 與頁面互動）",rotateHint:"在方塊外側拖曳以旋轉",arrowTailHint:"拖曳尾端調整長度與方向",resizeHint:"拖曳調整大小",operationFailed:"操作失敗，回饋並未清除。",storageUnavailable:"本機儲存空間無法使用，請在重新整理前匯出回饋。",feedbackLoading:"回饋仍在載入中。",pageLoading:"正在載入目前頁面的回饋",localOnly:"僅本機",copied:"回饋已複製",exported:"回饋已匯出",saved:"回饋已儲存",cleared:"已清除目前頁面的全部標註",remoteDeleted:"正在編輯的標註已被遠端刪除，文字已保留，可重新選取目標。",annotationDeleted:"標註已刪除，未儲存的文字已保留，可重新選取目標。",originalDeleted:"原回饋已刪除，目前內容已保留為新標註草稿。",missingAnnotation:"此回饋已不存在。",changedWhileDrawing:"繪圖期間標註發生了變更。",imageLimit:"每則標註最多加入 8 張圖片。",imagePending:"圖片尚未儲存至本機，請等待同步。",imageAttached:"圖片已加入，儲存回饋後即可共用。",imageAttachFailed:"無法加入圖片。",shapesLimit:"每張圖片最多繪製 200 個圖形。",captureUnavailable:"無法使用螢幕擷取，請貼上、拖入或選擇圖片。",captureStopped:"螢幕分享已結束。",captureEnded:"螢幕分享已結束，請取消後重新截圖，或匯入圖片。",captureTimeout:"螢幕擷取已停止或逾時。",frameWaiting:"擷取已停止或沒有新的畫面。",frameFailed:"無法讀取擷取畫面，請重試。",cropNeedsTab:"即時裁切請選擇目前瀏覽器分頁。視窗或全螢幕可先截圖，再裁切圖片附件。",invalidDimensions:(e,t)=>`畫面尺寸無效（${e} × ${t}），請重試截圖。`,imageTooLarge:(e,t)=>`圖片過大（${e} × ${t}），請使用不超過 1600 萬像素的圖片。`,encodeFailed:(e,t)=>`無法編碼圖片（${e} × ${t}），請重試截圖。`,imageBytesLimit:"圖片超過 8 MB，請使用較小的圖片。",imageFitFailed:"無法將截圖縮小至 8 MiB 的附件限制內。",drawingUnavailable:"圖片繪製功能無法使用。",imageType:"請選擇 PNG、JPEG 或 WebP 圖片。",imageValidation:"圖片附件驗證失敗。",composeTimeout:"圖片合成已停止或逾時。",zipLimit:"匯出內容超過 ZIP 大小限制，請個別下載圖片。",cropOutside:"裁切區域位於可見範圍外，請移回檢視區或清除裁切。",cropInvalid:"裁切尺寸無效。",cropOutsideImage:"裁切區域位於圖片之外。",invalidEndpoint:"MCP 位址必須是本機回環 HTTP(S) 位址。",invalidToken:"請輸入有效的配對權杖。",bridgeUnavailable:"開發環境連線無法使用。",sessionMismatch:"MCP 傳回了其他工作階段。",clipboardFailed:"無法存取剪貼簿，請使用匯出；回饋仍已儲存。",exportImagesPending:"部分圖片尚未儲存至本機，請同步完成後再匯出。",preferencesUnsaved:"設定已在本次工作階段生效，但無法儲存。",connectionRestoreFailed:"無法還原 MCP 設定，請手動連線。",syncConnecting:"正在連接 MCP 伺服器",syncConnected:"已連接 MCP 伺服器",syncFailed:"MCP 同步失敗，本機變更已保留，稍後重試。",syncUnavailable:"MCP 無法使用，本機回饋已保留，正在重試。"},jh={...Dh,...Eh,settings:"設定",language:"言語",theme:"テーマ",light:"ライト",dark:"ダーク",save:"保存",add:"追加",cancel:"キャンセル",delete:"削除",openInspector:"インスペクターを開く",closeInspector:"インスペクターを閉じる",inspector:"Ainotation インスペクター",moveInspector:"インスペクターを移動",inspectorSettings:"インスペクターの設定",copy:"フィードバックをコピー",copyHint:"このプロジェクトの全ページのフィードバックをコピー",exportJson:"JSON をエクスポート",exportImages:"フィードバックと画像をエクスポート",exportJsonHint:"このページの JSON をエクスポート",exportImagesHint:"このページと添付画像をエクスポート",clear:"すべての注釈を消去",clearHint:"このページのすべての注釈を消去",offline:"未接続",connecting:"接続中",connected:"接続済み",error:"エラー",loadingLocal:"ローカルのフィードバックを読み込み中…",syncing:"フィードバックを同期中…",storageNotice:"ローカルストレージを利用できません。変更はローカルに保存されません。",detail:"出力の詳細度",compact:"簡潔",standard:"標準",detailed:"詳細",forensic:"すべて",compactDescription:"短い説明、セレクター、テキストの引用。",standardDescription:"要素の位置、ビューポート、選択したテキスト。",detailedDescription:"クラス名、境界、周辺テキスト、取得時の状態を追加。",forensicDescription:"DOM の祖先、スタイル、アクセシビリティ、環境情報を追加。",copiedMarkdown:"コピーする Markdown に適用されます。",switchTo:(e)=>`${e}に切り替え`,labeled:(e,t)=>`${e}：${t}`,shortcut:(e,t)=>`${e}（${t}）`,switchTheme:(e)=>`${e}テーマに切り替え`,connection:"MCP 接続",localMode:"ローカルのみ",syncStorageFailed:"ローカルに保存済みです。サービスのストレージを修復するため ainotation-mcp doctor を実行して再試行してください。",syncRecoveryNeeded:"ローカルに保存済みです。サービスが別のバージョンを復元したため、同期を一時停止しました。",syncImagesMissing:"フィードバックは保存済みですが、一部の画像がありません。画像を保存しているブラウザーを開いて再アップロードしてください。",retrySync:"接続を再試行",recoverProject:"このブラウザーからプロジェクトを復元",exportRecovery:"以前のローカル版をエクスポート",recoveryLocalPreview:(e)=>`ブラウザーのバージョン（${e} 件）`,recoveryServerPreview:(e)=>`復元したサーバーのバージョン（${e} 件）`,syncProjectProgress:(e,t)=>`保存済みページ ${t} 件中 ${e} 件を確認しました。`,syncProjectReview:"確認が必要なページがあります。開いてバージョンや不足している画像を確認してください。",recoveryHelp:"先にコピーをエクスポートしてから、このページに残すバージョンを選択してください。サービスは両方のフィードバックを保持します。",recoveryBrowser:"ブラウザーのバージョンを使用",recoveryServer:"復元したサーバーのバージョンを使用",localModeDescription:"フィードバックはこのブラウザーに保存されます。コピーまたはエクスポートして共有できます。",automaticConnected:"開発サーバー経由で接続済みです。",automaticConnection:"開発サーバー経由で自動接続します。",manualConnected:"ローカル MCP サーバーに接続し、フィードバックを同期しています。",manualConnection:"ローカルのフィードバックを利用できます。MCP サーバーに接続すると Agent と共有できます。",endpoint:"エンドポイント",token:"ペアリングトークン",connect:"接続",disconnect:"切断",newAnnotation:"新しい注釈",newFeedback:"新しいフィードバック",editFeedback:"フィードバックを編集",targetUnavailable:"対象を利用できません",selectedElements:"選択した要素",copySelector:"セレクターをコピー",parentTarget:"親要素を選択",previousTarget:"直前の子要素に戻る",selectorCopied:"セレクターをコピーしました",locatorDetails:"位置情報の詳細",selectedText:"選択したテキスト",feedbackContent:"フィードバックの内容",attachedImages:"添付画像",editAnnotation:(e)=>`注釈 ${e} を編集`,editImage:(e)=>`画像 ${e} を編集`,attachedImage:(e)=>`添付画像 ${e}`,image:(e)=>`画像 ${e}`,downloadImage:"画像をダウンロード",removeImage:"画像を削除",downloadImageNumber:(e)=>`画像 ${e} をダウンロード`,removeImageNumber:(e)=>`画像 ${e} を削除`,screenshot:"スクリーンショット",screenshotHint:"ページに描画して撮影",chooseImage:"画像を選択",pasteImage:"ここに画像を貼り付けるかドロップ",imageEditor:"画像注釈エディター",drawingSurface:"描画領域",drawingTools:"描画ツール",imageToAnnotate:"注釈を付ける画像",select:"選択",selectMove:"選択／移動",arrow:"矢印",rectangle:"長方形",ellipse:"楕円",pen:"ペン",freeDraw:"自由描画",crop:"切り抜き",color:"色",colorValue:(e)=>`色 ${e}`,colorCycle:(e)=>`色：${e}（C で順に切り替え）`,red:"赤",amber:"琥珀色",green:"緑",blue:"青",white:"白",black:"黒",colorPalette:"カラーパレット",lineWidth:"線の太さ",lineWidths:"線の太さの選択肢",widthCycle:"線の太さ（S で順に切り替え）",undo:"元に戻す",redo:"やり直す",deleteShape:"図形を削除",deleteShapes:(e)=>`${e} 個の図形を削除`,clearCrop:"切り抜きを解除",cancelDrawing:"描画をキャンセル",captureAttach:"撮影して添付",attachImage:"画像を添付",captureHint:"撮影して添付（Command / Ctrl + Enter；Option / Alt を押しながらページを操作）",rotateHint:"四角の外側をドラッグして回転",arrowTailHint:"始点をドラッグして長さと方向を調整",resizeHint:"ドラッグしてサイズを変更",operationFailed:"操作に失敗しました。フィードバックは消去されていません。",storageUnavailable:"ローカルストレージを利用できません。再読み込み前にフィードバックをエクスポートしてください。",feedbackLoading:"フィードバックを読み込み中です。",pageLoading:"このページのフィードバックを読み込み中",localOnly:"ローカルのみ",copied:"フィードバックをコピーしました",exported:"フィードバックをエクスポートしました",saved:"フィードバックを保存しました",cleared:"このページの注釈をすべて消去しました",remoteDeleted:"編集中の注釈がリモートで削除されました。テキストを保持しているので、対象を選び直せます。",annotationDeleted:"注釈が削除されました。未保存のテキストは保持されています。",originalDeleted:"元のフィードバックが削除されました。内容を新しい注釈の下書きとして保持しました。",missingAnnotation:"このフィードバックは存在しません。",changedWhileDrawing:"描画中に注釈が変更されました。",imageLimit:"1 件の注釈に添付できる画像は 8 枚までです。",imagePending:"画像はまだローカルにありません。同期待ちです。",imageAttached:"画像を添付しました。フィードバックを保存すると共有できます。",imageAttachFailed:"画像を添付できませんでした。",shapesLimit:"画像 1 枚につき図形は 200 個までです。",captureUnavailable:"画面キャプチャを利用できません。画像を貼り付け、ドロップ、または選択してください。",captureStopped:"画面共有が終了しました。",captureEnded:"画面共有が終了しました。キャンセルして撮影し直すか、画像を読み込んでください。",captureTimeout:"画面キャプチャが停止したか、タイムアウトしました。",frameWaiting:"キャプチャが停止したか、新しいフレームがありません。",frameFailed:"画面を取得できませんでした。撮影し直してください。",cropNeedsTab:"リアルタイムの切り抜きには現在のタブを選択してください。ウィンドウや画面全体は撮影後に添付画像を切り抜けます。",invalidDimensions:(e,t)=>`フレームサイズが無効です（${e} × ${t}）。撮影し直してください。`,imageTooLarge:(e,t)=>`画像が大きすぎます（${e} × ${t}）。1600 万画素以下の画像を使用してください。`,encodeFailed:(e,t)=>`画像をエンコードできません（${e} × ${t}）。撮影し直してください。`,imageBytesLimit:"画像が 8 MB を超えています。小さい画像を使用してください。",imageFitFailed:"添付上限の 8 MiB 以内に画像を縮小できませんでした。",drawingUnavailable:"画像描画を利用できません。",imageType:"PNG、JPEG、WebP の画像を選択してください。",imageValidation:"添付画像の検証に失敗しました。",composeTimeout:"画像の合成が停止したか、タイムアウトしました。",zipLimit:"ZIP のサイズ上限を超えています。画像を個別にダウンロードしてください。",cropOutside:"切り抜き領域が表示範囲外です。表示範囲に戻すか、切り抜きを解除してください。",cropInvalid:"切り抜きサイズが無効です。",cropOutsideImage:"切り抜き領域が画像の外側です。",invalidEndpoint:"MCP の接続先はループバックの HTTP(S) アドレスである必要があります。",invalidToken:"有効なペアリングトークンを入力してください。",bridgeUnavailable:"開発環境の接続を利用できません。",sessionMismatch:"MCP が別のセッションを返しました。",clipboardFailed:"クリップボードにアクセスできません。エクスポートを使用してください。フィードバックは保存されています。",exportImagesPending:"一部の画像がまだローカルにありません。同期完了後にエクスポートしてください。",preferencesUnsaved:"このセッションでは設定を変更しましたが、保存できませんでした。",connectionRestoreFailed:"MCP 設定を復元できませんでした。手動で接続してください。",syncConnecting:"MCP サーバーに接続中",syncConnected:"MCP サーバーに接続しました",syncFailed:"MCP の同期に失敗しました。ローカルの変更を保持して再試行します。",syncUnavailable:"MCP を利用できません。ローカルのフィードバックを保持して再試行しています。"},Vh={...Lh,...Th,settings:"설정",language:"언어",theme:"테마",light:"라이트",dark:"다크",save:"저장",add:"추가",cancel:"취소",delete:"삭제",openInspector:"검사기 열기",closeInspector:"검사기 닫기",inspector:"Ainotation 검사기",moveInspector:"검사기 이동",inspectorSettings:"검사기 설정",copy:"피드백 복사",copyHint:"이 프로젝트의 모든 페이지 피드백 복사",exportJson:"JSON 내보내기",exportImages:"피드백 및 이미지 내보내기",exportJsonHint:"현재 페이지의 JSON 내보내기",exportImagesHint:"현재 페이지와 첨부 이미지 내보내기",clear:"모든 주석 지우기",clearHint:"현재 페이지의 모든 주석 지우기",offline:"연결 안 됨",connecting:"연결 중",connected:"연결됨",error:"오류",loadingLocal:"로컬 피드백을 불러오는 중…",syncing:"피드백 동기화 중…",storageNotice:"로컬 저장소를 사용할 수 없습니다. 변경 사항은 로컬에 저장되지 않습니다.",detail:"출력 상세 수준",compact:"간략",standard:"표준",detailed:"상세",forensic:"전체",compactDescription:"간단한 설명, 선택자 및 짧은 텍스트 인용.",standardDescription:"요소 위치, 뷰포트 및 선택한 텍스트.",detailedDescription:"클래스, 경계, 주변 텍스트 및 캡처 상태 추가.",forensicDescription:"DOM 상위 요소, 스타일, 접근성 및 환경 정보 추가.",copiedMarkdown:"복사한 Markdown에 적용됩니다.",switchTo:(e)=>`${e} 설정으로 전환`,labeled:(e,t)=>`${e}: ${t}`,shortcut:(e,t)=>`${e} (${t})`,switchTheme:(e)=>`${e} 테마로 전환`,connection:"MCP 연결",localMode:"로컬 전용",syncStorageFailed:"로컬에 저장되었습니다. 서비스 저장소를 복구하려면 ainotation-mcp doctor를 실행한 후 다시 시도하세요.",syncRecoveryNeeded:"로컬에 저장되었습니다. 서비스에서 다른 버전을 복구하여 동기화가 일시 중지되었습니다.",syncImagesMissing:"피드백은 저장되었지만 일부 이미지 파일이 없습니다. 이미지를 보관 중인 브라우저를 열어 다시 업로드하세요.",retrySync:"연결 재시도",recoverProject:"현재 브라우저에서 프로젝트 복구",exportRecovery:"이전 로컬 버전 내보내기",recoveryLocalPreview:(e)=>`브라우저 버전 (주석 ${e}개)`,recoveryServerPreview:(e)=>`복구된 서버 버전 (주석 ${e}개)`,syncProjectProgress:(e,t)=>`저장된 페이지 ${t}개 중 ${e}개를 확인했습니다.`,syncProjectReview:"일부 페이지를 확인해야 합니다. 해당 페이지를 열어 버전이나 누락된 이미지를 확인하세요.",recoveryHelp:"먼저 복사본을 내보낸 후 이 페이지에서 유지할 버전을 선택하세요. 서비스는 양쪽 피드백 스냅샷을 보관합니다.",recoveryBrowser:"브라우저 버전 사용",recoveryServer:"복구된 서버 버전 사용",localModeDescription:"피드백은 현재 브라우저에 저장됩니다. 복사하거나 내보내서 공유할 수 있습니다.",automaticConnected:"개발 서버를 통해 연결되었습니다.",automaticConnection:"개발 서버를 통해 자동으로 연결합니다.",manualConnected:"로컬 MCP 서버에 연결하여 피드백을 동기화합니다.",manualConnection:"로컬 피드백을 사용할 수 있습니다. MCP 서버에 연결하여 Agent와 공유하세요.",endpoint:"서버 주소",token:"페어링 토큰",connect:"연결",disconnect:"연결 해제",newAnnotation:"새 주석",newFeedback:"새 피드백",editFeedback:"피드백 편집",targetUnavailable:"대상을 사용할 수 없음",selectedElements:"선택한 요소",copySelector:"선택자 복사",parentTarget:"부모 요소 선택",previousTarget:"이전 하위 요소로 돌아가기",selectorCopied:"선택자가 복사되었습니다",locatorDetails:"위치 정보",selectedText:"선택한 텍스트",feedbackContent:"피드백 내용",attachedImages:"첨부 이미지",editAnnotation:(e)=>`주석 ${e} 편집`,editImage:(e)=>`이미지 ${e} 편집`,attachedImage:(e)=>`첨부 이미지 ${e}`,image:(e)=>`이미지 ${e}`,downloadImage:"이미지 다운로드",removeImage:"이미지 제거",downloadImageNumber:(e)=>`이미지 ${e} 다운로드`,removeImageNumber:(e)=>`이미지 ${e} 제거`,screenshot:"스크린샷",screenshotHint:"페이지에 그린 후 화면 캡처",chooseImage:"이미지 선택",pasteImage:"여기에 이미지를 붙여넣거나 놓으세요",imageEditor:"이미지 주석 편집기",drawingSurface:"그리기 영역",drawingTools:"그리기 도구",imageToAnnotate:"주석을 추가할 이미지",select:"선택",selectMove:"선택／이동",arrow:"화살표",rectangle:"사각형",ellipse:"타원",pen:"펜",freeDraw:"자유 그리기",crop:"자르기",color:"색상",colorValue:(e)=>`색상 ${e}`,colorCycle:(e)=>`색상: ${e} (C로 순환)`,red:"빨강",amber:"황색",green:"초록",blue:"파랑",white:"흰색",black:"검정",colorPalette:"색상 팔레트",lineWidth:"선 두께",lineWidths:"선 두께 옵션",widthCycle:"선 두께 (S로 순환)",undo:"실행 취소",redo:"다시 실행",deleteShape:"도형 삭제",deleteShapes:(e)=>`도형 ${e}개 삭제`,clearCrop:"자르기 해제",cancelDrawing:"그리기 취소",captureAttach:"캡처 후 첨부",attachImage:"이미지 첨부",captureHint:"캡처 후 첨부 (Command / Ctrl + Enter; Option / Alt를 누른 채 페이지 조작)",rotateHint:"사각형 바깥쪽을 드래그하여 회전",arrowTailHint:"시작점을 드래그하여 길이와 방향 조절",resizeHint:"드래그하여 크기 조절",operationFailed:"작업에 실패했습니다. 피드백은 지워지지 않았습니다.",storageUnavailable:"로컬 저장소를 사용할 수 없습니다. 새로고침 전에 피드백을 내보내세요.",feedbackLoading:"피드백을 불러오는 중입니다.",pageLoading:"현재 페이지의 피드백을 불러오는 중",localOnly:"로컬 전용",copied:"피드백을 복사했습니다",exported:"피드백을 내보냈습니다",saved:"피드백을 저장했습니다",cleared:"현재 페이지의 모든 주석을 지웠습니다",remoteDeleted:"편집 중인 주석이 원격으로 삭제되었습니다. 텍스트는 보관되었으니 대상을 다시 선택하세요.",annotationDeleted:"주석이 삭제되었습니다. 저장하지 않은 텍스트는 보관되었습니다.",originalDeleted:"원래 피드백이 삭제되었습니다. 내용을 새 주석 초안으로 보관했습니다.",missingAnnotation:"이 피드백은 더 이상 존재하지 않습니다.",changedWhileDrawing:"그리는 동안 주석이 변경되었습니다.",imageLimit:"주석 하나에 이미지를 최대 8개까지 첨부할 수 있습니다.",imagePending:"이미지가 아직 로컬에 없습니다. 동기화를 기다려 주세요.",imageAttached:"이미지를 첨부했습니다. 피드백을 저장하면 공유할 수 있습니다.",imageAttachFailed:"이미지를 첨부할 수 없습니다.",shapesLimit:"이미지 하나에 도형을 최대 200개까지 그릴 수 있습니다.",captureUnavailable:"화면 캡처를 사용할 수 없습니다. 이미지를 붙여넣거나 놓거나 선택하세요.",captureStopped:"화면 공유가 종료되었습니다.",captureEnded:"화면 공유가 종료되었습니다. 취소한 후 다시 캡처하거나 이미지를 가져오세요.",captureTimeout:"화면 캡처가 중지되었거나 시간이 초과되었습니다.",frameWaiting:"캡처가 중지되었거나 새 프레임이 없습니다.",frameFailed:"캡처 화면을 읽을 수 없습니다. 다시 시도하세요.",cropNeedsTab:"실시간 자르기에는 현재 브라우저 탭을 선택하세요. 창이나 전체 화면은 캡처 후 첨부 이미지를 자를 수 있습니다.",invalidDimensions:(e,t)=>`프레임 크기가 잘못되었습니다 (${e} × ${t}). 다시 캡처하세요.`,imageTooLarge:(e,t)=>`이미지가 너무 큽니다 (${e} × ${t}). 1,600만 픽셀 이하의 이미지를 사용하세요.`,encodeFailed:(e,t)=>`이미지를 인코딩할 수 없습니다 (${e} × ${t}). 다시 캡처하세요.`,imageBytesLimit:"이미지가 8 MB를 초과합니다. 더 작은 이미지를 사용하세요.",imageFitFailed:"스크린샷을 첨부 제한인 8 MiB 이내로 줄일 수 없습니다.",drawingUnavailable:"이미지 그리기를 사용할 수 없습니다.",imageType:"PNG, JPEG 또는 WebP 이미지를 선택하세요.",imageValidation:"첨부 이미지 검증에 실패했습니다.",composeTimeout:"이미지 합성이 중지되었거나 시간이 초과되었습니다.",zipLimit:"내보내기 내용이 ZIP 크기 제한을 초과합니다. 이미지를 개별로 다운로드하세요.",cropOutside:"자르기 영역이 보이는 범위 밖에 있습니다. 화면 안으로 옮기거나 자르기를 해제하세요.",cropInvalid:"자르기 크기가 잘못되었습니다.",cropOutsideImage:"자르기 영역이 이미지 밖에 있습니다.",invalidEndpoint:"MCP 주소는 루프백 HTTP(S) 주소여야 합니다.",invalidToken:"유효한 페어링 토큰을 입력하세요.",bridgeUnavailable:"개발 환경 연결을 사용할 수 없습니다.",sessionMismatch:"MCP가 다른 세션을 반환했습니다.",clipboardFailed:"클립보드에 접근할 수 없습니다. 내보내기를 사용하세요. 피드백은 저장되어 있습니다.",exportImagesPending:"일부 이미지가 아직 로컬에 없습니다. 동기화 후 내보내세요.",preferencesUnsaved:"이번 세션의 설정은 변경되었지만 저장하지 못했습니다.",connectionRestoreFailed:"MCP 설정을 복원할 수 없습니다. 수동으로 연결하세요.",syncConnecting:"MCP 서버에 연결 중",syncConnected:"MCP 서버에 연결되었습니다",syncFailed:"MCP 동기화에 실패했습니다. 로컬 변경 사항을 보관하고 재시도합니다.",syncUnavailable:"MCP를 사용할 수 없습니다. 로컬 피드백을 보관하고 재시도 중입니다."},_i=["zh-Hans","zh-Hant","en","ja","ko"],Yu={"zh-Hans":"简体中文","zh-Hant":"繁體中文",en:"English",ja:"日本語",ko:"한국어"},Fh={en:ht,"zh-Hans":Rh,"zh-Hant":Zh,ja:jh,ko:Vh};Dr=class extends Error{description;constructor(e,t){super(mt("en",e),t);this.description=e,this.name="UiError"}};ln=[{value:"compact",label:ht.compact,description:ht.compactDescription},{value:"standard",label:ht.standard,description:ht.standardDescription},{value:"detailed",label:ht.detailed,description:ht.detailedDescription},{value:"forensic",label:ht.forensic,description:ht.forensicDescription}]});function Ht(e){return Fe(e,{"aria-hidden":"true",focusable:"false"})}function od(){if(!customElements.get("ainotation-inspector-shell"))customElements.define("ainotation-inspector-shell",Gh)}var Hh,Jh,qh,Kh,Wh,rd,Gh;var id=Se(()=>{Fn();Hh=[["path",{d:"M10.733 5.076a10.744 10.744 0 0 1 11.205 6.575 1 1 0 0 1 0 .696 10.747 10.747 0 0 1-1.444 2.49"}],["path",{d:"M14.084 14.158a3 3 0 0 1-4.242-4.242"}],["path",{d:"M17.479 17.499a10.75 10.75 0 0 1-15.417-5.151 1 1 0 0 1 0-.696 10.75 10.75 0 0 1 4.446-5.143"}],["path",{d:"m2 2 20 20"}]],Jh=[["path",{d:"M2.062 12.348a1 1 0 0 1 0-.696 10.75 10.75 0 0 1 19.876 0 1 1 0 0 1 0 .696 10.75 10.75 0 0 1-19.876 0"}],["circle",{cx:"12",cy:"12",r:"3"}]],qh=[["path",{d:"M20.985 12.486a9 9 0 1 1-9.473-9.472c.405-.022.617.46.402.803a6 6 0 0 0 8.268 8.268c.344-.215.825-.004.803.401"}]],Kh=[["path",{d:"M9.671 4.136a2.34 2.34 0 0 1 4.659 0 2.34 2.34 0 0 0 3.319 1.915 2.34 2.34 0 0 1 2.33 4.033 2.34 2.34 0 0 0 0 3.831 2.34 2.34 0 0 1-2.33 4.033 2.34 2.34 0 0 0-3.319 1.915 2.34 2.34 0 0 1-4.659 0 2.34 2.34 0 0 0-3.32-1.915 2.34 2.34 0 0 1-2.33-4.033 2.34 2.34 0 0 0 0-3.831A2.34 2.34 0 0 1 6.35 6.051a2.34 2.34 0 0 0 3.319-1.915"}],["circle",{cx:"12",cy:"12",r:"3"}]],Wh=[["circle",{cx:"12",cy:"12",r:"4"}],["path",{d:"M12 2v2"}],["path",{d:"M12 20v2"}],["path",{d:"m4.93 4.93 1.41 1.41"}],["path",{d:"m17.66 17.66 1.41 1.41"}],["path",{d:"M2 12h2"}],["path",{d:"M20 12h2"}],["path",{d:"m6.34 17.66-1.41 1.41"}],["path",{d:"m19.07 4.93-1.41 1.41"}]],rd=We`<svg
  class="brand-mark"
  xmlns="http://www.w3.org/2000/svg"
  viewBox="0 0 256 256"
  fill="currentColor"
  aria-hidden="true"
  focusable="false"
>
  <path d="M86.01,233.06 C77.42,241.65 66.34,245.99 54.65,245.99 C42.98,245.99 31.91,241.68 23.29,233.06 C14.7,224.47 10.38,213.41 10.38,201.72 C10.38,190.05 14.67,178.98 23.29,170.36 C31.88,161.77 42.96,157.44 54.65,157.44 C66.32,157.44 77.39,161.74 86.01,170.36 C94.6,178.95 98.92,190.03 98.92,201.72 C98.9,213.39 94.6,225.06 86.01,233.06 Z" />
  <path d="M244.58,27.77 L184.34,223.83 L184.34,224.45 C184.34,225.08 183.74,226.28 183.13,226.89 C182.5,227.52 181.9,228.74 181.27,229.35 L180.65,229.35 C180.02,229.98 180.04,229.96 179.43,230.57 L178.81,231.19 C178.18,231.19 178.19,231.82 177.58,231.82 C177.58,231.82 176.96,231.82 176.96,232.45 C176.33,232.45 175.72,233.06 175.72,233.06 L175.11,233.06 L172.65,233.06 L170.81,233.06 C170.18,233.06 169.58,233.08 169.58,232.45 L168.96,232.45 C168.33,232.45 168.33,231.82 167.72,231.82 C167.09,231.82 167.12,231.19 166.51,231.19 C166.51,231.19 165.89,231.2 165.89,230.57 C165.26,229.94 164.65,229.96 164.65,229.35 L164.03,228.72 C163.4,228.09 163.43,228.11 162.82,227.5 C162.82,226.87 162.19,226.89 162.19,226.28 C162.19,225.65 161.56,225.66 161.56,225.05 C161.56,224.42 161.58,224.44 160.95,223.83 C160.95,223.2 160.95,223.21 160.32,222.6 L160.32,221.37 L160.32,220.15 L160.32,219.52 L160.32,217.68 C162.17,207.23 149.26,176.51 114.23,141.48 C79.2,106.45 48.46,93.53 38.01,95.38 L36.79,95.38 L36.16,95.38 L34.32,95.38 L33.69,95.38 C33.06,95.38 32.46,94.76 31.24,94.76 L30.62,94.13 C29.99,94.13 29.39,93.51 29.39,93.51 L28.76,92.88 C28.13,92.25 28.15,92.27 27.54,91.66 L26.92,91.03 L26.92,90.41 C26.92,89.78 26.3,89.8 26.3,89.19 C26.3,88.56 25.67,88.57 25.67,87.96 C25.67,87.33 25.68,87.34 25.05,86.73 C25.05,86.1 25.05,86.12 24.42,85.51 L24.42,84.89 L24.42,84.26 L24.42,83.04 L24.42,81.82 L24.42,80.58 L24.42,79.36 C24.42,78.73 24.42,78.75 25.05,78.14 C25.05,77.51 25.04,77.53 25.67,76.92 C25.67,76.29 26.3,76.3 26.3,75.69 C26.3,75.06 26.92,75.08 26.92,74.47 L27.54,73.84 C28.17,73.21 28.15,73.22 28.76,73.22 C28.76,73.22 28.76,72.6 29.39,72.6 L30.02,72.6 C30.65,72.6 30.63,71.97 31.24,71.97 C31.87,71.97 31.86,71.35 32.47,71.35 C33.1,71.35 33.08,71.35 33.69,70.72 L34.32,70.72 L230.37,10.48 C234.67,9.26 239.59,10.49 242.66,13.56 C245.8,16.7 246.41,23.47 244.58,27.77 Z" />
</svg>`;Gh=class e extends on{static properties={view:{attribute:!1},expanded:{type:Boolean,reflect:!0},endpointDraft:{state:!0},token:{state:!0},settingsOpen:{state:!0}};static styles=wt`
    ${Ft}
    .icon[aria-checked='true'] {
      background: var(--ain-selected);
    }
    .launcher[data-preview='true']::after {
      content: '';
      position: absolute;
      right: 3px;
      top: 3px;
      width: 7px;
      height: 7px;
      border: 1px solid var(--ain-on-accent);
      border-radius: 50%;
      background: var(--ain-accent);
    }
    :host {
      position: fixed;
      right: 16px;
      bottom: 16px;
      z-index: 2147483647;
      display: block;
      box-sizing: border-box;
      width: ${48}px;
      height: ${48}px;
      color: var(--ain-text);
      font:
        13px/1.5 system-ui,
        sans-serif;
      letter-spacing: 0;
      color-scheme: var(--ain-scheme);
    }
    :host([expanded]) {
      width: ${294}px;
      height: ${52}px;
      max-width: min(calc(100vw - 32px), calc(var(--inspector-viewport-width, 100vw) - 32px));
    }
    * {
      box-sizing: border-box;
    }
    [hidden],
    .toolbar[hidden] {
      display: none !important;
    }
    .toolbar {
      display: flex;
      height: ${52}px;
      border: 1px solid var(--ain-border);
      border-radius: 999px;
      background: var(--ain-surface);
      color: var(--ain-text);
      box-shadow: 0 6px 24px var(--ain-shadow);
    }
    header,
    .row,
    .actions {
      display: flex;
      align-items: center;
      gap: 6px;
    }
    header {
      width: 100%;
      border-radius: inherit;
      padding: 5px 8px;
      gap: 4px;
      cursor: grab;
      touch-action: none;
      user-select: none;
    }
    h2,
    p {
      margin: 0;
    }
    h2 {
      font-size: 15px;
      font-weight: 650;
    }
    .row,
    .actions {
      flex-wrap: wrap;
    }
    .muted {
      color: var(--ain-muted);
      font-size: 12px;
    }
    button,
    input,
    select {
      font: inherit;
    }
    button {
      display: inline-flex;
      align-items: center;
      justify-content: center;
      gap: 5px;
      min-height: 32px;
      padding: 5px 9px;
      border: 1px solid var(--ain-border);
      border-radius: 4px;
      background: var(--ain-surface);
      color: var(--ain-text);
      cursor: pointer;
    }
    button:hover:not(:disabled) {
      background: var(--ain-hover);
    }
    button:disabled {
      opacity: 0.45;
      cursor: default;
    }
    .launcher {
      display: flex;
      width: ${48}px;
      height: ${48}px;
      padding: 0;
      border: 0;
      border-radius: 50%;
      background: var(--ain-brand-container);
      color: var(--ain-on-brand);
      touch-action: none;
      user-select: none;
      box-shadow: 0 4px 20px var(--ain-shadow);
    }
    .launcher:hover:not(:disabled) {
      background: var(--ain-brand-container);
      box-shadow: 0 6px 24px var(--ain-shadow);
    }
    .brand-mark {
      pointer-events: none;
    }
    .launcher .brand-mark {
      width: 24px;
      height: 24px;
    }
    .settings-title {
      display: inline-flex;
      align-items: center;
      gap: 7px;
    }
    .settings-heading .brand-mark {
      width: 16px;
      height: 16px;
      color: var(--ain-brand-mark);
    }
    .icon {
      width: 40px;
      height: 40px;
      min-width: 0;
      flex: 1 1 40px;
      padding: 8px;
      border: 0;
      border-radius: 50%;
      background: transparent;
      color: inherit;
    }
    .icon:hover:not(:disabled),
    .icon[aria-expanded='true'] {
      background: var(--ain-hover);
    }
    .icon svg {
      width: 19px;
      height: 19px;
    }
    .separator {
      width: 1px;
      height: 22px;
      background: var(--ain-border);
      margin: 0 5px;
      flex-shrink: 0;
    }
    .settings {
      position: absolute;
      bottom: calc(100% + 10px);
      right: 0;
      width: 300px;
      max-width: calc(100vw - 24px);
      padding: 16px;
      border: 1px solid var(--ain-border);
      border-radius: 14px;
      background: var(--ain-surface);
      box-shadow: 0 8px 32px var(--ain-shadow);
      overflow: auto;
      overscroll-behavior: contain;
    }
    .settings-heading {
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-bottom: 12px;
    }
    .connection-status {
      display: inline-flex;
      align-items: center;
      gap: 6px;
      font-size: 12px;
      color: var(--ain-muted);
    }
    .settings a {
      color: var(--ain-text);
      overflow-wrap: anywhere;
    }
    .recovery-preview {
      max-height: 180px;
      overflow: auto;
      overflow-wrap: anywhere;
      font-size: 12px;
    }
    .connection-status::before {
      content: '';
      width: 6px;
      height: 6px;
      border-radius: 50%;
      background: var(--ain-idle);
    }
    .connection-status[data-state='connected']::before {
      background: var(--ain-success);
    }
    .connection-status[data-state='error']::before {
      background: var(--ain-error);
    }
    .toolbar:focus-within {
      border-color: var(--ain-focus);
    }
    .notices:empty {
      display: none;
    }
    .notices {
      position: absolute;
      right: 0;
      bottom: calc(100% + 10px);
      width: max-content;
      max-width: min(300px, calc(100vw - 32px));
      padding: 8px 12px;
      border: 1px solid var(--ain-border);
      border-radius: 10px;
      background: var(--ain-surface);
      box-shadow: 0 4px 16px var(--ain-shadow);
      pointer-events: none;
    }
    .toolbar .icon:focus-visible {
      outline-color: var(--ain-focus);
    }
    @media (prefers-reduced-motion: no-preference) {
      .icon {
        transition: background 120ms;
      }
    }
    button:focus-visible,
    header:focus-visible,
    input:focus-visible,
    select:focus-visible {
      outline: 2px solid var(--ain-focus);
      outline-offset: 2px;
    }
    svg {
      width: 16px;
      height: 16px;
      flex-shrink: 0;
    }
    label {
      display: grid;
      gap: 5px;
    }
    input {
      width: 100%;
      min-width: 0;
      border: 1px solid var(--ain-field-border);
      border-radius: 4px;
      padding: 7px 8px;
      background: var(--ain-field);
      color: var(--ain-text);
    }
    input::placeholder {
      color: var(--ain-muted);
      opacity: 1;
    }
    .connection-form {
      display: grid;
      gap: 9px;
      padding-top: 4px;
    }
    .language-choice {
      max-width: 160px;
      min-height: 28px;
      padding: 2px 6px;
      border: 1px solid var(--ain-border);
      border-radius: 4px;
      color: var(--ain-text);
      background: var(--ain-field);
    }
    .output-detail {
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 8px;
      margin: 4px 0;
    }
    .detail-label {
      display: inline-flex;
      align-items: center;
      gap: 5px;
      color: var(--ain-muted);
    }
    .detail-choice {
      display: inline-flex;
      align-items: center;
      justify-content: flex-end;
      gap: 8px;
      min-width: 104px;
      height: 28px;
      min-height: 28px;
      max-height: 28px;
      font-weight: 650;
      color: inherit;
      background: transparent;
      border: 0;
      border-radius: 4px;
      padding: 4px;
    }
    .detail-dots {
      display: flex;
      flex-direction: column;
      align-items: center;
      width: 4px;
      gap: 2px;
    }
    .detail-dot {
      width: 3px;
      height: 3px;
      border-radius: 50%;
      background: currentColor;
      opacity: 0.25;
    }
    .detail-dot.active {
      width: 4px;
      height: 4px;
      opacity: 1;
    }
    .detail-description {
      font-size: 12px;
      color: var(--ain-muted);
      margin: 0 0 16px;
    }
    .notices {
      display: grid;
      gap: 3px;
      margin-top: 7px;
      overflow-wrap: anywhere;
      font-size: 12px;
    }
    .message {
      color: var(--ain-message);
    }
  `;opensLeft=!0;hasPosition=!1;positionMoved=!1;drag=null;listeners;resizeObserver;frame=0;suppressClick=!1;suppressTimer;focusExpanded;constructor(){super();this.view=Vr(),this.expanded=!1,this.endpointDraft=null,this.token="",this.settingsOpen=!1}connectedCallback(){if(super.connectedCallback(),!this.hasAttribute("data-ainotation-ui"))this.setAttribute("data-ainotation-ui","inspector");this.listeners=new AbortController;let{signal:t}=this.listeners;this.addEventListener("pointerdown",this.clearSuppressedClick,{capture:!0,signal:t}),this.addEventListener("keydown",this.clearSuppressedClick,{capture:!0,signal:t}),this.addEventListener("click",this.onClickCapture,{capture:!0,signal:t}),this.addEventListener("pointermove",this.onPointerMove,{signal:t}),this.addEventListener("pointerup",this.onPointerEnd,{signal:t}),this.addEventListener("pointercancel",this.onPointerEnd,{signal:t}),this.addEventListener("lostpointercapture",this.onPointerEnd,{signal:t}),this.ownerDocument.addEventListener("keydown",this.onShortcut,{capture:!0,signal:t}),this.ownerDocument.addEventListener("pointerdown",this.onOutsidePointer,{capture:!0,signal:t}),window.addEventListener("resize",this.scheduleClamp,{signal:t}),window.visualViewport?.addEventListener("resize",this.scheduleClamp,{signal:t}),window.visualViewport?.addEventListener("scroll",this.scheduleClamp,{signal:t}),this.resizeObserver=new ResizeObserver(this.scheduleClamp),this.resizeObserver.observe(this),this.scheduleClamp()}disconnectedCallback(){this.listeners?.abort(),this.listeners=void 0,this.resizeObserver?.disconnect(),this.resizeObserver=void 0,cancelAnimationFrame(this.frame),this.frame=0,this.finishDrag(),this.clearSuppressedClick(),this.focusExpanded=void 0,super.disconnectedCallback()}willUpdate(t){if(this.dataset.theme=this.view.theme,this.lang=this.view.locale,!t.has("expanded")||!this.hasUpdated)return;if(!this.expanded)this.settingsOpen=!1;this.finishDrag();let n=this.getBoundingClientRect();if(this.expanded){let r=td(n,this.opensLeft,cn());this.opensLeft=r.opensLeft,this.setPosition(r.point)}else this.setPosition(Si(n,!0,this.opensLeft))}updated(){if(!this.isConnected)return;if(this.clampPosition(),this.focusExpanded===this.expanded){let t=this.expanded?'button[data-command="close"]':".launcher";this.renderRoot.querySelector(t)?.focus({preventScroll:!0})}this.focusExpanded=void 0}expand(t){this.focusExpanded=t,this.expanded=t,this.onaction({type:"set-picking",value:t})}toggleSettings(){if(this.settingsOpen=!this.settingsOpen,this.settingsOpen)this.updateComplete.then(()=>{if(this.settingsOpen&&this.expanded)this.renderRoot.querySelector(".settings")?.focus({preventScroll:!0})})}onOutsidePointer=(t)=>{if(this.settingsOpen&&!t.composedPath().includes(this))this.settingsOpen=!1};positionSettings(){let t=this.renderRoot.querySelector(".settings");if(!t||!this.settingsOpen||!this.expanded)return;let{left:n,top:r,width:o,height:i}=cn(),a=this.getBoundingClientRect(),s=Math.min(300,Math.max(0,o-24));t.style.width=`${s}px`,t.style.left=`${Math.max(n+12,Math.min(a.right-s,n+o-s-12))-a.left}px`,t.style.right="auto";let c=a.top-r-20,l=r+i-a.bottom-20,p=c<t.scrollHeight+2&&l>c;t.style.top=p?"calc(100% + 10px)":"auto",t.style.bottom=p?"auto":"calc(100% + 10px)",t.style.maxHeight=`${Math.max(0,p?l:c)}px`}onShortcut=(t)=>{let n=t.composedPath().find((r)=>r instanceof e);if(n&&n!==this)return;if(!t.defaultPrevented&&this.expanded&&this.settingsOpen&&t.key==="Escape"&&!t.altKey&&!t.isComposing){t.preventDefault(),t.stopImmediatePropagation(),this.settingsOpen=!1,this.renderRoot.querySelector('[data-command="settings"]')?.focus({preventScroll:!0});return}if(t.defaultPrevented||t.isComposing||t.repeat||t.code!=="KeyA"||!t.altKey||!t.shiftKey||t.ctrlKey||t.metaKey||!this.expanded&&this.view.storage==="loading")return;t.preventDefault(),t.stopPropagation(),this.clearSuppressedClick(),this.expand(!this.expanded)};setPosition({left:t,top:n}){this.style.left=`${t}px`,this.style.top=`${n}px`,this.style.right="auto",this.style.bottom="auto"}getPosition(){if(!this.isConnected||!this.hasPosition)return;let t=this.getBoundingClientRect();if(!t.width||!t.height)return;return Si(t,this.hasAttribute("expanded"),this.opensLeft)}restorePosition(t){if(!this.isConnected||this.positionMoved)return;this.hasPosition=!0,this.opensLeft=t.opensLeft;let n=this.getBoundingClientRect();this.setPosition(ed(t,n,this.hasAttribute("expanded"))),this.clampPosition()}notifyPosition(){if(this.getPosition())this.dispatchEvent(new CustomEvent("ainotation-position-change"))}clampPosition(t){if(t)this.positionMoved=!0,this.hasPosition=!0;let n=cn(),{width:r,height:o}=n;this.style.setProperty("--inspector-viewport-width",`${r}px`),this.style.setProperty("--inspector-viewport-height",`${o}px`);let i=this.getBoundingClientRect(),{left:a,top:s}=Qu(t??i,i,n);if(t||a!==i.left||s!==i.top)this.setPosition({left:a,top:s});if(!this.expanded&&t)this.opensLeft=!0;if(this.positionSettings(),!t&&(a!==i.left||s!==i.top))this.notifyPosition()}scheduleClamp=()=>{if(!this.isConnected||this.frame)return;this.frame=requestAnimationFrame(()=>{if(this.frame=0,this.isConnected&&this.hasUpdated)this.clampPosition()})};clearSuppressedClick=()=>{this.suppressClick=!1,clearTimeout(this.suppressTimer),this.suppressTimer=void 0};onClickCapture=(t)=>{if(!this.suppressClick||t.detail===0)return;t.preventDefault(),t.stopImmediatePropagation(),this.clearSuppressedClick()};onPointerDown=(t)=>{if(this.drag||!t.isPrimary||t.button!==0)return;let n=t.currentTarget,r=n.localName==="header"&&t.composedPath().some((a)=>a instanceof Element&&a!==n&&a.matches("button, input, textarea, select, details, a, [contenteditable]"));if(r&&t.pointerType!=="touch")return;let{left:o,top:i}=this.getBoundingClientRect();if(this.drag={target:n,pointerId:t.pointerId,x:t.clientX,y:t.clientY,left:o,top:i,dragged:!1,deferredCapture:r},!r)n.setPointerCapture(t.pointerId)};onPointerMove=(t)=>{let n=this.drag;if(!n||n.pointerId!==t.pointerId)return;let r=t.clientX-n.x,o=t.clientY-n.y;if(!n.dragged&&Math.hypot(r,o)<5)return;if(n.deferredCapture)n.deferredCapture=!1,n.target.setPointerCapture(t.pointerId);n.dragged=!0,t.preventDefault(),this.clampPosition({left:n.left+r,top:n.top+o})};onPointerEnd=(t)=>{let n=this.drag;if(!n||n.pointerId!==t.pointerId)return;if(t.type==="lostpointercapture"&&t.composedPath()[0]!==n.target)return;if(t.type==="pointerup")this.onPointerMove(t);if(n.dragged||t.type!=="pointerup")this.suppressClick=!0,this.suppressTimer=setTimeout(this.clearSuppressedClick,500);if(n.dragged)this.notifyPosition();this.finishDrag()};finishDrag(){let t=this.drag;if(this.drag=null,t?.target.hasPointerCapture(t.pointerId))t.target.releasePointerCapture(t.pointerId)}onMoveKey=(t)=>{if(t.composedPath()[0]!==t.currentTarget||t.altKey||t.ctrlKey||t.metaKey)return;let n=t.shiftKey?1:10,r={ArrowLeft:[-n,0],ArrowRight:[n,0],ArrowUp:[0,-n],ArrowDown:[0,n]}[t.key];if(!r){if(t.key===" "&&t.currentTarget.localName==="header")t.preventDefault();return}t.preventDefault(),t.stopPropagation();let{left:o,top:i}=this.getBoundingClientRect();this.clampPosition({left:o+r[0],top:i+r[1]}),this.notifyPosition()};onaction(t){this.dispatchEvent(new CustomEvent("ainotation-action",{detail:t,bubbles:!0,composed:!0}))}render(){let t=this.view,n=st(t.locale),r=Math.max(0,ln.findIndex((l)=>l.value===t.outputDetail)),o=ln[r],i=ln[(r+1)%ln.length],a=this.endpointDraft??t.endpoint,s={offline:n.offline,connecting:n.connecting,connected:n.connected,error:n.error}[t.connection],c=J`
      ${t.storage==="loading"?J`<p>${n.loadingLocal}</p>`:K}
      ${t.storage==="unavailable"?J`<p>${n.storageNotice}</p>`:K}
      ${t.syncing?J`<p>${n.syncing}</p>`:K}
      ${t.message?J`<p class="message">${t.message}</p>`:K}
    `;return J`
      <button
        class="launcher"
        data-preview=${String(t.styleEditor.globalPreview&&t.styleEditor.globalCount>0)}
        type="button"
        aria-label=${n.openInspector}
        title=${n.shortcut(n.openInspector,"Option/Alt + Shift + A")}
        aria-keyshortcuts="Alt+Shift+A"
        aria-expanded=${this.expanded}
        aria-controls="inspector-toolbar"
        ?hidden=${this.expanded}
        ?disabled=${t.storage==="loading"}
        aria-busy=${t.storage==="loading"}
        @pointerdown=${this.onPointerDown}
        @keydown=${this.onMoveKey}
        @click=${()=>this.expand(!0)}
      >
        ${rd}
      </button>
      <section
        id="inspector-toolbar"
        class="toolbar"
        role="toolbar"
        aria-label=${n.inspector}
        ?hidden=${!this.expanded}
      >
        <header
          role="group"
          tabindex="0"
          aria-label=${n.moveInspector}
          @pointerdown=${this.onPointerDown}
          @keydown=${this.onMoveKey}
        >
          <button
            class="icon"
            type="button"
            aria-label=${n.copy}
            title=${n.copyHint}
            ?disabled=${!t.document}
            @click=${()=>this.onaction({type:"copy"})}
          >
            ${Ht(Mr)}
          </button>
          <button
            class="icon"
            type="button"
            aria-label=${t.document?.annotations.some((l)=>l.images?.length)?n.exportImages:n.exportJson}
            title=${t.document?.annotations.some((l)=>l.images?.length)?n.exportImagesHint:n.exportJsonHint}
            ?disabled=${!t.document}
            @click=${()=>this.onaction({type:"export"})}
          >
            ${Ht(Nr)}
          </button>
          <button
            class="icon"
            type="button"
            role="switch"
            aria-label=${n.styleGlobalPreview}
            title=${n.styleGlobalHint(t.styleEditor.globalCount)}
            aria-checked=${String(t.styleEditor.globalPreview)}
            ?disabled=${t.storage==="loading"}
            @click=${()=>this.onaction({type:"global-style-preview",value:!t.styleEditor.globalPreview})}
          >
            ${Ht(t.styleEditor.globalPreview?Jh:Hh)}
          </button>
          <button
            class="icon"
            type="button"
            aria-label=${n.clear}
            title=${n.clearHint}
            ?disabled=${t.saving||t.storage==="loading"||!t.document?.annotations.length&&!t.selected.length&&!t.draft}
            @click=${()=>this.onaction({type:"clear-all"})}
          >
            ${Ht(Vt)}
          </button>
          <button
            class="icon"
            type="button"
            aria-label=${n.settings}
            title=${n.settings}
            data-command="settings"
            aria-expanded=${this.settingsOpen}
            aria-controls="inspector-settings"
            aria-haspopup="dialog"
            @click=${()=>this.toggleSettings()}
          >
            ${Ht(Kh)}
          </button>
          <span class="separator" role="separator" aria-orientation="vertical"></span>
          <button
            class="icon"
            type="button"
            aria-label=${n.closeInspector}
            title=${n.shortcut(n.closeInspector,"Option/Alt + Shift + A")}
            data-command="close"
            aria-keyshortcuts="Alt+Shift+A"
            @click=${()=>this.expand(!1)}
          >
            ${Ht(an)}
          </button>
        </header>
      </section>
      <section
        class="settings"
        id="inspector-settings"
        role="dialog"
        aria-label=${n.inspectorSettings}
        tabindex="-1"
        ?hidden=${!this.expanded||!this.settingsOpen}
      >
        <div class="settings-heading">
          <div class="settings-title">
            ${rd}
            <h2>${n.settings}</h2>
          </div>
          ${t.localOnly?J`<span class="muted local-mode">${n.localMode}</span>`:J`<span class="connection-status" data-state=${t.connection} role="status"
                  >${s}</span
                >`}
        </div>
        <div class="output-detail">
          <label class="detail-label" for="inspector-language">${n.language}</label>
          <select
            id="inspector-language"
            class="language-choice"
            aria-label=${n.language}
            @change=${(l)=>{let p=l.currentTarget.value;if(Ut(p))this.onaction({type:"set-locale",value:p})}}
          >
            ${_i.map((l)=>J`<option value=${l} lang=${l} ?selected=${t.locale===l}>${Yu[l]}</option>`)}
          </select>
        </div>
        <div class="output-detail">
          <label class="detail-label" for="inspector-theme">${n.theme}</label>
          <button
            id="inspector-theme"
            class="detail-choice"
            type="button"
            aria-label=${n.labeled(n.theme,t.theme==="dark"?n.dark:n.light)}
            title=${n.switchTheme(t.theme==="dark"?n.light:n.dark)}
            @click=${()=>this.onaction({type:"set-theme",value:t.theme==="dark"?"light":"dark"})}
          >
            <span>${t.theme==="dark"?n.dark:n.light}</span
            >${Ht(t.theme==="dark"?qh:Wh)}
          </button>
        </div>
        <div class="output-detail">
          <label class="detail-label" for="output-detail">${n.detail}</label>
          <button
            class="detail-choice"
            type="button"
            id="output-detail"
            data-level=${o.value}
            aria-label=${n.labeled(n.detail,n[o.value])}
            title=${n.switchTo(n[i.value])}
            aria-describedby="output-detail-description"
            @click=${()=>this.onaction({type:"set-output-detail",value:i.value})}
          >
            <span>${n[o.value]}</span>
            <span class="detail-dots" aria-hidden="true"
              >${ln.map((l)=>J`<span class=${`detail-dot${l.value===o.value?" active":""}`}></span>`)}</span
            >
          </button>
        </div>
        <p class="detail-description" id="output-detail-description">
          ${n[`${o.value}Description`]} ${n.copiedMarkdown}
        </p>
        ${t.localOnly?J`<p class="muted local-mode-description">${n.localModeDescription}</p>`:J` <p>${n.connection}</p>
                ${t.hasRecoveryCopy?J`<button @click=${()=>this.onaction({type:"export-recovery"})}>${n.exportRecovery}</button>`:K}
                ${t.connection==="error"||t.recoveryNeeded||t.recoveringProject||t.recoveryPages.length>0?J`<button
                        data-command="recover-project"
                        ?disabled=${t.recoveringProject||t.connection==="offline"}
                        @click=${()=>this.onaction({type:"recover-project"})}
                      >
                        ${n.recoverProject}
                      </button>`:K}
                ${t.recoveryPages.length?J`<p class="muted">${n.syncProjectReview}</p>
                        <ul>
                          ${t.recoveryPages.map((l)=>J`<li><a href=${l}>${l}</a></li>`)}
                        </ul>`:K}
                ${t.syncProblem?J`<p class="muted" role="status">${mt(t.locale,t.syncProblem)}</p>`:K}
                ${t.recoveryNeeded?J`<p class="muted">${n.recoveryHelp}</p>
                        <details>
                          <summary>
                            ${n.recoveryLocalPreview(t.document?.annotations.length??0)}
                          </summary>
                          <ul class="recovery-preview">
                            ${t.document?.annotations.map((l)=>J`<li>${l.comment}</li>`)}
                          </ul>
                        </details>
                        ${t.recoveredDocument?J`<details>
                                <summary>
                                  ${n.recoveryServerPreview(t.recoveredDocument.annotations.length)}
                                </summary>
                                <ul class="recovery-preview">
                                  ${t.recoveredDocument.annotations.map((l)=>J`<li>${l.comment}</li>`)}
                                </ul>
                              </details>`:K}
                        <div class="row">
                          <button @click=${()=>this.onaction({type:"copy"})}>${n.copy}</button
                          ><button
                            @click=${()=>this.onaction({type:"resolve-recovery",source:"browser"})}
                          >
                            ${n.recoveryBrowser}</button
                          ><button
                            @click=${()=>this.onaction({type:"resolve-recovery",source:"server"})}
                          >
                            ${n.recoveryServer}
                          </button>
                        </div>`:K}
                ${t.connection==="error"?J`<button @click=${()=>this.onaction({type:"retry-sync"})}>${n.retrySync}</button>`:K}
                ${t.managedConnection?J`<p class="muted">${t.projectName}</p>
                        <p class="muted">
                          ${t.connection==="connected"?n.automaticConnected:n.automaticConnection}
                        </p>`:J`
                        <p class="muted">
                          ${t.connection==="connected"?n.manualConnected:n.manualConnection}
                        </p>
                        <form
                          class="connection-form"
                          @submit=${(l)=>{if(l.preventDefault(),!a.trim()||!this.token.trim()||t.connection==="connecting"||t.connection==="connected")return;let p=this.token.trim();this.token="",this.onaction({type:"connect",endpoint:a.trim(),token:p})}}
                        >
                          <label
                            >${n.endpoint}<input
                              type="url"
                              required
                              .value=${a}
                              placeholder="http://127.0.0.1:4748"
                              @input=${(l)=>{this.endpointDraft=l.currentTarget.value}}
                          /></label>
                          <label
                            >${n.token}<input
                              type="password"
                              autocomplete="off"
                              .value=${this.token}
                              @input=${(l)=>{this.token=l.currentTarget.value}}
                          /></label>
                          <div class="row">
                            <button
                              type="submit"
                              ?disabled=${t.storage==="loading"||!a.trim()||!this.token.trim()||t.connection==="connecting"||t.connection==="connected"}
                            >
                              ${n.connect}
                            </button>
                            ${t.connection!=="offline"?J`
                                    <button
                                      type="button"
                                      @click=${()=>this.onaction({type:"disconnect"})}
                                    >
                                      ${n.disconnect}
                                    </button>
                                  `:K}
                          </div>
                        </form>
                      `}`}
      </section>
      <div
        class="notices"
        role="status"
        ?hidden=${!this.expanded||this.settingsOpen||t.storage==="ready"&&!t.syncing&&!t.message}
      >
        ${c}
      </div>
    `}}});function Xh(){return ad||(ad=[IDBDatabase,IDBObjectStore,IDBIndex,IDBCursor,IDBTransaction])}function Yh(){return sd||(sd=[IDBCursor.prototype.advance,IDBCursor.prototype.continue,IDBCursor.prototype.continuePrimaryKey])}function Qh(e){let t=new Promise((n,r)=>{let o=()=>{e.removeEventListener("success",i),e.removeEventListener("error",a)},i=()=>{n(Jt(e.result)),o()},a=()=>{r(e.error),o()};e.addEventListener("success",i),e.addEventListener("error",a)});return Fr.set(t,e),t}function em(e){if(Ai.has(e))return;let t=new Promise((n,r)=>{let o=()=>{e.removeEventListener("complete",i),e.removeEventListener("error",a),e.removeEventListener("abort",a)},i=()=>{n(),o()},a=()=>{r(e.error||new DOMException("AbortError","AbortError")),o()};e.addEventListener("complete",i),e.addEventListener("error",a),e.addEventListener("abort",a)});Ai.set(e,t)}function dd(e){Ei=e(Ei)}function tm(e){if(Yh().includes(e))return function(...t){return e.apply(Ti(this),t),Jt(this.request)};return function(...t){return Jt(e.apply(Ti(this),t))}}function nm(e){if(typeof e==="function")return tm(e);if(e instanceof IDBTransaction)em(e);if(Pi(e,Xh()))return new Proxy(e,Ei);return e}function Jt(e){if(e instanceof IDBRequest)return Qh(e);if(Ci.has(e))return Ci.get(e);let t=nm(e);if(t!==e)Ci.set(e,t),Fr.set(t,e);return t}function pd(e,t,{blocked:n,upgrade:r,blocking:o,terminated:i}={}){let a=indexedDB.open(e,t),s=Jt(a);if(r)a.addEventListener("upgradeneeded",(c)=>{r(Jt(a.result),c.oldVersion,c.newVersion,Jt(a.transaction),c)});if(n)a.addEventListener("blocked",(c)=>n(c.oldVersion,c.newVersion,c));return s.then((c)=>{if(i)c.addEventListener("close",()=>i());if(o)c.addEventListener("versionchange",(l)=>o(l.oldVersion,l.newVersion,l))}).catch(()=>{}),s}function cd(e,t){if(!(e instanceof IDBDatabase&&!(t in e)&&typeof t==="string"))return;if(zi.get(t))return zi.get(t);let n=t.replace(/FromIndex$/,""),r=t!==n,o=om.includes(n);if(!(n in(r?IDBIndex:IDBObjectStore).prototype)||!(o||rm.includes(n)))return;let i=async function(a,...s){let c=this.transaction(a,o?"readwrite":"readonly"),l=c.store;if(r)l=l.index(s.shift());return(await Promise.all([l[n](...s),o&&c.done]))[0]};return zi.set(t,i),i}async function*sm(...e){let t=this;if(!(t instanceof IDBCursor))t=await t.openCursor(...e);if(!t)return;t=t;let n=new Proxy(t,am);fd.set(n,t),Fr.set(n,Ti(t));while(t)yield n,t=await(Oi.get(n)||t.continue()),Oi.delete(n)}function ud(e,t){return t===Symbol.asyncIterator&&Pi(e,[IDBIndex,IDBObjectStore,IDBCursor])||t==="iterate"&&Pi(e,[IDBIndex,IDBObjectStore])}var Pi=(e,t)=>t.some((n)=>e instanceof n),ad,sd,Ai,Ci,Fr,Ei,Ti=(e)=>Fr.get(e),rm,om,zi,im,ld,Oi,fd,am;var hd=Se(()=>{Ai=new WeakMap,Ci=new WeakMap,Fr=new WeakMap;Ei={get(e,t,n){if(e instanceof IDBTransaction){if(t==="done")return Ai.get(e);if(t==="store")return n.objectStoreNames[1]?void 0:n.objectStore(n.objectStoreNames[0])}return Jt(e[t])},set(e,t,n){return e[t]=n,!0},has(e,t){if(e instanceof IDBTransaction&&(t==="done"||t==="store"))return!0;return t in e}};rm=["get","getKey","getAll","getAllKeys","count"],om=["put","add","delete","clear"],zi=new Map;dd((e)=>({...e,get:(t,n,r)=>cd(t,n)||e.get(t,n,r),has:(t,n)=>!!cd(t,n)||e.has(t,n)}));im=["continue","continuePrimaryKey","advance"],ld={},Oi=new WeakMap,fd=new WeakMap,am={get(e,t){if(!im.includes(t))return e[t];let n=ld[t];if(!n)n=ld[t]=function(...r){Oi.set(this,fd.get(this)[t](...r))};return n}};dd((e)=>({...e,get(t,n,r){if(ud(t,n))return sm;return e.get(t,n,r)},has(t,n){return ud(t,n)||e.has(t,n)}}))});async function md(e){if(e.length>65535||e.reduce((a,s)=>a+s.blob.size+76+2*new TextEncoder().encode(s.name).length,22)>4294967295)throw le("zipLimit");let t=[],n=[],r=0;for(let a of e){let s=new TextEncoder().encode(a.name),c=new Uint8Array(await a.blob.arrayBuffer()),l=4294967295;for(let f of c)l=l>>>8^cm[(l^f)&255];l=(l^4294967295)>>>0;let p=new Uint8Array(30+s.length),h=new DataView(p.buffer);h.setUint32(0,67324752,!0),h.setUint16(4,20,!0),h.setUint16(12,33,!0),h.setUint32(14,l,!0),h.setUint32(18,c.length,!0),h.setUint32(22,c.length,!0),h.setUint16(26,s.length,!0),p.set(s,30),t.push(p,c);let d=new Uint8Array(46+s.length),u=new DataView(d.buffer);u.setUint32(0,33639248,!0),u.setUint16(4,20,!0),u.setUint16(6,20,!0),u.setUint16(14,33,!0),u.setUint32(16,l,!0),u.setUint32(20,c.length,!0),u.setUint32(24,c.length,!0),u.setUint16(28,s.length,!0),u.setUint32(42,r,!0),d.set(s,46),n.push(d),r+=p.length+c.length}let o=new Uint8Array(22),i=new DataView(o.buffer);return i.setUint32(0,101010256,!0),i.setUint16(8,e.length,!0),i.setUint16(10,e.length,!0),i.setUint32(12,n.reduce((a,s)=>a+s.length,0),!0),i.setUint32(16,r,!0),new Blob([...t,...n,o],{type:"application/zip"})}var cm;var gd=Se(()=>{Fn();cm=Uint32Array.from({length:256},(e,t)=>{for(let n=0;n<8;n++)t=t>>>1^(t&1?3988292384:0);return t>>>0})});async function hm(e,t,n,r){r.throwIfAborted();let o=Bn(e,e.width,e.height,n),i=URL.createObjectURL(new Blob([t],{type:"image/svg+xml"})),a=new Image,s=AbortSignal.any([r,AbortSignal.timeout(1e4)]);try{a.src=i,await new Promise((l,p)=>{let h=()=>p(le("composeTimeout"));if(s.aborted){h();return}s.addEventListener("abort",h,{once:!0}),a.decode().then(l,p).finally(()=>s.removeEventListener("abort",h))}),r.throwIfAborted();let c=o.getContext("2d");if(!c)throw le("drawingUnavailable");if(n){let l=ji(n,e.width,e.height);c.drawImage(a,l.x,l.y,l.width,l.height,0,0,o.width,o.height)}else c.drawImage(a,0,0,o.width,o.height);return await Vi(o,r)}finally{a.src="",URL.revokeObjectURL(i),o.width=0,o.height=0}}function gm(e,t){t.throwIfAborted();let n=document.createElement("div");n.dataset.ainotationUi="drawing",n.dataset.theme=e,n.style.cssText="all:initial!important;position:fixed!important;inset:0!important;width:100%!important;height:100%!important;margin:0!important;border:0!important;padding:0!important;max-width:none!important;max-height:none!important;background:transparent!important;overflow:visible!important;pointer-events:none!important;z-index:2147483647!important;";let r=n.attachShadow({mode:"open"});n.popover="manual";let o=new WeakMap,i=0;function a(l,p){let h=[...l.querySelectorAll(p)].filter((d)=>d!==n);for(let d of l.querySelectorAll("*"))if(d!==n&&d.shadowRoot)h.push(...a(d.shadowRoot,p));return h}function s(){if(t.aborted)return;let l=a(document,"dialog:modal").at(-1),p=a(l??document,"[popover]:popover-open").sort((h,d)=>(o.get(h)??0)-(o.get(d)??0)).at(-1)??l??document.documentElement;if(n.parentElement!==p){if(n.matches(":popover-open"))n.hidePopover();p.append(n)}if(n.showPopover&&!n.matches(":popover-open"))n.showPopover()}let c=new MutationObserver((l)=>{if(!n.isConnected||l.some((p)=>p.type==="attributes"))s()});try{s(),t.throwIfAborted(),c.observe(document,{subtree:!0,childList:!0,attributes:!0,attributeFilter:["open"]})}catch(l){throw n.remove(),c.disconnect(),l}return window.addEventListener("beforetoggle",(l)=>{let p=l.composedPath()[0];if(!(p instanceof HTMLElement)||p===n||!p.hasAttribute("popover"))return;if(l.newState==="open")o.set(p,++i);queueMicrotask(s)},{capture:!0,signal:t}),t.addEventListener("abort",()=>{c.disconnect(),n.remove()},{once:!0}),{host:n,shadow:r,raise:s}}function ym(e){let{root:t,signal:n}=e,r=(o)=>o instanceof Element&&o.getRootNode()===t;for(let o of["pointerover","pointerout","mouseover","mouseout","pointerdown","pointermove","pointerup","pointercancel","mousedown","mousemove","mouseup","click","dblclick","contextmenu","touchstart","touchmove","touchend"])window.addEventListener(o,(i)=>{if(e.passthrough()||i instanceof MouseEvent&&i.altKey)return;let a=i.composedPath();if(a.find((c)=>r(c)&&(c.classList.contains("toolbar")||c.classList.contains("palette")))){if(!o.startsWith("touch"))i.preventDefault();i.stopImmediatePropagation();let c=a.find((l)=>r(l)&&l instanceof HTMLButtonElement);if(o==="pointermove"||o==="pointerover")e.hover(c instanceof HTMLButtonElement?c:null);else if(o==="pointerout"){let l=i.relatedTarget;e.hover(l&&r(l)?l.closest("[data-tooltip]"):null)}else if(o==="pointerdown")e.hover(null);else if(o==="click"&&c instanceof HTMLButtonElement)e.activate(c,i instanceof MouseEvent&&i.detail===0);return}let s=a.find((c)=>r(c)&&c instanceof SVGSVGElement&&c.classList.contains("drawing-surface"));if(!(s instanceof SVGSVGElement))return;if(i.preventDefault(),i.stopImmediatePropagation(),i instanceof PointerEvent&&["pointerdown","pointermove","pointerup","pointercancel"].includes(o))e.draw(o,i,a[0]instanceof Element?a[0]:s,s)},{capture:!0,passive:!1,signal:n})}function dn(e){let t=e.points.map((s)=>s.x),n=e.points.map((s)=>s.y),r=Math.min(...t),o=Math.min(...n),i=Math.max(...t),a=Math.max(...n);return{left:r,top:o,right:i,bottom:a,width:i-r,height:a-o,center:{x:(r+i)/2,y:(o+a)/2}}}function Br(e,t,n){let r=Math.cos(n),o=Math.sin(n),i=e.x-t.x,a=e.y-t.y;return{x:t.x+i*r-a*o,y:t.y+i*o+a*r}}function bd(e,t){return{...e,points:e.points.map((n)=>({x:n.x+t.x,y:n.y+t.y}))}}function bm(e,t,n,r){if(e.tool==="arrow")return{...e,points:[{x:e.points[0].x+n.x-t.x,y:e.points[0].y+n.y-t.y},e.points[1]]};let o=dn(e),i=e.rotation??0,a=Br({x:n.x-t.x,y:n.y-t.y},{x:0,y:0},-i),s=Math.max(r,o.width+a.x),c=Math.max(r,o.height+a.y),l={...e,points:e.points.map((u)=>({x:o.left+(o.width?(u.x-o.left)*s/o.width:0),y:o.top+(o.height?(u.y-o.top)*c/o.height:0)}))},p={x:o.left,y:o.top},h=Br(p,o.center,i),d=Br(p,dn(l).center,i);return bd(l,{x:h.x-d.x,y:h.y-d.y})}function vm(e,t,n){let{center:r}=dn(e),o=Math.atan2(n.y-r.y,n.x-r.x)-Math.atan2(t.y-r.y,t.x-r.x);return{...e,rotation:(e.rotation??0)+o}}function Mi(e,t){return{left:Math.min(e.x,t.x),top:Math.min(e.y,t.y),right:Math.max(e.x,t.x),bottom:Math.max(e.y,t.y)}}function vd(e){let t=dn(e),n=e.rotation??0,r=(e.strokeWidth??3)/2;if(e.tool==="ellipse"){let a=Math.hypot(t.width/2*Math.cos(n),t.height/2*Math.sin(n))+r,s=Math.hypot(t.width/2*Math.sin(n),t.height/2*Math.cos(n))+r;return{left:t.center.x-a,right:t.center.x+a,top:t.center.y-s,bottom:t.center.y+s}}let o=e.points;if(e.tool==="rectangle")o=[{x:t.left,y:t.top},{x:t.right,y:t.top},{x:t.right,y:t.bottom},{x:t.left,y:t.bottom}];else if(e.tool==="arrow"){let a=e.points[0],s=e.points.at(-1),c=Math.atan2(s.y-a.y,s.x-a.x);o=[...o,...[-0.45,0.45].map((l)=>({x:s.x-14*Math.cos(c+l),y:s.y-14*Math.sin(c+l)}))]}let i=o.map((a)=>Br(a,t.center,n));return{left:Math.min(...i.map((a)=>a.x))-r,right:Math.max(...i.map((a)=>a.x))+r,top:Math.min(...i.map((a)=>a.y))-r,bottom:Math.max(...i.map((a)=>a.y))+r}}function wm(e,t){return e.flatMap((n,r)=>{let o=vd(n);return o.right>=t.left&&o.left<=t.right&&o.bottom>=t.top&&o.top<=t.bottom?[r]:[]})}function $m(e){let t=`<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="-6 -6 40 40"><g transform="rotate(${Math.round((e*180/Math.PI+90)%360*10)/10} 14 14)" fill="none" stroke-linecap="round" stroke-linejoin="round"><path d="M5 7C18 6 22 12 21 23M11 2L5 7L10 13M15 18L21 23L26 17" stroke="white" stroke-width="5"/><path d="M5 7C18 6 22 12 21 23M11 2L5 7L10 13M15 18L21 23L26 17" stroke="black" stroke-width="3"/></g></svg>`;return`url("data:image/svg+xml,${encodeURIComponent(t)}") 10 10, crosshair`}async function wd(e){let t=new AbortController,n=AbortSignal.any([e.signal,t.signal]),r=e.i18n??Lr(),o=r.messages,i=(g)=>g==="select"?o.selectMove:g==="pen"?o.freeDraw:o[g],a=()=>[o.red,o.amber,o.green,o.blue,o.white,o.black],s=e.source,c="stream"in s?await _d(s.stream,n):null,l="blob"in s?await Fi(s.blob):null;if(n.aborted)c?.stop(),l?.close(),n.throwIfAborted();let p="blob"in s?URL.createObjectURL(s.blob):null,h;try{h=gm(e.theme,n)}catch(g){if(t.abort(),c?.stop(),l?.close(),p)URL.revokeObjectURL(p);throw g}let{host:d,shadow:u,raise:f}=h,x="arrow",k=zt[0],w=null,P=0,D=!1,T=()=>[...y].at(-1)??-1,Z=()=>H[T()]?.color??k,z=()=>H[T()]?.strokeWidth??ve;function W(g=!1){if(!w)return;let C=w,S=!!u.activeElement?.closest(".palette");if(w=null,q(),g&&(D||S))u.querySelector(`[data-action="${C}-palette"]`)?.focus({preventScroll:!0})}function j(g,C){if(w===g){W(C);return}if(w=g,D=C,P=g==="color"?zt.indexOf(Z()):un.indexOf(z()),me(null),q(),C)B()[P]?.focus({preventScroll:!0})}let B=()=>[...u.querySelectorAll(".palette button")];function ue(){let g=u.querySelector(".palette"),C=u.querySelector(`[data-action="${w}-palette"]`);if(!g||!C)return;let S=C.getBoundingClientRect(),F=g.getBoundingClientRect(),U=window.visualViewport,se=U?.offsetLeft??0,re=U?.offsetTop??0,be=U?.width??innerWidth;g.style.left=`${Math.max(se+8,Math.min(S.x+S.width/2-F.width/2,se+be-F.width-8))}px`,g.style.top=`${Math.max(re+8,S.top-F.height-8)}px`}let ve=3,H=[],Y=[],fe=[],y=new Set,N=null,ne=()=>l?{left:0,top:0,right:l.width,bottom:l.height}:{left:scrollX,top:scrollY,right:scrollX+innerWidth,bottom:scrollY+innerHeight},L=null,Q=!1,Ee=new Set,V=!1,pe=!1,ye="",ke=null;function me(g){if(V||pe||oe()||!g?.isConnected||w!==null&&g.dataset.action===`${w}-palette`)g=null;if(ke!==g)ke?.removeAttribute("aria-describedby");ke=g;let C=u.querySelector("#ainotation-drawing-tooltip");if(!C)return;if(C.hidden=!g,!g)return;g.setAttribute("aria-describedby","ainotation-drawing-tooltip"),C.textContent=g.dataset.tooltip??"";let S=g.getBoundingClientRect(),F=window.visualViewport,U=F?.offsetLeft??0,se=F?.offsetTop??0,re=F?.width??innerWidth,be=F?.height??innerHeight;C.style.maxWidth=`${Math.max(0,re-16)}px`;let Pe=C.getBoundingClientRect();C.style.left=`${Math.max(U+8,Math.min(S.x+S.width/2-Pe.width/2,U+re-Pe.width-8))}px`,C.style.top=`${Math.max(se+8,Math.min(S.top-Pe.height-8>=se+8?S.top-Pe.height-8:S.bottom+8,se+be-Pe.height-8))}px`}let Oe,oe=()=>!!c&&(Q||Ee.size>0),Ie=(g,C=[...y],S=N)=>{if(Y.push({shapes:g,selected:C,crop:S?{...S}:null}),Y.length>100)Y.shift();fe.length=0},G=(g)=>{let C=u.querySelector(".drawing-surface")?.getScreenCTM();if(!C)return{x:0,y:0};let S=new DOMPoint(g.clientX,g.clientY).matrixTransform(C.inverse());return{x:S.x,y:S.y}},ie=()=>{let g=u.querySelector(".drawing-surface")?.getScreenCTM();return g?1/Math.max(0.0001,Math.hypot(g.a,g.b)):1},de=(g)=>{let{center:C}=dn(g);return`rotate(${(g.rotation??0)*180/Math.PI} ${C.x} ${C.y})`};function he(){if(L){H=L.before,y=new Set(L.beforeSelected),N=L.beforeCrop;let g=u.querySelector(".drawing-surface");if(g?.hasPointerCapture(L.pointer))g.releasePointerCapture(L.pointer)}L=null}function ee(g,C,S){if(V||g.button!==0||L)return;me(null),W();let F=structuredClone(H),U=[...y],se=G(g),re=N?{...N}:null,be=C.closest("[data-control]")?.getAttribute("data-control"),Pe=Number(C.closest("[data-shape]")?.getAttribute("data-shape")??-1),Ne;if(x==="crop")y.clear(),Ne=N&&C.closest("[data-crop-resize]")?"crop-resize":N&&C.closest("[data-crop-move]")?"crop-move":"crop-new";else if((be==="resize"||be==="rotate")&&y.size===1&&H[T()])Ne=be;else if(C.closest("[data-group-selection]")&&y.size>1)Ne="move";else if(Pe>=0){if(g.shiftKey&&x==="select"){if(y.has(Pe)){y.delete(Pe),q();return}y.add(Pe)}else if(!y.has(Pe))y=new Set([Pe]);Ne="move"}else if(x==="select"){if(!g.shiftKey)y.clear();Ne="marquee"}else{if(H.length>=200){ye=we("shapesLimit"),q();return}H.push({tool:x,color:k,strokeWidth:ve,points:[se,se]}),y=new Set([H.length-1]),Ne="draw"}L={pointer:g.pointerId,start:se,current:se,before:F,beforeSelected:U,beforeCrop:re,additive:g.shiftKey,index:T(),mode:Ne},S.setPointerCapture(g.pointerId),q()}function $e(g){if(!L||L.pointer!==g.pointerId)return;let C=G(g);if(L.current=C,L.mode.startsWith("crop-")){let U=ne(),se={x:Math.max(U.left,Math.min(C.x,U.right)),y:Math.max(U.top,Math.min(C.y,U.bottom))},re=L.beforeCrop;if(L.mode==="crop-move"&&re){let be=Math.min(re.right-re.left,U.right-U.left),Pe=Math.min(re.bottom-re.top,U.bottom-U.top),Ne=Math.max(U.left,Math.min(re.left+C.x-L.start.x,U.right-be)),ot=Math.max(U.top,Math.min(re.top+C.y-L.start.y,U.bottom-Pe));N={left:Ne,top:ot,right:Ne+be,bottom:ot+Pe}}else if(L.mode==="crop-resize"&&re)N=Un({...re,right:Math.max(re.left+ie(),re.right+se.x-L.start.x),bottom:Math.max(re.top+ie(),re.bottom+se.y-L.start.y)},U);else N=Un(Mi(L.start,se),U);q();return}if(L.mode==="marquee"){let U=Math.hypot(C.x-L.start.x,C.y-L.start.y)>=3*ie();y=new Set([...L.additive?L.beforeSelected:[],...U?wm(H,Mi(L.start,C)):[]]),q();return}let S=H[L.index],F=L.before[L.index];if(L.mode==="move")for(let U of y)H[U]=bd(L.before[U],{x:C.x-L.start.x,y:C.y-L.start.y});else if(L.mode==="resize")H[L.index]=bm(F,L.start,C,2*ie());else if(L.mode==="rotate")H[L.index]=vm(F,L.start,C);else if(S.tool==="pen"){if(S.points.length<2000)S.points.push(C)}else S.points[1]=C;q()}function _(g){if(!L||L.pointer!==g.pointerId)return;if(L.mode==="marquee"||L.mode.startsWith("crop-"))$e(g);if(L.mode==="crop-new"&&N&&(N.right-N.left<3*ie()||N.bottom-N.top<3*ie()))N=L.beforeCrop;if(JSON.stringify(L.before)!==JSON.stringify(H)||JSON.stringify(L.beforeCrop)!==JSON.stringify(N))Ie(L.before,L.beforeSelected,L.beforeCrop);L=null,q()}function M(g,C){let S=g.points[0],F=g.points.at(-1),U=Math.min(S.x,F.x),se=Math.min(S.y,F.y),re=Math.abs(F.x-S.x),be=Math.abs(F.y-S.y),Pe=Math.atan2(F.y-S.y,F.x-S.x),Ne=(Ge)=>`${F.x-14*Math.cos(Pe+Ge)},${F.y-14*Math.sin(Pe+Ge)}`,ot=g.tool==="rectangle"?We`<rect x=${U} y=${se} width=${re} height=${be}/>`:g.tool==="ellipse"?We`<ellipse cx=${U+re/2} cy=${se+be/2} rx=${re/2} ry=${be/2}/>`:g.tool==="pen"?We`<polyline points=${g.points.map((Ge)=>`${Ge.x},${Ge.y}`).join(" ")}/>`:We`<path d=${`M${S.x},${S.y} L${F.x},${F.y} M${Ne(-0.45)} L${F.x},${F.y} L${Ne(0.45)}`}/>`;return We`<g data-shape=${C} data-selected=${y.has(C)} transform=${de(g)} stroke=${g.color} stroke-width=${g.strokeWidth??3} stroke-linecap="round" stroke-linejoin="round" fill="none" style="pointer-events:stroke;cursor:move">
      ${!V?We`<g stroke="transparent" stroke-width=${12*ie()}>${ot}</g>`:K}
      ${ot}
    </g>`}function R(){if(V||x==="crop"||L?.mode==="marquee")return K;if(y.size>1){let re=[...y].map((it)=>vd(H[it])),be=ie(),Pe=6*be,Ne=Math.min(...re.map((it)=>it.left))-Pe,ot=Math.min(...re.map((it)=>it.top))-Pe,Ge=Math.max(...re.map((it)=>it.right))+Pe,qr=Math.max(...re.map((it)=>it.bottom))+Pe;return We`<rect data-group-selection=${y.size} x=${Ne} y=${ot} width=${Ge-Ne} height=${qr-ot}
        fill="none" stroke="var(--ain-guide)" stroke-width=${be} stroke-dasharray=${`${4*be} ${3*be}`} pointer-events="stroke" style="cursor:move"/>`}let g=H[T()];if(!g||V)return K;let C=dn(g),S=g.tool==="arrow",F=S?g.points[0]:{x:C.right,y:C.bottom},U=ie(),se=Math.sin(2*((g.rotation??0)+Math.PI/4))>=0?"nwse-resize":"nesw-resize";return We`<g data-controls=${T()} transform=${de(g)}>
      ${!S?We`<circle data-control="rotate" cx=${F.x} cy=${F.y} r=${25*U} fill="transparent" pointer-events="all" class="rotate-control"><title>${o.rotateHint}</title></circle>`:K}
      <circle data-control="resize" cx=${F.x} cy=${F.y} r=${12*U} fill="transparent" pointer-events="all" style=${`cursor:${S?"crosshair":se}`}><title>${S?o.arrowTailHint:o.resizeHint}</title></circle>
      <rect class="control-handle" x=${F.x-3*U} y=${F.y-3*U} width=${6*U} height=${6*U} fill="var(--ain-handle)" stroke="var(--ain-guide)" stroke-width=${U} pointer-events="none"/>
    </g>`}function X(){if(V||L?.mode!=="marquee")return K;let g=Mi(L.start,L.current),C=ie();return We`<rect data-marquee x=${g.left} y=${g.top} width=${g.right-g.left} height=${g.bottom-g.top}
      fill="var(--ain-guide-fill)" stroke="var(--ain-guide-focus)" stroke-width=${C} stroke-dasharray=${`${4*C} ${3*C}`} pointer-events="none"/>`}function ge(){if(V||!N)return K;let g=ne(),C=Un(N,g);if(!C)return K;let S=ie(),F=C.right-C.left,U=C.bottom-C.top;return We`<g data-crop-guide pointer-events="none">
      <path d=${`M${g.left} ${g.top}H${g.right}V${g.bottom}H${g.left}Z M${C.left} ${C.top}V${C.bottom}H${C.right}V${C.top}Z`} fill="var(--ain-crop-shade)" fill-rule="evenodd"/>
      <rect data-crop-area data-crop-move x=${C.left} y=${C.top} width=${F} height=${U} fill="transparent" stroke="var(--ain-crop-edge)" stroke-width=${S} stroke-dasharray=${`${5*S} ${3*S}`} pointer-events=${x==="crop"?"all":"none"} style="cursor:move"/>
      ${x==="crop"?We`<rect data-crop-resize x=${C.right-12*S} y=${C.bottom-12*S} width=${24*S} height=${24*S} fill="transparent" pointer-events="all" style="cursor:nwse-resize"/>
        <rect x=${C.right-3*S} y=${C.bottom-3*S} width=${6*S} height=${6*S} fill="var(--ain-handle)" stroke="var(--ain-guide)" stroke-width=${S}/>`:K}
    </g>`}function q(){if(pe)return;if(o=r.messages,d.lang!==r.locale)d.lang=r.locale;let g=oe();if(V||g)w=null;let C=l?`0 0 ${l.width} ${l.height}`:`${scrollX} ${scrollY} ${innerWidth} ${innerHeight}`;jt(J` <style>
          ${mm}
        </style>
        <div class="editor" role="region" aria-label=${o.imageEditor}>
          <div class=${`stage${l?" import":""}${g?" pass":""}`}>
            <div
              class=${l?"image-stage":""}
              style=${l?`width:min(${l.width}px,calc((100vh - 130px) * ${l.width/l.height}));aspect-ratio:${l.width}/${l.height}`:""}
            >
              ${p?J`<img src=${p} alt=${o.imageToAnnotate} />`:K}
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class=${`drawing-surface${x==="select"?" select":""}${L?.mode==="rotate"?" rotating":""}`}
                style=${`--rotation-cursor:${$m(H[T()]?.rotation??0)}`}
                viewBox=${C}
                aria-label=${o.drawingSurface}
              >
                ${H.map(M)} ${R()} ${X()} ${ge()}
              </svg>
            </div>
          </div>
          ${ye&&!V?J`<div class="notice" role="status">${mt(r.locale,ye)}</div>`:K}
          <div
            class=${`toolbar${g?" pass":""}`}
            role="toolbar"
            aria-label=${o.drawingTools}
            ?hidden=${V}
          >
            ${Object.entries(yd).map(([S,F])=>J`<button
                  aria-label=${o[S]}
                  data-tooltip=${o.shortcut(i(S),Ur[S].toUpperCase())}
                  aria-keyshortcuts=${Ur[S]}
                  aria-pressed=${x===S}
                  data-tool=${S}
                >
                  ${qt(F)}
                </button>`)}
            <span class="separator"></span>
            <button
              class="color-trigger"
              aria-label=${o.color}
              data-tooltip=${o.colorCycle(a()[zt.indexOf(Z())])}
              aria-keyshortcuts="c"
              aria-haspopup="dialog"
              aria-expanded=${w==="color"}
              aria-controls="ainotation-color-palette"
              data-action="color-palette"
            >
              <span class="swatch" style=${`background:${Z()}`} aria-hidden="true"></span>
            </button>
            <span class="separator"></span>
            <button
              class="stroke-width"
              aria-label=${o.lineWidth}
              data-tooltip=${o.widthCycle}
              aria-keyshortcuts="s"
              data-action="width-palette"
              aria-haspopup="listbox"
              aria-expanded=${w==="width"}
              aria-controls="ainotation-width-palette"
            >
              <span>${z()} px</span>${qt(Di)}
            </button>
            <button
              aria-label=${o.undo}
              data-tooltip=${o.shortcut(o.undo,"Command / Ctrl + Z")}
              aria-keyshortcuts="Meta+z Control+z"
              ?disabled=${!Y.length}
              data-action="undo"
            >
              ${qt(Zi)}
            </button>
            <button
              aria-label=${o.redo}
              data-tooltip=${o.shortcut(o.redo,"Command / Ctrl + Shift + Z")}
              aria-keyshortcuts="Meta+Shift+z Control+Shift+z"
              ?disabled=${!fe.length}
              data-action="redo"
            >
              ${qt(Ri)}
            </button>
            <button
              aria-label=${x==="crop"?o.clearCrop:o.deleteShape}
              data-tooltip=${o.shortcut(x==="crop"?o.clearCrop:y.size>1?o.deleteShapes(y.size):o.deleteShape,"D")}
              aria-keyshortcuts="d"
              ?disabled=${x==="crop"?!N:y.size===0}
              data-action="delete"
            >
              ${qt(Vt)}
            </button>
            <span class="separator"></span>
            <button
              aria-label=${o.cancelDrawing}
              data-tooltip=${o.shortcut(o.cancelDrawing,"Esc")}
              aria-keyshortcuts="Escape"
              data-action="cancel"
            >
              ${qt(an)}
            </button>
            <button
              class="save"
              aria-label=${c?o.captureAttach:o.attachImage}
              data-tooltip=${c?o.captureHint:o.shortcut(o.attachImage,"Command / Ctrl + Enter")}
              aria-keyshortcuts="Meta+Enter Control+Enter"
              data-action="save"
            >
              ${qt(Ni)}
            </button>
          </div>
          ${w==="color"?J`<div
                  id="ainotation-color-palette"
                  class="palette color-popover"
                  role="dialog"
                  aria-label=${o.colorPalette}
                >
                  ${zt.map((S,F)=>J`<button
                      class="swatch"
                      style=${`background:${S}`}
                      aria-label=${o.colorValue(S)}
                      data-tooltip=${a()[F]}
                      aria-pressed=${Z()===S}
                      data-color=${S}
                      ?data-highlighted=${P===F}
                    ></button>`)}
                </div>`:K}
          ${w==="width"?J`<div
                  id="ainotation-width-palette"
                  class="palette width-popover"
                  role="listbox"
                  aria-label=${o.lineWidths}
                >
                  ${un.map((S,F)=>J`<button
                      role="option"
                      aria-label=${`${S} px`}
                      aria-selected=${z()===S}
                      data-width=${S}
                      data-tooltip=${`${S} px`}
                      ?data-highlighted=${P===F}
                    >
                      <span
                        class="width-preview"
                        style=${`height:${S}px`}
                        aria-hidden="true"
                      ></span
                      ><span>${S} px</span>
                    </button>`)}
                </div>`:K}
          <div id="ainotation-drawing-tooltip" class="tooltip" role="tooltip" hidden></div>
        </div>`,u),ue(),me(ke)}function ce(g){if(V||L)return;W(),x=g,y.clear(),q()}function Te(g){if(V||L)return;if(k=g,[...y].some((C)=>H[C].color!==g)){Ie(structuredClone(H));for(let C of y)H[C].color=g}W(!0),q()}function te(g){if(V||L||!un.includes(g))return;if(ve=g,[...y].some((C)=>(H[C].strokeWidth??3)!==g)){Ie(structuredClone(H));for(let C of y)H[C].strokeWidth=g}W(!0),q()}function Ce(g,C){if(V||L||g.disabled)return;let S=g.dataset;if(S.tool&&Object.hasOwn(yd,S.tool)){ce(S.tool);return}if(S.color&&zt.includes(S.color)){Te(S.color);return}if(S.width){te(Number(S.width));return}if(S.action==="color-palette"){j("color",C);return}if(S.action==="width-palette"){j("width",C);return}if(W(),S.action==="undo")He();else if(S.action==="redo")b();else if(S.action==="delete")A();else if(S.action==="cancel")v();else if(S.action==="save")m()}function He(){if(!Y.length||V)return;fe.push({shapes:H,selected:[...y],crop:N});let g=Y.pop();H=g.shapes,y=new Set(g.selected),N=g.crop,q()}function b(){if(!fe.length||V)return;Y.push({shapes:H,selected:[...y],crop:N});let g=fe.pop();H=g.shapes,y=new Set(g.selected),N=g.crop,q()}function A(){if(x==="crop"&&N&&!V){Ie(structuredClone(H)),N=null,q();return}if(!y.size||V)return;Ie(structuredClone(H)),H=H.filter((g,C)=>!y.has(C)),y.clear(),q()}async function m(){if(V||pe)return;he(),V=!0,q();try{let g,C=N?xd(N,ne()):void 0;if(c)g=await c.snapshot(C);else{let S=u.querySelector(".drawing-surface").cloneNode(!0);S.setAttribute("width",String(l.width)),S.setAttribute("height",String(l.height)),g=await hm(l,new XMLSerializer().serializeToString(S),C,n)}n.throwIfAborted(),await e.onSave(g),v()}catch(g){if(!pe)V=!1,ye=sn(g,"imageAttachFailed"),q()}}function v(){if(pe)return;if(pe=!0,clearTimeout(Oe),t.abort(),c?.stop(),l?.close(),p)URL.revokeObjectURL(p);jt(K,u),d.remove(),e.onClose()}n.addEventListener("abort",v,{once:!0}),ym({root:u,signal:n,passthrough:oe,hover:me,activate:Ce,draw(g,C,S,F){if(g==="pointerdown")ee(C,S,F);else if(g==="pointermove")$e(C);else if(g==="pointerup")_(C);else he(),q()}});let I=(g)=>g instanceof Element?g.closest("[data-tooltip]"):null;for(let g of["pointerover","pointermove","focusin"])u.addEventListener(g,(C)=>me(I(C.target)),{signal:n});u.addEventListener("focusin",(g)=>{let C=g.target;if(w&&C instanceof Element&&!C.closest(".palette")&&C.getAttribute("data-action")!==`${w}-palette`)W()},{signal:n}),u.addEventListener("pointerout",(g)=>me(I(g.relatedTarget)),{signal:n}),u.addEventListener("focusout",(g)=>me(I(g.relatedTarget)),{signal:n}),u.addEventListener("pointerdown",()=>me(null),{capture:!0,signal:n}),document.addEventListener("pointerdown",(g)=>{if(w&&!g.composedPath().some((C)=>C instanceof Element&&(C.classList.contains("palette")||C.getAttribute("data-action")?.endsWith("-palette"))))W()},{capture:!0,signal:n}),window.addEventListener("blur",()=>me(null),{signal:n});for(let g of["click","dblclick","pointerdown","pointerup"])u.addEventListener(g,(C)=>C.stopPropagation(),{signal:n});document.addEventListener("keydown",(g)=>{if(g.isComposing||g.defaultPrevented)return;if(g.key==="Alt"){Q=!0,w=null,he(),q();return}if(g.key==="Escape"){if(g.preventDefault(),g.stopImmediatePropagation(),w)W(!0);else if(!V)v();return}if((g.metaKey||g.ctrlKey)&&g.key==="Enter"){if(g.preventDefault(),g.stopImmediatePropagation(),!g.repeat)m();return}let C=g.composedPath().some((U)=>U instanceof HTMLElement&&(U.isContentEditable||["INPUT","TEXTAREA","SELECT"].includes(U.tagName)));if(V||L||oe()||g.altKey||C)return;if(w&&!g.metaKey&&!g.ctrlKey){let U=B(),se={ArrowRight:1,ArrowDown:1,ArrowLeft:-1,ArrowUp:-1}[g.key];if(se!==void 0||g.key==="Home"||g.key==="End"){if(g.preventDefault(),g.stopImmediatePropagation(),P=g.key==="Home"?0:g.key==="End"?U.length-1:(P+(se??0)+U.length)%U.length,q(),D)B()[P]?.focus({preventScroll:!0});return}if(g.key==="Enter"||g.key===" "){g.preventDefault(),g.stopImmediatePropagation();let re=U[P];if(!g.repeat&&re)Ce(re,D);return}}if((g.metaKey||g.ctrlKey)&&g.key.toLowerCase()==="z"){if(g.preventDefault(),g.stopImmediatePropagation(),!g.repeat)if(g.shiftKey)b();else He();return}if(g.metaKey||g.ctrlKey||g.shiftKey)return;if(g.key.toLowerCase()==="d"){if(g.preventDefault(),g.stopImmediatePropagation(),!g.repeat)A();return}let S=g.key.toLowerCase(),F=Object.keys(Ur).find((U)=>Ur[U]===S);if(!F&&S!=="c"&&S!=="s")return;if(g.preventDefault(),g.stopImmediatePropagation(),g.repeat)return;if(F)ce(F);else if(S==="c")Te(zt[(zt.indexOf(Z())+1)%zt.length]);else{let U=z();te(un[(un.indexOf(U)+1)%un.length])}},{capture:!0,signal:n}),document.addEventListener("keyup",(g)=>{if(g.key==="Alt"||Q&&!g.altKey){if(Q=!1,d.matches(":popover-open"))d.hidePopover();f(),q()}},{capture:!0,signal:n}),document.addEventListener("pointerdown",(g)=>{if(c&&(g.altKey||Q))Ee.add(g.pointerId),q()},{capture:!0,signal:n});let E=(g)=>{if(Ee.delete(g.pointerId))clearTimeout(Oe),Oe=setTimeout(q,0)};if(document.addEventListener("pointerup",E,{capture:!0,signal:n}),document.addEventListener("pointercancel",E,{capture:!0,signal:n}),window.addEventListener("blur",()=>{Q=!1,Ee.clear(),he(),q()},{signal:n}),window.addEventListener("resize",q,{signal:n}),document.addEventListener("scroll",q,{capture:!0,signal:n}),"stream"in s)s.stream.getVideoTracks()[0]?.addEventListener("ended",()=>{ye=we("captureEnded"),q()},{signal:n});return r.subscribe(q,n),q(),{close:v}}var lm,um,dm,pm,fm,mm,yd,zt,un,Ur,qt=(e)=>Fe(e,{width:18,height:18,"aria-hidden":"true",focusable:"false"});var $d=Se(()=>{Fn();Ui();lm=[["path",{d:"M7 7h10v10"}],["path",{d:"M7 17 17 7"}]],um=[["circle",{cx:"12",cy:"12",r:"10"}]],dm=[["path",{d:"M6 2v14a2 2 0 0 0 2 2h14"}],["path",{d:"M18 22V8a2 2 0 0 0-2-2H2"}]],pm=[["path",{d:"M4.037 4.688a.495.495 0 0 1 .651-.651l16 6.5a.5.5 0 0 1-.063.947l-6.124 1.58a2 2 0 0 0-1.438 1.435l-1.579 6.126a.5.5 0 0 1-.947.063z"}]],fm=[["rect",{width:"18",height:"18",x:"3",y:"3",rx:"2"}]];mm=wt`
  ${Ft}
  * {
    box-sizing: border-box;
  }
  :host::backdrop {
    background: transparent;
    pointer-events: none;
  }
  .editor {
    font:
      13px/1.4 system-ui,
      sans-serif;
    color: var(--ain-text);
    color-scheme: var(--ain-scheme);
  }
  .stage {
    position: fixed;
    inset: 0;
    pointer-events: auto;
    touch-action: none;
  }
  .import {
    background: var(--ain-surface-muted);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 20px 16px 110px;
  }
  .image-stage {
    position: relative;
    max-width: 100%;
    max-height: 100%;
  }
  img {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: contain;
    pointer-events: none;
  }
  .drawing-surface {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    touch-action: none;
    cursor: crosshair;
  }
  .select {
    cursor: default;
  }
  .rotate-control {
    cursor: var(--rotation-cursor);
  }
  .rotating,
  .rotating * {
    cursor: var(--rotation-cursor) !important;
  }
  .pass,
  .pass * {
    pointer-events: none !important;
  }
  .toolbar {
    position: fixed;
    bottom: 16px;
    left: 50%;
    transform: translateX(-50%);
    max-width: calc(100vw - 24px);
    display: flex;
    gap: 5px;
    padding: 8px;
    border: 1px solid var(--ain-border);
    border-radius: 10px;
    background: var(--ain-surface);
    box-shadow: 0 4px 20px var(--ain-shadow);
    pointer-events: auto;
    overflow-x: auto;
    align-items: center;
  }
  @media (max-width: 650px) {
    .toolbar {
      width: calc(100vw - 24px);
      flex-wrap: wrap;
      justify-content: center;
    }
    .notice {
      bottom: 116px !important;
    }
    .import {
      padding-bottom: 150px;
    }
  }
  button {
    flex-shrink: 0;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    border: 1px solid transparent;
    border-radius: 5px;
    background: transparent;
    color: var(--ain-text);
    cursor: pointer;
  }
  button:hover {
    background: var(--ain-hover);
  }
  button[aria-pressed='true'] {
    border-color: var(--ain-focus);
    background: var(--ain-selected);
  }
  button:focus-visible {
    outline: 2px solid var(--ain-focus);
  }
  .stroke-width {
    flex-shrink: 0;
    height: 32px;
    width: 68px;
    padding: 0 4px;
    font: inherit;
    color: var(--ain-text);
    background: var(--ain-surface);
    border: 1px solid var(--ain-border);
    border-radius: 5px;
    cursor: pointer;
    gap: 5px;
  }
  .stroke-width svg {
    width: 12px;
    height: 12px;
  }
  .stroke-width:hover {
    background: var(--ain-hover);
  }
  .stroke-width:focus-visible {
    outline: 2px solid var(--ain-focus);
  }
  button:disabled {
    opacity: 0.4;
    cursor: default;
  }
  .swatch {
    width: 22px;
    height: 22px;
    border-radius: 50%;
    border: 2px solid var(--ain-border);
    margin: 0 1px;
  }
  .swatch[aria-pressed='true'] {
    outline: 2px solid var(--ain-focus);
    outline-offset: 1px;
  }
  .color-trigger .swatch {
    display: block;
  }
  .palette {
    position: fixed;
    display: flex;
    gap: 8px;
    padding: 10px;
    border: 1px solid var(--ain-border);
    border-radius: 8px;
    background: var(--ain-surface);
    box-shadow: 0 4px 20px var(--ain-shadow);
    pointer-events: auto;
    max-height: calc(100vh - 32px);
    overflow: auto;
  }
  .width-popover {
    flex-direction: column;
    gap: 2px;
    padding: 6px;
  }
  .width-popover button {
    width: 100px;
    gap: 10px;
    padding: 0 8px;
    justify-content: space-between;
    font: inherit;
  }
  .width-popover button[aria-selected='true'] {
    background: var(--ain-selected);
  }
  .width-preview {
    display: block;
    width: 28px;
    background: currentColor;
    border-radius: 3px;
  }
  .palette [data-highlighted] {
    outline: 2px solid var(--ain-focus);
    outline-offset: 1px;
  }
  .separator {
    width: 1px;
    height: 24px;
    background: var(--ain-border);
    flex-shrink: 0;
    margin: 0 3px;
  }
  .save {
    background: var(--ain-accent);
    color: var(--ain-on-accent);
  }
  .save:hover:not(:disabled) {
    background: var(--ain-accent-hover);
  }
  .notice {
    position: fixed;
    bottom: 76px;
    left: 50%;
    transform: translateX(-50%);
    max-width: calc(100vw - 32px);
    padding: 5px 9px;
    background: var(--ain-surface);
    border-radius: 5px;
    text-align: center;
    pointer-events: none;
  }
  .tooltip {
    position: fixed;
    z-index: 1;
    width: max-content;
    max-width: calc(100vw - 16px);
    padding: 6px 9px;
    border-radius: 5px;
    background: var(--ain-tooltip);
    color: var(--ain-on-tooltip);
    font-size: 12px;
    line-height: 1.4;
    text-align: center;
    overflow-wrap: anywhere;
    box-shadow: 0 2px 8px var(--ain-shadow);
    pointer-events: none;
  }
  [hidden] {
    display: none !important;
  }
`;yd={select:pm,arrow:lm,rectangle:fm,ellipse:um,pen:Li,crop:dm},zt=["#ef4444","#f59e0b","#22c55e","#3b82f6","#ffffff","#111827"],un=[0.5,1,1.5,2,3,4,5],Ur={select:"v",arrow:"a",rectangle:"r",ellipse:"e",pen:"f",crop:"x"}});function Em(e){let t=new Set,n=(l)=>Rr(l,e.exclude),r=(l)=>l.type==="attributes"&&l.attributeName==="data-ainotation-ui",o=new MutationObserver((l)=>{let p=l.some((d)=>d.type==="childList"||r(d))&&[...t].some((d)=>d instanceof ShadowRoot&&(!d.host.isConnected||n(d.host))),h=l.filter((d)=>r(d)||!Bt(d.target)&&(d.type!=="childList"||[...d.addedNodes,...d.removedNodes].some((u)=>!Bt(u))));if(!h.length&&!p)return;if(h.some((d)=>d.type==="childList"?[...d.addedNodes,...d.removedNodes].some((u)=>u instanceof Element):d.type==="attributes"&&(d.attributeName==="data-ainotation-ui"||!!e.exclude))||p)c();else for(let d of h)if(d.target instanceof Element)i(d.target);e.onChange()});function i(l){if(l.shadowRoot&&!n(l))a(l.shadowRoot)}function a(l){if(t.has(l))return;t.add(l),o.observe(l,{childList:!0,subtree:!0,attributes:!0,characterData:!0});for(let p of l.querySelectorAll("*"))i(p)}function s(){o.disconnect(),t.clear()}function c(){s(),a(document)}return{refresh:c,disconnect:s}}function Wt(e){return e.length>0&&e.length<=100&&!/^(?:css-|sc-|emotion-|makeStyles-|:r|radix-|headlessui-)/i.test(e)&&!/^(?:(?:is|has)[-_])?(?:active|selected|open|closed|disabled|enabled|hidden|visible|focus|focused|hover|loading|checked)$/i.test(e)&&!/[a-f0-9]{8,}|\d{4,}/i.test(e)&&/[a-z\p{L}]/iu.test(e)}function kd(e){let t=CSS.escape(e.localName),n=[],r=e.getAttribute("id");if(r&&Wt(r))n.push(`#${CSS.escape(r)}`);for(let i of["data-testid","data-element","name","aria-label","alt","title"]){let a=e.getAttribute(i);if(a&&a.length<=100&&!/[\r\n]/.test(a))n.push(`${i.startsWith("data-")?"":t}${Tm(i,a)}`)}let o=Array.from(e.classList).filter(Wt).slice(0,5);for(let i of o)n.push(`.${CSS.escape(i)}`);for(let i of o)n.push(`${t}.${CSS.escape(i)}`);for(let i=0;i<o.length;i++)for(let a=i+1;a<o.length;a++)n.push(`.${CSS.escape(o[i])}.${CSS.escape(o[a])}`);if(r&&!Wt(r)&&r.length<=256)n.push(`#${CSS.escape(r)}`);return n.push(t),[...new Set(n)].slice(0,24)}function Sd(e,t){let n=0,r=(p)=>{if(p.length>4000)return!1;try{let h=t.querySelectorAll(p);return h.length===1&&h[0]===e}catch{return!1}},o=(p)=>++n<=256&&r(p),i=kd(e);for(let p of i)if(o(p))return p;let a=e.parentElement,s;for(let p=0;a&&p<8&&n<192;p++,a=a.parentElement){let h=kd(a).slice(0,6);for(let d of h)for(let u of i.slice(0,8)){if(n>=192)break;let f=`${d} ${u}`;if(o(f)){let x=f.length+(!/[.#[]/.test(d)?20:0)+(!/[.#[]/.test(u)?20:0);if(!s||x<s.cost)s={selector:f,cost:x}}}}if(s)return s.selector;let c=[];for(let p=e;p;p=p.parentElement){let h=Array.from(p.parentNode.children).filter((f)=>f.localName===p.localName&&f.namespaceURI===p.namespaceURI);c.unshift(`${CSS.escape(p.localName)}${h.length>1?`:nth-of-type(${h.indexOf(p)+1})`:""}`);let d=c.join(" > ");if(o(d))return d;let u=p.parentElement?.getAttribute("id");if(u&&Wt(u)&&o(`#${CSS.escape(u)} > ${d}`))return`#${CSS.escape(u)} > ${d}`}let l=c.join(" > ");if(!r(l))throw Error("Cannot create a unique target selector within the snapshot limits.");return l}function Vd(e){let t=e.localName==="img"?e.getAttribute("src"):null,n=t&&t.length<=1000&&!/[?#]/.test(t)&&!/^(?:data|blob):/i.test(t)?["src"]:[];return Object.fromEntries([...Om,...n].flatMap((r)=>{let o=e.getAttribute(r);return o===null?[]:[[r,o.slice(0,1000)]]}))}function Mm(e,t,n){let r=(k,w)=>({tagName:k.localName.slice(0,100),attributes:Object.fromEntries(["id","class","role","aria-label"].flatMap((P)=>{let D=k.getAttribute(P);return D===null?[]:[[P,D.slice(0,256)]]})),...w?{text:n(k).slice(0,160)}:{},...k.shadowRoot?{shadowHost:!0}:{}}),o=[],i=Qe(e);while(i&&o.length<32&&!t(i))o.unshift(r(i,!1)),i=Qe(i);let a=(k)=>{let w=getComputedStyle(k);return!t(k)&&k.getClientRects().length>0&&w.opacity!=="0"&&w.visibility!=="hidden"&&w.visibility!=="collapse"},s=e.parentNode,c=s instanceof Element||s instanceof ShadowRoot?Array.from(s.children):[],l=c.indexOf(e),p,h,d=[];for(let k=1;k<=Math.min(64,c.length);k++){let w=c[l-k],P=c[l+k];if(w&&a(w)){if(p??=w,d.length<4)d.push(w)}if(P&&a(P)){if(h??=P,d.length<4)d.push(P)}if(d.length===4&&p&&h)break}let u=Vd(e),f=u["data-element"]||u["aria-label"]||u.alt||u.placeholder||n(e).slice(0,80)||u.name||u.href||"",x=!Ii(e,"[inert]")&&!e.matches(':disabled, input[type="hidden"]')&&a(e)&&e.matches('a[href], area[href], button, input, select, textarea, summary, iframe, [tabindex], [contenteditable=""], [contenteditable="true"]');return{label:`${e.localName}${f?` ${JSON.stringify(f)}`:Array.from(e.classList).filter(Wt).slice(0,2).map((k)=>`.${k}`).join("")}`.slice(0,160),ancestors:o,ancestryTruncated:!!i,nearbyText:{before:p?n(p).slice(0,160):"",after:h?n(h).slice(0,160):""},nearbyElements:d.map((k)=>r(k,!0)),siblingCount:Math.max(0,c.length-1),accessibility:{focusable:x}}}function Fd(e,t){for(let n=e.parentElement;n;){if(t(n)||n.matches("input, textarea, select, script, style, noscript, [contenteditable]"))return!1;let r=getComputedStyle(n);if(r.display==="none"||r.opacity==="0"||r.visibility==="hidden"||r.visibility==="collapse")return!1;n=Qe(n)}return!0}function Bi(e,t,n,r=!1){let{element:o,shadowRoots:i}=Zr(e,t);if(!o||n(o)||Ii(o,"button, input, textarea, select, [contenteditable]"))return null;let a=document,s=a.caretPositionFromPoint?.(e,t,{shadowRoots:i}),c=s?null:a.caretRangeFromPoint?.(e,t),l=s?.offsetNode??c?.startContainer,p=s?.offset??c?.startOffset??0;if(!(l instanceof Text)||!Fd(l,n))return null;if(r){let h=document.createRange(),d=Math.max(0,Math.min(p,l.length-1));if(h.setStart(l,d),h.setEnd(l,Math.min(d+1,l.length)),![...h.getClientRects()].some((u)=>e>=u.left-3&&e<=u.right+3&&t>=u.top&&t<=u.bottom))return null}return{node:l,offset:p}}function Id(e,t){if(!e.node.isConnected||!t.node.isConnected||e.offset<0||e.offset>e.node.length||t.offset<0||t.offset>t.node.length||e.node.getRootNode()!==t.node.getRootNode())return null;let n=document.createRange();n.setStart(e.node,e.offset),n.collapse(!0);let r=document.createRange();r.setStart(t.node,t.offset),r.collapse(!0);let o=n.compareBoundaryPoints(Range.START_TO_START,r)>0,i=document.createRange();return i.setStart(o?t.node:e.node,o?t.offset:e.offset),i.setEnd(o?e.node:t.node,o?e.offset:t.offset),i}function Hi(e,t,n,r=!1){let o=e.commonAncestorContainer,i=document.createTreeWalker(o,NodeFilter.SHOW_TEXT),a=o instanceof Text?o:i.nextNode(),s="",c=0;while(a&&c++<4096){if(a instanceof Text&&e.intersectsNode(a)&&Fd(a,t)){let l=a===e.startContainer?e.startOffset:0,p=a===e.endContainer?e.endOffset:a.length;if(s+=a.data.slice(l,p),r)s=s.slice(-n);else if(s.length>n)return{text:s.slice(0,n),truncated:!0}}a=i.nextNode()}return{text:s,truncated:!!a}}function Nm(e,t){let n=e.commonAncestorContainer,r=n instanceof Element?n:n.parentElement;if(!r||t(r)||e.collapsed)return null;let o=Hi(e,t,1000);if(!o.text)return null;let i=r.parentElement&&!t(r.parentElement)?r.parentElement:r,a=document.createRange();a.selectNodeContents(i),a.setEnd(e.startContainer,e.startOffset);let s=document.createRange();return s.selectNodeContents(i),s.setStart(e.endContainer,e.endOffset),{element:r,selection:{exact:o.text,prefix:Hi(a,t,64,!0).text,suffix:Hi(s,t,64).text,truncated:o.truncated,rects:[...e.getClientRects()].filter((c)=>c.width>0&&c.height>0).slice(0,32).map(({x:c,y:l,width:p,height:h})=>({x:c,y:l,width:p,height:h}))}}}function Cd(e,t){try{return Array.from(e.querySelectorAll(t))}catch{return[]}}function Ji(e,t){let n=document.createTreeWalker(e,NodeFilter.SHOW_TEXT),r="",o=0;for(let i=n.nextNode();i&&o++<4096;i=n.nextNode()){let a=!0;for(let s=i.parentElement;s;s=Qe(s)){if(t(s)||/^(script|style|textarea|input|noscript)$/i.test(s.localName)){a=!1;break}let c=getComputedStyle(s);if(c.display==="none"||c.opacity==="0"||c.visibility==="hidden"||c.visibility==="collapse"){a=!1;break}}if(a)r=`${r} ${i.textContent??""}`.replace(/\s+/g," ").trimStart();if(r.length>=500)break}return r.trim().slice(0,500)}function zd(e){let{x:t,y:n,width:r,height:o}=e.getBoundingClientRect(),i=getComputedStyle(e);return{rect:{x:t,y:n,width:r,height:o},styles:Object.fromEntries(Lm.map((a)=>[a,i.getPropertyValue(a).slice(0,1000)]))}}function fn(){return{url:location.href.slice(0,8000),title:document.title.slice(0,1000),userAgent:navigator.userAgent.slice(0,1000),capturedAt:new Date().toISOString(),viewport:{width:Math.max(1,window.innerWidth||document.documentElement.clientWidth),height:Math.max(1,window.innerHeight||document.documentElement.clientHeight),devicePixelRatio:window.devicePixelRatio>0?window.devicePixelRatio:1,scrollX:window.scrollX,scrollY:window.scrollY}}}function Rm(e){let t=e.visible??!0,n=[],r=new WeakMap,o=new Map,i=new Map,a=new Map,s=!1,c=!1,l=!1,p=!1,h,d=!1,u=null,f=null,x=null,k=null,w=null,P=!1,D=!1,T=null,Z=0,z=new AbortController,W=(b)=>Rr(b,e.exclude),j=(b)=>b.isConnected&&b.ownerDocument===document&&!W(b),B=document.createElement("div");if(B.setAttribute("data-ainotation-ui","selection"),B.dataset.theme=e.appearance?.theme??"light",B.style.cssText="all:initial!important;position:fixed!important;inset:0!important;pointer-events:none!important;z-index:2147483646!important;",!t)B.style.setProperty("display","none","important");let ue=B.attachShadow({mode:"open"}),ve=document.createElement("style");ve.textContent=`${e.appearance?.cssText??""}:host{pointer-events:none}.rect{position:fixed;box-sizing:border-box;border:2px solid var(--ain-guide-focus, #00665a);background:var(--ain-guide-fill, transparent);pointer-events:none}.hover{border-style:dashed}`;let H=document.createElement("div");ue.append(ve,H),(document.body??document.documentElement).append(B);function Y(b){if(D)return{status:"missing"};let A=o.get(b.id);if(A){let E=A.deref();return E&&j(E)?{status:"available",element:E}:{status:"missing"}}let m=document;for(let E of b.shadowHosts){let g=Cd(m,E);if(g.length>1)return{status:"ambiguous"};let C=g[0];if(!C||W(C)||!C.shadowRoot)return{status:"missing"};m=C.shadowRoot}let v=Cd(m,b.selector);if(v.length>1)return{status:"ambiguous"};let I=v[0];if(!I||!j(I)||I.localName!==b.tagName||b.tagName==="img"&&["src","alt"].some((E)=>Object.hasOwn(b.attributes,E)&&I.getAttribute(E)!==b.attributes[E])||Ji(I,W)!==b.text||Dm.some((E)=>(I.getAttribute(E)?.slice(0,1000)??void 0)!==b.attributes[E]))return{status:"missing"};if(o.set(b.id,new WeakRef(I)),!r.has(I))r.set(I,b.id);return{status:"available",element:I}}function fe(b){let A=Y(b).status;if(!D)a.set(b.id,{target:structuredClone(b),status:A});return A}function y(b){let{element:A}=Y(b);if(!A)return null;let{x:m,y:v,width:I,height:E}=A.getBoundingClientRect();return{x:m,y:v,width:I,height:E}}function N(b,A){let{element:m}=Y(b),v=m?.getBoundingClientRect()??b.rect,I=m?!1:b.styles.position==="fixed";for(let F=m;F;F=Qe(F)??void 0)if(getComputedStyle(F).position==="fixed"){I=!0;break}let{scrollX:E,scrollY:g}=fn().viewport,C=A?.x??v.x+v.width,S=A?.y??v.y+v.height;return{x:C+(I?0:E),y:S+(I?0:g),space:I?"viewport":"document",targetId:b.id,ratioX:A&&v.width?(C-v.x)/v.width:1,ratioY:A&&v.height?(S-v.y)/v.height:1}}function ne(){return e.capture?e.capture(L):L()}function L(){return n=n.map((b)=>{let{element:A}=Y(b);return A?{...b,...zd(A)}:b}),structuredClone(n)}function Q(b,A){let m=ne(),v=m.find((I)=>I.id===A)??m.at(-1);if(v)e.onSelect?.(N(v,b),m)}function Ee(){if(!u)return;let{x:b,y:A}=u,{element:m}=Zr(b,A);if(!m||W(m))return;return[...n].reverse().find((v)=>{let{element:I}=Y(v);if(!I)return!1;let E=I.getBoundingClientRect();if(b<E.left||b>E.right||A<E.top||A>E.bottom)return!1;for(let g=m;g;g=Qe(g))if(g===I)return!0;return!1})}function V(b=!0){if(d=!1,!P)return;if(P=!1,e.onBatchChange?.(!1),b){let A=Ee();Q(A?u??void 0:void 0,A?.id)}}function pe(){if(Z=0,D||!t)return;let b=!1;for(let v of a.values()){let I=Y(v.target).status;if(I!==v.status)v.status=I,b=!0}let A=[],m=(v,I=!1)=>{let{x:E,y:g,width:C,height:S}=v.getBoundingClientRect(),F=document.createElement("div");F.className=I?"rect hover":"rect",F.style.cssText=`left:${E}px;top:${g}px;width:${C}px;height:${S}px`,A.push(F)};if(!l&&!c){if(n.forEach((v)=>{let{element:I}=Y(v);if(I)m(I)}),s&&T&&j(T))m(T,!0)}if(H.replaceChildren(...A),b)oe(),e.onChange()}function ye(){if(!D&&t&&!Z)Z=requestAnimationFrame(pe)}let ke=new ResizeObserver(ye),me=Em({onChange:ye,...e.exclude?{exclude:e.exclude}:{}});function Oe(){if(t)me.refresh();else me.disconnect()}function oe(){if(ke.disconnect(),!t)return;for(let b of n){let{element:A}=Y(b);if(A)ke.observe(A)}if(T&&j(T))ke.observe(T)}function Ie(){for(let b of n)fe(b);Oe(),oe(),ye(),e.onChange()}function G(b){return e.capture?e.capture(()=>ie(b)):ie(b)}function ie(b){let A=b.getRootNode();if(!(A instanceof Document||A instanceof ShadowRoot))throw Error("Target must be connected to the document.");let m=Sd(b,A),v=[];while(A instanceof ShadowRoot){if(A.mode!=="open")throw Error("Targets inside closed shadow roots are unavailable.");let g=A.host;if(A=g.getRootNode(),!(A instanceof Document||A instanceof ShadowRoot))throw Error("Target must be connected to the document.");v.unshift(Sd(g,A))}if(v.length>20)throw Error("Target exceeds the shadow root depth limit of 20.");let I=r.get(b)??crypto.randomUUID(),E={id:I,selector:m,shadowHosts:v,tagName:b.localName.slice(0,100),text:Ji(b,W),attributes:Vd(b),...Mm(b,W,(g)=>Ji(g,W)),...zd(b),states:{focused:b.matches(":focus"),focusWithin:b.matches(":focus-within")}};return r.set(b,I),o.set(I,new WeakRef(b)),E}function de(b){if(b>20)throw Error("Select at most 20 targets.")}function he(b,A=!1){if(D||!j(b))return;i.clear();let m=n.findIndex((v)=>o.get(v.id)?.deref()===b);if(A&&m>=0)n.splice(m,1);else{if(!A&&m===0&&n.length===1)return;de(A?n.length+1:1);let v=G(b);n=A?[...n,v]:[v]}Ie()}function ee(b){if(D)return;de(b.length);let A=structuredClone(b);if(JSON.stringify(A)===JSON.stringify(n))return;i.clear(),V(!1),n=A,Ie()}function $e(b){if(b=t&&b,D||s===b)return;if(!b)V(!l);if(!b)te(),k=null,f=null,w=null;if(s=b,h?.abort(),h=void 0,p=!1,_(!1),s){h=new AbortController;let A=h.signal;document.addEventListener("keydown",(m)=>{if(m.isComposing)return;if(m.key==="Alt"||m.code==="AltLeft"||m.code==="AltRight")_(m.altKey)},{capture:!0,signal:A}),window.addEventListener("keyup",(m)=>{if(m.key==="Alt"||m.code==="AltLeft"||m.code==="AltRight"||!m.altKey)_(m.altKey)},{capture:!0,signal:A}),window.addEventListener("blur",()=>{p=!1,_(!1)},{signal:A}),document.addEventListener("visibilitychange",()=>{if(document.visibilityState==="hidden")p=!1,_(!1)},{signal:A})}T=null,oe(),ye(),e.onPickingChange?.(b)}function _(b){if(b=s&&t&&b,l===b||D)return;if(l=b,te(),b&&f)p=!0;if(f=null,w=null,T=null,b)H.replaceChildren();ye(),e.onPassthroughChange?.(b)}function M(b){if(D||c===b)return;if(c=b,b)V(!1),k=f?.pointerId??k,f=null,te(),T=null,H.replaceChildren();oe(),ye()}function R(b){return _(b.altKey),l||b.altKey}function X(b){if(D||t===b)return;if(!b)V(!l);if(t=b,!t)$e(!1),T=null,me.disconnect(),ke.disconnect(),cancelAnimationFrame(Z),Z=0,H.replaceChildren(),B.style.setProperty("display","none","important");else B.style.removeProperty("display"),Oe(),oe(),ye()}function ge(b){let A=b.composedPath();if(A.some((v)=>v instanceof Element&&W(v)))return;let m=A.find((v)=>v instanceof Element);return m?q(m)??m:void 0}function q(b){for(let A=b;A;A=Qe(A))if(A.matches("button, input, select, textarea, option, optgroup"))return A.matches(":disabled")?A:void 0}function ce(b,A,m,v){if(!m&&!v&&e.onOutsideClick?.())return;if(i.clear(),u=A,m)if(d=!0,!P)n=[G(b)],P=!0,e.onBatchChange?.(!0),Ie();else{if(n.length===20&&!n.some((I)=>o.get(I.id)?.deref()===b))return;he(b,!0)}else V(!1),n=[{...G(b),...v?{textSelection:v}:{}}],Ie(),Q(A)}function Te(){if(D)return;if(i.clear(),f)k=f.pointerId;if(f=null,te(),V(!1),T=null,H.replaceChildren(),!n.length){oe();return}n=[],Ie()}function te(){if(!x)return;let b=document.getSelection();if(x&&b?.rangeCount){let A=b.getRangeAt(0);if(A.startContainer===x.startContainer&&A.startOffset===x.startOffset&&A.endContainer===x.endContainer&&A.endOffset===x.endOffset)b.removeAllRanges()}x=null}let Ce={capture:!0,signal:z.signal};document.addEventListener("pointermove",(b)=>{if(!s)return;if(u={x:b.clientX,y:b.clientY},R(b)||c)return;if(f?.caret&&Math.hypot(b.clientX-f.x,b.clientY-f.y)>6){let m=Bi(b.clientX,b.clientY,W),v=m&&Id(f.caret,m);if(v){te(),x=v;let I=document.getSelection();I?.removeAllRanges(),I?.addRange(v)}}let A=ge(b)??null;if(T===A)return;T=A,oe(),ye()},Ce),document.addEventListener("pointerdown",(b)=>{if(k=null,te(),w=null,f=null,!s)return;if(p=R(b)||c,p)return;let A=ge(b);if(!s||!b.isPrimary||b.button!==0||!A)return;f={element:A,pointerId:b.pointerId,x:b.clientX,y:b.clientY,caret:b.shiftKey||d?null:Bi(b.clientX,b.clientY,W,!0)},b.preventDefault(),b.stopImmediatePropagation()},Ce),document.addEventListener("pointercancel",()=>{k=null,te(),f=null,w=null,p=!1},Ce),document.addEventListener("pointerup",(b)=>{let A=f;if(f=null,!s)return;if((R(b)||p||c)&&!(c&&k===b.pointerId))return;if(k===b.pointerId){k=null,w={pointerId:b.pointerId,x:b.clientX,y:b.clientY,time:performance.now()},b.preventDefault(),b.stopImmediatePropagation();return}if(!s||!b.isPrimary||b.button!==0||!A||A.pointerId!==b.pointerId)return;if(Math.hypot(b.clientX-A.x,b.clientY-A.y)>6){w={pointerId:b.pointerId,x:b.clientX,y:b.clientY,time:performance.now()};let I=ge(b)&&A.caret?Bi(b.clientX,b.clientY,W):null,E=A.caret&&I?Id(A.caret,I):null,g=E&&Nm(E,W);if(te(),b.preventDefault(),b.stopImmediatePropagation(),g&&!b.shiftKey&&!d)ce(g.element,{x:b.clientX,y:b.clientY},!1,g.selection);return}if(te(),!ge(b))return;let{element:m}=Zr(b.clientX,b.clientY);if(!m||W(m))return;let v=q(m);if(!v||v!==A.element)return;b.preventDefault(),b.stopImmediatePropagation(),w={pointerId:b.pointerId,x:b.clientX,y:b.clientY,time:performance.now()},ce(v,{x:b.clientX,y:b.clientY},b.shiftKey||d)},Ce),document.addEventListener("click",(b)=>{if(!s)return;let A=w&&b.detail!==0&&performance.now()-w.time<500&&Math.hypot(b.clientX-w.x,b.clientY-w.y)<=6;if(R(b)||p||c){if(p=!1,c&&A)w=null,b.preventDefault(),b.stopImmediatePropagation();return}if(A){w=null,b.preventDefault(),b.stopImmediatePropagation();return}let m=ge(b);if(!m)return;b.preventDefault(),b.stopImmediatePropagation(),ce(m,{x:b.clientX,y:b.clientY},b.shiftKey||d)},Ce),document.addEventListener("keydown",(b)=>{if(w=null,!s||c||b.isComposing||l||b.altKey)return;if(b.key==="Shift"){if(!b.composedPath().some((A)=>A instanceof Element&&(W(A)||A.matches("input,textarea,select")||A instanceof HTMLElement&&A.isContentEditable)))d=!0}else if(b.key==="Escape")b.preventDefault(),b.stopImmediatePropagation(),Te(),e.onCancel?.()},Ce),window.addEventListener("keyup",(b)=>{if(b.key==="Shift")V(!(l||b.altKey))},Ce),window.addEventListener("blur",()=>{te(),u=null,f=null,w=null,p=!1,V(!l)},{signal:z.signal}),document.addEventListener("pointerout",(b)=>{if(!b.relatedTarget)u=null},Ce),document.addEventListener("visibilitychange",()=>{if(document.visibilityState==="hidden")te(),u=null,V(!l)},{signal:z.signal}),document.addEventListener("scroll",ye,Ce),window.addEventListener("resize",ye,{signal:z.signal}),Oe();function He(b,A){let m=n.find((g)=>g.id===b);if(!m)return;if(A==="parent"&&(i.get(b)?.length??0)>=64)return;let v=Y(m).element;if(!v||!j(v))return;let I=i.get(b)?.at(-1),E=A==="parent"?Qe(v):I&&Y(I).element;if(!E||!j(E)||E===document.documentElement||A==="back"&&Qe(E)!==v||n.some((g)=>g.id!==b&&Y(g).element===E))return;return{candidate:E,previous:I}}return{getNavigation(){return n.map((b)=>({target:structuredClone(b),history:structuredClone(i.get(b.id)??[])}))},restoreNavigation(b){if(D)return;de(b.length),V(!1),te(),i.clear(),n=b.map((A)=>structuredClone(A.target));for(let A of b){let m=structuredClone(A.history.slice(-64));i.set(A.target.id,m);for(let v of m)fe(v)}Ie()},targetNavigation(b){return{parent:!!He(b,"parent"),back:!!He(b,"back")}},navigateTarget(b,A,m=()=>!0){let v=He(b,A);if(!v)return!1;let I=n.findIndex((S)=>S.id===b),E=i.get(b)??[],g=A==="parent"?G(v.candidate):structuredClone(v.previous);if(!m(n.map((S,F)=>F===I?g:S)))return!1;let C=A==="parent"?[...E,structuredClone(n[I])]:E.slice(0,-1);return i.delete(b),i.set(g.id,C),n[I]=g,te(),Ie(),!0},resetPage(){Te(),o.clear(),a.clear(),i.clear(),r=new WeakMap},setTheme(b){if(!D&&B.dataset.theme!==b)B.dataset.theme=b},setVisible:X,setPicking:$e,setSuspended:M,select:he,setTargets:ee,remove(b){if(D||!n.some((A)=>A.id===b))return;n=n.filter((A)=>A.id!==b),Ie()},parent(){let b=n.at(-1),A=b&&Y(b).element,m=A&&Qe(A);if(m)he(m)},clear:Te,getTargets:ne,getRect:y,getElement:(b)=>Y(b).element??null,availability:fe,focus:ee,destroy(){if(D)return;if(te(),D=!0,h?.abort(),h=void 0,p=!1,l)l=!1,e.onPassthroughChange?.(!1);if(V(!1),z.abort(),me.disconnect(),ke.disconnect(),cancelAnimationFrame(Z),Z=0,B.remove(),T=null,f=null,w=null,n=[],o.clear(),a.clear(),i.clear(),r=new WeakMap,s)s=!1,e.onPickingChange?.(!1)}}}async function Ud(e,t,n,r=3000){let o=!1,i,a=pd(e,t,n).then((s)=>{if(o)s.close();return s});try{return await Promise.race([a,new Promise((s,c)=>{i=setTimeout(()=>{o=!0,c(Error("Local database open timed out"))},r)})])}finally{clearTimeout(i)}}function Zm(e,t){let n=(a)=>new Set(a.annotations.flatMap((s)=>s.targets.map(Ye))),r=n(t.document),o=new Set([...n(e.document)].filter((a)=>!r.has(a)));if(!o.size)return t;let i=(a)=>!o.has(Ye(a));if(t.styleDrafts)t.styleDrafts=t.styleDrafts.filter(i);if(t.draft.styleTargets)t.draft.styleTargets=t.draft.styleTargets.filter(i);if(t.stylePreview)t.stylePreview.disabledTargets=t.stylePreview.disabledTargets.filter((a)=>!o.has(a));return t}async function jm(e){let t,n=!1,r=new Map,o;try{o=new BroadcastChannel("ainotation-feedback")}catch{}if(o)o.onmessage=()=>{if(!n)e.onExternalChange()};let i=()=>{t?.close(),t=void 0,e.onUnavailable()};try{t=await Ud("ainotation-feedback",1,{upgrade(d){d.createObjectStore("pages")},blocking(){i()},terminated(){i()}})}catch{i()}let a=(d)=>({document:Iu(d),operations:[],draft:{text:"",editingId:null,targets:[]},authority:null}),s=(d,u)=>{try{if(!d||typeof d!=="object"||!("draft"in d)||!d.draft||typeof d.draft!=="object")throw Error("Invalid draft envelope");let f=d,x=nn(It.parse(f.document));if(u!==void 0&&x.url!==u)throw Error("Stored page mismatch");let k=ni.parse(f.draft.images??[]),w=new Set([...x.annotations.flatMap((Z)=>Z.images??[]),...k].map((Z)=>Z.id)),P=Object.fromEntries(Object.entries(f.images??{}).filter(([Z,z])=>w.has(Z)&&z instanceof Blob)),D={},T=Nn.pick({x:!0,y:!0}).safeParse(f.variantPosition);for(let[Z,z]of Object.entries(f.editorViews??{}).slice(-1001)){if(!rt.shape.id.safeParse(Z).success||!z||typeof z!=="object")continue;let W=rt.shape.id.array().max(20).safeParse(z.targets);if(!W.success)continue;let j=Nn.pick({x:!0,y:!0}).safeParse(z.position);D[Z]={tab:z.tab==="styles"?"styles":"feedback",targets:W.data,position:j.success?j.data:null};let B=z.navigation;if(B&&Array.isArray(B.slots)&&B.slots.length>0&&B.slots.length<=20){let ue=rt.shape.id.array().max(20).safeParse(B.referenceIds),ve=B.slots.map((H)=>{let Y=rt.safeParse(H?.target),fe=rt.array().max(64).safeParse(H?.history);if(!Y.success||!fe.success)return null;let y=[...fe.data,Y.data].map((N)=>N.id);if(new Set(y).size!==y.length)return null;return{target:Y.data,history:fe.data}});if(ue.success&&ve.every((H)=>H!==null)&&new Set(ve.map((H)=>H.target.id)).size===ve.length)D[Z].navigation={slots:ve,referenceIds:ue.data}}}return{images:P,...T.success?{variantPosition:T.data}:{},...f.variantFeedback&&rt.shape.id.safeParse(f.variantFeedback.explorationId).success&&typeof f.variantFeedback.text==="string"?{variantFeedback:{explorationId:f.variantFeedback.explorationId,text:f.variantFeedback.text.slice(0,1e4)}}:{},...Object.keys(D).length?{editorViews:D}:{},...f.styleDrafts?{styleDrafts:rt.array().max(20000).parse(f.styleDrafts)}:{},...f.stylePreview?{stylePreview:{enabled:f.stylePreview.enabled===!0,disabledTargets:f.stylePreview.disabledTargets.filter((Z)=>typeof Z==="string").slice(0,20000)}}:{},...typeof f.multipleSelection==="boolean"?{multipleSelection:f.multipleSelection}:{},document:x,...f.recoveryCopies?{recoveryCopies:f.recoveryCopies.slice(-3).map((Z)=>({document:It.parse(Z.document),images:Object.fromEntries(Object.entries(Z.images).filter(([,z])=>z instanceof Blob))}))}:{},...f.storageEpoch?{storageEpoch:ui.shape.storageEpoch.unwrap().parse(f.storageEpoch)}:{},...f.syncRecovery?{syncRecovery:ku.shape.recovery.unwrap().omit({source:!0}).extend({server:It.optional()}).parse(f.syncRecovery)}:{},operations:li.array().max(1000).parse(f.operations),authority:typeof f.authority==="string"?f.authority:null,draft:{text:typeof f.draft.text==="string"?f.draft.text.slice(0,1e4):"",...f.draft.variantsRequested?{variantsRequested:!0}:{},...f.draft.variantsRequested&&rt.shape.id.safeParse(f.draft.variantRequestBase).success?{variantRequestBase:f.draft.variantRequestBase}:{},editingId:typeof f.draft.editingId==="string"?f.draft.editingId:null,...rt.shape.id.safeParse(f.draft.editorSessionId).success?{editorSessionId:f.draft.editorSessionId}:{},...f.draft.targetsAdjusted?{targetsAdjusted:!0}:{},targets:rt.array().max(20).parse(f.draft.targets),...f.draft.styleTargets?{styleTargets:rt.array().max(20).parse(f.draft.styleTargets)}:{},...k.length?{images:k}:{},...f.draft.marker?{marker:Nn.parse(f.draft.marker)}:{},...f.draft.page?{page:si.parse(f.draft.page)}:{},...typeof f.draft.editorOpen==="boolean"?{editorOpen:f.draft.editorOpen}:{}}}}catch(f){throw new pn(f)}},c=Promise.resolve(),l;function p(d,u,f){if(n)return Promise.reject(Error("Storage closed"));let x=(w)=>Zm(w,s(f(structuredClone(w)),u)),k=c.then(async()=>{let w=r.get(d)??a(u),P;if(t){let D=!1;try{let T=t.transaction("pages","readwrite");try{let Z=await T.store.get(d);w=Z?s(Z,u):w,D=!0,P=x(w),D=!1,await T.store.put(P,d),await T.done}catch(Z){try{T.abort()}catch{}throw await T.done.catch(()=>{}),Z}r.set(d,P);try{o?.postMessage(d)}catch{}return structuredClone(P)}catch(T){if(D||T instanceof pn)throw T;i()}}if(w.document.url!==u)throw new pn("Stored page mismatch");return P??=x(w),r.set(d,P),structuredClone(P)});return c=k.catch(()=>{}),k}async function h(d,u){if(await c,n)throw Error("Storage closed");if(t)try{let f=await t.get("pages",d);if(f){let x=s(f,u);return r.set(d,x),structuredClone(x)}}catch(f){if(f instanceof pn)throw f;i()}return structuredClone(s(r.get(d)??a(u),u))}return{get available(){return!!t},read:h,async readProjectDocuments(d){if(await c,n)throw Error("Storage closed");let u=`${JSON.stringify([d]).slice(0,-1)},`;if(t){let f;try{let x=t.transaction("pages","readonly");f=x.done;let k=await x.store.openCursor(IDBKeyRange.bound(u,`${u}￿`)),w=[];while(k){let P=s(k.value);if(k.key!==JSON.stringify([d,P.document.url]))throw new pn("Stored page mismatch");r.set(k.key,P),w.push(P.document),k=await k.continue()}return await x.done,structuredClone(w.sort((P,D)=>P.url.localeCompare(D.url)))}catch(x){if(await f?.catch(()=>{}),x instanceof pn)throw x;i()}}return structuredClone([...r].filter(([f,x])=>f===JSON.stringify([d,x.document.url])).map(([,f])=>f.document).sort((f,x)=>f.url.localeCompare(x.url)))},update:p,async load(d,u){return p(d,u,(f)=>{if(f.draft.editingId&&!f.document.annotations.some((x)=>x.id===f.draft.editingId))f.draft={text:f.draft.text,...f.draft.images?.length?{images:f.draft.images}:{},editingId:null,targets:[],editorOpen:!1};return f})},bindAuthority(d,u,f){return p(d,u,(x)=>{if(x.authority&&x.authority!==f)delete x.storageEpoch,delete x.syncRecovery,x.document={...x.document,id:crypto.randomUUID()};return{...x,authority:f}})},mutate(d,u,f,x){return p(d,u,(k)=>{let w=(D)=>JSON.stringify([D.text,D.editingId,D.targets,D.marker,D.page,D.images??[],D.styleTargets??[],D.variantsRequested??!1,D.variantRequestBase??null]),P=x&&w(k.draft)===w(x);return{...k,document:zr(k.document,f),operations:[...k.operations,f],...P?{draft:{text:"",editingId:null,targets:[],editorOpen:!1}}:{}}})},applySync(d,u,f,x={}){return p(d,u,(k)=>{if(f.recovery)throw Error("Recovery conflicts must be resolved before applying server data.");if(k.document.id!==f.document.id||f.document.url!==u)throw Error("MCP session mismatch");if(k.syncRecovery)k.recoveryCopies=[...k.recoveryCopies??[],{document:k.document,images:k.images??{}}].slice(-3),delete k.syncRecovery;let w=new Set(f.acknowledged),P=k.operations.filter((z)=>!w.has(z.id)),D=P.reduce(zr,f.document),T=k.draft;if(T.editingId&&!D.annotations.some((z)=>z.id===T.editingId))T={text:T.text,...T.images?.length?{images:T.images}:{},editingId:null,targets:[],editorOpen:!1};let Z=[...k.operations].reverse().find((z)=>z.kind==="upsert"&&w.has(z.id)&&!D.annotations.some((W)=>W.id===z.annotation.id)&&!k.operations.some((W)=>W.kind==="delete"&&W.annotationId===z.annotation.id));if(!T.text&&Z?.kind==="upsert")T={text:Z.annotation.comment,...Z.annotation.images?.length?{images:Z.annotation.images}:{},editingId:null,targets:[],editorOpen:!1};return{...k,document:D,...f.storageEpoch?{storageEpoch:f.storageEpoch}:{},images:{...k.images,...x},operations:P,draft:T}})},clearAnnotations(d,u,f){let x=[...new Set(f)].map((k)=>({id:crypto.randomUUID(),kind:"delete",annotationId:k}));return p(d,u,(k)=>({...k,document:x.reduce(zr,k.document),operations:[...k.operations,...x],draft:{text:"",editingId:null,targets:[],editorOpen:!1},styleDrafts:[],stylePreview:{enabled:k.stylePreview?.enabled??!0,disabledTargets:[]},editorViews:{}}))},close(){if(n=!0,o)o.onmessage=null;return l??=c.then(()=>{o?.close(),t?.close(),r.clear()}),l}}}function Un(e,t){let n={left:Math.max(e.left,t.left),top:Math.max(e.top,t.top),right:Math.min(e.right,t.right),bottom:Math.min(e.bottom,t.bottom)};return n.right>n.left&&n.bottom>n.top?n:null}function xd(e,t){let n=Un(e,t);if(!n)throw le("cropOutside");return{x:(n.left-t.left)/(t.right-t.left),y:(n.top-t.top)/(t.bottom-t.top),width:(n.right-n.left)/(t.right-t.left),height:(n.bottom-n.top)/(t.bottom-t.top)}}function ji(e,t,n){if(![e.x,e.y,e.width,e.height,t,n].every(Number.isFinite)||e.width<=0||e.height<=0||t<=0||n<=0)throw le("cropInvalid");let r=(c)=>Math.abs(c-Math.round(c))<0.0000001?Math.round(c):c,o=Math.max(0,Math.floor(r(e.x*t))),i=Math.max(0,Math.floor(r(e.y*n))),a=Math.min(t,Math.ceil(r((e.x+e.width)*t))),s=Math.min(n,Math.ceil(r((e.y+e.height)*n)));if(a<=o||s<=i)throw le("cropOutsideImage");return{x:o,y:i,width:a-o,height:s-i}}async function Bd(e){return[...new Uint8Array(await crypto.subtle.digest("SHA-256",await e.arrayBuffer()))].map((t)=>t.toString(16).padStart(2,"0")).join("")}function Hd(e,t){if(!Number.isSafeInteger(e)||!Number.isSafeInteger(t)||e<=0||t<=0)throw le("invalidDimensions",e,t)}function Vm(e,t){if(Hd(e,t),e>16384||t>16384||e*t>Sr)throw le("imageTooLarge",e,t)}function Jd(e,t){Hd(e,t);let n=Math.min(1,16384/e,16384/t,Math.sqrt(Sr/(e*t)));return{width:Math.max(1,Math.floor(e*n)),height:Math.max(1,Math.floor(t*n))}}function Bn(e,t,n,r){let o=r?ji(r,t,n):null,i=Jd(o?.width??t,o?.height??n),a=document.createElement("canvas");a.width=i.width,a.height=i.height;try{let s=a.getContext("2d");if(!s)throw le("drawingUnavailable");if(s.imageSmoothingEnabled=!0,s.imageSmoothingQuality="high",o)s.drawImage(e,o.x,o.y,o.width,o.height,0,0,i.width,i.height);else s.drawImage(e,0,0,i.width,i.height);return a}catch(s){throw a.width=0,a.height=0,s}}async function Vi(e,t){t?.throwIfAborted();let n=Jd(e.width,e.height),r=e;try{if(n.width!==e.width||n.height!==e.height)r=Bn(e,n.width,n.height);while(!0){t?.throwIfAborted();let o=await new Promise((c,l)=>r.toBlob((p)=>{if(!p)l(le("encodeFailed",r.width,r.height));else c(p)},"image/png"));if(t?.throwIfAborted(),o.size<=tn)return o;if(r.width===1&&r.height===1)throw le("imageFitFailed");let i=Math.min(0.85,Math.sqrt(tn/o.size)*0.9),a=Math.max(1,Math.floor(r.width*i)),s=Math.max(1,Math.floor(r.height*i));if(r!==e)r.width=0,r.height=0;r=Bn(e,a,s)}}finally{if(r!==e)r.width=0,r.height=0}}async function Fi(e){if(!["image/png","image/jpeg","image/webp"].includes(e.type))throw le("imageType");if(e.size>tn)throw le("imageBytesLimit");let t=await createImageBitmap(e);try{Vm(t.width,t.height)}catch(n){throw t.close(),n}return t}async function Fm(e,t){let n=await Fi(e);try{return ti.parse({id:crypto.randomUUID(),mimeType:"image/png",width:n.width,height:n.height,size:e.size,sha256:await Bd(e),source:t})}finally{n.close()}}async function Pd(e,t){if(e.type!==t.mimeType||e.size!==t.size||await Bd(e)!==t.sha256)throw le("imageValidation")}async function Um(e){let t={},{connection:n,signal:r,uploaded:o}=e,i=new Map(e.document.annotations.flatMap((c)=>c.images??[]).map((c)=>[c.id,c])),a=(c,l)=>`${n.endpoint}:${n.token}:${e.document.id}:${c}:${l}`,s=new Set([...i.values()].map((c)=>a(c.id,c.sha256)));for(let c of o)if(!s.has(c))o.delete(c);for(let c of i.values()){let l=a(c.id,c.sha256),p=e.local[c.id];if(p&&o.has(l)&&!e.missing?.includes(c.id))continue;if(p)await Pd(p,c);let h=await fetch(`${n.endpoint}/sessions/${e.document.id}/images/${c.id}`,{method:p?"POST":"GET",headers:{Authorization:`Bearer ${n.token}`,...p?{"Content-Type":"image/png"}:{}},...p?{body:p}:{},credentials:"omit",redirect:"error",cache:"no-store",signal:AbortSignal.any([r,AbortSignal.timeout(15000)])});if(!h.ok){if(await h.body?.cancel(),h.status===404&&!p)throw le("syncImagesMissing");throw Error(`Image synchronization failed (${h.status}).`)}if(p){await h.body?.cancel(),o.add(l);continue}if(!h.body||h.headers.get("Content-Type")!=="image/png")throw await h.body?.cancel(),Error("Invalid image response.");let d=h.body.getReader(),u=[],f=0;try{while(!0){let k=await d.read();if(k.done)break;if(f+=k.value.length,f>tn)throw Error("Image response too large.");u.push(new Uint8Array(k.value))}}finally{await d.cancel().catch(()=>{}),d.releaseLock()}let x=new Blob(u,{type:"image/png"});await Pd(x,c),t[c.id]=x,o.add(l)}return t}function Bm(e){let t=(n)=>n.styleChanges!==void 0||n.styleTargetId!==void 0;return e.document.targetStyles!==void 0||e.document.annotations.some((n)=>n.targets.some(t))||e.operations.some((n)=>n.kind==="upsert"&&(Object.keys(n.styleLinks??{}).length>0||n.annotation.targets.some(t)))}function qn(e){let t=new URL(e.endpoint);if(e.transport==="same-origin"){if(t.origin!==location.origin||!["http:","https:"].includes(t.protocol)||t.username||t.password||t.search||t.hash)throw Error("Development MCP endpoint must be same-origin.")}else if(!["http:","https:"].includes(t.protocol)||!["127.0.0.1","localhost","[::1]"].includes(t.hostname)||t.username||t.password||t.search||t.hash||t.pathname!=="/")throw le("invalidEndpoint");if(!e.token.trim()||/\s/.test(e.token.trim())||e.token.length>512)throw le("invalidToken");return{endpoint:e.transport==="same-origin"?t.href.replace(/\/$/,""):t.origin,token:e.token.trim(),...e.transport?{transport:e.transport}:{}}}function Ad(e){let t=new AbortController,n=e.connection,r=typeof n==="function"?async()=>qn(await n(t.signal)):(()=>{let w=qn(n);return async()=>w})(),o=!1,i=!1,a=e.recovery,s,c=new Set,l=!1,p,h,d=()=>{throw i=!0,s=we("styleSyncUnsupported"),le("styleSyncUnsupported")},u=()=>{throw i=!0,s=we("variantsUnsupported"),e.onVariantsSupport?.(!1),le("variantsUnsupported")},f=()=>{if(l=!0,p)return p;return p=(async()=>{e.onSync(!0);try{while(l&&!o&&!i){l=!1;let w=await e.read();if(o)return;let P=await r();if(o)return;let D=Bm(w),T=!!w.document.variantCleanups?.length||w.document.annotations.some((j)=>j.variants)||w.operations.some((j)=>j.kind==="variants"||j.kind==="upsert"&&(j.variantRequest||j.annotation.variants));if(D||T){let j=await fetch(`${P.endpoint}/health`,{headers:{Authorization:`Bearer ${P.token}`},credentials:"omit",redirect:"error",cache:"no-store",signal:AbortSignal.any([t.signal,AbortSignal.timeout(1e4)])});if(j.status===404)if(T)u();else d();if(!j.ok)throw Error(`MCP capability check failed (${j.status}).`);let B=await j.json();if(T&&B?.capabilities?.uiVariants!==1)u();if(w.document.variantCleanups?.length&&B?.capabilities?.uiVariantsCleanup!==1)u();if(D&&!ri.safeParse(B?.capabilities).success)d();if(o)return}let Z=await fetch(`${P.endpoint}/sessions/${e.sessionId}/sync`,{method:"POST",headers:{Authorization:`Bearer ${P.token}`,"Content-Type":"application/json"},credentials:"omit",redirect:"error",cache:"no-store",body:JSON.stringify({document:w.document,operations:w.operations,storageEpoch:w.storageEpoch,...a?{recovery:a}:{}}),signal:AbortSignal.any([t.signal,AbortSignal.timeout(1e4)])});if(!Z.ok){let j=Su.safeParse(await Z.json().catch(()=>{return})),B=j.success?j.data.code:void 0;if(B==="variants-busy")throw i=!0,s=we("variantsBusy"),le("variantsBusy");if(B==="recovery-stale"&&a){a=void 0,l=!0;continue}if(B?.startsWith("storage-")||B==="service-ownership")throw i=!0,s=we("syncStorageFailed"),le("syncStorageFailed");throw Error(`MCP sync failed (${Z.status}). Check pairing token, allowed origin and request size.`)}let z=ui.parse(await Z.json());if(T&&z.uiVariants!==1)u();if(w.document.variantCleanups?.length&&z.uiVariantsCleanup!==1)u();if(e.onVariantsSupport?.(z.uiVariants===1),D&&!ri.safeParse(z).success)d();if(z.document.id!==e.sessionId||z.document.url!==w.document.url)throw Error("MCP returned another session.");if(z.recovery){i=!0,await e.onRecovery?.(z),e.onState("error",we("syncRecoveryNeeded"));return}if(a=void 0,z.storageEpoch&&z.storageEpoch!==w.storageEpoch)c.clear();if(!o)await e.apply(z,{});if(o)return;let W=await Um({document:z.document,local:w.images??{},connection:P,signal:t.signal,uploaded:c,...z.missingImages?{missing:z.missingImages}:{}});if(!o&&Object.keys(W).length)await e.apply(z,W)}}finally{if(p=void 0,!o)e.onSync(!1)}})(),p},x=()=>{if(!o&&!i)f().catch((w)=>{if(o)return;e.onState("error",s??sn(w,"syncFailed")),h?.abort()})},k=()=>new Promise((w)=>{let P=()=>{clearTimeout(D),t.signal.removeEventListener("abort",P),w()},D=setTimeout(P,2500);t.signal.addEventListener("abort",P,{once:!0})});return{finished:(async()=>{while(!o&&!i){e.onState("connecting",we("syncConnecting"));let w,P,D=(T)=>{clearTimeout(P),P=setTimeout(()=>h?.abort(),T)};try{if(await f(),o||i)break;if(e.once){e.onState("connected",we("syncConnected"));break}let T=await r();if(o)break;h=new AbortController,D(1e4);let Z=await fetch(`${T.endpoint}/sessions/${e.sessionId}/events`,{headers:{Authorization:`Bearer ${T.token}`},credentials:"omit",redirect:"error",cache:"no-store",signal:AbortSignal.any([t.signal,h.signal])});if(!Z.ok||!Z.body)throw await Z.body?.cancel(),Error("MCP event stream unavailable");w=Z.body.getReader(),D(35000),e.onState("connected",we("syncConnected"));let z=new TextDecoder,W="";while(!o){let j=await w.read();if(j.done)throw Error("MCP disconnected");if(D(35000),W+=z.decode(j.value,{stream:!0}).replace(/\r/g,""),W.length>65536)throw Error("Invalid MCP event stream");let B;while((B=W.indexOf(`

`))>=0){let ue=W.slice(0,B);if(W=W.slice(B+2),ue.includes("event: changed"))x()}}}catch(T){if(c.clear(),!o)e.onState("error",s??sn(T,"syncUnavailable"))}finally{clearTimeout(P),await w?.cancel().catch(()=>{}),w?.releaseLock()}if(e.once)break;if(!o&&!i)await k()}})(),request:x,stop(){o=!0,t.abort()}}}function Ym(e){let{root:t,signal:n}=e,r=(s)=>s instanceof Element&&(s===t.host||s.getRootNode()===t),o=new Set(["blur","focusout","pointerout","pointerleave","mouseout","mouseleave"]),i=new Set(["pointerdown","pointerup","mousedown","mouseup","click","dblclick","contextmenu"]),a=(s)=>{if(!e.active()||e.passthrough())return;if((s instanceof MouseEvent||s instanceof KeyboardEvent)&&s.altKey)return;if(s instanceof KeyboardEvent&&(s.key==="Alt"||s.code==="AltLeft"||s.code==="AltRight"))return;let c=s.composedPath().find(r),l=o.has(s.type)&&(s instanceof MouseEvent||s instanceof FocusEvent)&&r(s.relatedTarget);if(!c&&!l)return;if(s.stopImmediatePropagation(),!c)return;if(c.closest("button")&&i.has(s.type))s.preventDefault();e.handle(s,c)};for(let s of["pointerdown","pointerup","pointermove","pointercancel","lostpointercapture","pointerover","pointerout","pointerenter","pointerleave","mousedown","mouseup","mousemove","mouseover","mouseout","mouseenter","mouseleave","click","dblclick","contextmenu","touchstart","touchmove","touchend","touchcancel","focus","blur","focusin","focusout","keydown","keyup","keypress","beforeinput","input","change","compositionstart","compositionupdate","compositionend","copy","cut","paste","dragstart","dragenter","dragover","dragleave","drop","dragend","wheel"])window.addEventListener(s,a,{capture:!0,passive:!1,signal:n}),t.addEventListener(s,a,{capture:!0,passive:!1,signal:n})}function Qm(e,t){let n=null,r=null,o=null,i=()=>e.querySelector(".variant-confirm");function a(){if(r=null,i()?.close(),o?.isConnected&&!o.disabled&&o.checkVisibility())o.focus({preventScroll:!0});o=null}return{template(s){let c=st(s.locale);return J`<dialog
        class="variant-confirm"
        role="alertdialog"
        aria-labelledby="ain-variant-confirm-title"
        aria-describedby="ain-variant-confirm-description"
        @cancel=${(l)=>{l.preventDefault(),a()}}
      >
        <h2 id="ain-variant-confirm-title">${c.variantsCancelTitle}</h2>
        <p id="ain-variant-confirm-description">${c.variantsCancelDescription}</p>
        <div class="variant-confirm-actions">
          <button type="button" data-action="variant-keep" autofocus>
            ${c.variantsKeepComparing}
          </button>
          <button type="button" class="danger" data-action="variant-confirm-cancel">
            ${c.variantsCancel}
          </button>
        </div>
      </dialog>`},update(s,c){let l=s.document?.annotations.find((h)=>h.id===s.variantAnnotationId),p=l?.variants;if(n=c&&!s.localOnly&&!s.passthrough&&!s.variantMinimized&&!s.variantSaving&&p&&!["completed","cancelled"].includes(p.status)?JSON.stringify([s.document?.id,s.document?.url,l?.id,p.id,p.generation,p.revision]):null,r!==null&&r!==n)a()},handle(s,c){let l=c.closest("button");if(c.closest(".variant-confirm")){if(s.type==="keydown"&&s instanceof KeyboardEvent&&s.key==="Escape")s.preventDefault(),a();else if(s.type==="click"&&l&&!l.disabled){if(l.dataset.action==="variant-keep")a();else if(l.dataset.action==="variant-confirm-cancel"){let p=i()?.open&&r!==null&&r===n;if(a(),p)t({type:"variant-decision",decision:"cancel"})}}return!0}if(l?.dataset.action!=="variant-cancel"||!l.closest(".variants-controller"))return!1;if(s.type==="click"&&!l.disabled&&n!==null)o=l,r=n,l.focus({preventScroll:!0}),i()?.showModal();return!0},destroy:a}}function eg(e){let t=(e.ancestors??[]).filter((o)=>["header","nav","main","footer","dialog","form"].includes(o.tagName)||o.attributes["aria-label"]).slice(-2).map((o)=>{let i=o.attributes["aria-label"],a=(o.attributes.class??"").split(/\s+/).find(Wt);return`${o.tagName}${i?` ${JSON.stringify(i)}`:a?`.${a}`:""}`}),n=(e.attributes.class??"").split(/\s+/).filter(Wt).slice(0,2),r=e.label&&e.label!==e.tagName?e.label:`${e.tagName}${n.map((o)=>`.${o}`).join("")}`;return[...t,r].join(" › ")}function qd(e){return e.shadowHosts.length?`Shadow hosts:
${e.shadowHosts.join(`
`)}
Selector:
${e.selector}`:e.selector}function tg(e,t){let n=e.getAttribute("style"),r=Hr(e.style),o=new Set,i=document.createElement("span").style;for(let a of t){i.cssText="",i.setProperty(a.property,a.value,"important");for(let s of i)o.add(s);e.style.setProperty(a.property,a.value,"important")}return{element:e,original:n,before:r,after:new Map([...Hr(e.style)].filter(([a])=>o.has(a))),written:e.getAttribute("style")}}function ng(e){let t=Hr(e.element.style);return[...e.after].some(([n,r])=>!Kd(t.get(n),r))}function rg({element:e,original:t,written:n,before:r,after:o}){if(e.getAttribute("style")===n){if(t===null)e.removeAttribute("style");else e.setAttribute("style",t);return}let i=Hr(e.style);for(let[a,s]of o){if(!Kd(i.get(a),s))continue;let c=r.get(a);if(c)e.style.setProperty(a,c.value,c.priority);else e.style.removeProperty(a)}}function ct(e,t=!1){let n=e.startsWith("padding-")?"padding":e.startsWith("margin-")?"margin":null;if(!n||!t)return[e];return(t==="horizontal"?["left","right"]:t==="vertical"?["top","bottom"]:["top","right","bottom","left"]).map((r)=>`${n}-${r}`)}function Nd(e,t,n,r){let o=t.trim().match(/^(-?(?:\d+\.?\d*|\.\d+))([a-z%]*)$/i);if(!o)return null;let i=o[2]||(["opacity","font-weight","line-height"].includes(e)?"":"px");if(!CSS.supports(e,`${o[1]}${i}`))return null;let a=e==="opacity"&&i!=="%"?0.05:["em","rem"].includes(i)||e==="line-height"&&!i?0.1:1,s=Number(o[1])+n*a*(r?10:1);if(!Number.isFinite(s))return null;if(!e.startsWith("margin-"))s=Math.max(0,s);if(e==="opacity")s=Math.min(i==="%"?100:1,s);return`${Number(s.toFixed(4))}${i}`}function og(e,t=()=>{}){let n=new Map,r=new Map,o=new WeakMap,i=new Map,a=new Map,s=new Map,c=new Map,l=[],p=[],h=new Set,d="",u=[],f=new Map,x=!0,k=new Set,w=new Set,P=new Map,D=new Set,T=!1,Z=new Set,z=!1,W=new Map,j=[],B=[],ue=0,ve="",H=(_,M)=>JSON.stringify(_)===JSON.stringify(M),Y=(_)=>r.get(_)??_,fe=(_)=>r.get(_.id)??Y(Ye(_)),y=()=>d==="__selection__"?u.filter((_)=>l.includes(_)):d?[Y(d)].filter((_)=>l.includes(_)):l,N=(_)=>a.get(_)??i.get(_)??[],ne=(_)=>{let M=n.get(_),R=M&&e(M);if(!(R instanceof HTMLElement||R instanceof SVGElement))return null;let X=f.get(_);if(X&&X.deref()!==R)return null;if(!X)f.set(_,new WeakRef(R));return R},L=(_)=>{let M=ne(_);return!!M&&p.filter((R)=>fe(R)===_).every((R)=>Y(Ye(R))===_&&e(R)===M)},Q=new MutationObserver(()=>{if(Oe())t();else Ee()});function Ee(){if(z||T||!x||ue)return;let _=[...P].some(([R,X])=>X==="missing"&&!D.has(R));if(!W.size&&!_)return;let M=_?{subtree:!0,childList:!0,attributes:!0,attributeFilter:["hidden","open","class","style"]}:{subtree:!0,childList:!0};Q.observe(document,M);for(let[R,X]of n){let ge=ne(R),q=ge?.getRootNode();while(q instanceof ShadowRoot)Q.observe(q,M),q=q.host.getRootNode();if(_&&!ge){let ce=document;for(let Te of X.shadowHosts){let te;try{te=ce.querySelectorAll(Te)}catch{break}if(te.length!==1||!te[0]?.shadowRoot)break;ce=te[0].shadowRoot,Q.observe(ce,M)}}}}function V(){me(),Q.disconnect();for(let _ of W.values())rg(_);W.clear()}function pe(_){let M=c.get(_);if(!M){let R=ne(_);if(!R)return{};let X=getComputedStyle(R);M=Object.fromEntries(Nt.map((ge)=>[ge,X.getPropertyValue(ge)]));for(let ge of N(_))M[ge.property]=ge.before;c.set(_,M)}return M}function ye(_){for(let M of[..._].sort((R,X)=>Number(!!R.styleTargetId)-Number(!!X.styleTargetId))){let R=e(M),X=Y(Ye(M)),ge=n.get(X),q=ge&&e(ge);if(R&&q&&q!==R){let te=M.id,Ce=structuredClone(M);if(delete Ce.styleTargetId,delete Ce.styleChanges,te!==X)n.set(te,Ce),r.set(te,te),o.set(R,te);P.set(te,"missing"),D.add(te);continue}let ce=(R&&o.get(R))??X;if(R)o.set(R,ce);r.set(M.id,ce),r.set(Ye(M),ce);let Te=n.get(ce);if(!Te||!e(Te)&&R){let te=structuredClone(M);if(delete te.styleChanges,ce!==te.id)te.styleTargetId=ce;n.set(ce,te)}}}function ke(){if(V(),z||T||!x||ue)return;for(let[M,R]of P)if(R==="missing"&&!D.has(M)&&ne(M))P.delete(M);for(let M of n.keys())pe(M);let _=[];for(let M of n.keys()){let R=N(M);if(!R.length||k.has(M)||Z.has(M))continue;if(P.has(M))continue;let X=ne(M);if(!X){P.set(M,"missing");continue}let ge=getComputedStyle(X);if(R.some((q)=>!Ir.safeParse(q).success||!CSS.supports(q.property,q.value)||!w.has(M)&&ge.getPropertyValue(q.property)!==q.before&&ge.getPropertyValue(q.property)!==q.value)){P.set(M,"changed");continue}_.push([M,X,R])}for(let[M,R,X]of _)W.set(M,tg(R,X)),Q.observe(R,{attributes:!0,attributeFilter:["style"]});Ee()}function me(){let _=!1;for(let[M,R]of W){let X=ne(M)!==R.element||!R.element.isConnected?"missing":ng(R)?"changed":null;if(X)P.set(M,X),w.delete(M),_=!0}return _}function Oe(){let _=me();for(let[M,R]of P){if(R!=="missing"||D.has(M)||!ne(M))continue;P.delete(M),_=!0}if(_)ke();return _}function oe(_){if(!ue)V();ue++;try{return _()}finally{if(ue--,!ue)ke()}}function Ie(_,M){if(H(M,i.get(_)??[]))a.delete(_);else a.set(_,structuredClone(M))}function G(_,M){if([..._].every(([R,X])=>H(X,M.get(R)??[])))return;if(j.push({before:_,after:M}),j.length>80)j.shift();B=[];for(let[R,X]of M)Ie(R,X)}function ie(_,M){let R=y();for(let X=_.length-1;X>=0;X--){let ge=_[X];if(![...ge.after.keys()].some((q)=>R.includes(q)))continue;return[...ge[M]].every(([q,ce])=>H(N(q),ce))?X:-1}return-1}function de(_){return _.map((M)=>{let R=structuredClone(M),X=fe(R);if(delete R.styleTargetId,X!==R.id)R.styleTargetId=X;if(delete R.styleChanges,N(X).length||a.has(X))R.styleChanges=structuredClone(N(X));return R})}function he(_){let M=nn(_),R=JSON.stringify({annotations:M.annotations.map((X)=>[X.id,X.targets]),targetStyles:M.targetStyles});if(R===ve)return;ve=R,oe(()=>{let X=i;i=new Map,s=new Map;let ge=[...M.annotations].sort((q,ce)=>q.updatedAt.localeCompare(ce.updatedAt));ye(ge.flatMap((q)=>q.targets));for(let q of ge)for(let ce of q.targets){let Te=fe(ce);if(Y(Ye(ce))===Te)i.set(Te,[...new Map([...i.get(Te)??[],...structuredClone(M.targetStyles?.[Ye(ce)]??ce.styleChanges??[])].map((te)=>[te.property,te])).values()]);if(!s.has(Te))s.set(Te,new Set);s.get(Te).add(q.id)}for(let[q,ce]of a)if(H(ce,i.get(q)??[]))a.delete(q);else if(X.has(q)&&!i.has(q))a.delete(q);ee()})}function ee(){for(let[_,M]of n){if(s.has(_)||a.has(_)||h.has(_))continue;let R=e(M);if(R)o.delete(R);n.delete(_),c.delete(_),k.delete(_),w.delete(_),P.delete(_),D.delete(_),f.delete(_);for(let[X,ge]of r)if(ge===_)r.delete(X)}}function $e(_,M,R){return oe(()=>{let X=y();if(!X.length)return!1;if(X.some((ce)=>!L(ce)))return!1;let ge=new Map,q=new Map;for(let ce of X){let Te=ne(ce);if(!Te)return P.set(ce,"missing"),!1;if(P.has(ce))return!1;let te=pe(ce),Ce=ct(_,R),He=structuredClone(N(ce));ge.set(ce,He);let b=new Map(He.map((m)=>[m.property,m])),A=getComputedStyle(Te);for(let m of Ce){let v=M(ce,m);if(v===null||!CSS.supports(m,v))return!1;let I=Ir.safeParse({property:m,before:te[m]??"",value:v});if(!I.success)return!1;if(!w.has(ce)&&A.getPropertyValue(m)!==I.data.before&&A.getPropertyValue(m)!==I.data.value)return P.set(ce,"changed"),!1;if(I.data.value===I.data.before)b.delete(m);else b.set(m,I.data)}q.set(ce,[...b.values()])}return G(ge,q),!0})}return{reset(_=[]){V(),n=new Map,r=new Map,o=new WeakMap,i=new Map,a=new Map,s=new Map,c=new Map,l=[],h=new Set,p=[],d="",u=[],f=new Map,x=!0,k=new Set,w=new Set,P=new Map,D=new Set,T=!1,j=[],B=[],ve="",oe(()=>{ye(_),p=structuredClone(_),l=[...new Set(_.map(fe))],h=new Set(l);for(let M of _)if(M.styleChanges?.length)i.set(fe(M),structuredClone(M.styleChanges))})},sync:he,holdForVariants(_){let M=new Set(_.map(Y));if(M.size===Z.size&&[...M].every((R)=>Z.has(R)))return;Z=M,ke()},register:(_)=>oe(()=>ye(_)),begin(_,M=!1){oe(()=>{if(ye(_),p=structuredClone(_),l=[...new Set(_.map(fe))],d="",!M)h=new Set;for(let R of l)h.add(R)})},clearScope(){l=[],p=[],h.clear(),d="",u=[],ee(),ke()},capture:oe,suspend:()=>{T=!0,V()},resume:()=>{T=!1,ke()},destroy:()=>{z=!0,T=!0,V()},reconcile:Oe,active:()=>d,select(_){d=_,oe(()=>{for(let M of y())pe(M)})},editingTargets:()=>p.filter((_)=>y().includes(fe(_))).map((_)=>_.id),selectScope(_){let M=[...new Set(_.map(Y).filter((R)=>l.includes(R)))];if(M.length===l.length)d="";else if(M.length===1)d=p.find((R)=>fe(R)===M[0]).id;else if(M.length>1)d="__selection__",u=M;else d=p[0]?.id??""},global(_){if(x=_,_)T=!1;ke()},preference:()=>({enabled:x,disabledTargets:[...k]}),restorePreference(_){x=_?.enabled??!0,k=new Set(_?.disabledTargets??[])},preview(_,M=!1){if(z||!x)return!1;if(y().some((R)=>!L(R)||P.get(R)==="missing"))return!1;if(V(),_)T=!1;for(let R of y()){if(_)k.delete(R);else k.add(R);if(M)w.add(R),P.delete(R)}return ke(),!y().some((R)=>P.has(R))},targets:()=>de([...h].map((_)=>n.get(_)).filter((_)=>!!_)),scopeTargets:()=>de(p.filter((_,M,R)=>R.findIndex((X)=>fe(X)===fe(_))===M)),changedTargets:()=>de([...h].filter((_)=>a.has(_)||N(_).length).map((_)=>n.get(_)).filter((_)=>!!_)),drafts:()=>de([...a.keys()].map((_)=>n.get(_)).filter((_)=>!!_)),loadDrafts(_){oe(()=>{ye(_);for(let M of _)Ie(fe(M),M.styleChanges??[])})},links:()=>Object.fromEntries([...r].filter(([_,M])=>_!==M)),cancel(){for(let _ of h)a.delete(_);j=j.filter((_)=>![..._.after.keys()].some((M)=>h.has(M))),B=[],ke()},discardAll(){a.clear(),j=[],B=[],l=[],p=[],h.clear(),ke()},project:de,edit:(_,M,R=!1)=>$e(_,()=>M,R),step:(_,M,R,X=!1)=>$e(_,(ge,q)=>Nd(q,N(ge).find((ce)=>ce.property===q)?.value??pe(ge)[q]??"",M,R),X),remove(_,M=!1){let R=_?ct(_,M):[],X=new Map,ge=new Map;for(let q of y())X.set(q,structuredClone(N(q))),ge.set(q,_?N(q).filter((ce)=>!R.includes(ce.property)):[]);G(X,ge),ke()},history(_){let M=_==="undo"?j:B,R=_==="undo"?B:j,X=ie(M,_==="undo"?"after":"before");if(X<0)return;let[ge]=M.splice(X,1);R.push(ge);for(let[q,ce]of ge[_==="undo"?"before":"after"])Ie(q,ce);ke()},state(){let _=y(),M={},R={},X=[],ge=[],q=[],ce=new Map;for(let te of _)for(let Ce of N(te))ce.set(Ce.property,Ce);for(let te of Nt){let Ce=_.map((b)=>pe(b)[te]??""),He=_.map((b)=>N(b).find((A)=>A.property===te)?.value??pe(b)[te]??"");if(new Set(Ce).size>1)ge.push(te);else M[te]=Ce[0]??"";if(new Set(He).size>1)X.push(te);else R[te]=He[0]??"";if(He.length&&He.every((b)=>Nd(te,b,1,!1)!==null))q.push(te)}let Te=_.filter((te)=>!k.has(te)).length;return{preview:_.length>0&&Te===_.length,previewMixed:Te>0&&Te<_.length,globalPreview:x,globalCount:[...n.keys()].reduce((te,Ce)=>te+N(Ce).length,0),scopeCount:_.length,values:M,current:R,mixed:X,mixedOriginal:ge,stepProperties:q,changes:[...ce.values()],count:[...h].reduce((te,Ce)=>te+N(Ce).length,0),dirty:[...h].some((te)=>a.has(te)),sharedMarkers:Math.max(0,..._.map((te)=>s.get(te)?.size??0)),canUndo:ie(j,"after")>=0,canRedo:ie(B,"before")>=0,problem:_.some((te)=>!L(te)||P.get(te)==="missing")?"missing":_.some((te)=>P.get(te)==="changed")?"changed":null}}}}function cg(e,t){return t=t.trim(),/^-?(?:\d+\.?\d*|\.\d+)$/.test(t)&&!sg.includes(e)?`${t}px`:t}function ug(e,t,n){let r=new Map,o=new Set,i={padding:"all",margin:"all"},a=new Set(["styleSize"]),s=()=>`${e()?.styleTargets.map((d)=>d.styleTargetId??d.id).join(",")}:${e()?.styleTargetId}:`,c=(d,u=!1)=>`${s()}${ct(d,u).join("|")}`,l=(d)=>{for(let u of new Set([...r.keys(),...o]))if(u.startsWith(s())&&u.slice(s().length).split("|").some((f)=>d.includes(f)))r.delete(u),o.delete(u)},p=(d,u,f,x)=>{l(ct(d,x)),t({type:"style-step",property:d,direction:u,coarse:f,linked:x})},h=(d,u,f)=>{let x=c(d,f),k=ct(d,f);l(k),r.set(x,u);let w=cg(d,u);if(!oi.safeParse(w).success||k.some((P)=>!CSS.supports(P,w))){o.add(x),n();return}o.delete(x),t({type:"style-change",property:d,value:w,linked:f})};return{reset(){r=new Map,o=new Set,i.padding="all",i.margin="all"},invalid:()=>o.size>0,tabs(d){let u=st(d.locale);return J`<div class="editor-tabs" role="tablist" aria-label=${u.editFeedback}>
        ${["feedback","styles"].map((f)=>J`<button id=${`ain-${f}-tab`} role="tab" data-action="editor-tab" data-tab=${f} aria-controls=${`ain-${f}-panel`} aria-selected=${String(d.editorTab===f)} tabindex=${d.editorTab===f?0:-1}>${f==="feedback"?u.styleFeedbackTab:u.styleStylesTab}${f==="styles"&&d.styleEditor.count?J`<span class="style-count">${d.styleEditor.count}</span>`:K}</button>`)}
      </div>`},body(d){let u=st(d.locale),f=d.styleEditor,x=(w,P=!1,D=u.styleProperty(w),T=`style-${w}`)=>{let Z=ct(w,P),z=Z.some((fe)=>f.mixedOriginal.includes(fe))||new Set(Z.map((fe)=>f.values[fe]??"")).size>1,W=Z.some((fe)=>f.mixed.includes(fe))||new Set(Z.map((fe)=>f.current[fe]??"")).size>1,j=z?u.styleMixed:f.values[w]??"",B=f.changes.find((fe)=>Z.includes(fe.property)),ue=w.includes("color"),ve=c(w,P),H=r.get(ve)??(W?"":ue?Dd(f.current[w]??"")??f.current[w]??"":f.current[w]??""),Y=ag[w];return J`<div class="style-field">
          <div class="style-label">
            <label for=${T}>${D}</label
            >${B||o.has(ve)?J`<button class="style-reset" data-action="style-reset" data-property=${w} data-style-link=${P===!0?"all":P||"none"} data-field-id=${T} aria-label=${`${u.styleReset}: ${D}`} title=${u.styleBefore(j)}><span class="style-dot" aria-hidden="true"></span></button>`:K}
          </div>
          <div class="style-input">
            ${ue?J`<input type="color" aria-label=${`${u.color}: ${u.styleProperty(w)}`} data-style-property=${w} .value=${Dd(H)??"#000000"} ?disabled=${d.saving||!!f.problem} />`:K}
            ${Y?J`<select
                    id=${T}
                    data-style-property=${w}
                    ?disabled=${d.saving||!!f.problem}
                  >
                    ${f.mixed.includes(w)?J`<option value="" .selected=${H===""} disabled>${u.styleMixed}</option>`:K}
                    ${[...new Set([...f.mixedOriginal.includes(w)?[]:[j],...Y,H])].filter(Boolean).map((fe)=>J`<option value=${fe} .selected=${fe===H}>${fe}</option>`)}
                  </select>`:J`<input
                    id=${T}
                    type="text"
                    ?disabled=${d.saving||!!f.problem}
                    data-style-property=${w}
                    data-style-link=${P===!0?"all":P||"none"}
                    data-numeric=${ue?"false":"true"}
                    .value=${H}
                    placeholder=${W?u.styleMixed:K}
                    aria-invalid=${String(o.has(ve))}
                    title=${ue?K:u.styleNumericHint}
                    autocomplete="off"
                    spellcheck="false"
                  />`}
          </div>
          ${o.has(ve)?J`<span class="style-invalid" role="alert">${u.styleInvalid}</span>`:B?J`<span class="style-before">${u.styleBefore(j)}</span>`:K}
        </div>`},k=(w)=>{let P=w==="padding"?u.stylePadding:u.styleMargin,D=i[w],T=(Z)=>`${w}-${Z}`;return J`<section class="style-spacing" aria-label=${P}>
          <div class="spacing-main">
            ${x(T("top"),!0,P,`style-${w}-all`)}
            <div class="spacing-modes" role="group" aria-label=${`${P}: ${u.styleSpacingMode}`}>
              <button
                type="button"
                data-action="style-spacing-mode"
                data-family=${w}
                data-mode="axes"
                aria-label=${`${P}: ${u.styleSpacingAxes}`}
                title=${`${u.styleSpacingAxes} · ${u.styleSpacingToggleHint}`}
                aria-pressed=${String(D==="axes")}
                ?disabled=${d.saving}
              >
                ${Jn(Im)}
              </button>
              <button
                type="button"
                data-action="style-spacing-mode"
                data-family=${w}
                data-mode="sides"
                aria-label=${`${P}: ${u.styleSpacingSides}`}
                title=${`${u.styleSpacingSides} · ${u.styleSpacingToggleHint}`}
                aria-pressed=${String(D==="sides")}
                ?disabled=${d.saving}
              >
                ${Jn(Am)}
              </button>
            </div>
          </div>
          ${D==="all"?K:J`<div class="style-fields spacing-details">
                  ${D==="axes"?J`${x(T("left"),"horizontal",`${P}: ${u.styleSpacingHorizontal}`,`style-${w}-horizontal`)}${x(T("top"),"vertical",`${P}: ${u.styleSpacingVertical}`,`style-${w}-vertical`)}`:ct(T("top"),!0).map((Z)=>x(Z))}
                </div>`}
        </section>`};return J`<div id="ain-styles-panel" role="tabpanel" aria-labelledby="ain-styles-tab">
        ${d.styleTargets.length>1?J`<select
                class="style-target"
                aria-label=${u.styleTarget}
                data-action="style-target"
              >
                <option value="" .selected=${d.styleTargetId===""}>
                  ${u.styleAllTargets(d.styleTargets.length)}
                </option>
                ${d.styleTargetId==="__selection__"?J`<option value="__selection__" .selected=${!0}>${u.styleSelectedTargets(f.scopeCount)}</option>`:K}
                ${d.styleTargets.map((w)=>J`<option value=${w.id} .selected=${d.styleTargetId===w.id}>${w.label||w.tagName} — ${w.selector}</option>`)}
              </select>`:K}
        <div class="style-preview">
          <label
            ><input
              type="checkbox"
              data-action="style-preview"
              .checked=${f.preview}
              .indeterminate=${f.previewMixed}
              ?disabled=${!f.globalPreview||f.problem==="missing"}
            />${f.scopeCount>1?u.stylePreviewMany(f.scopeCount):u.stylePreview}</label
          ><button
            data-action="style-undo"
            aria-label=${u.undo}
            title=${u.undo}
            ?disabled=${!f.canUndo}
          >
            ${Jn(Zi)}</button
          ><button
            data-action="style-redo"
            aria-label=${u.redo}
            title=${u.redo}
            ?disabled=${!f.canRedo}
          >
            ${Jn(Ri)}
          </button>
        </div>
        ${!f.globalPreview?J`<p class="style-problem">${u.styleGlobalOff}</p>`:K}
        ${f.sharedMarkers>1?J`<p class="style-note">${u.styleShared(f.sharedMarkers)}</p>`:K}
        ${f.problem?J`<p class="style-problem" role="status">
                  ${f.problem==="missing"?u.styleMissing:u.styleChanged}
                </p>
                ${f.problem==="changed"?J`<button data-action="style-force" ?disabled=${!f.globalPreview}>${u.styleForcePreview}</button>`:K}`:K}
        <div class="style-scroll">
          ${ig.map(([w,P])=>J`<details class="style-group" .open=${a.has(w)}>
                <summary data-style-group=${w}>
                  <span class="style-chevron">${Jn(jd)}</span>${u[w]}
                </summary>
                <div class="style-fields">${P.map((D)=>x(D))}</div>
                ${w==="styleSize"?J`${k("padding")}${k("margin")}`:K}
              </details>`)}
        </div>
        <div class="style-restore">
          <button data-action="style-reset" ?disabled=${!f.count&&!o.size}>
            ${u.styleResetAll}</button
          ><span>${u.styleCount(f.count)}</span>
        </div>
      </div>`},handle(d,u){let f=e();if(!f)return!1;let x=u instanceof HTMLInputElement||u instanceof HTMLSelectElement?u:null,k=x?.dataset.styleProperty,w=x?Ld(x):!1;if(k&&(d.type==="input"&&x instanceof HTMLInputElement||d.type==="change"&&x instanceof HTMLSelectElement)){if(!f.saving){let T=x.value;if(x instanceof HTMLSelectElement)x.value=f.styleEditor.current[k]??"";h(k,T,w)}return!0}if(k&&d.type==="blur"){let T=c(k,w);if(!o.has(T)&&r.delete(T))queueMicrotask(n);return!0}if(k&&x instanceof HTMLInputElement&&x.dataset.numeric==="true"&&!f.saving){if(d instanceof WheelEvent){if(x.getRootNode()instanceof ShadowRoot&&x.getRootNode().activeElement!==x||d.ctrlKey||d.metaKey)return!0;let T=x.getBoundingClientRect();if(d.clientX<T.left||d.clientX>=T.right||d.clientY<T.top||d.clientY>=T.bottom)return!0;let Z=d.shiftKey&&Math.abs(d.deltaX)>Math.abs(d.deltaY)?d.deltaX:d.deltaY;if(!Z||!d.shiftKey&&Math.abs(d.deltaX)>Math.abs(d.deltaY))return!0;if(ct(k,w).every((z)=>f.styleEditor.stepProperties.includes(z))&&!o.has(c(k,w)))d.preventDefault(),p(k,Z<0?1:-1,d.shiftKey,w);return!0}if(d instanceof KeyboardEvent&&d.type==="keydown"&&["ArrowUp","ArrowDown"].includes(d.key)){if(ct(k,w).every((T)=>f.styleEditor.stepProperties.includes(T))&&!o.has(c(k,w)))d.preventDefault(),p(k,d.key==="ArrowUp"?1:-1,d.shiftKey,w);return!0}}if(d.type==="change"&&x?.dataset.action==="style-preview"){let T=x.checked;return x.checked=f.styleEditor.preview,t({type:"style-preview",value:T}),!0}if(d.type==="change"&&x?.dataset.action==="style-target")return t({type:"style-target",id:x.value}),!0;if(d.type==="click"&&u.closest("[data-style-group]")){d.preventDefault();let T=u.closest("[data-style-group]").dataset.styleGroup;if(a.has(T))a.delete(T);else a.add(T);return n(),!0}let P=u.closest("button");if(d instanceof KeyboardEvent&&d.type==="keydown"&&P?.dataset.action==="editor-tab"&&["ArrowLeft","ArrowRight","Home","End"].includes(d.key)){d.preventDefault();let T=d.key==="Home"?"feedback":d.key==="End"?"styles":f.editorTab==="feedback"?"styles":"feedback",Z=P.getRootNode();return t({type:"editor-tab",value:T}),Z.querySelector(`#ain-${T}-tab`)?.focus(),!0}if(d.type!=="click"||!P||P.disabled)return!1;let D=P.dataset.action;if(D==="editor-tab")return t({type:"editor-tab",value:P.dataset.tab}),!0;if(!D?.startsWith("style-"))return!1;if(f.saving)return!0;if(D==="style-spacing-mode"){let T=P.dataset.family,Z=P.dataset.mode;l(ct(`${T}-top`,!0)),i[T]=i[T]===Z?"all":Z,n()}else if(D==="style-force")t({type:"style-preview",value:!0,force:!0});else if(D==="style-undo"||D==="style-redo")r.clear(),o.clear(),t({type:"style-history",direction:D==="style-undo"?"undo":"redo"});else if(D==="style-reset"){let T=P.getRootNode(),Z=P.dataset.property,z=Ld(P);if(Z)l(ct(Z,z));else r.clear(),o.clear();if(t(Z?{type:"style-reset",property:Z,linked:z}:{type:"style-reset"}),Z)T.getElementById(P.dataset.fieldId??`style-${Z}`)?.focus({preventScroll:!0})}return!0}}}function pg(e){if(e.localOnly)return K;let t=st(e.locale),n=e.document?.annotations.find((a)=>a.id===e.editingId)?.variants,r=e.document&&ft(e.document).some((a)=>a.id!==e.editingId&&qe(a.variants)),o=e.connection==="connected"&&e.variantsSupported,i=qe(n);return J`<div class="variants-mode">
      <span>${t.variantsTitle}</span
      ><button
        type="button"
        class="variant-switch"
        role="switch"
        aria-label=${t.variantsTitle}
        aria-checked=${i||e.variantsRequested?"true":"false"}
        data-action="variants-toggle"
        ?disabled=${i||e.saving||!e.variantsRequested&&(!o||r)}
      ></button>
    </div>
    <p class="variants-hint">
      ${n?.status==="accepted"?t.variantsAccepted:n?.status==="cancelled"?t.variantsCancelled:r?t.variantsBusy:!o&&!i?e.connection==="connected"?t.variantsUnsupported:t.variantsNeedsConnection:t.variantsHint}
    </p>`}function fg(e){if(e.localOnly)return K;let t=e.document&&ft(e.document).find((u)=>u.id===e.variantAnnotationId),n=t?.variants,r=!!t&&"deletedAt"in t;if(!n||n.status==="completed"&&e.variantPreview.problem!=="cleanup"&&!e.variantConflict)return K;let o=st(e.locale),i=e.variantPreview.problem==="cleanup"?o.variantsCleanup:n.status==="completed"?o.variantsCompleted:n.status==="accepted"?o.variantsAccepted:n.status==="cancelled"?r?o.variantsDeleted:o.variantsCancelled:n.status==="requested"?o.variantsRequested:e.variantPreview.status==="ready"?null:e.variantPreview.status==="switching"?o.variantsSwitching:e.variantPreview.status==="error"?o.variantsBindingError:o.variantsWaiting,a=["published","accepted"].includes(n.status),s=[{id:"original",label:o.variantsOriginal,description:""},...n.manifest?.choices??[]],c=Math.max(0,s.findIndex((u)=>u.id===e.variantPreview.variantId)),l=s[c],p=["requested","published"].includes(n.status)&&!e.variantSaving,h=n.status!=="completed"&&n.status!=="cancelled",d=`${o.variantsGeneration} ${n.manifest?.generation??n.generation}`;return J`<section
    class="variants-controller"
    role="region"
    tabindex="0"
    aria-label=${o.variantsTitle}
    aria-description=${o.styleMovePanel}
    aria-keyshortcuts="ArrowUp ArrowDown ArrowLeft ArrowRight"
  >
    <div class="variants-heading">
      <strong>${o.variantsTitle}</strong>
      <div class="variants-heading-actions">
        <button
          type="button"
          class="variant-heading-button"
          data-action="variant-minimize"
          aria-label=${e.variantMinimized?o.variantsExpand:o.variantsMinimize}
          title=${e.variantMinimized?o.variantsExpand:o.variantsMinimize}
          aria-expanded=${e.variantMinimized?"false":"true"}
          aria-controls="ain-variants-content"
        >
          ${Fe(e.variantMinimized?Di:zm,{"aria-hidden":"true",focusable:"false"})}
        </button>
      </div>
    </div>
    <div id="ain-variants-content" ?hidden=${e.variantMinimized}>
      ${i?J`<p role="status">${i}</p>`:K}
      ${e.connection!=="connected"||e.syncing?J`<p class="variants-hint">${o.variantsSyncPending}</p>`:K}
      ${e.variantConflict?J`<p role="alert">${o.variantsConflict}</p>`:K}
      ${n.manifest?J`<div class="variants-navigation" role="group" aria-label=${o.variantsTitle}>
              <button
                type="button"
                class="variant-step"
                data-variant-step="previous"
                data-variant-id=${s[(c-1+s.length)%s.length].id}
                aria-label=${o.variantsPrevious}
                title=${o.variantsPrevious}
                ?disabled=${!p}
              >
                ${Fe(Sm,{"aria-hidden":"true",focusable:"false"})}
              </button>
              <div
                class="variant-current"
                data-current-variant=${l.id}
                role="status"
                aria-live="polite"
                aria-atomic="true"
              >
                <span class="variant-name" title=${l.description||l.label}
                  >${l.label}</span
                >
                <span
                  class="variant-page"
                  aria-label=${`${d}, ${o.variantsPage} ${c+1}/${s.length}`}
                  >${d} · ${c+1}/${s.length}</span
                >
              </div>
              <button
                type="button"
                class="variant-step"
                data-variant-step="next"
                data-variant-id=${s[(c+1)%s.length].id}
                aria-label=${o.variantsNext}
                title=${o.variantsNext}
                ?disabled=${!p}
              >
                ${Fe(jd,{"aria-hidden":"true",focusable:"false"})}
              </button>
            </div>`:K}
      ${h?J` <details>
              <summary>${o.variantsFeedback}</summary>
              <textarea
                data-variant-feedback
                aria-label=${o.variantsFeedback}
                maxlength="10000"
                rows="2"
                .value=${e.variantFeedback}
                ?disabled=${e.variantSaving}
              ></textarea>
            </details>`:K}
      <div class="variants-actions">
        ${h?J`
                <button
                  type="button"
                  class="primary"
                  data-variant-decision="accept"
                  ?disabled=${n.status!=="published"||e.variantPreview.status!=="ready"||e.variantPreview.generation!==n.generation||e.variantSaving}
                >
                  ${o.variantsAccept}
                </button>
                <button
                  type="button"
                  data-variant-decision="regenerate"
                  ?disabled=${!a||e.variantSaving}
                >
                  ${o.variantsRegenerate}
                </button>
              `:K}
        ${!r?J`<button type="button" data-action="variant-open" title=${o.variantsOpen}>
                ${o.variantsOpen}
              </button>`:K}
        ${h?J`<button type="button" class="danger" data-action="variant-cancel" aria-haspopup="dialog" ?disabled=${e.variantSaving}>${o.variantsCancel}</button>`:K}
      </div>
    </div>
  </section>`}function hg(e,t,n,r){if(!t.closest(".variants-controller"))return!1;if(e.type==="input"&&t instanceof HTMLTextAreaElement)r({type:"variant-feedback",value:t.value});if(e.type==="click"){let o=t.closest("button");if(o&&!o.disabled){if(o.dataset.variantId)r({type:"variant-preview",value:o.dataset.variantId});else if(o.dataset.variantDecision)r({type:"variant-decision",decision:o.dataset.variantDecision});else if(o.dataset.action==="variant-minimize")r({type:"variant-minimized",value:!n.variantMinimized}),o.focus({preventScroll:!0});else if(o.dataset.action==="variant-open")r({type:"edit",id:n.variantAnnotationId})}}return!0}function gg({onAction:e,getRect:t}){let n=document.createElement("div");n.setAttribute("data-ainotation-ui","markers"),n.style.cssText="all:initial!important;position:fixed!important;inset:0!important;pointer-events:none!important;z-index:2147483647!important;display:none!important;";let r=n.attachShadow({mode:"open"}),o=Qm(r,(y)=>e(y));(document.body??document.documentElement).append(n);let i=null,a=!1,s=!1,c=0,l=null,p=null,h=new Set,d=new AbortController,u=null,f=null,x="",k="",w=null,P=null,D='input, textarea, select, button, option, label, summary, a, area, code, pre, q, .style-input, [contenteditable]:not([contenteditable="false"]), [draggable="true"], [tabindex]:not([tabindex="-1"]), [role="button"], [role="checkbox"], [role="switch"], [role="radio"], [role="tab"], [role="slider"], [role="spinbutton"], [role="combobox"], [role="listbox"], [role="menuitem"], [role="link"]';function T(y){let N=w;if(w=null,!N)return;if(N.panel.classList.remove("dragging"),N.panel.hasPointerCapture(N.id))N.panel.releasePointerCapture(N.id);if(N.kind==="variants"&&(!y||N.cancelled))f=N.startPosition;if(y&&N.moved&&!N.cancelled&&N.panel.isConnected){let ne=N.panel.getBoundingClientRect();if(N.kind==="variants")x=JSON.stringify(i?.variantPosition??null);e({type:N.kind==="variants"?"variant-position":"editor-position",position:{x:ne.x,y:ne.y}})}}function Z(y,N){let ne=y.closest(".popover, .variants-controller");if(!ne||y.closest(D)&&y.closest(D)!==ne)return null;for(let L=y;L;L=L.parentElement){if(L instanceof HTMLElement){let Q=L.getBoundingClientRect();if(L.scrollHeight>L.clientHeight&&(N.pointerType==="touch"||N.clientX>=Q.left+L.clientLeft+L.clientWidth))return null;if(L.scrollWidth>L.clientWidth&&N.clientY>=Q.top+L.clientTop+L.clientHeight)return null}if(L===ne)break}return ne}let z=ug(()=>i,(y)=>e(y),()=>{if(i&&!s)fe.update(i,a)}),W=new Set,j=new Set;function B(){if(a&&!s&&!c)c=requestAnimationFrame(()=>{c=0,H()})}let ue=new ResizeObserver(B),ve=new MutationObserver((y)=>{if(y.some((N)=>!Bt(N.target)&&(N.type!=="childList"||[...N.addedNodes,...N.removedNodes].some((ne)=>!Bt(ne)))))B()});function H(){if(!i||!a||s)return;let y=st(i.locale),N=new Set([document]),ne=new Set,L=new Map,Q=i.document?.annotations??[];for(let G of[...Q.flatMap((ie)=>ie.targets),...i.selected]){if(L.has(G))continue;let ie=null;try{let de=t(G);if(de&&[de.x,de.y,de.width,de.height].every(Number.isFinite))ie=de}catch{}L.set(G,ie);try{let de=document,he=!0;for(let _ of G.shadowHosts){let M=de.querySelectorAll(_),R=M.length===1?M[0]:null;if(!R?.shadowRoot||Bt(R)){he=!1;break}de=R.shadowRoot,N.add(de)}if(!he)continue;let ee=de.querySelectorAll(G.selector),$e=ee.length===1?ee[0]:null;if(ie&&$e&&$e.localName===G.tagName&&!Bt($e))ne.add($e)}catch{}}function Ee(G,ie,de){let he=ie.at(-1);if(!G&&he&&de)G={x:he.rect.x+he.rect.width+de.page.viewport.scrollX,y:he.rect.y+he.rect.height+de.page.viewport.scrollY,space:"document",targetId:he.id,ratioX:1,ratioY:1};if(!G)return null;let ee=ie.find((M)=>M.id===G.targetId),$e=ee&&L.get(ee),_=$e&&Number.isFinite(G.ratioX)&&Number.isFinite(G.ratioY);return{x:_?$e.x+$e.width*G.ratioX:G.x-(G.space==="document"?window.scrollX:0),y:_?$e.y+$e.height*G.ratioY:G.y-(G.space==="document"?window.scrollY:0),unavailable:ie.some((M)=>!L.get(M))}}let{left:V,top:pe,width:ye,height:ke}=cn(),me=!i.editingId&&i.marker?Ee(i.marker,i.selected):null,Oe=new Map(Q.map((G)=>[G.id,i.editingId===G.id&&i.targetsAdjusted?Ee(i.marker??void 0,i.selected):Ee(G.marker,G.targets,G)]));for(let G of r.querySelectorAll(".marker")){let ie=G.dataset.annotationId,de=ie?Oe.get(ie):me;if(G.hidden=i.variantsComparing||!de||de.x<V||de.y<pe||de.x>V+ye||de.y>pe+ke,de)G.style.left=`${de.x}px`,G.style.top=`${de.y}px`,G.title=de.unavailable?y.targetUnavailable:ie?y.editFeedback:y.newAnnotation;if(G.hidden&&r.activeElement===G)G.blur()}if(i.editingId)me=Oe.get(i.editingId)??(i.marker?Ee(i.marker,i.selected):null);let oe=r.querySelector(".popover");if(oe){ne.add(oe),oe.style.width=`${Math.max(0,Math.min(320,ye-16))}px`,oe.style.maxHeight=`${Math.max(0,ke-16)}px`;let G=oe.getBoundingClientRect(),ie=me?.x??V+8,de=me?.y??pe+8,he=u?.x??(ie+18+G.width>V+ye-8?ie-18-G.width:ie+18),ee=u?.y??(de+18+G.height>pe+ke-8?de-18-G.height:de+18);oe.style.left=`${Math.max(V+8,Math.min(he,V+ye-G.width-8))}px`,oe.style.top=`${Math.max(pe+8,Math.min(ee,pe+ke-G.height-8))}px`}let Ie=r.querySelector(".variants-controller");if(Ie){ne.add(Ie);let G=ye<960?72:16;Ie.style.width=`${Math.max(0,Math.min(480,ye-16))}px`,Ie.style.maxHeight=`${Math.max(0,ke-G-8)}px`,Ie.style.transform="none",Ie.style.bottom="auto";let ie=Ie.getBoundingClientRect(),de=f?.x??V+(ye-ie.width)/2,he=f?.y??pe+ke-ie.height-G;Ie.style.left=`${Math.max(V+8,Math.min(de,V+ye-ie.width-8))}px`,Ie.style.top=`${Math.max(pe+8,Math.min(he,pe+ke-ie.height-8))}px`}for(let G of W)if(!ne.has(G))ue.unobserve(G),W.delete(G);for(let G of ne)if(!W.has(G))ue.observe(G),W.add(G);if([...j].some((G)=>!N.has(G)))ve.disconnect(),j.clear();for(let G of N)if(!j.has(G))ve.observe(G,{subtree:!0,childList:!0,attributes:!0,characterData:!0}),j.add(G)}function Y(y){if(y.disabled||!i)return;if(y.classList.contains("marker")){if(y.dataset.annotationId)e({type:"edit",id:y.dataset.annotationId});else if(!i.editorOpen)e({type:"open-edit"});else r.querySelector("textarea")?.focus({preventScroll:!0});return}let N=y.dataset.action;switch(N){case"variants-toggle":e({type:"variants-toggle",value:!i.variantsRequested});break;case"target-parent":case"target-back":if(y.dataset.targetId)e({type:"navigate-target",id:y.dataset.targetId,direction:N==="target-parent"?"parent":"back"});break;case"copy-selector":if(y.dataset.targetId)e({type:N,id:y.dataset.targetId});break;case"choose-image":r.querySelector("input[type=file]")?.click();break;case"screenshot":case"save":case"cancel-edit":e({type:N});break;case"delete":if(i.editingId)e({type:N,id:i.editingId});break;case"edit-image":case"remove-image":case"download-image":if(y.dataset.imageId)e({type:N,id:y.dataset.imageId})}}Ym({root:r,signal:d.signal,active:()=>a&&!s,passthrough:()=>!!i?.passthrough,handle(y,N){if(w&&(y.type==="touchstart"||y.type==="touchmove")){y.preventDefault();return}if(w&&y instanceof KeyboardEvent&&y.type==="keydown"&&y.key==="Escape"){if(y.preventDefault(),w.cancelled=!0,w.kind==="variants")f=w.startPosition;else u={x:w.left,y:w.top};H();return}if(w&&y instanceof PointerEvent&&y.pointerId===w.id){if(y.type==="pointermove"){if(y.preventDefault(),y.pointerType==="mouse"&&y.buttons===0){T(!0);return}if(w.cancelled)return;if(!w.moved&&Math.hypot(y.clientX-w.x,y.clientY-w.y)<3)return;w.moved=!0,w.panel.classList.add("dragging");let ne={x:w.left+y.clientX-w.x,y:w.top+y.clientY-w.y};if(w.kind==="variants")f=ne;else u=ne;H();return}if(y.type==="pointerup"||y.type==="pointercancel"||y.type==="lostpointercapture"){if(y.type==="pointerup"&&w.moved)P={x:y.clientX,y:y.clientY,time:performance.now()};T(!0);return}}if(y.type==="click"&&y instanceof MouseEvent&&P){let ne=P;if(P=null,y.detail>0&&performance.now()-ne.time<500&&Math.hypot(y.clientX-ne.x,y.clientY-ne.y)<6){y.preventDefault();return}}if(!w&&y instanceof PointerEvent&&y.type==="pointerdown"&&y.button===0&&y.isPrimary&&!i?.saving){let ne=Z(N,y);if(ne){y.preventDefault();let L=ne.getBoundingClientRect();w={panel:ne,kind:ne.matches(".variants-controller")?"variants":"editor",startPosition:ne.matches(".variants-controller")?f:u,id:y.pointerId,x:y.clientX,y:y.clientY,left:L.x,top:L.y,moved:!1,cancelled:!1},ne.setPointerCapture(y.pointerId);return}}if(o.handle(y,N))return;if(z.handle(y,N))return;if(N.matches(".popover, .variants-controller")&&!w&&y instanceof KeyboardEvent&&y.type==="keydown"&&y.key.startsWith("Arrow")){y.preventDefault();let ne=N.getBoundingClientRect(),L={x:ne.x+(y.key==="ArrowRight"?16:y.key==="ArrowLeft"?-16:0),y:ne.y+(y.key==="ArrowDown"?16:y.key==="ArrowUp"?-16:0)},Q=N.matches(".variants-controller");if(Q)f=L;else u=L;H();let Ee=N.getBoundingClientRect();e({type:Q?"variant-position":"editor-position",position:{x:Ee.x,y:Ee.y}});return}if(i&&hg(y,N,i,e))return;if(y.type==="click"){let ne=N.closest("button");if(ne)Y(ne)}else if(y.type==="keydown"&&y instanceof KeyboardEvent){if(y.isComposing||y.defaultPrevented)return;if(y.key==="Escape")y.preventDefault(),e({type:"cancel-edit"});else if(y.key==="Enter"&&y.metaKey!==y.ctrlKey&&!y.altKey&&!y.shiftKey){y.preventDefault();let ne=r.querySelector(".actions .primary");if(ne&&!y.repeat)Y(ne)}}else if(y.type==="input"&&N instanceof HTMLTextAreaElement)e({type:"draft",value:N.value});else if(y.type==="change"&&N instanceof HTMLInputElement&&N.type==="file"){let ne=N.files?.[0];if(N.value="",ne)e({type:"import-image",file:ne})}else if(y.type==="paste"&&y instanceof ClipboardEvent){let ne=[...y.clipboardData?.files??[]].find((L)=>L.type.startsWith("image/"));if(ne)y.preventDefault(),e({type:"import-image",file:ne})}else if(y instanceof DragEvent&&y.dataTransfer){if(y.type==="dragover"&&y.dataTransfer.types.includes("Files"))y.preventDefault(),y.dataTransfer.dropEffect="copy";else if(y.type==="drop"&&y.dataTransfer.files.length)y.preventDefault(),e({type:"import-image",file:y.dataTransfer.files[0]})}}}),window.addEventListener("blur",()=>T(!0),{signal:d.signal}),document.addEventListener("scroll",B,{capture:!0,signal:d.signal}),window.addEventListener("resize",B,{signal:d.signal}),window.visualViewport?.addEventListener("resize",B,{signal:d.signal}),window.visualViewport?.addEventListener("scroll",B,{signal:d.signal});let fe={update(y,N){if(s)return;if(!N||y.passthrough)T(!1);let ne=JSON.stringify([y.document?.id,y.document?.url]),L=JSON.stringify(y.variantPosition??null);if(k!==ne||w?.kind!=="variants"&&x!==L){if(k!==ne&&w?.kind==="variants")T(!1);f=y.variantPosition?{...y.variantPosition}:null,x=L,k=ne}n.dataset.theme=y.theme,n.lang=y.locale;let Q=st(y.locale),Ee=r.activeElement,V=Boolean(Ee?.closest(".popover")),pe=Ee?.dataset.annotationId,ye=Ee?.dataset.action==="target-parent"||Ee?.dataset.action==="target-back"?Ee.dataset.action:void 0,ke=ye?Ee?.closest("[data-target-index]")?.dataset.targetIndex:void 0,me=p;if(i=y,!N)Ee?.blur();a=N,n.style.setProperty("display",a?"block":"none","important");let Oe=y.document?.annotations??[],oe=y.variantsRequested||!!Oe.find((ee)=>ee.id===y.editingId)?.variants,Ie=y.editingId&&!y.targetsAdjusted&&!y.editorSessionId?Oe.find((ee)=>ee.id===y.editingId)?.targets??y.selected:y.selected,G=y.editorOpen&&Boolean(y.editingId||y.marker),ie=G?y.editingId||y.editorSessionId||(y.targetsAdjusted&&l?l:JSON.stringify([y.selected.map((ee)=>ee.id),y.marker])):null,de=a&&ie!==null&&ie!==l,he=l!==null&&ie===null;if(ie!==l){if(z.reset(),u=y.editorPosition?{...y.editorPosition}:null,w?.kind==="editor")T(!1)}else if(!u&&(y.editorTab==="styles"||y.styleEditor.count)){let ee=r.querySelector(".popover")?.getBoundingClientRect();if(ee)u={x:ee.x,y:ee.y}}if(l=ie,p=G?y.editingId:null,jt(J`
          <style>
            ${mg}
          </style>
          <div class=${`layer${y.passthrough?" passthrough":""}`}>
            ${Md(Oe,(ee)=>ee.id,(ee,$e)=>J`
                <button
                  class="marker"
                  type="button"
                  data-annotation-id=${ee.id}
                  aria-label=${Q.editAnnotation($e+1)}
                >
                  <span class="number">${$e+1}</span>
                  <span class="pencil"
                    >${Fe(Li,{"aria-hidden":"true",focusable:"false"})}</span
                  >
                </button>
              `)}
            ${!y.editingId&&y.marker?J`
                    <button class="marker" type="button" aria-label=${Q.newAnnotation}>
                      ${Fe(Pm,{"aria-hidden":"true",focusable:"false"})}
                    </button>
                  `:K}
            ${G?J`
                    <section
                      class="popover"
                      role="dialog"
                      tabindex="0"
                      aria-description=${Q.styleMovePanel}
                      aria-keyshortcuts="ArrowUp ArrowDown ArrowLeft ArrowRight"
                      aria-label=${y.editingId?Q.editFeedback:Q.newFeedback}
                    >
                      <ul class="target-list" aria-label=${Q.selectedElements}>
                        ${Md(Ie,(ee)=>ee.id,(ee,$e)=>J`<li data-target-index=${$e}>
                              <div class="target-heading">
                                <div class="target-description" title=${ee.text}>
                                  ${eg(ee)}
                                </div>
                                <div class="target-tools">
                                  ${y.targetNavigation[ee.id]?.back?J`<button type="button" data-action="target-back" data-target-id=${ee.id} aria-label=${Q.previousTarget} title=${Q.previousTarget} ?disabled=${y.saving||oe}>${Fe(xm,{"aria-hidden":"true",focusable:"false"})}</button>`:K}
                                  ${y.targetNavigation[ee.id]?.parent?J`<button type="button" data-action="target-parent" data-target-id=${ee.id} aria-label=${Q.parentTarget} title=${Q.parentTarget} ?disabled=${y.saving||oe}>${Fe(_m,{"aria-hidden":"true",focusable:"false"})}</button>`:K}
                                </div>
                              </div>
                              <div class="target-locator">
                                <code title=${ee.selector}>${ee.selector}</code
                                ><button
                                  class="copy-selector"
                                  type="button"
                                  data-action="copy-selector"
                                  data-target-id=${ee.id}
                                  aria-label=${Q.copySelector}
                                  title=${Q.copySelector}
                                >
                                  ${Fe(Mr,{"aria-hidden":"true",focusable:"false"})}
                                </button>
                              </div>
                              <details class="target-details">
                                <summary>${Q.locatorDetails}</summary>
                                <pre>${qd(ee)}</pre>
                                ${ee.ancestors?.length?J`<pre>${ee.ancestors.map((_)=>`${_.tagName}${_.attributes.id?`#${_.attributes.id}`:""}${_.shadowHost?" [shadow host]":""}`).join(" > ")} > ${ee.tagName}</pre>`:K}
                              </details>
                              ${ee.textSelection?J`<q class="text-quote" aria-label=${Q.selectedText}>${ee.textSelection.exact}${ee.textSelection.truncated?"…":""}</q>`:K}
                            </li>`)}
                      </ul>
                      ${z.tabs(y)}
                      <div
                        id="ain-feedback-panel"
                        role="tabpanel"
                        aria-labelledby="ain-feedback-tab"
                        ?hidden=${y.editorTab!=="feedback"}
                      >
                        ${pg(y)}
                        <textarea
                          aria-label=${Q.feedbackContent}
                          aria-keyshortcuts="Meta+Enter"
                          rows="3"
                          maxlength="10000"
                          .value=${y.draft}
                          ?disabled=${y.storage==="loading"}
                        ></textarea>
                        <div class="images" aria-label=${Q.attachedImages}>
                          ${y.images.map((ee,$e)=>J`<div class="image">
                              <button
                                class="image-preview"
                                type="button"
                                aria-label=${Q.editImage($e+1)}
                                ?disabled=${!y.imageUrls[ee.id]}
                                data-action="edit-image"
                                data-image-id=${ee.id}
                              >
                                ${y.imageUrls[ee.id]?J`<img src=${y.imageUrls[ee.id]} alt=${Q.attachedImage($e+1)} />`:Q.image($e+1)}
                              </button>
                              <button
                                class="image-action image-download"
                                type="button"
                                aria-label=${Q.downloadImageNumber($e+1)}
                                title=${Q.downloadImage}
                                ?disabled=${!y.imageUrls[ee.id]}
                                data-action="download-image"
                                data-image-id=${ee.id}
                              >
                                ${Fe(Nr,{"aria-hidden":"true"})}
                              </button>
                              <button
                                class="image-action image-remove"
                                type="button"
                                aria-label=${Q.removeImageNumber($e+1)}
                                title=${Q.removeImage}
                                data-action="remove-image"
                                data-image-id=${ee.id}
                              >
                                ${Fe(Vt,{"aria-hidden":"true"})}
                              </button>
                            </div>`)}
                        </div>
                      </div>
                      ${y.editorTab==="styles"?y.variantStyleBlocked?J`<p class="variants-hint">${Q.variantsStylesPaused}</p>`:z.body(y):K}
                      <div class="actions">
                        <button
                          type="button"
                          aria-label=${Q.screenshot}
                          title=${Q.screenshotHint}
                          ?disabled=${y.saving||y.storage==="loading"||y.images.length>=8}
                          data-action="screenshot"
                          ?hidden=${y.editorTab!=="feedback"}
                        >
                          ${Fe(km,{"aria-hidden":"true",focusable:"false"})}
                        </button>
                        <button
                          type="button"
                          aria-label=${Q.chooseImage}
                          title=${Q.chooseImage}
                          ?disabled=${y.saving||y.storage==="loading"||y.images.length>=8}
                          data-action="choose-image"
                          ?hidden=${y.editorTab!=="feedback"}
                        >
                          ${Fe(Cm,{"aria-hidden":"true",focusable:"false"})}
                        </button>
                        <input type="file" accept="image/png,image/jpeg,image/webp" hidden />
                        <div class="actions-end">
                          <button
                            type="button"
                            aria-label=${Q.cancel}
                            title=${Q.cancel}
                            data-action="cancel-edit"
                          >
                            ${Fe(an,{"aria-hidden":"true",focusable:"false"})}
                          </button>
                          <button
                            class="primary"
                            type="button"
                            aria-label=${y.editingId?Q.save:Q.add}
                            title=${Q.shortcut(y.editingId?Q.save:Q.add,"Command/Super + Enter")}
                            aria-keyshortcuts="Meta+Enter"
                            ?disabled=${!y.draft.trim()&&!y.styleEditor.count&&!y.styleEditor.dirty&&!y.variantsRequested||z.invalid()||y.saving||y.storage==="loading"}
                            data-action="save"
                          >
                            ${Fe(Ni,{"aria-hidden":"true",focusable:"false"})}
                          </button>
                          ${y.editingId?J`<button
                                  class="danger"
                                  type="button"
                                  aria-label=${Q.delete}
                                  title=${Q.delete}
                                  ?disabled=${y.saving}
                                  data-action="delete"
                                >
                                  ${Fe(Vt,{"aria-hidden":"true",focusable:"false"})}
                                </button>`:K}
                        </div>
                      </div>
                      <p class="import-hint" ?hidden=${y.editorTab!=="feedback"}>
                        ${Q.pasteImage}
                      </p>
                      ${y.styleEditor.count||y.styleEditor.dirty?J`<p class="style-note">${Q.styleRestoreOnSave}</p>`:K}
                      ${y.message?J`<p class="message" role="status">${y.message}</p>`:K}
                    </section>
                  `:K}
            ${fg(y)} ${o.template(y)}
          </div>
        `,r),o.update(y,N),w&&!w.panel.isConnected)T(!1);if(a){if(H(),B(),G&&ke!==void 0&&!de){let ee=r.querySelector(`[data-target-index="${ke}"]`);(ee?.querySelector(`[data-action="${ye}"]`)??ee?.querySelector(".target-tools button, .copy-selector"))?.focus({preventScroll:!0})}else if(de)if(y.editorTab==="styles")r.querySelector("[data-style-property]")?.focus({preventScroll:!0});else{let ee=r.querySelector("textarea");ee?.focus({preventScroll:!0}),ee?.setSelectionRange(y.draft.length,y.draft.length)}else if(he&&V||pe){let ee=he?me??Oe.find((_)=>!h.has(_.id))?.id:pe,$e=[...r.querySelectorAll("[data-annotation-id]")].find((_)=>_.dataset.annotationId===ee);if($e&&!$e.hidden)$e.focus({preventScroll:!0})}}else cancelAnimationFrame(c),c=0,ue.disconnect(),ve.disconnect(),W.clear(),j.clear();h=new Set(Oe.map((ee)=>ee.id))},destroy(){if(s)return;T(!1),s=!0,o.destroy(),d.abort(),ue.disconnect(),ve.disconnect(),W.clear(),j.clear(),cancelAnimationFrame(c),c=0,r.activeElement?.blur(),jt(K,r),n.remove(),i=null,h.clear(),l=null,p=null,e=()=>{},t=()=>null}};return fe}async function Rd(e){let t=await Ud("ainotation-settings",1,{upgrade(n){n.createObjectStore("preferences")}});try{return await e(t)}finally{t.close()}}function Jr(e){let t=new Map,n=new Set,r=new Map,o=(i)=>{let a=t.get(i);if(a)return a.value;if(e.cache&&!n.has(i))try{let s=localStorage.getItem(e.cache.key(i)),c=s===null?void 0:e.parse(e.cache.decode(s));if(c!==void 0)return c}catch{}};return{async read(i){let a=o(i);if(a!==void 0)return structuredClone(a);try{let s=await Rd((c)=>c.get("preferences",e.key(i)));return structuredClone(o(i)??e.parse(s))}catch{return structuredClone(o(i))}},write(i,a){let s=e.parse(a);if(s===void 0)return Promise.reject(Error("Invalid preference value"));let c={value:structuredClone(s)};t.set(i,c);let l=!1;if(e.cache)try{localStorage.setItem(e.cache.key(i),e.cache.encode(c.value)),n.delete(i),l=!0}catch{n.add(i);try{localStorage.removeItem(e.cache.key(i)),n.delete(i)}catch{}}let p=(r.get(i)??Promise.resolve()).catch(()=>{}).then(async()=>{try{await Rd((d)=>d.put("preferences",c.value,e.key(i)))}catch(d){if(!l)throw d}if(t.get(i)===c)t.delete(i)});r.set(i,p);let h=()=>{if(r.get(i)===p)r.delete(i)};return p.then(h,h),p}}}function Wd(e){if(!e||typeof e!=="object"||!("left"in e)||!("top"in e)||!("opensLeft"in e))return;if(typeof e.left!=="number"||!Number.isFinite(e.left)||typeof e.top!=="number"||!Number.isFinite(e.top)||typeof e.opensLeft!=="boolean")return;return{left:e.left,top:e.top,opensLeft:e.opensLeft}}async function yg(e){return await Qd.read(e)??ki()}function bg(e,t){return Qd.write(e,t)}async function vg(e){return await Gd.read(e)??"standard"}function wg(e,t){return Gd.write(e,t)}async function $g(e){return await Xd.read(e)??"light"}function xg(e,t){return Xd.write(e,t)}function _g(e){return Yd.read(e)}function kg(e,t){if(!Wd(t))return Promise.reject(Error("Invalid inspector position"));return Yd.write(e,t)}function Sg(e,t,n,r,o,i){let a=!0,s="",c={theme:0,detail:0,locale:0},l=()=>a&&!r.aborted,p=()=>{let f=e.getPosition();if(!f||JSON.stringify(f)===s)return;s=JSON.stringify(f),kg(t,f).catch(()=>{})},h=()=>{if(!a)return;p(),a=!1,e.removeEventListener("ainotation-position-change",p),r.removeEventListener("abort",h)};if(e.addEventListener("ainotation-position-change",p,{signal:r}),r.addEventListener("abort",h,{once:!0}),r.aborted)h();let d=_g(t).then(async(f)=>{if(!f||!l())return;if(await e.updateComplete,!l())return;let x=e.getPosition();if(e.restorePosition(f),!x&&!s)s=JSON.stringify(e.getPosition())??""}),u=yg(t).then((f)=>{if(l()&&c.locale===0)n.locale=f,o()});return vg(t).then((f)=>{if(l()&&c.detail===0)n.outputDetail=f,o()}),$g(t).then((f)=>{if(l()&&c.theme===0)n.theme=f,o()}),{ready:Promise.all([d,u]).then(()=>{}),destroy:h,async update(f){if(!l())return;if(f.type==="set-locale"){if(!Ut(f.value))return;c.locale++,n.locale=f.value,o(),await bg(t,f.value).catch(()=>{if(l())i(we("preferencesUnsaved"))})}else if(f.type==="set-theme"){if(f.value!=="light"&&f.value!=="dark")throw Error("Invalid inspector theme");c.theme++,n.theme=f.value,o(),await xg(t,f.value).catch(()=>{if(l())i(we("preferencesUnsaved"))})}else{let x=ai.parse(f.value);c.detail++,n.outputDetail=x,o(),await wg(t,x).catch(()=>{if(l())i(we("preferencesUnsaved"))})}}}}async function Ig(e,t,n,r){let o=e.then((a)=>{if(!r())throw Error("Inspector was unmounted before feedback could be copied.");let s=a.filter((l)=>l.annotations.length>0||l.variantCleanups?.some((p)=>p.variants.status!=="completed")),c=a.find((l)=>l.url===t.url)??{...t,annotations:[]};return(s.length?s:[c]).map((l)=>di(l,{detail:n})).join(`

---

`)}),i=o.then((a)=>new Blob([a],{type:"text/plain"}));i.catch(()=>{});try{if(typeof ClipboardItem<"u"&&navigator.clipboard?.write)await navigator.clipboard.write([new ClipboardItem({"text/plain":i})]);else await navigator.clipboard.writeText(await o)}catch{throw await o,le("clipboardFailed")}return o}function Cg(){let e=new Map,t=!1,n=(o)=>{clearTimeout(e.get(o)),e.delete(o),URL.revokeObjectURL(o)},r=(o,i)=>{if(t)throw Error("Inspector downloads are closed.");let a=URL.createObjectURL(o);e.set(a,setTimeout(()=>n(a),1e4));try{let s=document.createElement("a");s.href=a,s.download=i,s.click()}catch(s){throw n(a),s}};return{download:r,async exportImages(o,i){let a=[...new Map(o.annotations.flatMap((c)=>c.images??[]).map((c)=>[c.id,c])).values()].map((c)=>{let l=i[c.id];if(!l)throw le("exportImagesPending");return{name:kr(c),blob:l}});a.unshift({name:"feedback.json",blob:new Blob([JSON.stringify(ci(o),null,2)],{type:"application/json"})}),a.unshift({name:"feedback.md",blob:new Blob([di(o)],{type:"text/markdown"})});await Promise.resolve().then(() => gd());r(await md(a),`ainotation-${o.id}.zip`)},exportJson(o){r(new Blob([JSON.stringify(ci(o),null,2)],{type:"application/json"}),`ainotation-${o.id}.json`)},destroy(){if(t)return;t=!0;for(let o of e.keys())n(o)}}}function zg(e,t,n){let r=new URL(e.bridge,location.href);if(r.origin!==location.origin||r.username||r.password||r.search||r.hash||!["http:","https:"].includes(r.protocol))throw Error("Development bridge must be a same-origin HTTP(S) URL.");let o=`${r.href.replace(/\/$/,"")}/api`;return{authority:o,async resolve(i){let a=await fetch(`${r.href.replace(/\/$/,"")}/connect`,{method:"POST",headers:{"X-Ainotation-Client":"1"},credentials:"omit",cache:"no-store",redirect:"error",signal:AbortSignal.any([n,...i?[i]:[],AbortSignal.timeout(15000)])});if(!a.ok)throw Error("Development bridge is unavailable");let s=await a.json();if(s.projectId!==t||s.endpoint!==o||typeof s.token!=="string"||!/^[a-f0-9]{64}$/.test(s.token))throw Error("Development bridge returned a different project or invalid credentials");return{endpoint:o,token:s.token,transport:"same-origin"}}}}function Zd(e,t,n){let r=AbortSignal.any([t,AbortSignal.timeout(5000)]);return new Promise((o,i)=>{let a=!1,s=()=>{a=!0,i(le("captureTimeout"))};if(r.aborted)s();else r.addEventListener("abort",s,{once:!0});e.then((c)=>{if(r.removeEventListener("abort",s),a)n?.(c);else a=!0,o(c)},(c)=>{if(r.removeEventListener("abort",s),!a)a=!0,i(c)})})}function Pg(e){if(!navigator.mediaDevices?.getDisplayMedia)return Promise.reject(le("captureUnavailable"));return navigator.mediaDevices.getDisplayMedia({audio:!1,video:!0,preferCurrentTab:!0,selfBrowserSurface:"include",surfaceSwitching:"exclude"}).then((t)=>{if(e.aborted)t.getTracks().forEach((n)=>n.stop()),e.throwIfAborted();return t})}async function _d(e,t){let n=document.createElement("video");n.muted=!0,n.playsInline=!0,n.srcObject=e;let r=()=>{e.getTracks().forEach((i)=>i.stop()),n.pause(),n.srcObject=null,t.removeEventListener("abort",r)};if(t.addEventListener("abort",r,{once:!0}),t.aborted)r(),t.throwIfAborted();try{await Zd(n.play(),t)}catch(i){throw r(),i}async function o(){let i=performance.now(),a=AbortSignal.any([t,AbortSignal.timeout(5000)]);a.throwIfAborted();let s=new Promise((l,p)=>{let h=0,d=()=>{cancelAnimationFrame(h),p(le("frameWaiting"))},u=()=>{a.removeEventListener("abort",d),l()};a.addEventListener("abort",d,{once:!0}),h=requestAnimationFrame(()=>{h=requestAnimationFrame(u)})}),c=new Promise((l,p)=>{let h=0,d,u=()=>{if(h)n.cancelVideoFrameCallback(h);clearTimeout(d),a.removeEventListener("abort",f)},f=()=>{u(),p(le("frameWaiting"))},x=()=>{u(),l()};a.addEventListener("abort",f,{once:!0});let k=(w,P)=>{if(P.captureTime!==void 0&&P.captureTime<i){h=n.requestVideoFrameCallback(k);return}x()};if(n.requestVideoFrameCallback)h=n.requestVideoFrameCallback(k);if(!n.requestVideoFrameCallback||e.getVideoTracks()[0]?.getSettings().displaySurface!=="browser")d=setTimeout(x,200)});await Promise.all([s,c])}return{stop:r,async snapshot(i){if(e.getVideoTracks()[0]?.readyState!=="live")throw le("captureStopped");if(i&&e.getVideoTracks()[0]?.getSettings().displaySurface!=="browser")throw le("cropNeedsTab");await o(),t.throwIfAborted();let a=globalThis.ImageCapture,s=a?await Zd(Promise.resolve().then(()=>new a(e.getVideoTracks()[0]).grabFrame()),t,(l)=>l.close()).catch((l)=>{if(t.throwIfAborted(),e.getVideoTracks()[0]?.readyState==="live"&&n.readyState>=HTMLMediaElement.HAVE_CURRENT_DATA)return null;throw new Dr(we("frameFailed"),{cause:l})}):null,c;try{t.throwIfAborted(),c=Bn(s??n,s?.width??n.videoWidth,s?.height??n.videoHeight,i)}finally{s?.close()}try{return await Vi(c,t)}finally{c.width=0,c.height=0}}}}async function ep(e,t,n){let r=Vr(),o=Lr(ki());r.locale=o.locale;let i=!1,a=null,s=location.href,c=t.projectId||location.origin,l=t.mcp===!1;if(l&&t.development)throw Error("Local-only mode (mcp: false) cannot use a development bridge.");if(t.development&&t.mcp)throw Error("Choose either a development bridge or a manual MCP connection.");let p=t.development?zg(t.development,c,n):void 0;if(r.managedConnection=!!p,r.localOnly=l,l)r.endpoint="";r.projectName=t.development?.projectName??"";let h=(m)=>JSON.stringify([c,m]),d=h(s),u=0,f=0,x=!1,k=0,w=!1,P=null,D=null,T=new Map,Z,z,W,j={};function B(){if(!r.editorSessionId)return;j[r.editorSessionId]={tab:r.editorTab,targets:z?.editingTargets()??r.selected.map((m)=>m.id),position:r.editorPosition?{...r.editorPosition}:null,navigation:{slots:oe.getNavigation(),referenceIds:(a?.document.annotations.find((m)=>m.id===r.editorSessionId)?.targets??Q(r.selected)).map((m)=>m.id)}}}function ue(m){r.editorSessionId=m;let v=j[m]??a?.editorViews?.[m];if(r.editorTab=v?.tab??"feedback",r.editorPosition=v?.position?{...v.position}:null,v)z?.selectScope(v.targets)}function ve(m,v,I=!1){let E=(j[v]??a?.editorViews?.[v])?.navigation,g=I?m.map((F)=>F.id):E?.referenceIds??[],C=I?E?.slots.map((F)=>F.target.id)??[]:m.map((F)=>F.id),S=E&&g.length===C.length&&g.every((F)=>C.includes(F));w=!0;try{if(S){let F=new Map(m.map((se)=>[se.id,se])),U=E.slots.map((se)=>({target:structuredClone(F.get(se.target.id)??se.target),history:se.history.map((re)=>structuredClone(F.get(re.id)??re))}));oe.restoreNavigation(U),r.selected=U.map((se)=>se.target)}else oe.restoreNavigation(m.map((F)=>({target:F,history:[]}))),r.selected=structuredClone(m)}finally{w=!1}}let H=null,Y=!1,fe,y=null,N,ne=()=>{},L;function Q(m){let v=z?.project(m)??structuredClone(m);for(let I of z?.changedTargets()??[])if(!v.some((E)=>E.id===I.id))v.push(I);return v}let Ee=`ainotation:mcp:${c}`,V=()=>{if(!i&&!n.aborted){if(o.setLocale(r.locale),r.messageDescriptor)r.message=mt(r.locale,r.messageDescriptor);if(z)z.reconcile(),r.styleTargetId=z.active(),r.styleTargets=z.scopeTargets(),r.styleEditor=z.state();let m=a&&ft(a.document).find((E)=>qe(E.variants)),v=m?.variants;r.variantsComparing=!l&&v?.status==="published"&&v.manifest?.generation===v.generation,ne(r.theme,r.variantsComparing);let I=W?.elements()??[];r.variantStyleBlocked=r.variantsRequested||!!(r.editingId&&qe(a?.document.annotations.find((E)=>E.id===r.editingId)?.variants))||!!m?.variants?.targetIds.some((E)=>r.selected.some((g)=>g.id===E))||!!m&&r.selected.some((E)=>{let g=oe.getElement(E);return!!g&&I.some((C)=>jr([C,g]))}),r.variantPreview=W?.state()??r.variantPreview;for(let[E,g]of T)if(!r.images.some((C)=>C.id===E))URL.revokeObjectURL(g),T.delete(E);for(let E of r.images){let g=a?.images?.[E.id];if(g&&!T.has(E.id))T.set(E.id,URL.createObjectURL(g))}r.imageUrls=Object.fromEntries(T),e.view={...r,selected:[...r.selected],availability:{...r.availability}},Z?.update(e.view,!D&&e.expanded&&r.storage!=="loading")}},pe=(m)=>{if(typeof m==="string")delete r.messageDescriptor;else r.messageDescriptor=m;r.message=mt(r.locale,m),V()},ye=(m)=>pe(sn(m));V();let ke=Sg(e,c,r,n,V,pe),me=await jm({onUnavailable(){r.storage="unavailable",pe(we("storageUnavailable"))},onExternalChange(){let m=u;if(Y&&!i)me.read(d,s).then((v)=>{if(m===u&&!i)ie(v)}).catch(ye)}});if(n.aborted)throw await me.close(),new DOMException("Mount canceled","AbortError");Y=!0;function Oe(){r.targetNavigation=Object.fromEntries(r.selected.map((m)=>[m.id,oe.targetNavigation(m.id)])),r.availability=Object.fromEntries([...r.selected,...r.styleTargets,...a?.document.annotations.flatMap((m)=>m.targets)||[]].map((m)=>[m.id,oe.availability(m)]))}let oe=Rm({onOutsideClick(){if(!r.editorOpen)return!1;if(!x&&!D)Ce({type:"close-edit"}).catch(ye);return!0},capture:(m)=>z?z.capture(m):m(),visible:e.expanded,appearance:{theme:r.theme,cssText:Ft.cssText},onChange(){if(i||w)return;if(A())return;if(!r.editorOpen)r.selected=oe.getTargets();Oe(),V()},onPickingChange(m){r.picking=m,V()},onPassthroughChange(m){r.passthrough=m,V()},onSelect(m,v){if(i||A()||r.storage==="loading")return;let I=!r.editingId&&!!r.marker;if(I&&Q(v).length>20){oe.setTargets(r.selected),pe(we("styleTargetLimit"));return}k++,B();let E=r.editingId||!r.editorSessionId?crypto.randomUUID():r.editorSessionId;if(r.targetsAdjusted=!1,r.editingId)r.draft="",r.images=[],r.variantsRequested=!1;r.editingId=null,r.selected=v,z?.begin(v,I),ue(E),P=fn(),r.marker=m,r.editorOpen=!0,r.message="",delete r.messageDescriptor,Oe(),V(),he().catch(ye)},onBatchChange(m){if(i)return;if(!m){if(!oe.getTargets().length&&a&&r.storage!=="loading")he().catch(ye);return}if(k++,B(),r.editorSessionId="",r.editorPosition=null,r.targetsAdjusted=!1,r.editingId)r.draft="",r.images=[],r.variantsRequested=!1;r.editingId=null,r.editorOpen=!1,r.marker=null,P=null,V()},onCancel(){Ce({type:"cancel-edit"}).catch(ye)}});z=og((m)=>oe.getElement(m),V),W=nd({onChange(){if(!i&&!A())V()},onReport(m){if(i||A())return!1;let v=a?.document.annotations.find((E)=>E.id===r.variantAnnotationId),I=v?.variants;if(!v||!I||!r.variantsSupported||i)return!1;ee({id:crypto.randomUUID(),kind:"variants",annotationId:v.id,explorationId:I.id,generation:I.generation,revision:I.revision,action:{type:"report",report:m}}).catch(ye)}}),ne=(m,v)=>{oe.setTheme(m),oe.setSuspended(v)},Z=gg({onAction:(m)=>{Ce(m).catch(ye)},getRect:(m)=>W?.rect(m.id)??oe.getRect(m)});function Ie(m){let v=m.at(-1);if(!v)return null;let I=oe.getRect(v)??v.rect;return{x:I.x+I.width+scrollX,y:I.y+I.height+scrollY,space:"document",targetId:v.id,ratioX:1,ratioY:1}}function G(m=!1){if(B(),m)z?.cancel();z?.clearScope(),r.editorTab="feedback",r.editorSessionId="",r.editorPosition=null,D?.abort(),k++,r.targetsAdjusted=!1,r.targetNavigation={},P=null,r.selected=[],r.draft="",r.variantsRequested=!1,r.images=[],r.editingId=null,r.editorOpen=!1,r.marker=null,oe.clear(),V()}function ie(m){if(r.recoveryNeeded=!!m.syncRecovery,m.syncRecovery?.server)r.recoveredDocument=m.syncRecovery.server;else delete r.recoveredDocument;if(r.hasRecoveryCopy=!!m.recoveryCopies?.length,i)return;if(A()||m.document.url!==s)return;let v=a?.document.annotations.find((S)=>S.id===r.editingId)?.variants,I=m.document.annotations.find((S)=>S.id===r.editingId)?.variants;if(qe(v)&&I?.status==="completed")r.variantsRequested=!1;let E=a&&ft(a.document).find((S)=>S.id===r.variantAnnotationId)?.variants?.id,g=m.document.variantCleanups?.some((S)=>S.id===r.variantAnnotationId&&a?.document.annotations.some((F)=>F.id===S.id));a=m,r.variantPosition=m.variantPosition?{...m.variantPosition}:null;let C=ft(m.document).find((S)=>qe(S.variants))??ft(m.document).find((S)=>S.id===r.variantAnnotationId&&S.variants);if(r.variantAnnotationId!==(C?.id??"")||g||E!==C?.variants?.id)r.variantMinimized=!1;if(r.variantAnnotationId=C?.id??"",r.variantFeedback=m.variantFeedback?.explorationId===C?.variants?.id?m.variantFeedback?.text??"":"",W?.sync(l?void 0:C?.variants),z?.holdForVariants(C&&qe(C.variants)?C.variants.targetIds:r.variantsRequested?r.selected.map((S)=>S.id):[]),z?.sync(m.document),r.document=m.document,r.storage=me.available?"ready":"unavailable",!r.editorOpen&&!r.draft&&m.draft.text&&!m.draft.editorOpen)r.draft=m.draft.text,r.images=structuredClone(m.draft.images??[]),r.messageDescriptor=we("remoteDeleted");if(r.editingId&&!m.document.annotations.some((S)=>S.id===r.editingId))D?.abort(),z?.cancel(),z?.clearScope(),r.editingId=null,r.editorSessionId="",r.editorPosition=null,r.editorOpen=!1,r.variantsRequested=!1,r.marker=null,oe.clear(),r.messageDescriptor=we("annotationDeleted");Oe(),V()}function de(){let m=a?.document.annotations.find((v)=>v.id===r.editingId)?.variants;return{text:r.draft,...r.variantsRequested?{variantsRequested:!0}:{},...r.variantsRequested&&m?.status==="completed"?{variantRequestBase:m.id}:{},images:structuredClone(r.images),editingId:r.editingId,targets:r.editorOpen?Q(r.selected).slice(0,r.selected.length):oe.getTargets(),...r.marker?{marker:r.marker}:{},...P?{page:P}:{},editorOpen:r.editorOpen,...r.targetsAdjusted?{targetsAdjusted:!0}:{},...r.editorSessionId?{editorSessionId:r.editorSessionId}:{},...z?.changedTargets().length?{styleTargets:z.changedTargets()}:{}}}async function he(){B();let m=u,v=de(),I=z?.drafts()??[],E=z?.preference(),g=structuredClone(j),C=await me.update(d,s,(S)=>({...S,draft:v,styleDrafts:I,...E?{stylePreview:E}:{},editorViews:Object.fromEntries(Object.entries({...S.editorViews,...g}).filter(([F])=>F===v.editorSessionId||S.document.annotations.some((U)=>U.id===F)))}));if(m===u)ie(C)}async function ee(m,v){let I=u,E=await me.mutate(d,s,m,v);if(I===u&&!i)ie(E),L?.request()}async function $e(m){L?.stop(),L=void 0;let v=++f;if(l||!a||!y&&!p||i)return;let I=d,E=s,g=y;r.connection="connecting",r.variantsSupported=!1,V();let C=await me.bindAuthority(I,E,p?.authority??g.endpoint);if(v!==f||i)return;if(ie(C),v!==f||i)return;L=Ad({connection:p?(S)=>p.resolve(S):g,sessionId:C.document.id,recovery:m,async onRecovery(S){if(v!==f||i||!S.storageEpoch||!S.recovery)return;let F=await me.update(I,E,(U)=>({...U,syncRecovery:{epoch:S.storageEpoch,revision:S.recovery.revision,server:S.document}}));if(v===f&&!i)ie(F)},read:()=>me.read(I,E),async apply(S,F){if(v!==f||i)return;let U=await me.applySync(I,E,S,F);if(v===f&&!i)ie(U);if(S.variantConflicts?.length&&v===f&&!i)r.variantConflict=!0,pe(we("variantsConflict"))},onVariantsSupport(S){if(v===f&&!i)r.variantsSupported=S,V()},onState(S,F){if(v===f&&!i){if(r.connection=S,S==="error")r.syncProblem=F;else delete r.syncProblem;pe(F)}},onSync(S){if(v===f&&!i)r.syncing=S,V()}})}async function _(){L?.stop(),L=void 0,f++;let m=++u;r.storage="loading",V();let v=await me.load(d,s);if(m!==u||i||n.aborted)return;a=v,j=structuredClone(v.editorViews??{}),r.draft=v.draft.text,r.variantsRequested=v.draft.variantsRequested??!1,r.images=structuredClone(v.draft.images??[]),r.editingId=v.draft.editingId&&v.document.annotations.some((g)=>g.id===v.draft.editingId)?v.draft.editingId:null;let I=v.document.annotations.find((g)=>g.id===r.editingId)?.variants;if(I?.status==="completed"&&v.draft.variantRequestBase!==I.id)r.variantsRequested=!1;let E=r.editingId??v.draft.editorSessionId??(v.draft.editorOpen??v.draft.targets.length>0?crypto.randomUUID():"");ve(v.draft.targets,E,!0),r.marker=v.draft.marker??Ie(v.draft.targets),r.editorOpen=v.draft.editorOpen??v.draft.targets.length>0,r.targetsAdjusted=v.draft.targetsAdjusted??!1,z?.reset(),z?.restorePreference(v.stylePreview),z?.sync(v.document),z?.loadDrafts(v.styleDrafts??v.draft.styleTargets??[]),z?.begin([...v.document.annotations.find((g)=>g.id===r.editingId)?.targets??[],...v.draft.styleTargets??[]]),z?.begin(r.selected,!0),ue(E),P=v.draft.page??(r.editorOpen?fn():null),oe.setVisible(e.expanded),ie(v),oe.setPicking(H??e.expanded),H=null,await $e()}function M(){if(l)return;N?.abort(),L?.stop(),L=void 0,y=null,f++;try{sessionStorage.removeItem(Ee)}catch{}r.connection="offline",r.syncing=!1,delete r.syncProblem,pe(we("localOnly"))}async function R(m){if(l)return;N?.abort(),y=qn(m),r.endpoint=y.endpoint;try{sessionStorage.setItem(Ee,JSON.stringify(y))}catch{}await $e()}let X=Cg();async function ge(){if(l||r.recoveringProject||!y&&!p)return;let m=new AbortController;N=m;let v=AbortSignal.any([n,m.signal]),I=y;r.recoveringProject=!0,r.recoveryPages=[],V();try{let E=await me.readProjectDocuments(c);for(let g=0;g<E.length&&!i&&!v.aborted;g++){let C=E[g],S=h(C.url),F=await me.bindAuthority(S,C.url,p?.authority??I.endpoint);if(i||v.aborted)return;let U=!1,se=Ad({connection:p?(be)=>p.resolve(be):I,sessionId:F.document.id,once:!0,read:()=>me.read(S,C.url),async apply(be,Pe){let Ne=await me.applySync(S,C.url,be,Pe);if(S===d&&!i)ie(Ne)},async onRecovery(be){if(!be.storageEpoch||!be.recovery)return;let Pe=await me.update(S,C.url,(Ne)=>({...Ne,syncRecovery:{server:be.document,epoch:be.storageEpoch,revision:be.recovery.revision}}));if(S===d&&!i)ie(Pe)},onState(be){if(be==="error")U=!0},onSync(){}}),re=()=>se.stop();v.addEventListener("abort",re,{once:!0});try{await se.finished}finally{se.stop(),v.removeEventListener("abort",re)}if(i||v.aborted)return;if(U)r.recoveryPages.push(C.url);pe(we("syncProjectProgress",g+1,E.length))}if(r.recoveryPages.length)pe(we("syncProjectReview"))}finally{if(N===m)N=void 0;r.recoveringProject=!1,V()}}async function q(m){if(!r.editorOpen||x||D)return;let v=r.marker,I=r.selected.find((re)=>re.id===m.id),E=I&&oe.getRect(I),g=v?{x:E&&v.targetId===m.id&&Number.isFinite(v.ratioX)?E.x+E.width*v.ratioX:v.x-(v.space==="document"?scrollX:0),y:E&&v.targetId===m.id&&Number.isFinite(v.ratioY)?E.y+E.height*v.ratioY:v.y-(v.space==="document"?scrollY:0)}:null,C=r.selected.findIndex((re)=>re.id===m.id);if(C<0)return;let S=!1,F=!1,U=z?.editingTargets()??[];w=!0;try{z?.capture(()=>{if(S=oe.navigateTarget(m.id,m.direction,(re)=>(F=Q(re).length>20,!F)),!S)return;r.selected=oe.getTargets(),z.begin(r.selected,!0),z.selectScope(U.map((re)=>re===m.id?r.selected[C].id:re))})}finally{w=!1}if(!S){if(F)pe(we("styleTargetLimit"));return}k++,r.message="",delete r.messageDescriptor;let se=r.document?.annotations.find((re)=>re.id===r.editingId);if(r.targetsAdjusted=!se||se.targets.length!==r.selected.length||se.targets.some((re,be)=>re.id!==r.selected[be]?.id),P=fn(),v&&g&&v.targetId===m.id){let re=r.selected[C],be=oe.getRect(re)??re.rect;r.marker={...v,targetId:re.id,x:g.x+(v.space==="document"?scrollX:0),y:g.y+(v.space==="document"?scrollY:0),ratioX:be.width?(g.x-be.x)/be.width:void 0,ratioY:be.height?(g.y-be.y)/be.height:void 0}}Oe(),V(),await he()}function ce(m){let v=m.map((I)=>oe.getElement(I));if(!v.length||v.some((I)=>!(I instanceof Element)||!I.getBoundingClientRect().width||!I.getBoundingClientRect().height)||jr(v.filter((I)=>I instanceof Element)))throw le("variantsInvalidTargets")}async function Te(m){if(!a)throw le("feedbackLoading");if(m.type==="variant-minimized"){r.variantMinimized=m.value,V();return}if(m.type==="variant-position"){if(!Number.isFinite(m.position.x)||!Number.isFinite(m.position.y))return;let v=u,I=await me.update(d,s,(E)=>({...E,variantPosition:{...m.position}}));if(v===u&&!i)ie(I);return}if(m.type==="variants-toggle"){if(x||qe(a.document.annotations.find((v)=>v.id===r.editingId)?.variants))return;if(m.value){if(l||!r.variantsSupported||r.connection!=="connected")throw le("variantsNeedsConnection");if(ft(a.document).some((v)=>qe(v.variants)))throw le("variantsBusy");ce(r.selected)}r.variantsRequested=m.value,k++,z?.holdForVariants(m.value?r.selected.map((v)=>v.id):[]),V(),await he();return}if(m.type==="variant-preview"){W?.select(m.value),r.variantConflict=!1,V();return}if(m.type==="variant-feedback"){let v=a.document.annotations.find((g)=>g.id===r.variantAnnotationId)?.variants?.id;if(!v)return;r.variantFeedback=m.value.slice(0,1e4);let I=u,E=await me.update(d,s,(g)=>({...g,variantFeedback:{explorationId:v,text:m.value.slice(0,1e4)}}));if(I===u)ie(E);return}if(m.type==="variant-decision"){if(r.variantSaving||l)return;let v=a.document.annotations.find((se)=>se.id===r.variantAnnotationId),I=v?.variants;if(!v||!I)return;let E=W.state();if(m.decision==="accept"&&(E.status!=="ready"||E.generation!==I.generation))throw le("variantsBindingError");let g=m.decision==="accept"?{type:"accept",variantId:E.variantId,feedback:r.variantFeedback}:{type:m.decision,feedback:r.variantFeedback};r.variantSaving=!0,r.variantConflict=!1;let C=r.variantFeedback,S=u,F=d,U=s;V();try{await ee({id:crypto.randomUUID(),kind:"variants",annotationId:v.id,explorationId:I.id,generation:I.generation,revision:I.revision,action:g});let se=await me.update(F,U,(re)=>{if(re.variantFeedback?.explorationId===I.id&&re.variantFeedback.text===C)delete re.variantFeedback;return re});if(S!==u||i)return;ie(se),pe(we("variantsDecisionSaved"))}finally{r.variantSaving=!1,V()}return}}async function te(){if(A())throw le("pageLoading");if(!a)throw le("feedbackLoading");let m=await Ig(me.readProjectDocuments(c),a.document,r.outputDetail,()=>!i&&!n.aborted);return pe(we("copied")),m}async function Ce(m){if(i||n.aborted)return;if(m.type==="set-theme"||m.type==="set-output-detail"||m.type==="set-locale"){await ke.update(m);return}if(m.type==="set-picking"){if(!m.value)D?.abort();H=r.storage==="loading"?m.value:null,oe.setVisible(m.value&&r.storage!=="loading"),oe.setPicking(m.value&&r.storage!=="loading"),V();return}if(A()||r.storage==="loading"){pe(we("pageLoading"));return}if(m.type==="variants-toggle"||m.type==="variant-preview"||m.type==="variant-feedback"||m.type==="variant-position"||m.type==="variant-minimized"||m.type==="variant-decision"){await Te(m);return}if(m.type==="connect"){if(p)return;await R(m);return}if(m.type==="retry-sync"||m.type==="resolve-recovery"){if(l)return;let I=a?.syncRecovery;await $e(m.type==="resolve-recovery"&&I?{epoch:I.epoch,revision:I.revision,source:m.source}:void 0);return}if(m.type==="recover-project"){await ge();return}if(m.type==="export-recovery"){let I=a?.recoveryCopies?.at(-1);if(I)if(I.document.annotations.some((E)=>E.images?.length))await X.exportImages(I.document,I.images);else X.exportJson(I.document);return}if(m.type==="disconnect"){if(p)return;M();return}if(m.type==="draft"){k++,r.draft=m.value.slice(0,1e4),V(),await he();return}if(!a)throw le("feedbackLoading");if(m.type==="editor-tab"){r.editorTab=m.value,V(),await he();return}if(m.type==="editor-position"){if(!Number.isFinite(m.position.x)||!Number.isFinite(m.position.y))return;r.editorPosition={...m.position},await he();return}if(m.type==="style-target"){z?.select(m.id),V(),await he();return}if(m.type==="global-style-preview"){z?.global(m.value),V(),await he();return}if(m.type==="style-preview"){if(r.variantStyleBlocked)throw le("variantsStylesPaused");if(r.editorOpen&&!x&&!D)z?.preview(m.value,m.force);V(),await he();return}if(m.type==="style-change"||m.type==="style-step"||m.type==="style-reset"||m.type==="style-history"){if(!r.editorOpen||x||D||!z)return;if(r.variantStyleBlocked)throw le("variantsStylesPaused");if(m.type==="style-change"&&!z.edit(m.property,m.value,m.linked)){pe(we("styleInvalid"));return}if(m.type==="style-step"&&!z.step(m.property,m.direction,m.coarse,m.linked)){pe(we("styleInvalid"));return}if(m.type==="style-reset")z.remove(m.property,m.linked);if(m.type==="style-history")z.history(m.direction);k++,V(),await he();return}if(m.type==="navigate-target"){if(r.variantsRequested||a.document.annotations.find((I)=>I.id===r.editingId)?.variants)throw le("variantsInvalidTargets");await q(m);return}if(m.type==="screenshot"||m.type==="import-image"||m.type==="edit-image"){if(D||x||!r.editorOpen)return;if(m.type!=="edit-image"&&r.images.length>=vu)throw le("imageLimit");let I=new AbortController;D=I;let E=AbortSignal.any([n,I.signal]),g=u,C=k,S=e.style.visibility,F=()=>{if(D!==I)return;if(D=null,e.style.visibility=S,!i&&g===u)oe.setVisible(e.expanded),oe.setPicking(e.expanded&&r.storage!=="loading"),V()};E.addEventListener("abort",F,{once:!0});let U;try{let se=m.type==="screenshot"?Pg(E):null;if(oe.setPicking(!1),oe.setVisible(!1),e.style.visibility="hidden",V(),se)U=await se;E.throwIfAborted();let re=m.type==="import-image"?m.file:m.type==="edit-image"?a.images?.[m.id]:void 0;if(!U&&!re)throw le("imagePending");await Promise.resolve().then(() => $d());E.throwIfAborted(),await wd({i18n:o,source:U?{stream:U}:{blob:re},theme:r.theme,signal:E,async onSave(Pe){let Ne=await Fm(Pe,m.type==="screenshot"?"screen":"import");if(E.throwIfAborted(),A()||g!==u||C!==k)throw le("changedWhileDrawing");let ot=r.images;r.images=m.type==="edit-image"?ot.map((Ge)=>Ge.id===m.id?Ne:Ge):[...ot,Ne];try{let Ge=de(),qr=await me.update(d,s,(it)=>(E.throwIfAborted(),{...it,draft:Ge,images:{...it.images,[Ne.id]:Pe}}));if(g===u&&!i)ie(qr),r.messageDescriptor=we("imageAttached")}catch(Ge){if(g===u&&C===k&&!i)r.images=ot;throw Ge}},onClose:()=>I.abort()})}catch(se){U?.getTracks().forEach((be)=>be.stop());let re=E.aborted||se instanceof DOMException&&["NotAllowedError","AbortError"].includes(se.name);if(I.abort(),!re&&g===u)throw se}return}if(m.type==="remove-image"){r.images=r.images.filter((I)=>I.id!==m.id),k++,V(),await he();return}if(m.type==="download-image"){let I=a.images?.[m.id];if(!I)throw le("imagePending");X.download(I,`${m.id}.png`);return}if(m.type==="copy"){await te();return}if(m.type==="copy-selector"){let I=[...r.selected,...a.document.annotations.flatMap((E)=>E.targets)].find((E)=>E.id===m.id);if(!I)throw le("missingAnnotation");try{await navigator.clipboard.writeText(qd(I))}catch{throw le("clipboardFailed")}pe(we("selectorCopied"));return}if(m.type==="export"){if(a.document.annotations.flatMap((I)=>I.images??[]).length)await X.exportImages(a.document,a.images??{});else X.exportJson(a.document);pe(we("exported"));return}if(m.type==="cancel-edit"){G(!0),await he();return}if(m.type==="close-edit"){D?.abort(),r.editorOpen=!1,V(),await he();return}if(m.type==="open-edit"){r.editorOpen=!!r.marker,V(),await he();return}if(m.type==="clear-all"){if(x)return;x=!0,r.saving=!0;let I=u,E=k,g=a.document.annotations.map((C)=>C.id);V();try{z?.discardAll();let C=await me.clearAnnotations(d,s,g);if(I===u&&!i){if(E===k)G();ie(C),L?.request(),pe(we("cleared"))}}finally{x=!1,r.saving=!1,V()}return}if(m.type==="save"){if(x)return;let I=r.variantsRequested&&!qe(a.document.annotations.find((U)=>U.id===r.editingId)?.variants);if(I){let U=a.document.annotations.find((se)=>se.id===r.editingId);ce(Q(r.targetsAdjusted?r.selected:U?.targets??r.selected))}if(I&&(l||!r.variantsSupported||r.connection!=="connected"))throw le("variantsNeedsConnection");if(I&&ft(a.document).some((U)=>qe(U.variants)))throw le("variantsBusy");x=!0,r.saving=!0,V();let E=de(),{text:g,editingId:C}=E,S=k,F=u;try{let U=a.document.annotations.find((be)=>be.id===C);if(C&&!U){r.editingId=null,r.editorSessionId=crypto.randomUUID(),await he(),pe(we("originalDeleted"));return}let se=new Date().toISOString(),re=en.parse({id:U?.id||r.editorSessionId||crypto.randomUUID(),comment:g.trim()||(r.variantsRequested?mt(r.locale,we("variantsEmptyComment")):"")||(z&&(z.state().count||z.state().dirty)?mt(r.locale,we("styleOnlyFeedback")):g),createdAt:U?.createdAt||se,updatedAt:se,page:E.targetsAdjusted?E.page??fn():U?.page??P??fn(),targets:Q(E.targetsAdjusted?E.targets:U?.targets??structuredClone(r.selected)),...E.targetsAdjusted?E.marker?{marker:E.marker}:{}:U?U.marker?{marker:U.marker}:{}:r.marker?{marker:r.marker}:{},status:U?.status||"pending",replies:U?.replies||[],...I&&U?.variants?{variants:U.variants}:{},...r.images.length?{images:structuredClone(r.images)}:{}});if(await ee({id:crypto.randomUUID(),kind:"upsert",annotation:re,styleLinks:z?.links()??{},...I?{variantRequest:crypto.randomUUID()}:{}},E),F===u&&S===k)G(),await he();if(F===u)pe(we("saved"))}finally{x=!1,r.saving=!1,V()}return}if(!("id"in m))return;let v=a.document.annotations.find((I)=>I.id===m.id);if(!v)throw le("missingAnnotation");if(m.type==="edit"){if(v.id===r.variantAnnotationId)r.variantConflict=!1;if(r.editingId===v.id){r.editorOpen=!0,V(),await he();return}B(),z?.begin(v.targets),ve(v.targets,v.id),z?.begin(r.selected,!0),ue(v.id),k++,r.targetsAdjusted=r.selected.some((I)=>!v.targets.some((E)=>E.id===I.id)),r.message="",delete r.messageDescriptor,r.editingId=v.id,r.draft=v.comment,r.variantsRequested=qe(v.variants),r.images=structuredClone(v.images??[]),r.marker=v.marker??Ie(v.targets),r.editorOpen=!0,P=v.page,Oe(),V(),await he();return}if(m.type==="delete"){let I=r.editingId===v.id,E=k;if(await ee({id:crypto.randomUUID(),kind:"delete",annotationId:v.id}),I&&E===k)G(),await he();return}}let He=(m)=>{Ce(m.detail).catch(ye)};e.addEventListener("ainotation-action",He);function b(){if(i)return;N?.abort(),ke.destroy(),i=!0,z?.destroy(),W?.destroy(),D?.abort();for(let m of T.values())URL.revokeObjectURL(m);T.clear(),u++,f++,clearInterval(fe),window.removeEventListener("popstate",A),window.removeEventListener("hashchange",A),L?.stop(),oe.destroy(),Z?.destroy(),me.close(),e.removeEventListener("ainotation-action",He),n.removeEventListener("abort",b),X.destroy()}n.addEventListener("abort",b,{once:!0});function A(){if(location.href===s||i)return!1;return z?.reset(),W?.sync(),r.variantAnnotationId="",r.variantsRequested=!1,r.variantFeedback="",r.variantPosition=null,r.variantMinimized=!1,r.variantConflict=!1,j={},r.editorSessionId="",r.editorPosition=null,D?.abort(),s=location.href,d=h(s),a=null,r.document=null,r.draft="",r.images=[],r.editingId=null,r.marker=null,r.editorOpen=!1,P=null,k++,r.storage="loading",oe.setPicking(!1),oe.resetPage(),_().catch(ye),!0}try{if(l||p);else if(t.mcp)y=qn(t.mcp);else{let m=sessionStorage.getItem(Ee);if(m)y=qn(JSON.parse(m))}if(y)r.endpoint=y.endpoint}catch{pe(we("connectionRestoreFailed"))}try{await ke.ready,await _()}catch(m){throw b(),m}if(i)return{getDocument:()=>null,copy:te,destroy:b};return window.addEventListener("popstate",A,{signal:n}),window.addEventListener("hashchange",A,{signal:n}),fe=setInterval(A,500),window.addEventListener("pagehide",()=>z?.suspend(),{signal:n}),window.addEventListener("pageshow",()=>z?.resume(),{signal:n}),{getDocument(){return A(),a?structuredClone(a.document):null},copy:te,destroy:b}}var xm,_m,km,Ni,Di,Sm,jd,Im,Cm,zm,Li,Pm,Ri,Am,Zi,Tm=(e,t)=>`[${e}="${CSS.escape(t)}"]`,Om,Dm,Lm,pn,Hm,Jm=(e)=>(...t)=>({_$litDirective$:e,values:t}),qm=class{constructor(e){}get _$AU(){return this._$AM._$AU}_$AT(e,t,n){this._$Ct=e,this._$AM=t,this._$Ci=n}_$AS(e,t){return this.update(e,t)}update(e,t){return this.render(...t)}},Km,Ed=(e)=>e,Td=()=>document.createComment(""),Hn=(e,t,n)=>{let r=e._$AA.parentNode,o=t===void 0?e._$AB:t._$AA;if(n===void 0){let i=r.insertBefore(Td(),o),a=r.insertBefore(Td(),o);n=new Km(i,a,e,e.options)}else{let i=n._$AB.nextSibling,a=n._$AM,s=a!==e;if(s){let c;n._$AQ?.(e),n._$AM=e,n._$AP!==void 0&&(c=e._$AU)!==a._$AU&&n._$AP(c)}if(i!==o||s){let c=n._$AA;for(;c!==i;){let l=Ed(c).nextSibling;Ed(r).insertBefore(c,o),c=l}}}return n},Kt=(e,t,n=e)=>(e._$AI(t,n),e),Wm,Gm=(e,t=Wm)=>e._$AH=t,Xm=(e)=>e._$AH,qi=(e)=>{e._$AR(),e._$AA.remove()},Od=(e,t,n)=>{let r=new Map;for(let o=t;o<=n;o++)r.set(e[o],o);return r},Md,Hr=(e)=>new Map([...e].map((t)=>[t,{value:e.getPropertyValue(t),priority:e.getPropertyPriority(t)}])),Kd=(e,t)=>e?.value===t?.value&&e?.priority===t?.priority,Jn=(e)=>Fe(e,{"aria-hidden":"true",focusable:"false"}),ig,ag,Dd=(e)=>{if(/^#[\da-f]{6}$/i.test(e))return e;let t=e.match(/^rgb\(\s*(\d+)[, ]+\s*(\d+)[, ]+\s*(\d+)\s*\)$/);return t?`#${t.slice(1).map((n)=>Number(n).toString(16).padStart(2,"0")).join("")}`:null},sg,Ld=(e)=>e.dataset.styleLink==="all"?!0:e.dataset.styleLink==="horizontal"?"horizontal":e.dataset.styleLink==="vertical"?"vertical":!1,lg,dg,mg,Gd,Xd,Yd,Qd;var Ui=Se(()=>{Fn();pi();hd();xm=[["path",{d:"M12 5v14"}],["path",{d:"m19 12-7 7-7-7"}]],_m=[["path",{d:"m5 12 7-7 7 7"}],["path",{d:"M12 19V5"}]],km=[["path",{d:"M13.997 4a2 2 0 0 1 1.76 1.05l.486.9A2 2 0 0 0 18.003 7H20a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V9a2 2 0 0 1 2-2h1.997a2 2 0 0 0 1.759-1.048l.489-.904A2 2 0 0 1 10.004 4z"}],["circle",{cx:"12",cy:"13",r:"3"}]],Ni=[["path",{d:"M20 6 9 17l-5-5"}]],Di=[["path",{d:"m6 9 6 6 6-6"}]],Sm=[["path",{d:"m15 18-6-6 6-6"}]],jd=[["path",{d:"m9 18 6-6-6-6"}]],Im=[["rect",{width:"18",height:"18",x:"3",y:"3",rx:"2"}],["path",{d:"M12 3v18"}]],Cm=[["path",{d:"M16 5h6"}],["path",{d:"M19 2v6"}],["path",{d:"M21 11.5V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7.5"}],["path",{d:"m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"}],["circle",{cx:"9",cy:"9",r:"2"}]],zm=[["path",{d:"M5 12h14"}]],Li=[["path",{d:"M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z"}],["path",{d:"m15 5 4 4"}]],Pm=[["path",{d:"M5 12h14"}],["path",{d:"M12 5v14"}]],Ri=[["path",{d:"m15 14 5-5-5-5"}],["path",{d:"M20 9H9.5A5.5 5.5 0 0 0 4 14.5A5.5 5.5 0 0 0 9.5 20H13"}]],Am=[["path",{d:"M5 3a2 2 0 0 0-2 2"}],["path",{d:"M19 3a2 2 0 0 1 2 2"}],["path",{d:"M21 19a2 2 0 0 1-2 2"}],["path",{d:"M5 21a2 2 0 0 1-2-2"}],["path",{d:"M9 3h1"}],["path",{d:"M9 21h1"}],["path",{d:"M14 3h1"}],["path",{d:"M14 21h1"}],["path",{d:"M3 9v1"}],["path",{d:"M21 9v1"}],["path",{d:"M3 14v1"}],["path",{d:"M21 14v1"}]],Zi=[["path",{d:"M9 14 4 9l5-5"}],["path",{d:"M4 9h10.5a5.5 5.5 0 0 1 5.5 5.5a5.5 5.5 0 0 1-5.5 5.5H11"}]];Om=["id","class","data-testid","data-element","role","aria-label","disabled","aria-disabled","aria-expanded","aria-describedby","aria-labelledby","aria-hidden","aria-checked","aria-selected","aria-required","aria-invalid","tabindex","type","name","placeholder","alt","href","for","readonly","required"];Dm=["id","data-testid","aria-label","role"],Lm=["display","position","color","background-color","font-size","font-weight","padding","margin","width","height","transform","border","outline","box-shadow","border-color","border-radius","font-family","line-height","letter-spacing","text-align","top","right","bottom","left","z-index","flex-direction","justify-content","align-items","gap","opacity","visibility","overflow"];pn=class extends Error{constructor(e){super("Stored feedback is invalid or belongs to another page.",{cause:e});this.name="InvalidDraftRecordError"}};Hm={ATTRIBUTE:1,CHILD:2,PROPERTY:3,BOOLEAN_ATTRIBUTE:4,EVENT:5,ELEMENT:6},{I:Km}=Gu,Wm={},Md=Jm(class extends qm{constructor(e){if(super(e),e.type!==Hm.CHILD)throw Error("repeat() can only be used in text expressions")}dt(e,t,n){let r;n===void 0?n=t:t!==void 0&&(r=t);let o=[],i=[],a=0;for(let s of e)o[a]=r?r(s,a):a,i[a]=n(s,a),a++;return{values:i,keys:o}}render(e,t,n){return this.dt(e,t,n).values}update(e,[t,n,r]){let o=Xm(e),{values:i,keys:a}=this.dt(t,n,r);if(!Array.isArray(o))return this.ut=a,i;let s=this.ut??=[],c=[],l,p,h=0,d=o.length-1,u=0,f=i.length-1;for(;h<=d&&u<=f;)if(o[h]===null)h++;else if(o[d]===null)d--;else if(s[h]===a[u])c[u]=Kt(o[h],i[u]),h++,u++;else if(s[d]===a[f])c[f]=Kt(o[d],i[f]),d--,f--;else if(s[h]===a[f])c[f]=Kt(o[h],i[f]),Hn(e,c[f+1],o[h]),h++,f--;else if(s[d]===a[u])c[u]=Kt(o[d],i[u]),Hn(e,o[h],o[d]),d--,u++;else if(l===void 0&&(l=Od(a,u,f),p=Od(s,h,d)),l.has(s[h]))if(l.has(s[d])){let x=p.get(a[u]),k=x!==void 0?o[x]:null;if(k===null){let w=Hn(e,o[h]);Kt(w,i[u]),c[u]=w}else c[u]=Kt(k,i[u]),Hn(e,o[h],k),o[x]=null;u++}else qi(o[d]),d--;else qi(o[h]),h++;for(;u<=f;){let x=Hn(e,c[f+1]);Kt(x,i[u]),c[u++]=x}for(;h<=d;){let x=o[h++];x!==null&&qi(x)}return this.ut=a,Gm(e,c),Ct}});ig=[["styleSize",["width","height"]],["styleText",["font-size","line-height","font-weight","text-align","color"]],["styleAppearance",["background-color","border-color","border-width","border-radius","opacity"]],["styleLayout",["display","gap","flex-direction","align-items","justify-content"]]],ag={"font-weight":["400","500","600","700"],"text-align":["left","center","right"],display:["block","inline-block","flex","grid","inline-flex"],"flex-direction":["row","column","row-reverse","column-reverse"],"align-items":["normal","stretch","flex-start","center","flex-end"],"justify-content":["normal","flex-start","center","flex-end","space-between","space-around"]},sg=["opacity","font-weight","line-height"];lg=wt`
  .editor-tabs {
    display: flex;
    gap: 4px;
    border-bottom: 1px solid var(--ain-border);
    margin: 0 0 10px;
  }
  .editor-tabs button {
    padding: 2px 8px;
    border: 0;
    border-bottom: 2px solid transparent;
    border-radius: 0;
    background: transparent;
    font-size: 12px;
    color: var(--ain-muted);
  }
  .editor-tabs button[aria-selected='true'] {
    border-bottom-color: var(--ain-accent);
    color: var(--ain-text);
  }
  .style-count {
    padding: 0 4px;
    border-radius: 3px;
    background: var(--ain-surface-muted);
    font-size: 10px;
    margin-left: 4px;
  }
  .style-preview {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 0 0 8px;
    border-bottom: 1px solid var(--ain-border);
  }
  .style-preview label {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-right: auto;
    font-size: 11px;
  }
  .style-preview input {
    accent-color: var(--ain-focus);
  }
  .style-preview button {
    border: 0;
    padding: 4px;
    width: 24px;
    height: 24px;
    background: transparent;
  }
  .style-preview svg {
    width: 16px;
    height: 16px;
  }
  .style-scroll {
    max-height: min(390px, 45vh);
    overflow: auto;
    overscroll-behavior: contain;
    overflow-anchor: none;
    border-bottom: 1px solid var(--ain-border);
    scrollbar-width: thin;
  }
  .style-group {
    border-bottom: 1px solid var(--ain-border);
  }
  .style-group:last-child {
    border-bottom: 0;
  }
  .style-group summary {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 0 10px 4px;
    font-size: 12px;
    cursor: pointer;
    list-style: none;
  }
  .style-chevron {
    display: inline-flex;
    width: 14px;
    height: 14px;
    flex: 0 0 14px;
  }
  .style-chevron svg {
    width: 14px;
    height: 14px;
  }
  .style-group[open] .style-chevron {
    transform: rotate(90deg);
  }
  .style-fields {
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
    gap: 12px 10px;
    padding-bottom: 12px;
  }
  .style-field {
    min-width: 0;
  }
  .style-label {
    display: flex;
    align-items: center;
    gap: 4px;
    height: 23px;
    font-size: 10px;
    color: var(--ain-muted);
  }
  .style-label .style-reset {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    border: 0;
    border-radius: 50%;
    width: 20px;
    height: 20px;
    background: transparent;
  }
  .style-dot {
    width: 5px;
    height: 5px;
    border-radius: 50%;
    background: var(--ain-accent);
  }
  .style-input {
    display: flex;
    align-items: center;
    border: 1px solid var(--ain-field-border);
    border-radius: 4px;
    background: var(--ain-field);
  }
  .style-input input[type='text'],
  .style-input select {
    width: 100%;
    min-width: 0;
    border: 0;
    padding: 7px 8px;
    background: transparent;
    color: var(--ain-text);
    font: 12px system-ui;
  }
  .style-input input[type='color'] {
    width: 26px;
    min-width: 26px;
    height: 27px;
    padding: 2px;
    margin-left: 4px;
    border: 0;
    background: none;
  }
  .style-input:focus-within {
    outline: 2px solid var(--ain-focus);
    outline-offset: -2px;
  }
  .style-input input:focus,
  .style-input select:focus {
    outline: 0;
  }
  .style-input select option {
    background: var(--ain-field);
  }
  .style-before {
    display: block;
    font:
      10px/1.4 ui-monospace,
      monospace;
    color: var(--ain-muted);
    margin-top: 4px;
    overflow-wrap: anywhere;
  }
  .style-invalid {
    color: var(--ain-error);
    font-size: 10px;
  }
  .style-restore {
    display: flex;
    align-items: center;
    justify-content: space-between;
    color: var(--ain-muted);
    font-size: 10px;
  }
  .style-restore button {
    font-size: 10px;
    padding: 3px 5px;
  }
  .style-spacing {
    padding: 12px 0;
    border-top: 1px solid var(--ain-border);
  }
  .spacing-main {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    gap: 8px;
    align-items: start;
  }
  .spacing-modes {
    display: flex;
    gap: 4px;
    padding-top: 23px;
  }
  .spacing-modes button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 32px;
    padding: 4px;
    border-radius: 4px;
    background: var(--ain-field);
  }
  .spacing-modes button[aria-pressed='true'] {
    background: var(--ain-selected);
    border-color: var(--ain-focus);
  }
  .spacing-modes svg {
    width: 16px;
    height: 16px;
  }
  .spacing-details {
    padding: 8px 0 0;
  }
  .style-restore {
    margin-top: 8px;
  }
  .style-target {
    width: 100%;
    margin: 0 0 8px;
    padding: 5px;
    background: var(--ain-field);
    color: var(--ain-text);
    border: 1px solid var(--ain-field-border);
    font: 11px system-ui;
  }
  .style-problem,
  .style-note {
    margin: 8px 0;
    color: var(--ain-muted);
    font-size: 11px;
  }
`;dg=wt`
  .variants-mode {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    margin: 8px 0;
  }
  .variants-mode button[aria-checked='true'] {
    background: var(--ain-accent);
    color: var(--ain-on-accent);
    border-color: var(--ain-accent);
  }
  .variants-mode .variant-switch {
    position: relative;
    width: 32px;
    height: 20px;
    padding: 0;
    border-radius: 12px;
    background: var(--ain-field);
  }
  .variant-switch::after {
    content: '';
    position: absolute;
    top: 3px;
    left: 3px;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: var(--ain-muted);
  }
  .variant-switch[aria-checked='true']::after {
    transform: translateX(12px);
    background: var(--ain-on-accent);
  }
  .variants-hint {
    margin: 6px 0;
    color: var(--ain-muted);
    font-size: 11px;
  }
  .variants-controller {
    position: fixed;
    left: 50%;
    bottom: 16px;
    transform: translateX(-50%);
    width: min(480px, calc(100vw - 32px));
    max-height: calc(100dvh - 100px);
    overflow: auto;
    overscroll-behavior: contain;
    padding: 12px;
    border: 1px solid var(--ain-border);
    border-radius: 8px;
    background: var(--ain-surface);
    box-shadow: 0 4px 20px var(--ain-shadow);
    pointer-events: auto;
    cursor: grab;
  }
  .variants-controller.dragging,
  .variants-controller.dragging * {
    cursor: grabbing !important;
    user-select: none;
  }
  .variants-controller:focus-visible {
    outline: 2px solid var(--ain-focus);
    outline-offset: 2px;
  }
  .variants-heading {
    display: flex;
    justify-content: space-between;
    gap: 8px;
    align-items: center;
  }
  .variants-actions {
    display: flex;
    justify-content: center;
    flex-wrap: wrap;
    gap: 6px;
    margin-top: 10px;
  }
  .variants-actions button {
    padding: 5px 7px;
    white-space: nowrap;
  }
  .variants-actions .danger,
  .variant-confirm-actions .danger {
    color: var(--ain-error);
    border-color: var(--ain-error);
    background: var(--ain-error-surface);
  }
  .variants-actions .danger:hover:not(:disabled),
  .variant-confirm-actions .danger:hover:not(:disabled) {
    box-shadow: inset 0 0 0 1px var(--ain-error);
  }
  .variant-confirm {
    width: min(360px, calc(100vw - 32px));
    max-height: calc(100dvh - 32px);
    margin: auto;
    padding: 20px;
    overflow: auto;
    border: 1px solid var(--ain-border);
    border-radius: 8px;
    background: var(--ain-surface);
    color: var(--ain-text);
    box-shadow: 0 4px 20px var(--ain-shadow);
    pointer-events: auto;
  }
  .variant-confirm::backdrop {
    background: var(--ain-crop-shade);
    pointer-events: auto;
  }
  .variant-confirm h2 {
    margin: 0;
    font-size: 15px;
  }
  .variant-confirm p {
    margin: 10px 0 20px;
    color: var(--ain-muted);
  }
  .variant-confirm-actions {
    display: flex;
    justify-content: flex-end;
    flex-wrap: wrap;
    gap: 8px;
  }
  .variants-heading-actions {
    display: flex;
    align-items: center;
    gap: 2px;
  }
  .variants-heading .variant-heading-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    padding: 5px;
    border-color: transparent;
    background: transparent;
    color: var(--ain-muted);
  }
  .variants-heading .variant-heading-button:hover:not(:disabled) {
    color: var(--ain-text);
  }
  .variant-heading-button svg {
    width: 16px;
    height: 16px;
  }
  .variants-navigation {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-top: 10px;
  }
  .variant-step {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex: 0 0 48px;
    width: 48px;
    height: 32px;
    padding: 6px;
  }
  .variant-step svg {
    width: 16px;
    height: 16px;
  }
  .variant-current {
    flex: 1;
    min-width: 0;
    text-align: center;
  }
  .variant-name {
    display: block;
    font-weight: 600;
    overflow-wrap: anywhere;
  }
  .variant-page {
    display: block;
    color: var(--ain-muted);
    font-size: 11px;
    font-variant-numeric: tabular-nums;
  }
  .variants-controller p {
    margin: 8px 0;
  }
  .variants-controller details {
    margin-top: 10px;
  }
  .variants-controller summary {
    cursor: pointer;
    color: var(--ain-muted);
    font-size: 12px;
  }
  .variants-controller textarea {
    width: 100%;
    margin-top: 8px;
    padding: 8px;
    border: 1px solid var(--ain-border);
    border-radius: 4px;
    background: var(--ain-field);
    color: var(--ain-text);
    resize: vertical;
    min-height: 64px;
    cursor: text;
  }
  .passthrough .variants-controller,
  .passthrough .variants-controller * {
    pointer-events: none !important;
  }
`;mg=wt`
  ${Ft}
  ${lg}
  ${dg}
  * {
    box-sizing: border-box;
  }
  .layer {
    color: var(--ain-text);
    font:
      13px/1.5 system-ui,
      sans-serif;
    letter-spacing: 0;
    text-align: left;
    color-scheme: var(--ain-scheme);
    direction: ltr;
  }
  [hidden] {
    display: none !important;
  }
  button,
  textarea {
    font: inherit;
  }
  button {
    appearance: none;
    padding: 5px 9px;
    border: 1px solid var(--ain-border);
    border-radius: 4px;
    background: var(--ain-surface);
    color: var(--ain-text);
    cursor: pointer;
  }
  button:disabled {
    opacity: 0.45;
    cursor: default;
  }
  button:focus-visible,
  textarea:focus-visible {
    outline: 2px solid var(--ain-focus);
    outline-offset: 2px;
  }
  .primary {
    background: var(--ain-accent);
    border-color: var(--ain-accent);
    color: var(--ain-on-accent);
  }
  .primary:hover:not(:disabled) {
    background: var(--ain-accent-hover);
  }
  .actions button:not(.primary):hover:not(:disabled) {
    background: var(--ain-hover);
  }
  .marker {
    position: fixed;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    min-width: 24px;
    min-height: 24px;
    max-width: 24px;
    max-height: 24px;
    padding: 0;
    margin: 0;
    border: 1px solid var(--ain-on-accent);
    border-radius: 50%;
    background: var(--ain-accent);
    color: var(--ain-on-accent);
    font:
      600 12px/1 system-ui,
      sans-serif;
    transform: translate(-50%, -50%);
    pointer-events: auto;
  }
  .marker svg {
    width: 14px;
    height: 14px;
  }
  .passthrough .marker,
  .passthrough .popover,
  .passthrough .popover * {
    pointer-events: none !important;
  }
  .pencil {
    display: none;
  }
  .marker:hover .number,
  .marker:focus-visible .number {
    display: none;
  }
  .marker:hover .pencil,
  .marker:focus-visible .pencil {
    display: flex;
  }
  .popover {
    position: fixed;
    padding: 12px;
    border: 1px solid var(--ain-border);
    border-radius: 8px;
    background: var(--ain-surface);
    box-shadow: 0 4px 20px var(--ain-shadow);
    overflow: auto;
    overscroll-behavior: contain;
    pointer-events: auto;
    cursor: grab;
  }
  .popover input,
  .popover textarea,
  .popover select,
  .popover .style-input {
    cursor: auto;
  }
  .popover code,
  .popover pre,
  .popover q {
    cursor: text;
  }
  .popover label {
    cursor: pointer;
  }
  .popover.dragging,
  .popover.dragging * {
    cursor: grabbing !important;
    user-select: none;
  }
  .popover:focus-visible {
    outline: 2px solid var(--ain-focus);
    outline-offset: 2px;
  }
  .target-list {
    list-style: none;
    padding: 0;
    margin: 0 0 8px;
    max-height: 160px;
    overflow: auto;
    font:
      11px/1.5 ui-monospace,
      monospace;
    color: var(--ain-muted);
  }
  .target-list li {
    overflow-wrap: anywhere;
    padding: 2px 0;
  }
  .target-description {
    color: var(--ain-text);
    font:
      500 12px/1.5 system-ui,
      sans-serif;
  }
  .target-heading {
    display: flex;
    align-items: flex-start;
    gap: 4px;
  }
  .target-heading .target-description {
    flex: 1;
    min-width: 0;
  }
  .target-tools {
    display: flex;
    flex-shrink: 0;
  }
  .target-tools button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    padding: 4px;
    border: 0;
    background: transparent;
  }
  .target-tools svg {
    width: 14px;
    height: 14px;
  }
  .target-locator {
    display: flex;
    align-items: center;
    gap: 6px;
    min-width: 0;
  }
  .target-locator code {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .copy-selector {
    flex-shrink: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    padding: 4px;
    border: 0;
    background: transparent;
  }
  .copy-selector svg {
    width: 14px;
    height: 14px;
  }
  .target-details summary {
    cursor: pointer;
    font:
      11px/1.5 system-ui,
      sans-serif;
  }
  .target-details pre {
    margin: 6px 0;
    white-space: pre-wrap;
    overflow-wrap: anywhere;
    font: inherit;
  }
  .text-quote {
    display: block;
    margin-top: 4px;
    padding: 6px 8px;
    border-left: 2px solid var(--ain-quote-border);
    background: var(--ain-quote);
    font:
      12px/1.5 system-ui,
      sans-serif;
    white-space: pre-wrap;
  }
  textarea {
    display: block;
    width: 100%;
    min-width: 0;
    min-height: 80px;
    max-height: 240px;
    padding: 7px 8px;
    border: 1px solid var(--ain-field-border);
    border-radius: 4px;
    background: var(--ain-field);
    color: var(--ain-text);
    resize: vertical;
  }
  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-top: 10px;
  }
  .actions button {
    width: 32px;
    height: 32px;
    padding: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
  }
  .actions-end {
    display: flex;
    gap: 6px;
    margin-left: auto;
  }
  .actions .danger {
    color: var(--ain-error);
    border-color: var(--ain-error);
  }
  .actions .danger:hover:not(:disabled) {
    background: var(--ain-error-surface);
  }
  .actions svg {
    width: 16px;
    height: 16px;
  }
  .message {
    margin: 8px 0 0;
    overflow-wrap: anywhere;
    color: var(--ain-message);
  }
  .images {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
    margin-top: 8px;
  }
  .image {
    position: relative;
    width: 84px;
    height: 64px;
  }
  .image img {
    width: 76px;
    height: 56px;
    object-fit: contain;
    display: block;
  }
  .image-preview {
    display: block;
    width: 100%;
    height: 100%;
    padding: 3px;
  }
  .image svg {
    width: 14px;
    height: 14px;
  }
  .image-action {
    opacity: 0;
    pointer-events: none;
    position: absolute;
    right: 3px;
    z-index: 1;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 22px;
    height: 22px;
    padding: 0;
    box-shadow: 0 1px 3px var(--ain-shadow);
  }
  .image-action:hover:not(:disabled) {
    background: var(--ain-hover);
  }
  .image:hover .image-action,
  .image:focus-within .image-action {
    opacity: 1;
    pointer-events: auto;
  }
  @media (hover: none) {
    .image-action {
      opacity: 1;
      pointer-events: auto;
    }
  }
  .image-remove {
    top: 3px;
  }
  .image-download {
    bottom: 3px;
  }
  .import-hint {
    font-size: 11px;
    color: var(--ain-muted);
    margin: 8px 0 0;
  }
`;Gd=Jr({key:(e)=>e,parse(e){let t=ai.safeParse(e);return t.success?t.data:void 0}}),Xd=Jr({key:(e)=>["theme",e],parse:(e)=>e==="light"||e==="dark"?e:void 0,cache:{key:(e)=>`ainotation:theme:${e}`,encode:(e)=>e,decode:(e)=>e}}),Yd=Jr({key:(e)=>["position",e],parse:Wd,cache:{key:(e)=>`ainotation:position:${e}`,encode:JSON.stringify,decode:JSON.parse}}),Qd=Jr({key:(e)=>["locale",e],parse:(e)=>Ut(e)?e:void 0,cache:{key:(e)=>`ainotation:locale:${e}`,encode:(e)=>e,decode:(e)=>e}})});function tp(e={}){let t,n,r=0,o,i;function a(){if(r++,n=void 0,i?.abort(),o?.destroy(),o=void 0,!t)return;t.remove(),t=void 0,e.onDestroy?.()}return{mount(){if(n)return n;if(t)return Promise.resolve();if(typeof document>"u"||!document.body)return Promise.reject(Error("Mount Ainotation after the browser document is ready."));let s=r;i=new AbortController;let c=i.signal;return n=Promise.resolve().then(() => (id(),{})).then(async({})=>{if(s!==r)return;let p=e.container??document.body;od(),t=document.createElement("ainotation-inspector-shell"),t.dataset.ainotationUi="true",p.append(t);await Promise.resolve().then(() => Ui());if(s!==r)return;let d=await ep(t,e,c);if(s!==r)d.destroy();else o=d}).catch((l)=>{if(s!==r)return;throw a(),l}).finally(()=>{if(s===r)n=void 0}),n},destroy:a,getDocument(){return o?.getDocument()??null},copyFeedback(){return o?o.copy():Promise.reject(Error("Mount Ainotation first."))},get mounted(){return t!==void 0}}}var Ag="canon-flow";function np(e){tp({projectId:Ag,...e}).mount()}function rp(e){if(document.readyState==="loading")document.addEventListener("DOMContentLoaded",()=>np(e),{once:!0});else np(e)}async function Eg(){for(let e of["http://127.0.0.1:44091/connection.json","/assets/ainotation/connection.json"])try{let t=await fetch(e,{cache:"no-store"});if(!t.ok)continue;return await t.json()}catch{}return null}Eg().then((e)=>rp(e?{mcp:{endpoint:e.url,token:e.token}}:{})).catch(()=>rp({}));})();
