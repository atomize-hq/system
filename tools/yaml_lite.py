"""
yaml_lite.py

Minimal YAML subset loader/dumper used as a fallback when PyYAML isn't available.

Supported (enough for this repo's pipeline + stage front matter + state):
- Mappings and sequences (indent-based, 2+ spaces)
- "Indentless" sequences under a mapping key (YAML 1.2 feature)
- Scalars: strings, bool, int, float, null
- Quoted scalars: single-quoted and double-quoted (including multi-line)
- Block scalars: literal (|) and folded (>)
- YAML document streams separated by lines containing only '---'

Not supported:
- Anchors/aliases, tags, complex keys, flow style, advanced escapes, etc.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Dict, Iterable, Iterator, List, Optional, Tuple


class YamlLiteError(ValueError):
    pass


def safe_load(text: str) -> Any:
    docs = list(safe_load_all(text))
    if not docs:
        return None
    if len(docs) == 1:
        return docs[0]
    # Match PyYAML safe_load behavior (first document only).
    return docs[0]


def safe_load_all(text: str) -> Iterator[Any]:
    # Split on document markers. This is intentionally simplistic but matches repo usage.
    lines = _split_lines(text)
    doc_lines: List[str] = []
    saw_marker = False

    def flush() -> Optional[str]:
        nonlocal doc_lines
        if not doc_lines:
            return None
        s = "\n".join(doc_lines).strip("\n")
        doc_lines = []
        return s

    for line in lines:
        if line.strip() == "---":
            saw_marker = True
            s = flush()
            if s is not None:
                yield _parse_document(s)
            continue
        doc_lines.append(line)

    s = flush()
    if s is not None:
        yield _parse_document(s)
    elif not saw_marker:
        # Empty input with no markers -> yields nothing.
        return


def safe_dump(data: Any, sort_keys: bool = True) -> str:
    out = _dump_node(data, indent=0, sort_keys=sort_keys)
    if not out.endswith("\n"):
        out += "\n"
    return out


def _split_lines(text: str) -> List[str]:
    text = text.replace("\r\n", "\n").replace("\r", "\n")
    # Strip UTF-8 BOM if present
    if text.startswith("\ufeff"):
        text = text[1:]
    return text.split("\n")


def _parse_document(text: str) -> Any:
    lines = _split_lines(text)
    idx = _skip_ignorable(lines, 0)
    if idx >= len(lines):
        return None
    node, idx2 = _parse_node(lines, idx, base_indent=_indent(lines[idx]))
    idx2 = _skip_ignorable(lines, idx2)
    return node


def _skip_ignorable(lines: List[str], i: int) -> int:
    while i < len(lines):
        s = lines[i].strip()
        if s == "" or s.startswith("#"):
            i += 1
            continue
        return i
    return i


def _indent(line: str) -> int:
    n = 0
    for ch in line:
        if ch == " ":
            n += 1
        else:
            break
    return n


def _strip_inline_comment(s: str) -> str:
    # Very small heuristic: treat " #..." as a comment for unquoted scalars.
    # Do not attempt to handle quotes/escapes.
    if " #" in s:
        return s.split(" #", 1)[0].rstrip()
    return s.rstrip()


def _parse_node(lines: List[str], i: int, base_indent: int) -> Tuple[Any, int]:
    i = _skip_ignorable(lines, i)
    if i >= len(lines):
        return None, i
    line = lines[i]
    cur_indent = _indent(line)
    if cur_indent < base_indent:
        return None, i

    stripped = line[cur_indent:]
    # Scalar node (used in repo for multi-line quoted values under a key).
    if stripped.startswith('"'):
        return _parse_quoted_scalar(lines, i + 1, cur_indent, stripped, quote='"')
    if stripped.startswith("'"):
        return _parse_quoted_scalar(lines, i + 1, cur_indent, stripped, quote="'")

    if stripped.startswith("- "):
        return _parse_sequence(lines, i, seq_indent=cur_indent)
    # If the line doesn't look like a mapping entry, treat it as a scalar.
    if ":" not in stripped:
        return _parse_scalar(_strip_inline_comment(stripped)), i + 1
    return _parse_mapping(lines, i, map_indent=cur_indent)


def _parse_mapping(lines: List[str], i: int, map_indent: int) -> Tuple[Dict[str, Any], int]:
    out: Dict[str, Any] = {}
    while True:
        i = _skip_ignorable(lines, i)
        if i >= len(lines):
            break
        line = lines[i]
        cur_indent = _indent(line)
        if cur_indent < map_indent:
            break
        if cur_indent != map_indent:
            # Indentation belongs to a parent value; stop and let caller handle.
            break
        stripped = line[cur_indent:]
        if stripped.startswith("- "):
            break

        key, rest = _split_key_value(stripped)
        i += 1

        # Key with inline value
        if rest is not None and rest != "":
            v, i = _parse_inline_or_block_value(lines, i, cur_indent, rest)
            out[key] = v
            continue

        # Key with empty value -> nested node (or null)
        j = _skip_ignorable(lines, i)
        if j >= len(lines):
            out[key] = None
            i = j
            continue
        next_line = lines[j]
        next_indent = _indent(next_line)
        next_stripped = next_line[next_indent:]

        # Indentless sequence under mapping key (allowed in YAML)
        if next_stripped.startswith("- ") and next_indent == cur_indent:
            v, i = _parse_sequence(lines, j, seq_indent=next_indent)
            out[key] = v
            continue

        if next_indent <= cur_indent:
            out[key] = None
            i = j
            continue

        v, i = _parse_node(lines, j, base_indent=next_indent)
        out[key] = v

    return out, i


def _parse_sequence(lines: List[str], i: int, seq_indent: int) -> Tuple[List[Any], int]:
    out: List[Any] = []
    while True:
        i = _skip_ignorable(lines, i)
        if i >= len(lines):
            break
        line = lines[i]
        cur_indent = _indent(line)
        if cur_indent < seq_indent:
            break
        stripped = line[cur_indent:]
        if not stripped.startswith("- "):
            break

        item_indent = cur_indent
        rest = stripped[2:]
        i += 1

        if rest.strip() == "":
            # Nested node starts on next line.
            j = _skip_ignorable(lines, i)
            if j >= len(lines) or _indent(lines[j]) <= item_indent:
                out.append(None)
                i = j
                continue
            v, i = _parse_node(lines, j, base_indent=_indent(lines[j]))
            out.append(v)
            continue

        # Mapping inlined in the list item: "- key: value"
        if ":" in rest:
            k, r = _split_key_value(rest)
            item: Dict[str, Any] = {}
            if r is None or r == "":
                # Nested value for first key
                j = _skip_ignorable(lines, i)
                if j >= len(lines) or _indent(lines[j]) <= item_indent:
                    item[k] = None
                    i = j
                else:
                    if lines[j][_indent(lines[j]):].startswith("- ") and _indent(lines[j]) == item_indent:
                        v, i = _parse_sequence(lines, j, seq_indent=item_indent)
                    else:
                        v, i = _parse_node(lines, j, base_indent=_indent(lines[j]))
                    item[k] = v
            else:
                v, i = _parse_inline_or_block_value(lines, i, item_indent, r)
                item[k] = v

            # Merge additional mapping lines aligned under the item
            content_indent = item_indent + 2
            extra, i2 = _parse_mapping(lines, i, map_indent=content_indent)
            item.update(extra)
            i = i2
            out.append(item)
            continue

        out.append(_parse_scalar(_strip_inline_comment(rest)))

    return out, i


def _split_key_value(s: str) -> Tuple[str, Optional[str]]:
    if ":" not in s:
        raise YamlLiteError(f"Expected mapping entry, got: {s!r}")
    key, rest = s.split(":", 1)
    key = key.strip()
    rest = rest.lstrip()
    return key, rest


def _parse_inline_or_block_value(lines: List[str], i: int, parent_indent: int, rest: str) -> Tuple[Any, int]:
    rest = _strip_inline_comment(rest)
    if rest == "|":
        return _parse_block_scalar(lines, i, parent_indent, style="|")
    if rest == ">":
        return _parse_block_scalar(lines, i, parent_indent, style=">")
    if rest.startswith('"'):
        return _parse_quoted_scalar(lines, i, parent_indent, rest, quote='"')
    if rest.startswith("'"):
        return _parse_quoted_scalar(lines, i, parent_indent, rest, quote="'")
    return _parse_scalar(rest), i


def _parse_block_scalar(lines: List[str], i: int, parent_indent: int, style: str) -> Tuple[str, int]:
    # Consume lines while indented more than parent_indent, allowing blank lines.
    start = i
    content_lines: List[str] = []
    min_indent: Optional[int] = None
    while i < len(lines):
        line = lines[i]
        if line.strip() == "":
            content_lines.append("")
            i += 1
            continue
        cur_indent = _indent(line)
        if cur_indent <= parent_indent:
            break
        if min_indent is None or cur_indent < min_indent:
            min_indent = cur_indent
        content_lines.append(line)
        i += 1

    if min_indent is None:
        return "", i

    normalized: List[str] = []
    for raw in content_lines:
        if raw == "":
            normalized.append("")
            continue
        if len(raw) >= min_indent:
            normalized.append(raw[min_indent:])
        else:
            normalized.append("")

    if style == "|":
        return "\n".join(normalized).rstrip("\n"), i

    # Folded: join non-blank lines with spaces, preserve blank lines as paragraph breaks.
    out: List[str] = []
    buf: List[str] = []
    for ln in normalized:
        if ln == "":
            if buf:
                out.append(" ".join(buf).rstrip())
                buf = []
            out.append("")
        else:
            buf.append(ln.rstrip())
    if buf:
        out.append(" ".join(buf).rstrip())
    return "\n".join(out).rstrip("\n"), i


def _parse_quoted_scalar(
    lines: List[str], i: int, parent_indent: int, rest: str, quote: str
) -> Tuple[str, int]:
    # Multi-line quoted scalars are supported in repo docs; parse until closing quote.
    # We treat newlines literally; for double-quoted scalars we then unescape.
    assert quote in {"'", '"'}

    # Remove leading quote on first line.
    buf = rest[1:]
    s, done = _consume_until_quote(buf, quote)
    if done:
        return _unescape_quoted(s, quote), i

    parts = [s]
    continuation_indent = parent_indent + 2
    while i < len(lines):
        line = lines[i]
        # Include blank lines as literal newlines.
        if line.strip() == "":
            parts.append("")
            i += 1
            continue
        # Strip common continuation indent (heuristic).
        if _indent(line) >= continuation_indent:
            seg = line[continuation_indent:]
        else:
            seg = line.lstrip()
        s2, done2 = _consume_until_quote(seg, quote)
        parts.append(s2)
        i += 1
        if done2:
            joined = "\n".join(parts)
            return _unescape_quoted(joined, quote), i

    raise YamlLiteError("Unterminated quoted scalar")


def _consume_until_quote(s: str, quote: str) -> Tuple[str, bool]:
    if quote == '"':
        out_chars: List[str] = []
        esc = False
        for idx, ch in enumerate(s):
            if esc:
                out_chars.append("\\" + ch)  # keep escapes for later decoding
                esc = False
                continue
            if ch == "\\":
                esc = True
                continue
            if ch == '"':
                # Ignore any trailing content after closing quote.
                return "".join(out_chars), True
            out_chars.append(ch)
        return "".join(out_chars), False

    # Single-quoted: '' is an escaped quote.
    out_chars: List[str] = []
    idx = 0
    while idx < len(s):
        ch = s[idx]
        if ch != "'":
            out_chars.append(ch)
            idx += 1
            continue
        # ch is '
        if idx + 1 < len(s) and s[idx + 1] == "'":
            out_chars.append("'")
            idx += 2
            continue
        return "".join(out_chars), True
    return "".join(out_chars), False


def _unescape_quoted(s: str, quote: str) -> str:
    if quote == "'":
        return s
    # Double-quoted: interpret backslash escapes (including \uXXXX).
    try:
        return bytes(s, "utf-8").decode("unicode_escape")
    except Exception:
        return s


def _parse_scalar(s: str) -> Any:
    s = s.strip()
    # Very small flow-style support (enough for fixtures):
    # - [] / [a, b, "c"]
    # - {} / {k: v, "k2": [1, 2]}
    if s.startswith("[") and s.endswith("]"):
        return _parse_flow_sequence(s)
    if s.startswith("{") and s.endswith("}"):
        return _parse_flow_mapping(s)
    if s == "" or s in {"null", "Null", "NULL", "~"}:
        return None
    if s in {"true", "True", "TRUE", "yes", "Yes", "1"}:
        return True
    if s in {"false", "False", "FALSE", "no", "No", "0"}:
        return False
    # Numbers
    try:
        if "." in s:
            return float(s)
        return int(s)
    except Exception:
        return s


def _parse_flow_sequence(s: str) -> List[Any]:
    inner = s[1:-1].strip()
    if inner == "":
        return []
    parts = _split_flow_items(inner)
    return [_parse_flow_value(p) for p in parts if p.strip() != ""]


def _parse_flow_mapping(s: str) -> Dict[str, Any]:
    inner = s[1:-1].strip()
    if inner == "":
        return {}
    parts = _split_flow_items(inner)
    out: Dict[str, Any] = {}
    for part in parts:
        if part.strip() == "":
            continue
        k, v = _split_flow_kv(part)
        key_val = _parse_flow_value(k)
        key = str(key_val)
        out[key] = _parse_flow_value(v)
    return out


def _parse_flow_value(s: str) -> Any:
    s = s.strip()
    if s == "":
        return None
    if s.startswith('"'):
        # Single-line quoted
        val, done = _consume_until_quote(s[1:], '"')
        if done:
            return _unescape_quoted(val, '"')
    if s.startswith("'"):
        val, done = _consume_until_quote(s[1:], "'")
        if done:
            return _unescape_quoted(val, "'")
    # Fallback to scalar parsing (including nested flow structures)
    return _parse_scalar(s)


def _split_flow_items(s: str) -> List[str]:
    items: List[str] = []
    buf: List[str] = []
    depth = 0
    in_single = False
    in_double = False
    esc = False
    for ch in s:
        if in_double:
            if esc:
                esc = False
            elif ch == "\\":
                esc = True
            elif ch == '"':
                in_double = False
            buf.append(ch)
            continue
        if in_single:
            if ch == "'":
                in_single = False
            buf.append(ch)
            continue

        if ch == '"':
            in_double = True
            buf.append(ch)
            continue
        if ch == "'":
            in_single = True
            buf.append(ch)
            continue

        if ch in "[{":
            depth += 1
            buf.append(ch)
            continue
        if ch in "]}":
            depth = max(0, depth - 1)
            buf.append(ch)
            continue

        if ch == "," and depth == 0:
            items.append("".join(buf).strip())
            buf = []
            continue
        buf.append(ch)

    if buf:
        items.append("".join(buf).strip())
    return items


def _split_flow_kv(s: str) -> Tuple[str, str]:
    depth = 0
    in_single = False
    in_double = False
    esc = False
    for idx, ch in enumerate(s):
        if in_double:
            if esc:
                esc = False
            elif ch == "\\":
                esc = True
            elif ch == '"':
                in_double = False
            continue
        if in_single:
            if ch == "'":
                in_single = False
            continue

        if ch == '"':
            in_double = True
            continue
        if ch == "'":
            in_single = True
            continue

        if ch in "[{":
            depth += 1
            continue
        if ch in "]}":
            depth = max(0, depth - 1)
            continue

        if ch == ":" and depth == 0:
            return s[:idx].strip(), s[idx + 1 :].strip()
    raise YamlLiteError(f"Expected flow mapping entry 'key: value', got: {s!r}")


def _dump_node(data: Any, indent: int, sort_keys: bool) -> str:
    pad = " " * indent
    if data is None:
        return "null"
    if isinstance(data, bool):
        return "true" if data else "false"
    if isinstance(data, (int, float)):
        return str(data)
    if isinstance(data, str):
        # Keep it simple; quote only when needed.
        if data == "" or any(ch in data for ch in [":", "#", "\n", "\t", "\r"]) or data.strip() != data:
            escaped = data.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n")
            return f"\"{escaped}\""
        return data
    if isinstance(data, list):
        if not data:
            return "[]"
        lines: List[str] = []
        for item in data:
            if isinstance(item, (dict, list)):
                lines.append(f"{pad}- {_dump_node(item, indent + 2, sort_keys)}")
            else:
                lines.append(f"{pad}- {_dump_node(item, 0, sort_keys)}")
        return "\n".join(lines)
    if isinstance(data, dict):
        if not data:
            return "{}"
        keys = list(data.keys())
        if sort_keys:
            keys = sorted(keys, key=lambda x: str(x))
        lines: List[str] = []
        for k in keys:
            v = data[k]
            ks = str(k)
            if isinstance(v, (dict, list)):
                lines.append(f"{pad}{ks}:")
                lines.append(_dump_node(v, indent + 2, sort_keys))
            else:
                lines.append(f"{pad}{ks}: {_dump_node(v, 0, sort_keys)}")
        return "\n".join(lines)
    # Fallback
    return _dump_node(str(data), indent, sort_keys)
