```
WARNING: this is not yet ready, still in active development
```

# Shrink My URL

Simple app to shrink your URLs! All done in 
- axum
- askama
- flexi_logger
- tower_http
- htmx
- tailwindcss

### Set up local

To set up local, ensure you have gone through the process of installing the rust programming language.
Next, you will need to run the following to generate 
```bash
npx tailwindcss -i ./templates/styles/main.css -o ./templates/styles/styles.css
```
- If you want live generation, add `--watch` to the end

Then, download `htmx.min.js` and put it in `templates/scripts/`

- Note, these steps will be simplified into an "install" script in the future.

Finally, run 
```bash
cargo run
```
and you should now have a local server running on `127.0.0.1:3000` or `localhost:3000`

### Color Palette

tailwind colors
- `blue-950` : background
- `cyan-700` : ???
- `teal-600` : actions
- `lime-200` : bright notifications / highlights
- `pink-600` : warning

### Planned features

- [ ] main page can take a URL and give back a "minified" URL
- [ ] server will forward users from minified to the linked page
- [ ] simple logging setup
- [ ] deploy!
- [ ] option for federated authentication (google, github, ...?)
  - need to find best integration for federation
- [ ] show users their history of minification

