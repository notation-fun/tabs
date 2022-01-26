# Fun Notation Tabs Template

This is a template for writing fun notation tabs.

Check the [main repository](https://github.com/notation-fun/notation) for more information.

## How to add new tabs

Just create new files in src folders, use [src/test.rs](src/test.rs) as an example.

## How to compile the tabs

Add the new file into [src/main.rs](src/main.rs), then run `cargo run` in the project root to generate tabs.

## Load tab source to viewer

Notation viewer's desktop version support loading the tab source file (e.g. test.rs) directly, there is a hidden requirement for this to work, the last statement in the file has to be the function to return Tab, check `src/test.rs` for an working example.
