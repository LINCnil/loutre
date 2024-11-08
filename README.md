![Logo LOUTRE](https://raw.githubusercontent.com/LINCnil/loutre/main/assets/banner.png)

# LOgiciel Unique de TRaitement des Empreintes (LOUTRE)


## Build

Dependencies are liste in the [Dioxus documentation][dioxus_doc].

You can compile a production version using the following command:

```
cargo build --release
```

For a debug version, remove the `--release` flag.

[dioxus_doc]: https://dioxuslabs.com/learn/0.5/getting_started


## Licence

![Logo EUPL](https://raw.githubusercontent.com/LINCnil/loutre/main/LICENSE/Logo_EUPL.png)

You are free tu use, copy, modify, and/or distribute this software under the
terms of the [European Union Public License (EUPL) v1.2][eupl_12].

A copy of this license is available in the `LICENSE` directory.

[eupl_12]: https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12


## Configuration

The configuration should always be modified through the software's graphical
interface. For use cases where this option is not available (for example in
automatic deployments), it is possible to directly edit the configuration file.
This file is named `config.toml` and is written in the [TOML format][toml]. Its
location varies depending on the operating system.

| Operating System  | Value                                             | Example                                                  |
| ----------------- | ------------------------------------------------- | -------------------------------------------------------- |
| Microsoft Windows | `{FOLDERID_RoamingAppData}\CNIL\loutre`           | `C:\Users\john_doe\AppData\Roaming\CNIL\loutre`          |
| GNU/Linux         | `${XDG_CONFIG_HOME}/cnil/loutre`                  | `/home/john_doe/.config/cnil/loutre`                     |
| macOS             | `${HOME}/Library/Application Support/cnil/loutre` | `/home/john_doe/Library/Application Support/cnil/loutre` |

[toml]: https://toml.io/

### Choosing a hashing function

As for 2024, all supported hashing functions uses a robust public algorithm
without known vulnerability. All of them can therefore be safely used.

Those functions mostly varies in terms of fingerprint size, execution speed and
popularity.

The most popular function is SHA-256. Aside from its large popularity, it has a
small fingerprint size and almost every recent x86 CPU supports the [Intel SHA
extensions][x86_sha], which means it will be hardware-accelerated and therefore
very quick to process. Those characteristics makes it an excellent choice.

The intrinsically fastest function is Blake-3. It is very recent (2020) and
therefore rarely used, but its modern conception makes it so fast that, without
hardware acceleration, its performances may be similar to a
hardware-accelerated SHA-256. Its fingerprint size is equal to the SHA-256 one.
This is a very good choice when used with CPU that does not support the Intel
SHA extensions.

All the other functions are neither fastest nor more popular than SHA-256 and
Blake-3. In most cases, the additional security brought by the longest
fingerprint of some functions is not considered useful. Those additional
functions are supported for two main reasons:

1. A better integration in processes that already uses those functions.
2. The possibility to quickly react to an eventual vulnerability discovered in
   the most popular functions.

[x86_sha]: https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sha-extensions.html


## Favicon

The favicon is generated from the SVG template using the following method:

1. a set of PNG icons are generated using [Inkscape][inkscape]
2. those images are converted into a single favicon using [ImageMagick][magick]
3. temporary PNG images are deleted

```
for size in "256" "96" "80" "72" "64" "48" "40" "36" "32" "30" "24" "20" "16"; do inkscape -w "$size" -h "$size" "assets/favicon-template.svg" -o "assets/AppList.targetsize-${size}.png"; done
magick "assets/AppList.targetsize-*.png" -colors 256 "assets/favicon.ico"
rm assets/AppList.targetsize-*.png
```

Read more about application icons on the [Microsoft Windows
documentation][ms_icons] and [Wikipedia][wiki_icons].

[inkscape]: https://inkscape.org/
[magick]: https://imagemagick.org/
[ms_icons]: https://learn.microsoft.com/en-us/windows/apps/design/style/iconography/app-icon-construction
[wiki_icons]: https://en.wikipedia.org/wiki/ICO_(file_format)
