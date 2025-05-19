<details>

<summary>Yew with Tailwind</summary>

## Yew with Tailwind CLI

Pertama, install Tailwind CLI

```bash
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
chmod +x tailwindcss-linux-x64
mv tailwindcss-linux-x64 tailwindcss
```

Kedua, initializing tailwind `./tailwindcss init` lalu modifikasi config file `tailwind.config.js`
```javascript
// tailwind.config.js
module.exports = {
  content: [
    './src/**/*.rs',
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
  ]
}
```

Ketiga, tambahkan konfigurasi `Trunk.toml`
```toml
# Trunk.toml
[build]
target = "index.html"
dist = "dist"

[[hooks]]
stage = "build"
command = "sh"
command_arguments = ["-c", "./tailwindcss -i src/tailwind.css -o $TRUNK_STAGING_DIR/tailwind.css"]
```

lalu tambahkan file `tailwind.css` di folder `/src`
```css
/* tailwind.css */
@tailwind base;
@tailwind components;
@tailwind utilities;
```
Terakhir, tambahkan link css ke `index.html` yang akan di render trunk.
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    
    <!-- Disini -->
    <link rel="stylesheet" href="/tailwind.css"/>
    <base data-trunk-public-url/>

    <title>Yew App</title>
  </head>
</html>
```

Saat menjalankan `trunk serve`, akan dilakukan compile css tailwind saat proses build dan akan menampilkan output seperti ini
```bash
➜  yew-app git:(main) trunk serve
Jan 04 12:42:30.060  INFO 📦 starting build
Jan 04 12:42:30.063  INFO spawning asset pipelines
Jan 04 12:42:30.470  INFO spawned hook sh command_arguments=["-c", "./tailwindcss -i src/tailwind.css -o $TRUNK_STAGING_DIR/tailwind.css"]
Jan 04 12:42:30.470  INFO spawning hook stage=Build command=sh
Jan 04 12:42:30.470  INFO building yew-app
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
Jan 04 12:42:30.816  INFO fetching cargo artifacts
Jan 04 12:42:31.106  INFO processing WASM
Jan 04 12:42:31.148  INFO using system installed binary app="wasm-bindgen" version="0.2.78"
Jan 04 12:42:31.150  INFO calling wasm-bindgen
Jan 04 12:42:31.428  INFO copying generated wasm-bindgen artifacts

Done in 329ms.
Jan 04 12:42:32.114  INFO finished hook sh
Jan 04 12:42:32.115  INFO applying new distribution
Jan 04 12:42:32.117  INFO ✅ success
Jan 04 12:42:32.119  INFO 📡 serving static assets at -> /
Jan 04 12:42:32.119  INFO 📡 server listening at 0.0.0.0:8080
```

Struktur app akan terlihat seperti ini
```bash
➜  yew-app git:(main) tree .
.
├── Cargo.lock
├── Cargo.toml
├── Trunk.toml
├── index.html
├── src
│   ├── main.rs
│   └── tailwind.css
├── tailwind.config.js
├── tailwindcss
```
</details>