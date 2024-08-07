## Theme

theme = Thème
theme_dark = sombre
theme_light = clair

## Content file

content_file_name = contenu.txt
content_file_header =
    .name = Nom du document
    .size = Taille (octets)


## Configuration options

config = Configuration
config_not_available = Configuration indisponible lorsqu'un dossier est ouvert
label_nb_files_start = Numéro de la première pièce
label_content_file = Nom du fichier d’empreintes
label_hash_function = Fonction de hachage
language = Langue
number_representation = Représentation des nombres
letters = lettres
western_arabic_numerals = chiffres arabes
duplicate_file_warning = Montrer les fichiers dupliqués
empty_file_warning = Montrer les fichiers vides
clipboard_threshold = Seuil du presse-papier
clipboard_threshold_help = Nombre de fichiers à partir duquel le presse papier contient par défaut l'empreinte du fichier de contenu plutôt que les empreintes de chaque pièce
clipboard_persistence = Persistance du presse-papier
clipboard_persistence_help = Indique si l’objet interne gérant le presse-papier soit être conservé en mémoire ou non. Ce paramètre dépends du système d’exploitation. En l'absence de problème avec le presse-papier, il est recommandé de conserver la valeur par défaut.
default = par défaut
activated = activée
deactivated = désactivée
apply = Appliquer
cancel = Annuler


## Selection buttons

btn_select_dir = Ouvrir un dossier
btn_select_receipt = Ouvrir un AR
btn_trash_tip = Réinitialiser
label_receipt = Accusé de réception


## Actions

btn_calc_fingerprints = Calculer les empreintes
btn_check_fingerprints = Vérifier les empreintes


## Progress

progress = Réalisé : { $done } / { $total }


## Clipboard buttons

btn_clipboard_tip = Copier l’empreinte de l’ensemble des pièces
btn_clipboard_ctn_file_tip = Copier l’empreinte du fichier contenant les empreintes


## File inclusion choices

btn_file_choice =
    .yes = Oui
    .yes_all = Oui pour tous
    .no = Non
    .no_all = Non pour tous
msg_file_choice_dir_hidden = { $file_name } est un dossier caché.
msg_file_choice_dir_system = { $file_name } est un dossier système.
msg_file_choice_file_hidden = { $file_name } est un fichier caché.
msg_file_choice_file_system = { $file_name } est un fichier système.
msg_file_choice_include = { $file_desc } Souhaitez-vous l’inclure ?


## Clipboard

msg_exhibit = PIÈCE N{ $sup_open }o{ $sup_close } { $nb } :
msg_ctn_file =
    { $nb ->
        [one] copie sur support informatique d’un document remis au responsable des lieux, intitulé « { $file_name } » contenant l’intitulé, la taille et l’empreinte numérique au format { $hash_func } de la pièce numérique copiée durant la mission de contrôle.
        *[other] copie sur support informatique d’un document remis au responsable des lieux, intitulé « { $file_name } » contenant l’inventaire des { $nb_str } pièces numériques copiées durant la mission de contrôle. Pour chaque pièce est précisé son intitulé, sa taille et son empreinte numérique au format { $hash_func }.
    }
msg_directory =
    { $nb ->
        [one] copie sur support informatique d’un dossier intitulé « { $dir_name } » contenant { $nb_str } document :
        *[other] copie sur support informatique d’un dossier intitulé « { $dir_name } » contenant { $nb_str } documents :
    }
msg_file = copie sur support informatique d’un document intitulé « { $file_name } »
msg_file_unit =
    { $nb ->
        [zero] octet
        [one] octet
        *[other] octets
    }


## Check errors

msg_info_check_error = Échec de la vérification des empreintes
view_errors = Voir les erreurs
title_invalid_ctn_file = Empreinte différente de celle du fichier de contenu
title_invalid_receipt = Empreinte différente de celle de l'accusé de réception
title_missing_ctn_file = Fichier non trouvé en local et présent dans le fichier de contenu
title_missing_receipt = Fichier non trouvé en local et présent dans l'accusé de réception
back = Retour


## Messages

msg_info_check_ok = Les empreintes correspondent.
msg_info_duplicate_hash = Fichiers identiques :
msg_info_empty_file = Fichier vide : { $file_name }
msg_info_has_ctn_file = Le dossier comporte un fichier { $file_name }
msg_info_hash_done = Calcul des empreintes effectué.
msg_info_hash_ignored_files = Les fichiers suivants n'ont pas été trouvés dans le fichier des empreintes et ont donc été ignorés :
msg_info_nb_files = Le dossier comporte { $nb } fichiers.
error_desc = { $error } : { $description }
msg_err_fl_not_found = Erreur interne : liste de fichiers non trouvée.
msg_err_load_dir = Erreur lors du chargement du dossier.
msg_check_invalid_format = format du fichier non valide
msg_err_fl = erreur lors de la création de la liste des fichiers
msg_err_fl_interrupted = la création de la liste des fichiers a été interrompue prématurément.
msg_err_fl_not_ready = la liste des fichiers n’a pas encore pu être construite.
msg_err_diff_calc_ar = Différences avec l’accusé de réception :
msg_err_diff_calc_ctn = Différences avec le fichier { $file_name } :


## Numbers

nb_main_sep = -
nb_last_sep = { nb_main_sep }

zero = zéro
one = un
two = deux
three = trois
four = quatre
five = cinq
six = six
seven = sept
eight = huit
nine = neuf
ten = dix
eleven = onze
twelve = douze
thirteen = treize
fourteen = quatorze
fifteen = quinze
sixteen = seize
seventeen = dix-sept
eighteen = dix-huit
nineteen = dix-neuf
twenty = vingt
twenty-one = vingt-et-un
twenty-two = vingt-deux
twenty-three = vingt-trois
twenty-four = vingt-quatre
twenty-five = vingt-cinq
twenty-six = vingt-six
twenty-seven = vingt-sept
twenty-eight = vingt-huit
twenty-nine = vingt-neuf
thirty = trente
thirty-one = trente-et-un
thirty-two = trente-deux
thirty-three = trente-trois
thirty-four = trente-quatre
thirty-five = trente-cinq
thirty-six = trente-six
thirty-seven = trente-sept
thirty-eight = trente-huit
thirty-nine = trente-neuf
forty = quarante
forty-one = quarante-et-un
forty-two = quarante-deux
forty-three = quarante-trois
forty-four = quarante-quatre
forty-five = quarante-cinq
forty-six = quarante-six
forty-seven = quarante-sept
forty-eight = quarante-huit
forty-nine = quarante-neuf
fifty = cinquante
fifty-one = cinquante-et-un
fifty-two = cinquante-deux
fifty-three = cinquante-trois
fifty-four = cinquante-quatre
fifty-five = cinquante-cinq
fifty-six = cinquante-six
fifty-seven = cinquante-sept
fifty-eight = cinquante-huit
fifty-nine = cinquante-neuf
sixty = soixante
sixty-one = soixante-et-un
sixty-two = soixante-deux
sixty-three = soixante-trois
sixty-four = soixante-quatre
sixty-five = soixante-cinq
sixty-six = soixante-six
sixty-seven = soixante-sept
sixty-eight = soixante-huit
sixty-nine = soixante-neuf
seventy = septante
seventy-one = septante-et-un
seventy-two = septante-deux
seventy-three = septante-trois
seventy-four = septante-quatre
seventy-five = septante-cinq
seventy-six = septante-six
seventy-seven = septante-sept
seventy-eight = septante-huit
seventy-nine = septante-neuf
eighty = octante
eighty-one = octante-et-un
eighty-two = octante-deux
eighty-three = octante-trois
eighty-four = octante-quatre
eighty-five = octante-cinq
eighty-six = octante-six
eighty-seven = octante-sept
eighty-eight = octante-huit
eighty-nine = octante-neuf
ninety = nonante
ninety-one = nonante-et-un
ninety-two = nonante-deux
ninety-three = nonante-trois
ninety-four = nonante-quatre
ninety-five = nonante-cinq
ninety-six = nonante-six
ninety-seven = nonante-sept
ninety-eight = nonante-huit
ninety-nine = nonante-neuf
-hundred-mult =
    { $nb_after ->
        [zero] cents
        *[other] cent
    }
hundred =
    { $nb ->
        [one] cent
        *[other] { $nb_str }{ nb_main_sep }{ -hundred-mult }
    }
thousand =
    { $nb ->
        [one] mille
        *[other] { $nb_str }{ nb_main_sep }mille
    }
million =
    { $nb ->
        [one] un{ nb_main_sep }million
        *[other] { $nb_str }{ nb_main_sep }millions
    }
billion =
    { $nb ->
        [one] un{ nb_main_sep }milliard
        *[other] { $nb_str }{ nb_main_sep }milliards
    }
