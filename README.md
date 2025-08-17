# Taxer CSV

This is a simple crate to write financial operation log in [Taxer](https://taxer.ua/uk/kb/import-vipiski-v-csv-fajl-samostijne-stvorennya-csv-fajlu) format.

The crate is intended to be used in a [monotax](https://github.com/dimasmith/monotax) project suite.

## Releasing

This crate isn't published to `crates.io` and won't be, as it's too niche.

Release steps:

1. Checkout the `main` branch.
2. Remove the `-alpha.x` version suffix from the `cargo.toml`.
3. Commit the changes with a message `chore(release): release version x.y.z`,
4. Tag the commit with the `x.y.z` tag.
5. Bump the version to `x.y.z-alpha.0` in `cargo.toml`.
6. Commit changes with message `chore(release): start version x.y.z development`.
7. Push changes to the `main` branch.

You can automate the process if you have `cocogitto` and `cargo-bump` installed.

```bash
git checkout main
git pull
cog bump --patch
git push

```
