![Logo LOUTRE](https://raw.githubusercontent.com/LINCnil/loutre/main/assets/banner.png)

# LOgiciel Unique de TRaitement des Empreintes (LOUTRE)

Lors d'un contrôle, les agents de la CNIL collectent des pièces numériques qui
serviront à l'instruction du dossier. Afin de s'assurer de l'intégrité de ces
pièces, ils calculent l'empreinte numérique de chacune d'entre elles.
L'évolution des pratiques internes, en particulier l'arrivée de la plateforme
d'échanges sécurisés, a conduit à la réalisation de différents outils, chacun
utilisant des technologies différentes. Il a dont été décidé de regrouper ces
différents outils en un seul : le logiciel unique de traitement des empreintes
(LOUTRE).


## Compilation

Afin de compiler le logiciel, il est nécessaire de disposer d'une version
récente du [compilateur Rust](https://www.rust-lang.org/tools/install). Une
fois Rust et ses outils installés, lancez la commande suivante :

```
cargo build --release
```

L'exécutable se trouve alors dans le dossier `target/release/`.


## Droit d'auteur

![Logo EUPL](https://raw.githubusercontent.com/LINCnil/loutre/main/LICENSE/Logo_EUPL.png)

Vous êtes libres d'utiliser, de modifier et de redistribuer ces outil sous les
termes de la [licence publique de l'Union européenne (EUPL)
v1.2](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12).
Une copie de cette licence est disponible dans le dossier `LICENSE`.


## Configuration

La configuration du logiciel se fait depuis le panneau de configuration de
l'interface graphique (recommandé) ou bien directement dans un fichier
`config.toml`. Ce fichier est au format [TOML](https://toml.io/fr/) et ce situe
par défaut dans l’un des répertoires suivants :

| Système d'exploitation | Valeur                                            | Exemple pour l'utilisateur toto                      |
| ---------------------- | ------------------------------------------------- | ---------------------------------------------------- |
| Microsoft Windows      | `{FOLDERID_RoamingAppData}\CNIL\loutre`           | `C:\Users\toto\AppData\Roaming\CNIL\loutre`          |
| GNU/Linux              | `${XDG_CONFIG_HOME}/cnil/loutre`                  | `/home/toto/.config/cnil/loutre`                     |
| macOS                  | `${HOME}/Library/Application Support/cnil/loutre` | `/home/toto/Library/Application Support/cnil/loutre` |

Ce fichier est automatiquement généré lors du lancement du logiciel. Les
options de configuration possible sont :

- `theme` : chaîne de caractères
  * `light` (défaut)
  * `dark`
- `lang` : chaîne de caractères représentant une étiquettes d’identification de
  langues IETF telle que définiee par la recommandation standard BCP 47
  * `en-US`
  * `fr-BE`
  * `fr-FR` (défaut)
- `content_file_name` : chaîne de caractères définissant le nom du fichier dans
  lequel sont contenues les empreintes des pièces (par défaut, `contenu.txt`)
- `number_representation` : chaîne de caractères définissant la manière dont
  est représenté le nombre de pièces dans un dossier
  * `letters` : en toutes lettres
  * `western arabic numerals` (défaut) : en chiffres arabes
- `hash_function` : chaîne de caractères définissant la fonction de hachage à
  utiliser
  * `sha-256`
  * `sha-384`
  * `sha-512`
  * `sha3-256`
  * `sha3-384`
  * `sha3-512`
  * `blake2s`
  * `blake2b`
  * `blake3`
- `clipboard_threshold` : nombre entier positif représentant le nombre de
  fichiers à partir duquel le presse papier contient par défaut l'empreinte du
  fichier de contenu plutôt que les empreintes de chaque pièce
- `clipboard_persistence` : booléen définissant si le gestionnaire de
  presse-papier doit persister ou non en mémoire (ne définissez ce paramètre
  que si vous rencontrez des problèmes avec le copier/coller)
- `enable_duplicate_file_warning` : booléen définissant s'il convient ou non
  d'ajouter un avertissement en cas de fichiers identiques
- `enable_empty_file_warning` : booléen définissant s'il convient ou non
  d'ajouter un avertissement en cas de fichier vide


## Architecture technique

Les sources, situées dans le dossier `src/`, ont les rôles suivants :

- `analyse_hash.rs` : fonction tentant de deviner le type de hash en fonction
  de sa longueur
- `app.rs` : gestion de la base de l'application
- `checker.rs` : comparaison des empreintes préalablement calculées
- `clipboard.rs` : gestion du presse-papier
- `config.rs` : gestion de la configuration
- `content_file.rs` : gestion du fichier contenant les empreintes des pièces
- `file_list.rs` : gestion de la liste des fichiers
- `file.rs` : représentation interne d'un fichier
- `hasher.rs` : calcul des empreintes numériques
- `i18n.rs` : gestion de l'[internationalisation](https://fr.wikipedia.org/wiki/Internationalisation_(informatique))
- `main.rs` : point d'entrée du logiciel
- `nb_repr.rs` : gère la représentation des nombres
- `parsers.rs` : gestion de l'analyse syntaxique
- `parsers/cksum_gnu.rs` : analyse syntaxique des accusés de réception générés
  à l'aide de `sha256sum` et des commandes dérivées (variante GNU)
- `parsers/cnil_platform_email.rs` : analyse syntaxique des accusés de
  réception envoyés par la plateforme d'échanges de la CNIL via courrier
  électronique
- `path_cmp.rs` : comparaison et classement des noms de fichiers
- `receipt.rs` : gestion des accusés de réception
- `theme.rs` : gestion des thèmes
- `theme/button.rs` : gestion des boutons
- `theme/color.rs` : gestion des couleurs
- `theme/icon.rs` : gestion des icônes
- `theme/infobox.rs` : gestion des infobulles
- `views.rs`: interface des différentes vues
- `views/check_errors.rs`: gestion de l'interface de visualisation des erreurs
  de vérification des empreintes
- `views/config.rs`: gestion de l'interface de configuration
- `views/main.rs`: gestion de la vue principale

### Interface graphique en mode immédiat

Contrairement à la plupart des bibliothèques d'interface graphique qui
utilisent un mode retenu, la bibliothèque utilisée ici utilise le mode
immédiat. Le paradigme n'étant pas le même, les développeurs n'ayant pas encore
d'expérience avec ce mode peuvent avoir besoin de se renseigner sur le sujet.

- [immediate mode GUI](https://en.wikipedia.org/wiki/Immediate_mode_GUI)
- [why immediate mode](https://github.com/emilk/egui#why-immediate-mode)
- [understanding immediate mode](https://docs.rs/egui/latest/egui/#understanding-immediate-mode)
- [documentation d'egui](https://docs.rs/egui/latest/egui/)
- [documentation d'eframe](https://docs.rs/eframe/latest/eframe/)

### Chargement des fichiers

Lorsqu'un dossier est sélectionné, le programme liste les fichiers et récupère
leur taille. Afin de ne pas bloquer l'interface graphique, cette opération est
réalisée dans nouveau fil d'exécution et une icône de chargement est affichée.
De plus, afin de savoir s'il faut inclure ou non les fichiers cachés et les
fichiers système, ce fil doit communiquer avec l'interface graphique.

Le lancement du nouveau fil d'exécution ainsi que la communication avec
l'interface graphique sont gérés par `file_list::FileListBuilder`. La
communication entre les fils d'exécution s'effectue grâce à [un
canal](https://jimskapt.github.io/rust-book-fr/ch16-02-message-passing.html)
(voir également
[std::sync::mpsc](https://doc.rust-lang.org/std/sync/mpsc/index.html)).

### Calcul des empreintes

Le calcul des empreintes étant une opération pouvant de révéler longue, elle
est effectuée sur plusieurs fils d'exécution. Répartir ainsi les fichiers en
plusieurs sous-ensembles de manière à ce que la somme des tailles de fichiers
soit la plus similaire possible entre les ensemble est [un problème
NP-complet](https://en.wikipedia.org/wiki/Partition_problem) (cf. [optimal job
scheduling](https://en.wikipedia.org/wiki/Optimal_job_scheduling)). Afin de
rester simple tout en étant relativement efficace, l'implémentation actuelle
repose sur [un
LPT](https://en.wikipedia.org/wiki/Longest-processing-time-first_scheduling) :
la liste des fichiers est triée du plus lourd au plus léger puis chaque fil
d'exécution, à l'aide d'une
[mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html), s'octroie le
fichier le plus lord encore disponible.

Il est à noter que, afin que la barre de progression soit en mesure d'afficher
une mesure fiable, chaque fil d'exécution remonte à intervalle régulier le
nombre d'octets qu'il a traité. Tout comme pour le chargement des fichiers, la
communication entre les fils d'exécution s'effectue à l'aide d'un canal.

### L'analyse syntaxique

Afin d'extraire les informations nécessaires des courriers électroniques
faisant office d'accusés de réception, il est fait usage d'une bibliothèque
d'[analyse syntaxique par
combinaison](https://en.wikipedia.org/wiki/Parser_combinator) :
[nom](https://github.com/Geal/nom). L'idée est de combiner entre eux plusieurs
analyseurs syntaxiques d'éléments de base afin de créer des analyseurs
syntaxiques plus poussés pouvant également à leur tour être combinés pour en
former de nouveaux.

### Équivalence Unicode et normalisation

Afin de pouvoir comparer des noms de fichiers provenant de différentes sources,
il est important de prendre en compte les [équivalences
Unicode](https://fr.wikipedia.org/wiki/%C3%89quivalence_Unicode) et leur
impact. Afin d'éviter de considérer comme différents deux noms de fichiers
identiques mais dont certains caractères sont représentés sous des formes
différentes, il est nécessaire d'effectuer une
[normalisation](https://fr.wikipedia.org/wiki/Normalisation_Unicode). Dans le
cas présent, l'algorithme de normalisation utilisé est NFKC.

### Choix d'une fonction de hachage

À l'heure actuelle (début 2023), toutes les fonctions de hachage supportées
utilisent un algorithme public réputé fort et exempt de vulnérabilité connue.
Elles peuvent donc toutes être utilisées.

Ces fonctions diffèrent principalement par leur taille d'empreinte, leur
vitesse d'exécution et leur popularité.

La fonction la plus répandue est SHA-256. Cette fonction extrêmement populaire,
dispose de la taille d'empreinte la plus faible parmi les fonctions supportées
et les processeurs modernes permettent généralement d'accélérer ses
performances directement au niveau du matériel afin d'être extrêmement rapide.
C'est donc un excellent choix qui est grandement reconnu et fait consensus.

La fonction intrinsèquement la plus rapide est Blake-3. Très récente (2020) et
de conception moderne, elle est encore peu répandue mais se démarque par sa
rapidité d'exécution exceptionnelle qui, sans accélération matérielle, rivalise
avec une SHA-256 matériellement accélérée sans toutefois être nécessairement
plus rapide que cette dernière. Sa taille d'empreinte est identique à celle de
SHA-256. Il s'agit donc également d'un excellent choix particulièrement adapté
au calcul d'empreintes de gros volumes de données sur les machines ne disposant
pas d'accélération matérielle pour SHA-256.

Les autres fonctions ne sont ni plus rapides ni plus populaires que SHA-256 et
Blake-3. Elles n'apportent pas de gain significatif en terme de sécurité malgré
des tailles d'empreintes supérieures ou égales. Leur présence est
principalement motivé par la possibilité de les utiliser en urgence dans
l'hypothèse où des vulnérabilités seraient découvertes dans SHA-256 et Blake-3.


## Favicon

Une favicon peut être générée à partir de plusieurs images PNJ. Pour cela, il
faut que les images PNG utilisent une colormap de 8 bits. Toutes ces opérations
sont réalisables à l'aide de l'outil `convert` fourni avec ImageMagick.

Convertir la colormap 16 bits d'un PNG en une colormap 8 bits :

```
convert name.png PNG8:name.png
```

Créer le fichier `.ico` à partir des différentes images :

```
convert *.png -colors 256 favicon.ico
```
