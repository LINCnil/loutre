## Content file

content_file_name = contenu.txt
content_file_header =
    .name = Nom du document
    .size = Taille (octets)


## Configuration options

label_nb_files_start = Numéro de la première pièce
label_content_file = Nom du fichier d’empreintes


## Selection buttons

btn_select_dir = { $icon } Ouvrir un dossier…
btn_select_mail = { $icon } Ouvrir un AR…
btn_trash_tip = Réinitialiser
label_email = Courrier électronique


## Actions

btn_calc_fingerprints = Calculer les empreintes
btn_check_fingerprints = Vérifier les empreintes


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
        *[other] copie sur support informatique d’un document remis au responsable des lieux, intitulé « { $file_name } » contenant l’inventaire des { $nb } pièces numériques copiées durant la mission de contrôle. Pour chaque pièce est précisé son intitulé, sa taille et son empreinte numérique au format { $hash_func }.
    }
msg_directory =
    { $nb ->
        [one] copie sur support informatique d’un dossier intitulé « { $dir_name } » contenant { $nb } document :
        *[other] copie sur support informatique d’un dossier intitulé « { $dir_name } » contenant { $nb } documents :
    }
msg_file = copie sur support informatique d’un document intitulé « { $file_name } »
msg_file_unit =
    { $nb ->
        [zero] octet
        [one] octet
        *[other] octets
    }


## Messages

msg_info_check_ok = Les empreintes correspondent.
msg_info_has_ctn_file = Le dossier comporte un fichier { $file_name }
msg_info_del_ctn_file = supprimer
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
