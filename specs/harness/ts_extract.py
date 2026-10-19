#!/usr/bin/env python3
"""ts_extract — tree-sitter fact cards + deterministic syntax signals.

Two outputs, one parse:

1. Fact cards (for the jev semantic layer): per top-level item —
   kind/name/visibility/signature/doc-comment/called-symbols/unsafe
   count. jev judges name-behavior correspondence, comment rubric and
   placement against these structured facts instead of raw text.

2. Deterministic signals (this CLI, mode:file): zero-LLM findings with
   category + suggestion, so the coding agent can act directly:
   - TS-CLONE-OWNED: clone() on an owned parameter -> WARN borrow + lifetime
   - TS-UNSAFE-NOINVARIANT: unsafe block whose item doc comment and
     the comment run directly above the block both lack a safety
     invariant (safety/invariant keyword) -> FAIL (hard block)
     (transparent holders: the wrapping expression_statement, and the
     condition/iterator/value slot of if/while/for/let statements;
     bodies stay strict)

   Known blind spot (by design): macro arguments parse as opaque
   token_tree nodes, so `assert!(unsafe { ... })` produces no
   unsafe_block node at all — neither the block nor a note above it
   is visible to this rule. Text-level scanning of macro arguments
   would close this gap; it is out of scope for the tree-based
   extractor.

Soft dependency: tree_sitter + tree_sitter_rust. Missing -> silent
skip (empty findings), same policy as lizard in ccn_gate.

Gate protocol: stdout = JSON array of
{"id","severity","path","line","message","category","suggestion"}.
stdin (mode:file): "===== FILE: rel =====\\n<content>" blocks.
"""
import json
import re
import sys

try:
    import tree_sitter_rust as _tsr
    from tree_sitter import Language as _Language
    from tree_sitter import Parser as _Parser
    _PARSER = _Parser(_Language(_tsr.language()))
except Exception:  # noqa: BLE001 - soft dep, degrade silently
    _PARSER = None

COPY_PRIMITIVES = {
    "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64",
    "i128", "usize", "isize", "bool", "char", "f32", "f64",
}
ITEM_KINDS = {
    "function_item": "fn", "struct_item": "struct", "enum_item": "enum",
    "trait_item": "trait", "impl_item": "impl", "macro_definition": "macro",
    "mod_item": "mod", "const_item": "const", "type_item": "type",
}
DOC_KEYWORDS = ("safety", "invariant")
SLOT_TRANSPARENT = (
    # expression-side nodes through which the condition / iterator /
    # value slot of an `if` / `while` / `for` / `let` statement stays
    # transparent to a note above the statement:
    # - `binary_expression`: `if unsafe { .. } != 0` parses the block
    #   as one operand of the condition;
    # - `parenthesized_expression`: `if (unsafe { .. }) { .. }`.
    # A call site (`arguments`, e.g. `if check(unsafe { .. })`) is
    # deliberately NOT transparent — that form is out of scope for this
    # rule, so a block buried in a call argument keeps its own
    # direct-above note. Bodies (`block` nodes) are likewise excluded,
    # so a block in a then/else/for body still needs its own note.
    "binary_expression", "parenthesized_expression",
)
FILE_MARKER = re.compile(r"^===== FILE: (.+) =====$")
TYPE_NODE_TYPES = (
    "generic_type", "type_identifier", "tuple_type",
    "reference_type", "qualified_type",
)


def _text(src: bytes, node) -> str:
    return src[node.start_byte:node.end_byte].decode("utf-8", "replace")


def _name(src: bytes, item) -> str:
    for c in item.children:
        if c.type in ("identifier", "type_identifier"):
            return _text(src, c)
    return "?"


def _doc_before(root, src: bytes, item) -> str:
    """Doc comment lines immediately preceding the item (sibling walk)."""
    lines = []
    idx = None
    for i, c in enumerate(root.children):
        if c.id == item.id:
            idx = i
            break
    if idx is None:
        return ""
    for c in reversed(root.children[:idx]):
        if c.type == "line_comment" and _text(src, c).lstrip().startswith("///"):
            lines.append(_text(src, c).lstrip().lstrip("/").strip())
        else:
            break
    return " ".join(reversed(lines))


def _calls(src: bytes, node) -> list:
    out = []
    if node.type == "call_expression":
        fn = None
        for c in node.children:
            if c.type in ("identifier", "field_expression",
                          "scoped_identifier"):
                fn = c
        if fn is not None:
            out.append(_text(src, fn).rsplit(".", 1)[-1].rsplit("::", 1)[-1])
    for c in node.children:
        out.extend(_calls(src, c))
    return out


def _params(src: bytes, item) -> list:
    """[(name, raw_param_text)] for function items; [] otherwise."""
    for c in item.children:
        if c.type == "parameters":
            out = []
            for p in c.children:
                if p.type != "parameter":
                    continue
                raw = _text(src, p)
                m = re.match(r"(\w+)\s*(?::|$)", raw)
                if m:
                    out.append((m.group(1), raw))
            return out
    return []


def parse_items(content: str) -> list:
    """Parse Rust source -> fact cards (dicts). Raises on no grammar."""
    if _PARSER is None:
        raise ImportError("tree_sitter_rust unavailable")
    src = content.encode("utf-8")
    tree = _PARSER.parse(src)
    cards = []
    for node in tree.root_node.children:
        if node.type not in ITEM_KINDS:
            continue
        card = {
            "kind": ITEM_KINDS[node.type],
            "name": _name(src, node),
            "line": node.start_point[0] + 1,
            "vis": "pub" if any(c.type == "visibility_modifier"
                                for c in node.children) else "",
            "doc": _doc_before(tree.root_node, src, node),
        }
        seen_params = False
        for c in node.children:
            if c.type == "parameters":
                card["params"] = _text(src, c)
                seen_params = True
            elif c.type in TYPE_NODE_TYPES and seen_params \
                    and "ret" not in card:
                card["ret"] = _text(src, c).strip()
        card["calls"] = sorted(set(_calls(src, node)))[:24]
        card["unsafe"] = sum(1 for d in _walk(node) if d.type == "unsafe_block")
        if node.type == "struct_item":
            card["fields"] = _struct_fields(src, node)
        if node.type == "impl_item":
            card["trait"] = _impl_trait(src, node)
            card["methods"] = [
                _name(src, d) for d in _walk(node) if d.type == "function_item"
            ]
        cards.append(card)
    return cards


def _walk(node):
    yield node
    for c in node.children:
        yield from _walk(c)


def _struct_fields(src: bytes, struct_node) -> list:
    for c in struct_node.children:
        if c.type == "field_declaration_list":
            return [_text(src, f).strip() for f in c.children
                    if f.type == "field_declaration"]
    return []


def _impl_trait(src: bytes, impl_node) -> str:
    for c in impl_node.children:
        if c.type == "type_identifier":
            return _text(src, c)
    return ""


def fact_cards_text(cards: list) -> str:
    """Render fact cards compactly for the jev state."""
    out = []
    for c in cards:
        head = f"[{c['kind']}] {c['name']}"
        if c.get("vis"):
            head += f" ({c['vis']})"
        sig = c.get("params", "") + (f" -> {c['ret']}" if c.get("ret") else "")
        if sig:
            head += f" {sig}"
        out.append(head)
        if c.get("doc"):
            out.append(f"  doc: {c['doc']}")
        if c.get("fields"):
            out.append(f"  fields: {'; '.join(c['fields'][:16])}")
        if c.get("trait"):
            out.append(f"  impl of: {c['trait']}")
        if c.get("methods"):
            out.append(f"  methods: {', '.join(c['methods'][:16])}")
        if c.get("calls"):
            out.append(f"  calls: {', '.join(c['calls'])}")
        if c.get("unsafe"):
            out.append(f"  unsafe blocks: {c['unsafe']}")
    return "\n".join(out)


def _receiver_of_call(src: bytes, call) -> str:
    for c in call.children:
        if c.type == "field_expression":
            return _text(src, c).rsplit(".", 1)[0].strip()
    return ""


def signals(rel: str, content: str) -> list:
    """Deterministic syntax signals with category + suggestion."""
    if _PARSER is None:
        return []
    src = content.encode("utf-8")
    tree = _PARSER.parse(src)
    root = tree.root_node
    findings = []
    owned_params_by_item = {}
    for item in root.children:
        if item.type == "function_item":
            owned_params_by_item[item.id] = [
                (n, raw) for n, raw in _params(src, item)
                if not raw.lstrip().startswith("&")
                and "&" not in raw
                and not _is_copy_param(raw)
            ]
    for node in _walk(root):
        line = node.start_point[0] + 1
        if node.type == "call_expression":
            for c in node.children:
                if (c.type == "field_expression"
                        and _text(src, c).rsplit(".", 1)[-1] == "clone"):
                    recv = _receiver_of_call(src, node)
                    if _find_owner(owned_params_by_item, node, recv):
                        findings.append({
                            "id": "TS-CLONE-OWNED", "severity": "WARN",
                            "path": rel, "line": line,
                            "category": "syntax",
                            "message": f"clone() of owned parameter "
                                       f"'{recv}' — borrow instead",
                            "suggestion": f"take &{recv} (add a "
                                          f"lifetime if it outlives "
                                          f"the call)",
                        })
        elif node.type == "unsafe_block" and not _has_safety_doc(root, src, node):
            findings.append({
                    "id": "TS-UNSAFE-NOINVARIANT", "severity": "FAIL",
                    "path": rel, "line": line,
                    "category": "unsafe",
                    "message": "unsafe block without a documented safety "
                               "invariant (why unsafe is required, what "
                               "must hold)",
                    "suggestion": "add a SAFETY comment above the block "
                                  "stating the invariant and why the "
                                  "caller must uphold it",
                })
    return findings


def _is_copy_param(raw: str) -> bool:
    m = re.search(r":\s*([\w<>]+)", raw)
    return bool(m) and m.group(1) in COPY_PRIMITIVES


def _find_owner(owned_by_item: dict, call_node, recv: str):
    """Walk up from the call to its enclosing function_item; check recv."""
    if not recv:
        return None
    return any(n == recv for n, _ in
               owned_by_item.get(_enclosing(call_node), ()))


def _enclosing(node):
    cur = node
    while cur is not None and cur.type != "function_item":
        cur = cur.parent
    return cur.id if cur is not None else None


def _has_safety_doc(root, src: bytes, unsafe_node) -> bool:
    """The item containing the unsafe block carries a safety invariant
    (doc comment on the item) or a `// SAFETY:` line-comment run
    immediately above the block."""
    cur = unsafe_node
    while cur is not None and cur.type not in ITEM_KINDS:
        cur = cur.parent
    if cur is not None:
        doc = _doc_before(root, src, cur).lower()
        if any(k in doc for k in DOC_KEYWORDS):
            return True
    return _safety_comment_above(unsafe_node, src)


def _note_context(unsafe_node):
    """The node whose preceding siblings carry this block's note.

    Transparent holders (a note above them belongs to the block):
    - `expression_statement`: `unsafe { ... };` as a statement;
    - `if` / `while` / `for` / `let` **only** when the block sits in
      that statement's condition slot or iterator/value slot — see
      _in_condition_or_value_slot(). Bodies (then/else/for body) stay
      strict: a block inside one keeps its own direct-above note.
    """
    # Walk up through the transparent expression nodes so that
    # `if unsafe { .. } != 0` reaches the if_expression via its
    # binary_expression.
    cur = unsafe_node
    while (cur.parent is not None
           and cur.parent.type in SLOT_TRANSPARENT):
        cur = cur.parent
    stmt = cur.parent
    if stmt is None:
        return unsafe_node
    if stmt.type in ("if_expression", "while_expression",
                     "for_expression"):
        # The block is in the statement's condition / iterator slot —
        # the note belongs above the whole statement (its wrapping
        # expression_statement, when present, is transparent too).
        if not _in_condition_or_value_slot(stmt, cur):
            return unsafe_node
        holder = stmt
        if (holder.parent is not None
                and holder.parent.type == "expression_statement"):
            holder = holder.parent
        return holder
    if stmt.type == "let_declaration":
        # Value slot of a let: note above the let itself.
        return stmt if _in_condition_or_value_slot(stmt, cur) \
            else unsafe_node
    # Bare `unsafe { .. };` statement (or any other context): the
    # expression_statement wrapper stays transparent as before.
    return stmt if stmt.type == "expression_statement" else unsafe_node


def _in_condition_or_value_slot(stmt, unsafe_node) -> bool:
    """Whether `unsafe_node` holds the expression slot of statement
    `stmt` that a statement-level note describes:
    - `if` / `while`: the condition = the first expression child after
      the `if` / `while` keyword (then/else/body are `block` nodes and
      never match — a block in a body keeps its own note rule);
    - `for`: the iterator = the child right after the `in` keyword;
    - `let`: the value = the child right after `=`.
    The slot may reach the block through the transparent expression
    nodes in SLOT_TRANSPARENT only (`if unsafe { .. } != 0` parses the
    block inside a binary_expression under the condition).
    """
    t = stmt.type
    if t in ("if_expression", "while_expression"):
        slot = next((c for c in stmt.children
                     if c.type not in ("if", "while", "else", "block")), None)
    elif t in ("for_expression", "let_declaration"):
        key = "in" if t == "for_expression" else "="
        children = stmt.children
        i = next((j for j, c in enumerate(children) if c.type == key), None)
        slot = children[i + 1] if i is not None and i + 1 < len(children) \
            else None
    else:
        return False
    return slot is not None and _reaches_in_expr_side(slot, unsafe_node)


def _reaches_in_expr_side(slot, target) -> bool:
    """Whether `target` is `slot` itself or sits under it through
    SLOT_TRANSPARENT expression nodes only — never through a body
    `block`, which is what keeps then/else/body blocks strict."""
    cur = slot
    while cur.id != target.id:
        if cur.type not in SLOT_TRANSPARENT:
            return False
        nxt = next((c for c in cur.children
                    if c.start_byte <= target.start_byte
                    and target.end_byte <= c.end_byte), None)
        if nxt is None:
            return False
        cur = nxt
    return True


def _safety_comment_above(unsafe_node, src: bytes) -> bool:
    """A `// SAFETY:` line-comment run directly above the block.

    Boundary decisions (kept in sync with test_ts_extract.py):
    - a consecutive run of `line_comment` nodes directly above the
      block counts: interleaved plain comments and multi-line
      `// SAFETY:` sections are fine; the wrapping
      `expression_statement` is transparent, so a comment above
      `unsafe { ... };` belongs to the block. The condition / iterator
      / value slot of an `if` / `while` / `for` / `let` statement is
      transparent the same way (`if unsafe { ... } != 0`,
      `while unsafe { ... }`, `for .. in unsafe { ... }`,
      `let rc = unsafe { ... };`), see _note_context(); bodies stay
      strict and need their own note directly above the block.
    - blank lines are tree whitespace, never nodes, so they do not
      break the run; any other node (code, `#[attr]`) does — a note
      separated from the block by code or an attribute does not
      belong to it.
    - keyword match is case-insensitive over the same DOC_KEYWORDS
      the doc channel uses (`SAFETY:` / `safety:` / `invariant:`).
    - a comment on the first line inside the block never counts:
      only preceding siblings of the block's own context are walked.
    """
    holder = _note_context(unsafe_node)
    parent = holder.parent
    if parent is None:
        return False
    idx = None
    for i, c in enumerate(parent.children):
        if c.id == holder.id:
            idx = i
            break
    if idx is None:
        return False
    run = []
    for c in reversed(parent.children[:idx]):
        if c.type != "line_comment":
            break
        run.append(_text(src, c).lower())
    return any(k in c for c in run for k in DOC_KEYWORDS)


def parse_files(payload: str) -> dict:
    files: dict = {}
    rel = None
    chunks: list = []
    for line in payload.splitlines():
        m = FILE_MARKER.match(line)
        if m:
            if rel is not None:
                files[rel] = "\n".join(chunks)
            rel = m.group(1)
            chunks = []
        elif rel is not None:
            chunks.append(line)
    if rel is not None:
        files[rel] = "\n".join(chunks)
    return files


def main() -> int:
    if _PARSER is None:
        print(json.dumps([]))
        return 0
    findings = []
    for rel, content in parse_files(sys.stdin.read()).items():
        try:
            findings.extend(signals(rel, content))
        except Exception as e:  # noqa: BLE001 - 解析失败按文件跳过降级
            print(f"ts_extract: skipping {rel} ({e})",
                  file=sys.stderr)
    print(json.dumps(findings, ensure_ascii=False))
    return 0


if __name__ == "__main__":
    sys.exit(main())
