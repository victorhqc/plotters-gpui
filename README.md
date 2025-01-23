# plotters-gpui

[GPUI](https://gpui.rs) backend for [plotters](https://github.com/plotters-rs/plotters).

Please goto [https://github.com/JakkuSakura/plotters-gpui](https://github.com/JakkuSakura/plotters-gpui)
for more information.

Due to gpui not being published on crates.io, you need to add the following to your `Cargo.toml`:

```toml
[dependencies]
plotters-gpui = { git = "https://github.com/JakkuSakura/plotters-gpui" }
```

If you failed to build on linux due to font-kit, you might need to add the following to your `Cargo.toml`:

```toml
[dependencies]
font-kit = { git = "https://github.com/zed-industries/font-kit", features = ["source-fontconfig-dlopen"] }
[patch.crates-io]
# because plotters' font-kit might fail
font-kit = { git = "https://github.com/zed-industries/font-kit" }
```

You might be interested in [https://github.com/JakkuSakura/gpui-plot](https://github.com/JakkuSakura/gpui-plot), as it
provides interactivity and more stuff on top of plotters-gpui

## Show cases

<img width="300" alt="image" src="https://github.com/user-attachments/assets/276b75c2-d5fe-4b0e-93b1-1215317d4b73" /> <img width="300" alt="image" src="https://github.com/user-attachments/assets/8b1f7c80-ef09-4ffd-aff8-123315ecf1b3" />
<img width="300" alt="image" src="https://github.com/user-attachments/assets/7e9ec94e-a8f0-4e0d-97eb-9399f0145e39" /> <img width="300" alt="image" src="https://github.com/user-attachments/assets/03ea4351-d079-4372-af84-bd2429ccc098" />
<img width="300" alt="image" src="https://github.com/user-attachments/assets/56c29590-c120-4a5e-8b65-b272afe732dc" />

## Performance

plotters-gpui try to maintain very good performance compared to that in javascript/python, even better than CPU based
plotting libraries

### Metal Performance HUD

Use the following environment variable to enable Metal Performance HUD:

```text
MTL_HUD_ENABLED=1
Enables the Metal Performance HUD.
```

```shell
MTL_HUD_ENABLED=1 cargo run ...
```

