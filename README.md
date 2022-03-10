# Videre

YAT for stdin piping visualization.

## Commands

### Tree

Given a list of paths

```
.
./README.md
./CHANGELOG.md
./LICENSE
./Cargo.toml
./Cargo.lock
./examples
./src
./examples/editor.rs
./src/history.rs
./src/validate.rs
./src/prompts
./src/theme.rs
./src/edit.rs
./src/paging.rs
./src/prompts/fuzzy_select.rs
./src/prompts/select.rs
./src/prompts/input.rs
./src/prompts/mod.rs
./src/prompts/multi_select.rs
./src/lib.rs
./src/prompts/confirm.rs
./src/prompts/sort.rs
./src/prompts/password.rs
./src/completion.rs
./examples/buffered.rs
./examples/fuzzyselect.rs
./examples/input.rs
./examples/password.rs
./examples/history.rs
./examples/wizard.rs
./examples/multi_select.rs
./examples/completion.rs
./examples/select.rs
./examples/confirm.rs
./examples/paging.rs
./examples/sort.rs
```

Outputs it as a tree view

```
.
	CHANGELOG.md
	Cargo.lock
	Cargo.toml
	LICENSE
	README.md
	examples
	src
./examples
	buffered.rs
	completion.rs
	confirm.rs
	editor.rs
	fuzzyselect.rs
	history.rs
	input.rs
	multi_select.rs
	paging.rs
	password.rs
	select.rs
	sort.rs
	wizard.rs
./src
	completion.rs
	edit.rs
	history.rs
	lib.rs
	paging.rs
	prompts
	theme.rs
	validate.rs
./src/prompts
	confirm.rs
	fuzzy_select.rs
	input.rs
	mod.rs
	multi_select.rs
	password.rs
	select.rs
	sort.rs
```