## Content file

content_file_name = content.txt
content_file_header =
    .name = Document name
    .size = Size (octets)


## Configuration options

label_nb_files_start = Number of the first evidence
label_content_file = Name of the checksum file


## Selection buttons

btn_select_dir = { $icon } Open a directory…
btn_select_mail = { $icon } Open an notice of receipt…
btn_trash_tip = Reset
label_email = Email


## Actions

btn_calc_fingerprints = Checksum calculation
btn_check_fingerprints = Data integrity check


## Progress

progress = Progress: { $done } / { $total }


## Clipboard buttons

btn_clipboard_tip = Copy the checksum of each evidences
btn_clipboard_ctn_file_tip = Copy the checksum of the checksum file


## File inclusion choices

btn_file_choice =
    .yes = Yes
    .yes_all = Yes for all
    .no = No
    .no_all = No for all
msg_file_choice_dir_hidden = { $file_name } is a hidden directory.
msg_file_choice_dir_system = { $file_name } is a system directory.
msg_file_choice_file_hidden = { $file_name } is a hidden file.
msg_file_choice_file_system = { $file_name } is a system file.
msg_file_choice_include = { $file_desc } Do you want to include it?


## Clipboard

msg_exhibit = EVIDENCE #{ $nb }:
msg_ctn_file =
    { $nb ->
        [one] copy on a digital media of a document given to the person in charge of the premises for the mission, named "{ $file_name }" containing name, size and checksum ({ $hash_func }) of the digital evidence collected during the on-site investigation.
        *[other] copy on a digital media of a document given to the person in charge of the premises for the mission, named "{ $file_name }" containing name, size and checksum ({ $hash_func }) of the digital { $nb } evidences collected during the on-site investigation.
    }
msg_directory =
    { $nb ->
        [one] copy on a digital media of a directory named "{ $dir_name }" containing { $nb } document:
        *[other] copy on a digital media of a directory named "{ $dir_name }" containing { $nb } documents:
    }
msg_file = copy on a digital media of a file named "{ $file_name }"
msg_file_unit =
    { $nb ->
        [zero] octet
        [one] octet
        *[other] octets
    }


## Messages

msg_info_check_ok = Data integrity check passed.
msg_info_has_ctn_file = The directory contains a { $file_name } file
msg_info_del_ctn_file = delete
msg_info_nb_files = The directory contains { $nb } files.
error_desc = { $error }: { $description }
msg_err_fl_not_found = Internal error: file list not found.
msg_err_load_dir = Error while loading the directory.
msg_check_invalid_format = invalid file format
msg_err_fl = error during the file list creation
msg_err_fl_interrupted = the file list creation has been unexpectedly interupted.
msg_err_fl_not_ready = the file list has not be created yet.
msg_err_diff_calc_ar = The following items do not match with the ones of the notice of receipt:
msg_err_diff_calc_ctn = The following items do not match with the ones of the { $file_name }:
