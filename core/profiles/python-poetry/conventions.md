# Profile conventions: Python (Poetry)

Same conventions as Python (uv), but commands are run via Poetry.

## Layout assumptions
- `src/` for code, `tests/` for tests (unless repo differs)

## Style / Typing / Tests
- Format + lint: ruff
- Typing: mypy
- Tests: pytest

Core prompts should reference command keys (e.g., `commands.tests`) and never embed raw Poetry commands.
