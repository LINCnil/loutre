# LOgiciel Unique de TRaitement des Empreintes (LOUTRE)

Lors d'un contrôle, les agents de la CNIL collectent des pièces numériques qui serviront à l'instruction du dossier. Afin de s'assurer de l'intégrité de ces pièces, ils calculent l'empreinte numérique de chacune d'entre elles. L'évolution des pratiques internes, en particulier l'arrivée de la plateforme d'échanges sécurisés, a conduit à la réalisation de différents outils, chacun utilisant des technologies différentes. Il a dont été décidé de regrouper ces différents outils en un seul : le logiciel unique de traitement des empreintes (LOUTRE).


## Compilation

Afin de compiler le logiciel, il est nécessaire de disposer d'une version récente du [compilateur Rust](https://www.rust-lang.org/tools/install). Une fois Rust et ses outils installés, lancez la commande suivante :

```
cargo build --release
```

L'exécutable se trouve alors dans le dossier `target/release/`.


## Configuration

La configuration du logiciel se fait dans un fichier `config.toml`. Ce fichier est au format [TOML](https://toml.io/fr/) et ce situe par défaut dans l’un des répertoires suivants :

- Microsoft Windows : `C:\Users\{UTILISATEUR}\AppData\Roaming\CNIL\loutre`
- GNU/Linux : `$XDG_CONFIG_HOME/cnil/loutre`
- GNU/Linux : `$HOME/.config/cnil/loutre`
- macOS : `$HOME/Library/Application Support/CNIL/loutre`

Ce fichier est automatiquement généré lors du lancement du logiciel. Les options de configuration possible sont :

- `theme` : chaîne de caractères
  * `light` (défaut)
  * `dark`
- `lang` : chaîne de caractères représentant une étiquettes d’identification de langues IETF telle que définiee par la recommandation standard BCP 47
  * `en-US`
  * `fr-BE`
  * `fr-FR` (défaut)
- `content_file_name` : chaîne de caractères définissant le nom du fichier dans lequel sont contenues les empreintes des pièces (par défaut, `contenu.txt`)
- `number_representation` : chaîne de caractères définissant la manière dont est représenté le nombre de pièces dans un dossier
  * `letters` : en toutes lettres
  * `western arabic numerals` (défaut) : en chiffres arabes
- `hash_function` : chaîne de caractères définissant la fonction de hachage à utiliser
  * `sha-256`
  * `sha-384`
  * `sha-512`
  * `sha3-256`
  * `sha3-384`
  * `sha3-512`


## Architecture technique

Les sources, situées dans le dossier `src/`, ont les rôles suivants :

- `app.rs` : gestion de l'interface graphique
- `checker.rs` : comparaison des empreintes préalablement calculées
- `clipboard.rs` : gestion du presse-papier
- `config.rs` : gestion de la configuration
- `content_file.rs` : gestion du fichier contenant les empreintes des pièces
- `email.rs` : analyse syntaxique des accusés de réception envoyés par la plateforme d'échanges via courrier électronique
- `file_list.rs` : gestion de la liste des fichiers
- `file.rs` : représentation interne d'un fichier
- `hasher.rs` : calcul des empreintes numériques
- `i18n.rs` : gestion de l'[internationalisation](https://fr.wikipedia.org/wiki/Internationalisation_(informatique))
- `main.rs` : point d'entrée du logiciel
- `nb_repr.rs` : gère la représentation des nombres
- `path_cmp.rs` : comparaison et classement des noms de fichiers
- `theme.rs` : gestion des thèmes

### Interface graphique en mode immédiat

Contrairement à la plupart des bibliothèques d'interface graphique qui utilisent un mode retenu, la bibliothèque utilisée ici utilise le mode immédiat. Le paradigme n'étant pas le même, les développeurs n'ayant pas encore d'expérience avec ce mode peuvent avoir besoin de se renseigner sur le sujet.

- [immediate mode GUI](https://en.wikipedia.org/wiki/Immediate_mode_GUI)
- [why immediate mode](https://github.com/emilk/egui#why-immediate-mode)
- [understanding immediate mode](https://docs.rs/egui/latest/egui/#understanding-immediate-mode)
- [documentation d'egui](https://docs.rs/egui/latest/egui/)
- [documentation d'eframe](https://docs.rs/eframe/latest/eframe/)

### Chargement des fichiers

Lorsqu'un dossier est sélectionné, le programme liste les fichiers et récupère leur taille. Afin de ne pas bloquer l'interface graphique, cette opération est réalisée dans nouveau fil d'exécution et une icône de chargement est affichée. De plus, afin de savoir s'il faut inclure ou non les fichiers cachés et les fichiers système, ce fil doit communiquer avec l'interface graphique.

Le lancement du nouveau fil d'exécution ainsi que la communication avec l'interface graphique sont gérés par `file_list::FileListBuilder`. La communication entre les fils d'exécution s'effectue grâce à [un canal](https://jimskapt.github.io/rust-book-fr/ch16-02-message-passing.html) (voir également [std::sync::mpsc](https://doc.rust-lang.org/std/sync/mpsc/index.html)).

### Calcul des empreintes

Le calcul des empreintes étant une opération pouvant de révéler longue, elle est effectuée sur plusieurs fils d'exécution. Répartir ainsi les fichiers en plusieurs sous-ensembles de manière à ce que la somme des tailles de fichiers soit la plus similaire possible entre les ensemble est [un problème NP-complet](https://en.wikipedia.org/wiki/Partition_problem) (cf. [optimal job scheduling](https://en.wikipedia.org/wiki/Optimal_job_scheduling)). Afin de rester simple tout en étant relativement efficace, l'implémentation actuelle repose sur [un LPT](https://en.wikipedia.org/wiki/Longest-processing-time-first_scheduling) : la liste des fichiers est triée du plus lourd au plus léger puis chaque fil d'exécution, à l'aide d'une [mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html), s'octroie le fichier le plus lord encore disponible.

Il est à noter que, afin que la barre de progression soit en mesure d'afficher une mesure fiable, chaque fil d'exécution remonte à intervalle régulier le nombre d'octets qu'il a traité. Tout comme pour le chargement des fichiers, la communication entre les fils d'exécution s'effectue à l'aide d'un canal.

### L'analyse syntaxique

Afin d'extraire les informations nécessaires des courriers électroniques faisant office d'accusés de réception, il est fait usage d'une bibliothèque d'[analyse syntaxique par combinaison](https://en.wikipedia.org/wiki/Parser_combinator) : [nom](https://github.com/Geal/nom). L'idée est de combiner entre eux plusieurs analyseurs syntaxiques d'éléments de base afin de créer des analyseurs syntaxiques plus poussés pouvant également à leur tour être combinés pour en former de nouveaux.

### Équivalence Unicode et normalisation

Afin de pouvoir comparer des noms de fichiers provenant de différentes sources, il est important de prendre en compte les [équivalences Unicode](https://fr.wikipedia.org/wiki/%C3%89quivalence_Unicode) et leur impact. Afin d'éviter de considérer comme différents deux noms de fichiers identiques mais dont certains caractères sont représentés sous des formes différentes, il est nécessaire d'effectuer une [normalisation](https://fr.wikipedia.org/wiki/Normalisation_Unicode). Dans le cas présent, l'algorithme de normalisation utilisé est NFKC.
