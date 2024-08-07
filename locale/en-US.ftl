## Theme

theme = Theme
theme_dark = dark
theme_light = light

## Content file

content_file_name = content.txt
content_file_header =
    .name = Document name
    .size = Size (octets)


## Configuration options

config = Configuration
config_not_available = Configuration unavailable when a directory is opened
label_nb_files_start = Number of the first evidence
label_content_file = Name of the checksum file
label_hash_function = Hash function
language = Language
number_representation = Number representation
letters = letters
western_arabic_numerals = western Arabic numerals
clipboard_threshold = Clipboard threshold
clipboard_persistence = Clipboard persistence
clipboard_persistence_help = Defines whether or not the internal clipboard management object should be kept into memory. This parameter depends on your operating system. It is recommended to keep the default value unless you encounter troubles with the clipboard.
default = default
activated = activated
deactivated = deactivated
apply = Apply
cancel = Cancel


## Selection buttons

btn_select_dir = Open a directory
btn_select_receipt = Open a notice of receipt
btn_trash_tip = Reset
label_receipt = Receipt


## Actions

btn_calc_fingerprints = Checksum calculation
btn_check_fingerprints = Data integrity check


## Progress

progress = Progress: { $done } / { $total }


## Clipboard buttons

btn_clipboard_tip = Copy the checksum of each evidence
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
        [one] copy on a digital media of a document given to the person in charge of the premises for the mission, named “{ $file_name }” containing name, size and checksum ({ $hash_func }) of the digital evidence collected during the on-site investigation.
        *[other] copy on a digital media of a document given to the person in charge of the premises for the mission, named “{ $file_name }” containing name, size and checksum ({ $hash_func }) of the digital { $nb } evidences collected during the on-site investigation.
    }
msg_directory =
    { $nb ->
        [one] copy on a digital media of a directory named “{ $dir_name }” containing { $nb } document:
        *[other] copy on a digital media of a directory named “{ $dir_name }” containing { $nb } documents:
    }
msg_file = copy on a digital media of a file named “{ $file_name }”
msg_file_unit =
    { $nb ->
        [zero] octet
        [one] octet
        *[other] octets
    }


## Check errors

msg_info_check_error = Data integrity check failed
view_errors = View errors
title_invalid_ctn_file = Checksum different from the one in the content file
title_invalid_receipt = Checksum different from the one in the receipt
title_missing_ctn_file = File not found locally but present in the content file
title_missing_receipt = File not found locally but present in the receipt
back = Back


## Messages

msg_info_check_ok = Data integrity check passed.
msg_info_duplicate_hash = Identical files:
msg_info_empty_file = Empty file: { $file_name }
msg_info_has_ctn_file = The directory contains a { $file_name } file
msg_info_hash_done = Checksum calculation completed.
msg_info_hash_ignored_files = The following files were not found in the content file and therefore ignored:
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


## Numbers

nb_main_sep = { $space }
nb_last_sep = { $space }and{ $space }

zero = zero
one = one
two = two
three = three
four = four
five = five
six = six
seven = seven
eight = eight
nine = nine
ten = ten
eleven = eleven
twelve = twelve
thirteen = thirteen
fourteen = fourteen
fifteen = fifteen
sixteen = sixteen
seventeen = seventeen
eighteen = eighteen
nineteen = nineteen
twenty = twenty
twenty-one = twenty-one
twenty-two = twenty-two
twenty-three = twenty-three
twenty-four = twenty-four
twenty-five = twenty-five
twenty-six = twenty-six
twenty-seven = twenty-seven
twenty-eight = twenty-eight
twenty-nine = twenty-nine
thirty = thirty
thirty-one = thirty-one
thirty-two = thirty-two
thirty-three = thirty-three
thirty-four = thirty-four
thirty-five = thirty-five
thirty-six = thirty-six
thirty-seven = thirty-seven
thirty-eight = thirty-eight
thirty-nine = thirty-nine
forty = forty
forty-one = forty-one
forty-two = forty-two
forty-three = forty-three
forty-four = forty-four
forty-five = forty-five
forty-six = forty-six
forty-seven = forty-seven
forty-eight = forty-eight
forty-nine = forty-nine
fifty = fifty
fifty-one = fifty-one
fifty-two = fifty-two
fifty-three = fifty-three
fifty-four = fifty-four
fifty-five = fifty-five
fifty-six = fifty-six
fifty-seven = fifty-seven
fifty-eight = fifty-eight
fifty-nine = fifty-nine
sixty = sixty
sixty-one = sixty-one
sixty-two = sixty-two
sixty-three = sixty-three
sixty-four = sixty-four
sixty-five = sixty-five
sixty-six = sixty-six
sixty-seven = sixty-seven
sixty-eight = sixty-eight
sixty-nine = sixty-nine
seventy = seventy
seventy-one = seventy-one
seventy-two = seventy-two
seventy-three = seventy-three
seventy-four = seventy-four
seventy-five = seventy-five
seventy-six = seventy-six
seventy-seven = seventy-seven
seventy-eight = seventy-eight
seventy-nine = seventy-nine
eighty = eighty
eighty-one = eighty-one
eighty-two = eighty-two
eighty-three = eighty-three
eighty-four = eighty-four
eighty-five = eighty-five
eighty-six = eighty-six
eighty-seven = eighty-seven
eighty-eight = eighty-eight
eighty-nine = eighty-nine
ninety = ninety
ninety-one = ninety-one
ninety-two = ninety-two
ninety-three = ninety-three
ninety-four = ninety-four
ninety-five = ninety-five
ninety-six = ninety-six
ninety-seven = ninety-seven
ninety-eight = ninety-eight
ninety-nine = ninety-nine
hundred = { $nb_str }{ nb_main_sep }hundred
thousand = { $nb_str }{ nb_main_sep }thousand
million = { $nb_str }{ nb_main_sep }million
billion = { $nb_str }{ nb_main_sep }billion
