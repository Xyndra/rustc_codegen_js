// rustc_codegen_js runtime preamble (inlined into every bundle).
// Single-threaded semantics throughout; BigInt for 64/128-bit integers.

const __enc = (typeof TextEncoder !== "undefined") ? new TextEncoder() : null;
const __dec = (typeof TextDecoder !== "undefined") ? new TextDecoder() : null;

class RustPanic extends Error {
  constructor(message) {
    super(message);
    this.name = "RustPanic";
    this.__rustPanic = true;
  }
}

function __str_cell(s) { return [String(s)]; }
function __panic(msg) { throw new RustPanic(msg); }
function __unreachable() { throw new RustPanic("internal error: entered unreachable code"); }
function __abort(msg) { throw new RustPanic(msg === undefined ? "abort" : msg); }

// ---- option/result constructors (discriminant values passed by emitter) ----
function __some(t, x) { return { t: t, v: [x] }; }
function __none(t) { return { t: t, v: [] }; }
function __ok(t, x) { return { t: t, v: [x] }; }
function __err(t, x) { return { t: t, v: [x] }; }

// ---- bounds-checked indexing (slice/Vec `[]`, `index`/`index_mut`) ----
function __idx(a, i) {
  if (i < 0 || i >= a.length) {
    __panic("index out of bounds: the len is " + a.length + " but the index is " + i);
  }
  return i;
}

// ---- Vec helpers ----
function __vec_retain(v, pred, predFn) {  let w = 0;
  for (let i = 0; i < v.length; i++) {
    if (predFn(pred, v[i])) { v[w++] = v[i]; }
  }
  v.length = w;
}
function __vec_extend_slice(v, s) { for (const x of s) { v.push(x); } }
function __copy_from_slice(dst, src) {
  if (dst.length !== src.length) {
    __panic("source slice length (" + src.length + ") does not match destination slice length (" + dst.length + ")");
  }
  for (let i = 0; i < dst.length; i++) { dst[i] = src[i]; }
}
function __copy_range(base, s, n, src) {
  if (src.length !== n) {
    __panic("source slice length (" + src.length + ") does not match destination slice length (" + n + ")");
  }
  for (let i = 0; i < n; i++) { base[s + i] = src[i]; }
}

// ---- str helpers (byte-oriented, like Rust) ----
function __str_len(s) { return __enc.encode(s).length; }
function __str_bytes(s) { return Array.from(__enc.encode(s)); }
function __str_slice(s, a, b) {
  const bytes = __enc.encode(s);
  return __dec.decode(bytes.slice(a, b));
}
function __str_split_at(s, i) { return [__str_slice(s, 0, i), __str_slice(s, i, __str_len(s))]; }
function __is_boundary_b(b, i) {
  return i <= b.length && (i === b.length || (b[i] & 0xc0) !== 0x80);
}
function __check_str_idx(s, i, what) {
  const b = __enc.encode(s);
  if (i > b.length) {
    __panic("byte index " + i + " is out of bounds of `" + s + "`");
  }
  if (!__is_boundary_b(b, i)) {
    __panic("byte index " + i + " is not a char boundary; it is inside a char in `" + s + "`");
  }
  return b;
}
// Checked str::split_at (panics like native on OOB / non-boundary).
function __str_split_at_chk(s, i) {
  const b = __check_str_idx(s, i, "split_at");
  return [__dec.decode(b.slice(0, i)), __dec.decode(b.slice(i))];
}
// Checked str indexing s[a..b] (panics like native).
function __str_index(s, a, b) {
  const bb = __check_str_idx(s, a, "index");
  __check_str_idx(s, b, "index");
  if (a > b) { __panic("slice index starts at " + a + " but ends at " + b); }
  return __dec.decode(bb.slice(a, b));
}
function __str_index_from(s, a) {
  const bb = __check_str_idx(s, a, "index");
  return __dec.decode(bb.slice(a));
}
function __str_index_to(s, b) {
  const bb = __check_str_idx(s, b, "index");
  return __dec.decode(bb.slice(0, b));
}
// str::get with Range: {ok, val} (never panics).
function __str_get_range(s, a, b) {
  const bytes = __enc.encode(s);
  if (a > b || b > bytes.length || !__is_boundary_b(bytes, a) || !__is_boundary_b(bytes, b)) {
    return { ok: false, val: "" };
  }
  return { ok: true, val: __dec.decode(bytes.slice(a, b)) };
}
// String::truncate (panics like native; returns the truncated string).
function __str_truncate(s, n) {
  const b = __check_str_idx(s, n, "truncate");
  return __dec.decode(b.slice(0, n));
}
function __str_find(s, pat) {
  const sb = __enc.encode(s), pb = __enc.encode(pat);
  outer: for (let i = 0; i + pb.length <= sb.length; i++) {
    for (let j = 0; j < pb.length; j++) { if (sb[i + j] !== pb[j]) { continue outer; } }
    return i;
  }
  return -1;
}
function __str_matches(s, pat) {
  // `pat` is a char (code point) or a str.
  const t = (typeof pat === "number") ? String.fromCodePoint(pat) : pat;
  const out = [];
  if (t === "") { return out; }
  let i = s.indexOf(t);
  while (i >= 0) { out.push(t); i = s.indexOf(t, i + t.length); }
  return out;
}
// Join pieces into a string. Only `char`, `&str` and `String` can legally
// collect into a `String`, so sniffing is total: chars arrive as code-point
// numbers (the backend's `char` repr), strs as cells or raw strings.
function __str_join(arr) {
  return arr
    .map((e) => (typeof e === "number" ? String.fromCodePoint(e) : (Array.isArray(e) ? e[0] : e)))
    .join("");
}
// Idempotent string-cell unwrap for cross-crate calls: cells arrive as
// `[str]`, but values that already passed through a deref are raw strings —
// unwrapping those again would take the first *character*.
function __str_raw(s) {
  return Array.isArray(s) ? s[0] : s;
}
// char predicates: Unicode properties replace core's tables.
function __char_is_lower(cp) { return /\p{Ll}/u.test(String.fromCodePoint(cp)); }
function __char_is_upper(cp) { return /\p{Lu}/u.test(String.fromCodePoint(cp)); }
function __char_is_alpha(cp) { return /\p{Alphabetic}/u.test(String.fromCodePoint(cp)); }
function __char_is_numeric(cp) { return /\p{Nd}/u.test(String.fromCodePoint(cp)); }
function __char_is_alnum(cp) { return /\p{Alphabetic}|\p{Nd}/u.test(String.fromCodePoint(cp)); }
function __char_is_ws(cp) { return /\p{White_Space}/u.test(String.fromCodePoint(cp)); }
function __char_is_ctrl(cp) { return /\p{Cc}/u.test(String.fromCodePoint(cp)); }
function __char_is_ascii_letter(cp) { return (cp >= 65 && cp <= 90) || (cp >= 97 && cp <= 122); }
function __char_is_ascii_digit(cp) { return cp >= 48 && cp <= 57; }
function __str_parse_int(s) {
  // Returns {ok, val}. Mirrors Rust saturating-failure semantics via ok=false.
  if (!/^[+-]?\d+$/.test(s.trim())) { return { ok: false, val: 0 }; }
  const n = Number(s.trim());
  if (!Number.isSafeInteger(n) && !Number.isInteger(n)) { return { ok: false, val: 0 }; }
  return { ok: true, val: Math.trunc(n) };
}
function __str_parse_float(s) {
  const t = s.trim();
  if (t === "inf" || t === "+inf" || t === "infinity" || t === "+infinity") {
    return { ok: true, val: Infinity };
  }
  if (t === "-inf" || t === "-infinity") { return { ok: true, val: -Infinity }; }
  if (t === "NaN") { return { ok: true, val: NaN }; }
  if (!/^[+-]?(\d+\.?\d*|\.\d+)([eE][+-]?\d+)?$/.test(t)
    && !/^[+-]?0[xX][0-9a-fA-F]+(\.[0-9a-fA-F]*)?[pP][+-]?\d+$/.test(t)) {
    return { ok: false, val: 0 };
  }
  const n = Number(t);
  return Number.isNaN(n) ? { ok: false, val: 0 } : { ok: true, val: n };
}
function __str_parse_bool(s) {
  if (s === "true") { return { ok: true, val: true }; }
  if (s === "false") { return { ok: true, val: false }; }
  return { ok: false, val: false };
}

// ---- structural equality (slice helpers, derived-eq fallback) ----
function __deep_eq(a, b) {
  if (a === b) { return true; }
  if (typeof a === "number" && typeof b === "number" && Number.isNaN(a) && Number.isNaN(b)) {
    return false; // Rust: NaN != NaN
  }
  if (Array.isArray(a) && Array.isArray(b)) {
    if (a.length !== b.length) { return false; }
    for (let i = 0; i < a.length; i++) { if (!__deep_eq(a[i], b[i])) { return false; } }
    return true;
  }
  if (a !== null && b !== null && typeof a === "object" && typeof b === "object") {
    if (a.t !== undefined || b.t !== undefined) {
      if (a.t !== b.t) { return false; }
      return __deep_eq(a.v, b.v);
    }
    return false;
  }
  return false;
}
function __slice_contains(a, x) { for (const e of a) { if (__deep_eq(e, x)) { return true; } } return false; }
function __slice_starts_with(a, b) {
  if (b.length > a.length) { return false; }
  for (let i = 0; i < b.length; i++) { if (!__deep_eq(a[i], b[i])) { return false; } }
  return true;
}
function __slice_ends_with(a, b) {
  if (b.length > a.length) { return false; }
  const o = a.length - b.length;
  for (let i = 0; i < b.length; i++) { if (!__deep_eq(a[o + i], b[i])) { return false; } }
  return true;
}

// ---- bit operations ----
function __popcnt32(x) {
  x = x >>> 0;
  x = x - ((x >>> 1) & 0x55555555);
  x = (x & 0x33333333) + ((x >>> 2) & 0x33333333);
  return (((x + (x >>> 4)) & 0x0f0f0f0f) * 0x01010101) >>> 24;
}
function __popcnt_big(x) {
  if (x < 0n) { x = -x - 1n; } // popcount of two's complement magnitude
  let n = 0;
  while (x > 0n) { n += Number(x & 1n); x >>= 1n; }
  return n;
}
function __ctlz_big(x, bits) {
  if (x === 0n) { return bits; }
  if (x < 0n) { return 0; }
  let n = 0, b = BigInt(bits);
  let v = x;
  while (v < (1n << (b - 1n))) { v <<= 1n; n++; if (n > bits) { break; } }
  return n;
}
function __cttz_big(x, bits) {
  x = BigInt.asUintN(bits, x);
  if (x === 0n) { return bits; }
  let n = 0;
  while ((x & 1n) === 0n) { x >>= 1n; n++; }
  return n;
}
function __bswap32(x) {
  x = x >>> 0;
  return (((x & 255) << 24) | (((x >>> 8) & 255) << 16) | (((x >>> 16) & 255) << 8) | ((x >>> 24) & 255)) >>> 0;
}
function __bswap_big(x, bits) {
  x = BigInt.asUintN(bits, x);
  let r = 0n;
  for (let i = 0; i < bits / 8; i++) { r = (r << 8n) | (x & 255n); x >>= 8n; }
  return r;
}
function __bitrev32(x) {
  x = x >>> 0;
  x = ((x & 0x55555555) << 1) | ((x >>> 1) & 0x55555555);
  x = ((x & 0x33333333) << 2) | ((x >>> 2) & 0x33333333);
  x = ((x & 0x0f0f0f0f) << 4) | ((x >>> 4) & 0x0f0f0f0f);
  x = (x << 24) | ((x & 0xff00) << 8) | ((x >>> 8) & 0xff00) | (x >>> 24);
  return x >>> 0;
}
function __bitrev_big(x, bits) {
  x = BigInt.asUintN(bits, x);
  let r = 0n;
  for (let i = 0; i < bits; i++) { r = (r << 1n) | (x & 1n); x >>= 1n; }
  return r;
}
function __lshr_big(x, n, bits) {
  n = BigInt.asUintN(bits, n);
  if (n >= BigInt(bits)) { return 0n; }
  return BigInt.asUintN(bits, x) >> n;
}
function __rotl32(x, n) { n &= 31; return ((x << n) | (x >>> (32 - n))) | 0; }
function __rotr32(x, n) { n &= 31; return ((x >>> n) | (x << (32 - n))) | 0; }
function __rotl_big(x, n, bits) {
  const b = BigInt(bits), m = (1n << b) - 1n;
  n = ((n % b) + b) % b;
  x = BigInt.asUintN(bits, x);
  return ((x << n) | (x >> (b - n))) & m;
}
function __rotr_big(x, n, bits) {
  const b = BigInt(bits), m = (1n << b) - 1n;
  n = ((n % b) + b) % b;
  x = BigInt.asUintN(bits, x);
  return ((x >> n) | (x << (b - n))) & m;
}
function __round_ties_even(x) {
  const f = Math.floor(x), d = x - f;
  if (d < 0.5) { return f; }
  if (d > 0.5) { return f + 1; }
  return (f % 2 === 0) ? f : f + 1;
}
function __fmin(a, b) {
  if (Number.isNaN(a)) { return b; }
  if (Number.isNaN(b)) { return a; }
  return Math.min(a, b);
}
function __fmax(a, b) {
  if (Number.isNaN(a)) { return b; }
  if (Number.isNaN(b)) { return a; }
  return Math.max(a, b);
}
function __fminimum(a, b) {
  if (Number.isNaN(a) || Number.isNaN(b)) { return NaN; }
  if (a === 0 && b === 0) { return (1 / a < 1 / b) ? a : b; }
  return Math.min(a, b);
}
function __fmaximum(a, b) {
  if (Number.isNaN(a) || Number.isNaN(b)) { return NaN; }
  if (a === 0 && b === 0) { return (1 / a < 1 / b) ? b : a; }
  return Math.max(a, b);
}
function __fround_away(x) {
  if (Number.isNaN(x) || x === Infinity || x === -Infinity) { return x; }
  return Math.sign(x) * Math.round(Math.abs(x));
}
function __fclamp(v, lo, hi) {
  if (!(lo <= hi)) { __panic("min > max, or either was NaN"); }
  if (v < lo) { return lo; }
  if (v > hi) { return hi; }
  return v;
}
function __char_encode_utf8_into(c, dst) {
  const s = String.fromCodePoint(c);
  if (__enc) {
    const bytes = __enc.encode(s);
    for (let i = 0; i < bytes.length; i++) { dst[i] = bytes[i]; }
  }
  return s;
}
function __str_trim_start_matches(h, n) {
  if (n.length === 0) { return h; }
  while (h.startsWith(n)) { h = h.slice(n.length); }
  return h;
}
function __vec_extend_iter(v, it) {
  for (const x of __it_collect(it)) { v.push(x); }
}
// ---- HashMap (JS `Map` emulation) ----------------------------------------
// Rust `HashMap<K, V>` (`K: Hash + Eq`) emulated with a `Map` keyed by a
// canonical string form of the key, values stored in 1-element wrappers so
// "absent" is unambiguous (a real key maps to an array, never `undefined`).
// Insertion/rehash details are internal to hashbrown and unobservable;
// iteration order is unspecified in Rust, so insertion order is valid.
function __map_key_str(k) {
  if (k === null) { return "p:null"; }
  const t = typeof k;
  if (t === "string") { return "s:" + k; }
  if (t === "number") { return "d:" + k; }
  if (t === "boolean") { return "b:" + (k ? 1 : 0); }
  if (t === "bigint") { return "n:" + k; }
  if (t === "undefined") { return "u:"; }
  // A single-string cell (`String` value) canonicalizes as the string
  // itself: strings cross function boundaries both wrapped (`[s]`) and raw
  // (`s`, after deref) depending on the call path, and map lookups must hit
  // regardless. (Genuine 1-tuple keys cannot collide: every map instance has
  // a single key type, and a `String` key and a 1-tuple key never share one.)
  if (Array.isArray(k)) {
    if (k.length === 1 && typeof k[0] === "string") { return "s:" + k[0]; }
    return "a:[" + k.map(__map_key_str).join(",") + "]";
  }
  // Structs/tuples as objects, enums as `{t, v}`.
  if (t === "object") {
    if (k.t !== undefined || k.v !== undefined) {
      return "e:" + __map_key_str(k.t) + "," + __map_key_str(k.v);
    }
    return "o:" + Object.keys(k).map((key) => key + ":" + __map_key_str(k[key])).join(",");
  }
  if (t === "function") { throw new RustPanic("function values are not valid map keys"); }
  throw new RustPanic("unsupported map key");
}
function __map_new() { return new Map(); }
function __map_get(m, k) {
  const key = __map_key_str(k);
  return m.has(key) ? m.get(key) : undefined;
}
function __map_set(m, k, v) {
  const key = __map_key_str(k);
  const prev = m.has(key) ? m.get(key) : undefined;
  // Store the ORIGINAL key object too, so iteration yields real `&K`
  // (String cells, tuples, ...) rather than the canonical lookup form.
  m.set(key, [k, v]);
  return prev;
}
function __map_del(m, k) {
  const key = __map_key_str(k);
  const prev = m.has(key) ? m.get(key) : undefined;
  m.delete(key);
  return prev;
}
function __map_has(m, k) { return m.has(__map_key_str(k)); }
function __map_len(m) { return m.size; }
function __map_items(m) { return Array.from(m.values()); }
function __copysign(a, b) {
  const m = a < 0 || Object.is(a, -0) ? -1 : 1;
  const s = b < 0 || Object.is(b, -0) ? -1 : 1;
  const r = Math.abs(a) * s;
  return (m === s) ? a : (Object.is(a, -0) || a === 0 ? s * 0 : r);
}
function __checked_add_num(a, b, bits, signed) {
  const s = a + b;
  let lo, hi;
  if (bits === 64) { lo = -8e18; hi = 8e18; } else {
    hi = signed ? Math.pow(2, bits - 1) - 1 : Math.pow(2, bits) - 1;
    lo = signed ? -Math.pow(2, bits - 1) : 0;
  }
  const ov = !(s >= lo && s <= hi) || !Number.isInteger(s);
  const w = signed ? (s | 0) : (s >>> 0);
  return [w, ov];
}
function __checked_sub_num(a, b, bits, signed) {
  return __checked_add_num(a, -b, bits, signed);
}
function __checked_mul_num(a, b, bits, signed) {
  const s = a * b;
  let lo, hi;
  if (bits === 64) { lo = -8e18; hi = 8e18; } else {
    hi = signed ? Math.pow(2, bits - 1) - 1 : Math.pow(2, bits) - 1;
    lo = signed ? -Math.pow(2, bits - 1) : 0;
  }
  const ov = !(s >= lo && s <= hi) || !Number.isInteger(s);
  const w = signed ? (s | 0) : (s >>> 0);
  return [w, ov];
}
function __wrap_big(x, bits, signed) {
  return signed ? BigInt.asIntN(bits, x) : BigInt.asUintN(bits, x);
}
function __checked_add_big(a, b, bits, signed) {
  const s = a + b, w = __wrap_big(s, bits, signed);
  return [w, s !== w];
}
function __checked_sub_big(a, b, bits, signed) {
  const s = a - b, w = __wrap_big(s, bits, signed);
  return [w, s !== w];
}
function __checked_mul_big(a, b, bits, signed) {
  const s = a * b, w = __wrap_big(s, bits, signed);
  return [w, s !== w];
}
// float -> int with Rust saturation semantics (NaN -> 0, infinities clamp).
function __cast_num(x, lo, hi) {
  if (Number.isNaN(x)) { return 0; }
  if (x <= lo) { return lo; }
  if (x >= hi) { return hi; }
  return Math.trunc(x);
}
function __cast_big(x, bits, signed) {
  if (Number.isNaN(x)) { return 0n; }
  const b = BigInt(bits);
  const hi = signed ? (1n << (b - 1n)) - 1n : (1n << b) - 1n;
  const lo = signed ? -(1n << (b - 1n)) : 0n;
  let t = BigInt(Math.trunc(x));
  if (t <= lo) { return lo; }
  if (t >= hi) { return hi; }
  return t;
}

// ---- formatting ----
// Arg descriptor grammar (emitted by the backend from static types):
// {k:'n'} number-ish primitive (int/uint/float/char/bool use same path with kind)
// {k:'s'} string, {k:'a',e:inner} array/vec/slice, {k:'t',e:[...]} tuple,
// {k:'S',n:name,f:[fieldNames]} struct, {k:'E',n:name,v:{discr:[vname,isStruct,[fields]]}} enum.
function __esc_debug_str(s) {
  let out = "";
  for (const c of s) {
    if (c === '"') { out += '\\"'; }
    else if (c === "\\") { out += "\\\\"; }
    else if (c === "\n") { out += "\\n"; }
    else if (c === "\r") { out += "\\r"; }
    else if (c === "\t") { out += "\\t"; }
    else if (c === "\0") { out += "\\0"; }
    else {
      const cp = c.codePointAt(0);
      if (cp < 0x20 || cp === 0x7f) { out += "\\u{" + cp.toString(16) + "}"; }
      else { out += c; }
    }
  }
  return out;
}
// Float Display/Debug, matching Rust: the shortest decimal that round-trips
// through the same width, always in fixed notation, `inf`/`-inf`/`NaN`.
function __fmt_float(x, bits, debug) {
  if (typeof x !== "number") { x = Number(x); }
  if (Number.isNaN(x)) { return "NaN"; }
  if (x === Infinity) { return "inf"; }
  if (x === -Infinity) { return "-inf"; }
  const is32 = bits === 32;
  const maxp = is32 ? 9 : 17;
  let s = "" + x;
  for (let p = 1; p <= maxp; p++) {
    const t = x.toPrecision(p);
    const v = Number(t);
    if (is32 ? Math.fround(v) === x : v === x) { s = t; break; }
  }
  let out;
  const e = s.indexOf("e");
  if (e < 0) {
    out = s;
  } else {
    // toPrecision may use exponent form; Rust never does for Display/Debug.
    const neg = s[0] === "-";
    const mant = (neg ? s.slice(1, e) : s.slice(0, e)).replace(".", "");
    const exp = parseInt(s.slice(e + 1), 10);
    let digits, zeros;
    if (exp >= 0) {
      const frac = mant.length - 1;
      if (frac <= exp) {
        digits = mant + "0".repeat(exp - frac);
        zeros = "";
      } else {
        digits = mant.slice(0, exp + 1) + "." + mant.slice(exp + 1);
        zeros = "";
      }
    } else {
      digits = "0." + "0".repeat(-exp - 1) + mant;
      zeros = "";
    }
    out = (neg ? "-" : "") + digits + zeros;
  }
  if (debug && out.indexOf(".") < 0) { out += ".0"; }
  return out;
}
function __debug_val(d, v) {
  if (d === null || d === undefined || d.k === "n" || d.k === "s") {
    if (typeof v === "string") { return '"' + __esc_debug_str(v) + '"'; }
    if (typeof v === "boolean") { return v ? "true" : "false"; }
    if (typeof v === "bigint") { return v.toString(); }
    if (typeof v === "number") { return "" + v; }
    return String(v);
  }
  if (d.k === "f") { return __fmt_float(v, d.bits, true); }
  if (d.k === "u") { return "()"; }
  if (d.k === "p") { return "0x0"; }
  if (d.k === "T") {
    return d.n + "(" + v.map((x, i) => __debug_val(d.e[i], x)).join(", ") + ")";
  }
  if (d.k === "a") {
    return "[" + v.map((x) => __debug_val(d.e, x)).join(", ") + "]";
  }
  if (d.k === "t") {
    if (d.e.length === 1) { return "(" + __debug_val(d.e[0], v[0]) + ",)"; }
    return "(" + v.map((x, i) => __debug_val(d.e[i], x)).join(", ") + ")";
  }
  if (d.k === "S") {
    const fs = d.f.map((n, i) => n + ": " + __debug_val(null, v[i]));
    return d.n + " { " + fs.join(", ") + " }";
  }
  if (d.k === "E") {
    const entry = d.v[v.t];
    if (entry === undefined) { return d.n + "(?)"; }
    const [vname, isStruct, fields] = entry;
    if (v.v.length === 0) { return vname; }
    if (isStruct) {
      const fs = fields.map((n, i) => n + ": " + __debug_val(null, v.v[i]));
      return vname + " { " + fs.join(", ") + " }";
    }
    return vname + "(" + v.v.map((x) => __debug_val(null, x)).join(", ") + ")";
  }
  return String(v);
}
// kind: 0 display, 1 debug, 2 noop, 3 octal, 4 lowerhex, 5 upperhex,
//       6 pointer, 7 binary, 8 lowerexp, 9 upperexp.
function __fmt_int_radix(v, radix, upper, alternate, bits, signed) {
  let mag;
  if (typeof v === "bigint") {
    mag = BigInt.asUintN(bits, v).toString(radix);
  } else {
    v = v >>> 0;
    if (bits < 32) { v = v & ((1 << bits) - 1); }
    mag = v.toString(radix);
  }
  if (upper) { mag = mag.toUpperCase(); }
  let pre = "";
  if (alternate) {
    pre = radix === 16 ? (upper ? "0X" : "0x") : radix === 8 ? "0o" : "0b";
  }
  return pre + mag;
}
function __fmt_num(v, kind, alternate, bits, signed) {
  if (typeof v === "boolean") { v = v ? 1 : 0; }
  switch (kind) {
    case 3: return __fmt_int_radix(v, 8, false, alternate, bits, signed);
    case 4: return __fmt_int_radix(v, 16, false, alternate, bits, signed);
    case 5: return __fmt_int_radix(v, 16, true, alternate, bits, signed);
    case 7: return __fmt_int_radix(v, 2, false, alternate, bits, signed);
    case 8: return Number(v).toExponential();
    case 9: return Number(v).toExponential().toUpperCase().replace("E", "E");
    case 6: return "0x0";
    default: break;
  }
  if (typeof v === "bigint") { return v.toString(); }
  return "" + v;
}
function __apply_width(s, isNum, flags, width) {
  if (width === undefined || s.length >= width) { return s; }
  const fill = String.fromCodePoint(flags & 0x1fffff) || " ";
  const align = (flags >>> 29) & 3;
  const left = align === 0 || (align === 3 && !isNum);
  const zeroPad = (flags & (1 << 24)) !== 0 && isNum && (fill === " " || fill === "0");
  const f = zeroPad ? "0" : fill;
  if (left) { return s + f.repeat(width - s.length); }
  if (zeroPad && (s[0] === "-" || s[0] === "+")) {
    return s[0] + f.repeat(width - s.length) + s.slice(1);
  }
  if (align === 2) {
    const total = width - s.length, l = Math.floor(total / 2);
    return f.repeat(l) + s + f.repeat(total - l);
  }
  return f.repeat(width - s.length) + s;
}
function __fmt_arg(d, v, kind, flags, width, prec) {
  if (kind === 2) { return ""; }
  const alternate = (flags & (1 << 23)) !== 0;
  const plus = (flags & (1 << 21)) !== 0;
  let s, isNum = true;
  if (d && d.k === "f" && kind <= 1) {
    // Float Display/Debug: the precision, when given, is applied by Rust to
    // the shortest round-tripping form, so round-trip at that width.
    if (kind === 1) {
      s = __fmt_float(v, d.bits, true);
    } else if (prec !== undefined) {
      s = __fmt_float(Number(v.toFixed ? Number(v).toFixed(prec) : v), d.bits, false);
    } else {
      s = __fmt_float(v, d.bits, false);
    }
    isNum = true;
  } else if (kind === 1) {
    s = __debug_val(d, v);
    isNum = typeof v === "number" || typeof v === "bigint";
  } else if (kind >= 3) {
    const bits = (d && d.bits) || 32, signed = !d || d.signed !== false;
    s = __fmt_num(v, kind, alternate, bits, signed);
  } else {
    // display
    if (typeof v === "bigint") { s = v.toString(); }
    else if (typeof v === "boolean") { s = v ? "true" : "false"; isNum = false; }
    else if (typeof v === "string") { s = v; isNum = false; }
    else if (typeof v === "number") {
      s = "" + v;
      if (prec !== undefined && Number.isFinite(v) && !Number.isInteger(v)) {
        s = v.toFixed(prec);
      } else if (prec !== undefined && Number.isInteger(v)) {
        s = v.toFixed(prec);
      }
    } else { s = String(v); isNum = false; }
    if (typeof v === "number" && plus && v >= 0) { s = "+" + s; }
  }
  if (typeof width === "number") { s = __apply_width(s, isNum, flags, width); }
  return s;
}
// Template decoder: bytes = number array; args = [[desc, value, kind], ...]
// with kinds resolved by the emitter (Count args pre-resolved to u16 numbers).
function __fmt_template(bytes, args) {
  let out = "", i = 0, auto = 0;
  const takeU16 = () => {
    const n = bytes[i] | (bytes[i + 1] << 8);
    i += 2;
    return n;
  };
  while (i < bytes.length) {
    const n = bytes[i++];
    if (n === 0) { break; }
    if (n < 128) {
      out += __dec.decode(new Uint8Array(bytes.slice(i, i + n)));
      i += n;
      continue;
    }
    if (n === 128) {
      const len = takeU16();
      out += __dec.decode(new Uint8Array(bytes.slice(i, i + len)));
      i += len;
      continue;
    }
    // placeholder, first byte n (0b11......)
    let flags = 0, width, prec, argIndex;
    if (n & 1) { flags = bytes[i] | (bytes[i + 1] << 8) | (bytes[i + 2] << 16) | (bytes[i + 3] << 24); i += 4; }
    if (n & 2) { width = takeU16(); }
    if (n & 4) { prec = takeU16(); }
    if (n & 8) { argIndex = takeU16(); } else { argIndex = auto++; }
    if (n & 16) { const w = args[width]; width = (typeof w === "number") ? w : w[1]; }
    if (n & 32) { const p = args[prec]; prec = (typeof p === "number") ? p : p[1]; }
    const a = args[argIndex];
    if (a === undefined) { __panic("fmt: argument index out of range"); }
    if (typeof a === "number") { out += a; continue; } // Count passthrough (should not render)
    out += __fmt_arg(a[0], a[1], a[2], flags, width, prec);
  }
  return out;
}
function __slice_sort_inplace(a) {
  const b = __slice_sort(a);
  a.length = 0;
  for (const x of b) { a.push(x); }
  return a;
}
function __slice_sort(a) {
  const b = a.slice();
  for (let i = 1; i < b.length; i++) {
    const k = b[i];
    let j = i - 1;
    while (j >= 0 && (b[j] > k || (b[j] !== b[j] && k !== k))) { b[j + 1] = b[j]; j--; }
    b[j + 1] = k;
  }
  return b;
}

// ---- iterators ([arr, i, end, step]) ----
// `next` advances the shared triple, so mutation is visible to the caller.
function __it_new(a) { return [a, 0, a.length, 1]; }
function __it_new_rev(a) { return a.length === 0 ? [a, 0, -1, -1] : [a, a.length - 1, -1, -1]; }
// Ranges are stored as `[lo, hi, inc]`. They are accepted anywhere an
// iterator is expected, and the first use memoizes the adapter quad so that
// repeated `next` calls advance the same cursor.
const __it_ranges = new WeakMap();
function __it_norm(it) {
  if (it.length === 3) {
    let q = __it_ranges.get(it);
    if (q === undefined) {
      const arr = [];
      if (it[2]) { for (let v = it[0]; v <= it[1]; v++) { arr.push(v); } }
      else { for (let v = it[0]; v < it[1]; v++) { arr.push(v); } }
      q = [arr, 0, arr.length, 1];
      __it_ranges.set(it, q);
    }
    return q;
  }
  return it;
}
function __it_rem(it) { return it[3] > 0 ? it[2] - it[1] : it[1] - it[2] + 1; }
function __it_len(it) { return __it_rem(__it_norm(it)); }
function __it_next_raw(it) {
  it = __it_norm(it);
  if (it[3] > 0) { return it[1] < it[2] ? { ok: true, v: it[0][it[1]++] } : null; }
  return it[1] > it[2] ? { ok: true, v: it[0][it[1]--] } : null;
}
function __it_slice(it) {
  it = __it_norm(it);
  return it[3] > 0 ? it[0].slice(it[1], it[2]) : it[0].slice(it[2] + 1, it[1] + 1);
}
// Collect an iterator into a JS array.
function __it_collect(it) {
  const out = [];
  for (;;) {
    const r = __it_next_raw(it);
    if (r === null) { return out; }
    out.push(r.v);
  }
}
function __it_count(it) { return __it_collect(it).length; }
function __it_last(it) {
  const a = __it_collect(it);
  return a.length === 0 ? undefined : a[a.length - 1];
}
function __it_nth(it, n) {
  const a = __it_collect(it);
  return n < a.length ? a[n] : undefined;
}
function __it_position(it, fn, env) {
  let k = 0;
  for (;;) {
    const r = __it_next_raw(it);
    if (r === null) { return null; }
    if (fn(env, r.v)) { return k; }
    k++;
  }
}
function __it_find(it, fn, env) {
  for (;;) {
    const r = __it_next_raw(it);
    if (r === null) { return undefined; }
    if (fn(env, r.v)) { return r.v; }
  }
}
function __it_any(it, fn, env) {
  for (;;) {
    const r = __it_next_raw(it);
    if (r === null) { return false; }
    if (fn(env, r.v)) { return true; }
  }
}
function __it_all(it, fn, env) {
  for (;;) {
    const r = __it_next_raw(it);
    if (r === null) { return true; }
    if (!fn(env, r.v)) { return false; }
  }
}
function __it_fold(it, init, fn, env) {
  let acc = init;
  for (;;) {
    const r = __it_next_raw(it);
    if (r === null) { return acc; }
    acc = fn(env, acc, r.v);
  }
}
function __it_for_each(it, fn, env) {
  for (;;) {
    const r = __it_next_raw(it);
    if (r === null) { return; }
    fn(env, r.v);
  }
}
function __it_map(it, fn, env) { return __it_new(__it_collect(it).map((x) => fn(env, x))); }
function __it_filter(it, fn, env) {
  return __it_new(__it_collect(it).filter((x) => fn(env, x)));
}
function __it_filter_map(it, fn, env) {
  const out = [];
  for (const x of __it_collect(it)) {
    const y = fn(env, x);
    // Option discriminants are plain numbers (None = 0, Some = 1).
    if (y.t !== 0) { out.push(y.v[0]); }
  }
  return __it_new(out);
}
function __it_flat_map(it, fn, env) {
  const out = [];
  for (const x of __it_collect(it)) {
    const it2 = fn(env, x);
    if (Array.isArray(it2) && it2.length === 4 && it2[3] !== undefined && typeof it2[0] !== "string") {
      for (const y of __it_collect(it2)) { out.push(y); }
    } else {
      for (const y of it2) { out.push(y); }
    }
  }
  return __it_new(out);
}
function __it_flatten(it, present) {
  const out = [];
  for (const x of __it_collect(it)) {
    if (Array.isArray(x)) { for (const y of x) { out.push(y); } }
    else if (present !== undefined) { if (x.t === present) { out.push(x.v[0]); } }
  }
  return __it_new(out);
}
function __it_rev(it) { return __it_new(__it_collect(it).reverse()); }
function __it_enumerate(it) { return __it_new(__it_collect(it).map((x, i) => [i, x])); }
function __it_zip(a, b) {
  const x = __it_collect(a), y = __it_collect(b);
  const out = [];
  for (let i = 0; i < Math.min(x.length, y.length); i++) { out.push([x[i], y[i]]); }
  return __it_new(out);
}
function __it_chain(a, b) { return __it_new(__it_collect(a).concat(__it_collect(b))); }
function __it_take(it, n) { return __it_new(__it_collect(it).slice(0, n)); }
function __it_skip(it, n) { return __it_new(__it_collect(it).slice(n)); }
function __it_step_by(it, n) {
  const a = __it_collect(it);
  const out = [];
  for (let i = 0; i < a.length; i += n) { out.push(a[i]); }
  return __it_new(out);
}
function __it_cloned(it) {
  const a = __it_collect(it);
  return __it_new(a.map((x) => Array.isArray(x) && x.length === 1 && x.kind !== "enum" ? x.slice() : x));
}
function __it_copied(it) { return __it_collect(it).length >= 0 ? __it_new(__it_collect(it)) : it; }
function __it_sum(it, z) {
  let s = z;
  for (const x of __it_collect(it)) { s = s + x; }
  return s;
}
function __it_product(it, one) {
  let s = one;
  for (const x of __it_collect(it)) { s = s * x; }
  return s;
}
function __it_min(it, dflt) {
  const a = __it_collect(it);
  if (a.length === 0) { return dflt; }
  return a.reduce((x, y) => (x < y ? x : y));
}
function __it_max(it, dflt) {
  const a = __it_collect(it);
  if (a.length === 0) { return dflt; }
  return a.reduce((x, y) => (x > y ? x : y));
}
function __it_min_by(it, fn, env) {
  const a = __it_collect(it);
  if (a.length === 0) { return undefined; }
  let best = a[0];
  for (let i = 1; i < a.length; i++) {
    const o = fn(env, [best, a[i]]);
    if (o === 1) { best = a[i]; }
  }
  return best;
}
function __it_max_by(it, fn, env) {
  const a = __it_collect(it);
  if (a.length === 0) { return undefined; }
  let best = a[0];
  for (let i = 1; i < a.length; i++) {
    const o = fn(env, [best, a[i]]);
    if (o === -1) { best = a[i]; }
  }
  return best;
}
function __it_eq(a, b) {
  const x = __it_collect(a), y = __it_collect(b);
  return x.length === y.length && x.every((v, i) => __deep_eq(v, y[i]));
}
function __it_next_back(it) {
  if (it[3] > 0) { return it[2] > it[1] ? { ok: true, v: it[0][--it[2]] } : null; }
  return it[1] < it[2] - 1 ? null : (it[1] >= 0 ? { ok: true, v: it[0][it[1]++] } : null);
}
// collect::<Result<V, E>>() over an iterator of Result items.
function __it_collect_result(it, errDiscr) {
  const v = [];
  for (;;) {
    const r = __it_next_raw(it);
    if (r === null) { return { v: v, err: undefined }; }
    const item = r.v;
    if (item.t === errDiscr) { return { v: undefined, err: item.v[0] }; }
    v.push(item.v[0]);
  }
}

// ---- Option / Result helpers (enum payload convention: {t, v}) ----
// `on = discriminant of Some/O Ok`, `en = None/Err`.
function __opt_map(x, on, en, fn, env) {
  return x.t === on ? { t: on, v: [fn(env, x.v[0])] } : x;
}
function __opt_and_then(x, on, en, fn, env) {
  return x.t === on ? fn(env, x.v[0]) : x;
}
function __opt_map_or(x, on, en, dflt, fn, env) {
  return x.t === on ? fn(env, x.v[0]) : dflt;
}
function __opt_map_or_else(x, on, en, dfltFn, fn, env) {
  return x.t === on ? fn(env, x.v[0]) : dfltFn(env);
}
function __opt_filter(x, on, en, fn, env) {
  if (x.t !== on) { return x; }
  return fn(env, x.v[0]) ? x : { t: en, v: [] };
}
function __opt_unwrap_or(x, on, dflt) { return x.t === on ? x.v[0] : dflt; }
function __opt_and(x, y, on) { return x.t === on ? y : x; }
function __opt_or(x, y, on, en) { return x.t === on ? x : y; }
function __opt_xor(x, y, on, en) {
  if (x.t === on && y.t === on) { return { t: en, v: [] }; }
  return x.t === on ? x : y;
}
function __opt_inspect(x, on, fn, env) {
  if (x.t === on) { fn(env, x.v[0]); }
  return x;
}
function __res_map(x, ok, err, fn, env) {
  return x.t === ok ? { t: ok, v: [fn(env, x.v[0])] } : x;
}
function __res_map_err(x, ok, err, fn, env) {
  return x.t === ok ? x : { t: err, v: [fn(env, x.v[0])] };
}
function __res_and_then(x, ok, err, fn, env) {
  return x.t === ok ? fn(env, x.v[0]) : x;
}
function __res_or_else(x, ok, err, fn, env) {
  return x.t === ok ? x : fn(env, x.v[0]);
}
function __res_unwrap_or_else(x, ok, dfltFn, env) {
  return x.t === ok ? x.v[0] : dfltFn(env, x.v[0]);
}
// Deep structural copy used by `Clone` for our object representations.
function __clone(x) {
  if (x === null || typeof x !== "object") { return x; }
  if (x.__rustPanic) { return x; }
  if (Array.isArray(x)) {
    // Iterator triples/quads and other internal kinds too: copy the outer
    // array (and clone elements) — semantically fine for our reprs.
    return x.map(__clone);
  }
  if (x.t !== undefined && Array.isArray(x.v)) {
    return { t: x.t, v: x.v.map(__clone) };
  }
  const o = {};
  for (const k of Object.keys(x)) { o[k] = __clone(x[k]); }
  return o;
}
