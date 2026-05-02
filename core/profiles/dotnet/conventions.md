# Profile conventions: .NET (dotnet CLI)

## Style
- Use `dotnet format` for formatting and basic analyzers.
- Treat warnings as errors in CI where feasible.

## Testing
- `dotnet test` must pass.

## Security
- `dotnet list package --vulnerable` for basic vulnerability surfacing.

Core prompts should reference command keys in `commands.yaml`, not hardcode dotnet commands.
