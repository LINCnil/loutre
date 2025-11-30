![Logo LOUTRE](https://raw.githubusercontent.com/LINCnil/loutre/main/assets/banner.png)

# LOgiciel Unique de TRaitement des Empreintes (LOUTRE)


## Build

Dependencies are listed in the [Dioxus documentation][dioxus_doc].

You can compile a production version using the following command:

```
cargo build --release
```

[dioxus_doc]: https://dioxuslabs.com/learn/0.7/getting_started

### Development

For a debug version, remove the `--release` flag.

When developing, be sure to:
- format the code using `cargo fmt`
- fix all warnings detected by `cargo clippy`
- detect outdated dependencies using `cargo outdated` (requires
[cargo-outdated][cargo-outdated])
- check the dependencies using `cargo deny check` (requires
[cargo-deny][cargo-deny])

[cargo-outdated]: https://github.com/kbknapp/cargo-outdated
[cargo-deny]: https://github.com/EmbarkStudios/cargo-deny


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

As for 2025, all supported hashing functions uses a robust public algorithm
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

### Choosing a content file format

After calculating the fingerprint of each file in the specified directory,
Loutre stores those hashes in a file. You may chose the format of this file.

Important note: the Cksum format has no official name. We named it this way
because of the cksum tool. There is two very different variants of this format:
one is used by default on several BSD systems, the other one is used by default
in systems using GNU coreutils (mostly GNU/Linux distributions). All known
recent versions of those operating systems include tools capable of reading and
writing both variants.

#### Cksum (BSD variant)

This is the default choice. This variant is recommended since it explicitly
stores the hashing function that has been used to generate the fingerprints.

This format is compatible with the output of the following tools:

- On GNU/Linux and other systems using GNU coreutils: [cksum][gnu_cksum] (using
  one of the following algorithms: sha256, sha384, sha512, blake2b)
- On GNU/Linux and other systems using GNU coreutils:
  [sha256sum][gnu_sha256sum], [sha384sum][gnu_sha384sum],
  [sha512sum][gnu_sha512sum], [b2sum][gnu_b2sum] (using the `--tag` option)
- On FreeBSD: [sha256][freebsd_sha256], [sha384][freebsd_sha384],
  [sha512][freebsd_sha512]
- On OpenBSD: [cksum][openbsd_cksum] (using the following algorithms: sha256,
  sha384, sha512)

#### Cksum (GNU variant)

This variant does not stores the hashing function that has been used to
generate the fingerprints. The content file's name should therefore indicate it
in its name.

This format is compatible with the output of the following tools:

- On GNU/Linux and other systems using GNU coreutils: [cksum][gnu_cksum] (using
  the `--untagged` option and one of the following algorithms: sha256, sha384,
  sha512, blake2b)
- On GNU/Linux and other systems using GNU coreutils:
  [sha256sum][gnu_sha256sum], [sha384sum][gnu_sha384sum],
  [sha512sum][gnu_sha512sum], [b2sum][gnu_b2sum]
- On FreeBSD: [sha256sum][freebsd_sha256sum], [sha384sum][freebsd_sha384sum],
  [sha512sum][freebsd_sha512sum]
- On OpenBSD: [cksum][openbsd_cksum] (using the `-r` option and one of the
  following algorithms: sha256, sha384, sha512)

#### Cnil

This format has been created by the French data protection authority
(Commission nationale de l'informatique et des libertés, CNIL). It is not
standard and no other use of this format is known. Its usage is discouraged
unless you work at the CNIL.

[gnu_sha256sum]: https://man.archlinux.org/man/sha256sum.1
[gnu_sha384sum]: https://man.archlinux.org/man/sha384sum.1
[gnu_sha512sum]: https://man.archlinux.org/man/sha512sum.1
[gnu_b2sum]: https://man.archlinux.org/man/b2sum.1
[gnu_cksum]: https://man.archlinux.org/man/cksum.1
[freebsd_sha256sum]: https://man.freebsd.org/cgi/man.cgi?query=sha256sum
[freebsd_sha384sum]: https://man.freebsd.org/cgi/man.cgi?query=sha384sum
[freebsd_sha512sum]: https://man.freebsd.org/cgi/man.cgi?query=sha512sum
[freebsd_sha256]: https://man.freebsd.org/cgi/man.cgi?query=sha256
[freebsd_sha384]: https://man.freebsd.org/cgi/man.cgi?query=sha384
[freebsd_sha512]: https://man.freebsd.org/cgi/man.cgi?query=sha512
[openbsd_cksum]: https://man.openbsd.org/cksum.1

### Custom clipboard content

Once the file's hashes are calculated, the clipboard is automatically filled
with a textual representation of those hashes. The default value consists of a
human-readable list of those files if the number of files does not exceed the
configured threshold. If this threshold is reached, the valus is set to the
hash of the content file. These values are both available both as plain-text
and as HTML.

The configuration allows you to set a different content. There is no automatic
translation from the HTML form to the plain-text one, you should configure both
and ensure those values matches. The syntax is available in the [MiniJinja
documentation][minijinja_doc].

[minijinja_doc]: https://docs.rs/minijinja/latest/minijinja/syntax/


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
