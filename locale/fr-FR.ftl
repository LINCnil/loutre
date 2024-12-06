## Main view

view_main_open_dir = Ouvrir un dossier
view_main_open_receipt = Ouvrir un AR
view_main_calc_fingerprints = Calculer les empreintes
view_main_check_fingerprints = Vérifier les empreintes
view_main_check_result_title = Vérification des empreintes
view_main_check_result_ok_text = Les empreintes correspondent.
view_main_check_result_err_text = Échec de la vérification des empreintes.

cpn_file_list_delete = Réinitialiser

cpn_progress_bar_status = Réalisé : { $done } / { $total } ({ $percent } %)

cpn_notif_empty_files_title = Fichiers vides détectés
cpn_notif_empty_files_text = Un ou plusieurs fichiers contenus dans le dossier sélectionné sont vides.

cpn_notif_excluded_files_title = Fichiers ignoré
cpn_notif_excluded_files_text = Des fichiers cachés ou des fichiers système ont été automatiquement ignorés.

cpn_notif_duplicated_files_title = Fichiers dupliqués détectés
cpn_notif_duplicated_files_text = Plusieurs fichiers contenus dans le dossier sélectionné sont identiques.

## Configuration

view_config_title = Configuration

cpn_config_menu_files_title = Fichiers
cpn_config_menu_hash_title = Empreintes
cpn_config_menu_messages_title = Messages
cpn_config_menu_clipboard_title = Presse-papier

view_config_main_msg_include_hidden_files = Inclure les fichiers cachés
view_config_main_msg_include_system_files = Inclure les fichiers système
view_config_main_msg_set_files_readonly = Passer les fichiers en lecture seule

view_config_hash_msg_hash_func = Algorithme de hachage
view_config_hash_msg_content_file_format = Format du fichier de contenu
view_config_hash_msg_content_file_name = Nom du fichier de contenu

view_config_messages_msg_empty_files_warning = Afficher un avertissement en cas de chargement d'un dossier comprenant des fichiers vides
view_config_messages_msg_duplicated_files_warning = Afficher un avertissement lorsque des fichiers avec la même empreinte sont détectés

view_config_clipboard_msg_numbers = Représentation des nombres
view_config_clipboard_msg_letters = Lettres
view_config_clipboard_msg_western_arabic_numerals = Chiffres arabes

view_config_clipboard_start_msg = Numéro de la première pièce

view_config_clipboard_msg_threshold = Seuil du presse-papier
view_config_clipboard_msg_threshold_help = Nombre de fichiers à partir duquel le presse papier contient par défaut l'empreinte du fichier de contenu plutôt que les empreintes de chaque pièce.

view_config_clipboard_msg_persistence = Persistance du presse-papier
view_config_clipboard_msg_persistence_help = Indique si l’objet interne gérant le presse-papier soit être conservé en mémoire ou non. Ce paramètre dépends du système d’exploitation. En l'absence de problème avec le presse-papier, il est recommandé de conserver la valeur par défaut.
view_config_clipboard_msg_persistence_default = Par défaut
view_config_clipboard_msg_persistence_activated = Activée
view_config_clipboard_msg_persistence_deactivated = Désactivée

## Header

cpn_header_config = Configuration

## Theme

cpn_theme_change = Modifier le thème

## Clipboard

-cpn_clipboard_ctn_file =
    { $nb_evidences ->
        [one] copie sur support informatique d’un document remis au responsable des lieux, intitulé « {"{{"} evidence.name {"}}"} » contenant l’intitulé, la taille et l’empreinte numérique au format {"{{"} hash_func {"}}"} de la pièce numérique copiée durant la mission de contrôle.
        *[other] copie sur support informatique d’un document remis au responsable des lieux, intitulé « {"{{"} evidence.name {"}}"} » contenant l’inventaire des {"{{"} nb_evidences {"}}"} pièces numériques copiées durant la mission de contrôle. Pour chaque pièce est précisé son intitulé, sa taille et son empreinte numérique au format {"{{"} hash_func {"}}"}.
    }
-cpn_clipboard_file_data_txt = {"{{"} evidence.size {"}}"} octets, {"{{"} evidence.hash_func {"}}"} : {"{{"} evidence.hash {"}}"}
-cpn_clipboard_file_data_html = <i>{"{{"} evidence.size {"}}"}</i> octets, {"{{"} evidence.hash_func {"}}"} : <i>{"{{"} evidence.hash {"}}"}</i>

cpn_clipboard_ctn_file_full_txt = PIÈCE No {"{{"} nb_start {"}}"} : { -cpn_clipboard_ctn_file }
    { -cpn_clipboard_file_data_txt }

cpn_clipboard_ctn_file_full_html = <p><b>PIÈCE N<sup>o</sup> {"{{"} nb_start {"}}"} :</b> { -cpn_clipboard_ctn_file }<br>
    { -cpn_clipboard_file_data_html }</p>

cpn_clipboard_list_full_txt = {"{"}% set nb = 1 %{"}"}{"{"}% set evidences = evidences|add_dir_level %{"}"}{"{"}% for entry in evidences -%{"}"}
    PIÈCE No {"{{"} nb {"}}"} : copie sur support informatique d’un {"{"}% if entry.is_file %{"}"}document{"{"}% else %{"}"}dossier{"{"}% endif %{"}"} intitulé « {"{{"} entry.name {"}}"} »{"{"}% if entry.is_dir %{"}"} contenant {"{{"} entry.size {"}}"} documents :{"{"}% endif %{"}"}
    {"{"}%- if entry.is_dir %{"}"}{"{"}% for sub_entry in entry.evidences %{"}"}
    « {"{{"} sub_entry.name {"}}"} » {"{"}% with evidence = sub_entry %{"}"}{ -cpn_clipboard_file_data_txt }{"{"}% endwith %{"}"}
    {"{"}%- endfor %{"}"}{"{"}% endif -%{"}"}
    {"{"}% if entry.is_file %{"}"}{"{"}% with evidence = entry %{"}"}{ -cpn_clipboard_file_data_txt }{"{"}% endwith %{"}"}{"{"}% endif %{"}"}
    {"{"}% set nb = nb + 1 %{"}"}
    {"{"}% endfor %{"}"}

cpn_clipboard_list_full_html = {"{"}% set nb = 1 %{"}"}{"{"}% set evidences = evidences|add_dir_level %{"}"}{"{"}% for entry in evidences %{"}"}<p>
    <b>PIÈCE N<sup>o</sup> {"{{"} nb {"}}"} :</b> copie sur support informatique d’un {"{"}% if entry.is_file %{"}"}document{"{"}% else %{"}"}dossier{"{"}% endif %{"}"} intitulé « {"{{"} entry.name {"}}"} »{"{"}% if entry.is_dir %{"}"} contenant {"{{"} entry.size {"}}"} documents :{"{"}% else %{"}"}<br>{"{"}% endif %{"}"}
    {"{"}% if entry.is_dir %{"}"}<ul>{"{"}% for sub_entry in entry.evidences %{"}"}
    <li>« {"{{"} sub_entry.name {"}}"} »<br>{"{"}% with evidence = sub_entry %{"}"}{ -cpn_clipboard_file_data_html }{"{"}% endwith %{"}"}</li>
    {"{"}% endfor %{"}"}</ul>{"{"}% endif %{"}"}
    {"{"}% if entry.is_file %{"}"}{"{"}% with evidence = entry %{"}"}{ -cpn_clipboard_file_data_html }{"{"}% endwith %{"}"}{"{"}% endif %{"}"}
    </p>{"{"}% set nb = nb + 1 %{"}"}{"{"}% endfor %{"}"}

## Numbers

cpn_nb_letters = lettres
cpn_nb_western_arabic_numerals = chiffres arabes

cpn_nb_main_sep = -
cpn_nb_last_sep = { cpn_nb_main_sep }

cpn_nb_zero = zéro
cpn_nb_one = un
cpn_nb_two = deux
cpn_nb_three = trois
cpn_nb_four = quatre
cpn_nb_five = cinq
cpn_nb_six = six
cpn_nb_seven = sept
cpn_nb_eight = huit
cpn_nb_nine = neuf
cpn_nb_ten = dix
cpn_nb_eleven = onze
cpn_nb_twelve = douze
cpn_nb_thirteen = treize
cpn_nb_fourteen = quatorze
cpn_nb_fifteen = quinze
cpn_nb_sixteen = seize
cpn_nb_seventeen = dix-sept
cpn_nb_eighteen = dix-huit
cpn_nb_nineteen = dix-neuf
cpn_nb_twenty = vingt
cpn_nb_twenty-one = vingt-et-un
cpn_nb_twenty-two = vingt-deux
cpn_nb_twenty-three = vingt-trois
cpn_nb_twenty-four = vingt-quatre
cpn_nb_twenty-five = vingt-cinq
cpn_nb_twenty-six = vingt-six
cpn_nb_twenty-seven = vingt-sept
cpn_nb_twenty-eight = vingt-huit
cpn_nb_twenty-nine = vingt-neuf
cpn_nb_thirty = trente
cpn_nb_thirty-one = trente-et-un
cpn_nb_thirty-two = trente-deux
cpn_nb_thirty-three = trente-trois
cpn_nb_thirty-four = trente-quatre
cpn_nb_thirty-five = trente-cinq
cpn_nb_thirty-six = trente-six
cpn_nb_thirty-seven = trente-sept
cpn_nb_thirty-eight = trente-huit
cpn_nb_thirty-nine = trente-neuf
cpn_nb_forty = quarante
cpn_nb_forty-one = quarante-et-un
cpn_nb_forty-two = quarante-deux
cpn_nb_forty-three = quarante-trois
cpn_nb_forty-four = quarante-quatre
cpn_nb_forty-five = quarante-cinq
cpn_nb_forty-six = quarante-six
cpn_nb_forty-seven = quarante-sept
cpn_nb_forty-eight = quarante-huit
cpn_nb_forty-nine = quarante-neuf
cpn_nb_fifty = cinquante
cpn_nb_fifty-one = cinquante-et-un
cpn_nb_fifty-two = cinquante-deux
cpn_nb_fifty-three = cinquante-trois
cpn_nb_fifty-four = cinquante-quatre
cpn_nb_fifty-five = cinquante-cinq
cpn_nb_fifty-six = cinquante-six
cpn_nb_fifty-seven = cinquante-sept
cpn_nb_fifty-eight = cinquante-huit
cpn_nb_fifty-nine = cinquante-neuf
cpn_nb_sixty = soixante
cpn_nb_sixty-one = soixante-et-un
cpn_nb_sixty-two = soixante-deux
cpn_nb_sixty-three = soixante-trois
cpn_nb_sixty-four = soixante-quatre
cpn_nb_sixty-five = soixante-cinq
cpn_nb_sixty-six = soixante-six
cpn_nb_sixty-seven = soixante-sept
cpn_nb_sixty-eight = soixante-huit
cpn_nb_sixty-nine = soixante-neuf
cpn_nb_seventy = soixante-dix
cpn_nb_seventy-one = soixante-et-onze
cpn_nb_seventy-two = soixante-douze
cpn_nb_seventy-three = soixante-treize
cpn_nb_seventy-four = soixante-quatorze
cpn_nb_seventy-five = soixante-quinze
cpn_nb_seventy-six = soixante-seize
cpn_nb_seventy-seven = soixante-dix-sept
cpn_nb_seventy-eight = soixante-dix-huit
cpn_nb_seventy-nine = soixante-dix-neuf
cpn_nb_eighty = quatre-vingts
cpn_nb_eighty-one = quatre-vingt-un
cpn_nb_eighty-two = quatre-vingt-deux
cpn_nb_eighty-three = quatre-vingt-trois
cpn_nb_eighty-four = quatre-vingt-quatre
cpn_nb_eighty-five = quatre-vingt-cinq
cpn_nb_eighty-six = quatre-vingt-six
cpn_nb_eighty-seven = quatre-vingt-sept
cpn_nb_eighty-eight = quatre-vingt-huit
cpn_nb_eighty-nine = quatre-vingt-neuf
cpn_nb_ninety = quatre-vingt-dix
cpn_nb_ninety-one = quatre-vingt-onze
cpn_nb_ninety-two = quatre-vingt-douze
cpn_nb_ninety-three = quatre-vingt-treize
cpn_nb_ninety-four = quatre-vingt-quatorze
cpn_nb_ninety-five = quatre-vingt-quinze
cpn_nb_ninety-six = quatre-vingt-seize
cpn_nb_ninety-seven = quatre-vingt-dix-sept
cpn_nb_ninety-eight = quatre-vingt-dix-huit
cpn_nb_ninety-nine = quatre-vingt-dix-neuf
-cpn_nb_hundred_mult =
    { $nb_after ->
        [zero] cents
        *[other] cent
    }
cpn_nb_hundred =
    { $nb ->
        [one] cent
        *[other] { $nb_str }{ cpn_nb_main_sep }{ -cpn_nb_hundred_mult }
    }
cpn_nb_thousand =
    { $nb ->
        [one] mille
        *[other] { $nb_str }{ cpn_nb_main_sep }mille
    }
cpn_nb_million =
    { $nb ->
        [one] un{ cpn_nb_main_sep }million
        *[other] { $nb_str }{ cpn_nb_main_sep }millions
    }
cpn_nb_billion =
    { $nb ->
        [one] un{ cpn_nb_main_sep }milliard
        *[other] { $nb_str }{ cpn_nb_main_sep }milliards
    }
