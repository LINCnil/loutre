## Main view

view_main_greeting = What do you wish to open?
view_main_open_dir = Open a directory
view_main_open_receipt = Open a notice of receipt
view_main_calc_fingerprints = Checksum calculation
view_main_check_fingerprints = Data integrity check
view_main_check_result_title = Data integrity check result
view_main_check_result_ok_text = Data integrity check passed.
view_main_check_result_err_text = Data integrity check failed.
view_main_check_result_err_link = See errors
view_main_clipboard_btn_list = Copy list
view_main_clipboard_btn_file = Copy content file

cpn_file_list_delete = Reset

cpn_progress_bar_status = Progress: { $done } / { $total } ({ $percent } %)

cpn_notif_empty_files_title =
    { $nb ->
        [one] Empty file detected
        *[other] Empty files detected
    }
cpn_notif_empty_files_text =
    { $nb ->
        [one] A file located in the selected directory is empty.
        *[other] { $nb } files located in the selected directory are empty.
    }
cpn_notif_empty_files_link =
    { $nb ->
        [one] See file
        *[other] See list
    }

cpn_notif_excluded_files_title =
    { $nb ->
        [one] Ignored file
        *[other] Ignored files
    }
cpn_notif_excluded_files_text =
    { $nb ->
        [one] A hidden or system file has automatically been ignored.
        *[other] Hidden or system files has automatically been ignored.
    }
cpn_notif_excluded_files_link = { cpn_notif_empty_files_link }

cpn_notif_duplicated_files_title = Duplicated files detected
cpn_notif_duplicated_files_text = Several files located in the selected directory are identical.
cpn_notif_duplicated_files_link = See list

## Check errors view

view_check_errors_title = Verification errors
view_check_errors_ctn_file_parse = Unable to parse the content file.
view_check_errors_ctn_file_missing = File is listed in the content file but does not exists on disk.
view_check_errors_ctn_file_match = File differs from the content file.
view_check_errors_receipt_missing = File is listed in the receipt but does not exists on disk.
view_check_errors_receipt_match = File differs from the receipt.

## Empty files view

view_empty_files_title =
    { $nb ->
        [one] Empty file
        *[other] Empty files
    }

## Excluded files view

view_excluded_files_title = { cpn_notif_excluded_files_title }

## Duplicated files view

view_duplicated_files_title = Duplicated files

## Debug view

view_debug_title = Debug interface
view_debug_notif_title = Notifications
view_debug_notif_level_error = Error
view_debug_notif_level_warning = Warning
view_debug_notif_level_success = Success
view_debug_notif_level_info = Info
view_debug_progress_bar_title = Progress bar
view_debug_loading_bar_title = Loading bar
view_debug_submit = Submit
view_debug_reset = Reset
view_debug_toogle = Toogle

## Configuration

view_config_title = Configuration
cpn_form_apply_config = Save
cpn_form_apply_config_ok = Settings saved

cpn_config_menu_files_title = Files
cpn_config_menu_hash_title = Fingerprints
cpn_config_menu_messages_title = Messages
cpn_config_menu_clipboard_title = Clipboard

view_config_main_msg_include_hidden_files = Include hidden files
view_config_main_msg_include_system_files = Include system files
view_config_main_msg_set_files_readonly = Set files as read-only

view_config_hash_msg_hash_func = Hashing algorithm
view_config_hash_msg_content_file_format = Content file format
view_config_hash_msg_content_file_name = Content file name
view_config_hash_msg_customize_ctn_file_name = Edit

view_config_ctn_file_name_custom_value = Use a custom name
view_config_ctn_file_name_default_value = Use the default name

view_config_messages_msg_empty_files_warning = Display a warning when loading a directory that contains empty files
view_config_messages_msg_duplicated_files_warning = Display a warning when files with the same fingerprint are detected

view_config_clipboard_start_msg = Number of the first evidence

view_config_clipboard_msg_threshold = Clipboard threshold
view_config_clipboard_msg_threshold_help = Number of files from which the clipboard contains, by default, the checksum of the checksum file instead of the checksum of each individual files.

view_config_clipboard_msg_persistence = Clipboard persistence
view_config_clipboard_msg_persistence_help = Defines whether or not the internal clipboard management object should be kept into memory. This parameter depends on your operating system. It is recommended to keep the default value unless you encounter troubles with the clipboard.
view_config_clipboard_msg_persistence_default = Default
view_config_clipboard_msg_persistence_activated = Activated
view_config_clipboard_msg_persistence_deactivated = Deactivated
view_config_clipboard_msg_tpl_list_html = Clipboard content (HTML list)
view_config_clipboard_msg_tpl_list_txt = Clipboard content (plain text list)
view_config_clipboard_msg_tpl_ctn_file_html = Clipboard content (HTML content file)
view_config_clipboard_msg_tpl_ctn_file_txt = Clipboard content (plain text content file)
view_config_clipboard_msg_has_default_value = Default value
view_config_clipboard_msg_has_custom_value = Custom value
view_config_clipboard_msg_edit_value = Edit
view_config_clipboard_msg_reset_value = Reset

## Header

cpn_header_config = Configuration

## Theme

cpn_theme_change = Change the theme

## Clipboard

-cpn_clipboard_ctn_file =
    { $nb_evidences ->
        [one] copy on a digital media of a document given to the person in charge of the premises for the mission, named “{"{{"} evidence.name {"}}"}” containing name, size and checksum ({"{{"} hash_func {"}}"}) of the digital evidence collected during the on-site investigation.
        *[other] copy on a digital media of a document given to the person in charge of the premises for the mission, named “{"{{"} evidence.name {"}}"}” containing name, size and checksum ({"{{"} hash_func {"}}"}) of the digital {"{{"} nb_evidences|nb_letters {"}}"} evidences collected during the on-site investigation.
    }
-cpn_clipboard_file_data_txt = {"{{"} evidence.size {"}}"} octets, {"{{"} evidence.hash_func {"}}"}: {"{{"} evidence.hash {"}}"}
-cpn_clipboard_file_data_html = <i>{"{{"} evidence.size {"}}"}</i> octets, {"{{"} evidence.hash_func {"}}"}: <i>{"{{"} evidence.hash {"}}"}</i>

cpn_clipboard_ctn_file_full_txt = EVIDENCE #{"{{"} nb_start {"}}"}: { -cpn_clipboard_ctn_file }
    { -cpn_clipboard_file_data_txt }

cpn_clipboard_ctn_file_full_html = <p><b>EVIDENCE #{"{{"} nb_start {"}}"}:</b> { -cpn_clipboard_ctn_file }<br>
    { -cpn_clipboard_file_data_html }</p>

cpn_clipboard_list_full_txt = {"{"}% set nb = nb_start %{"}"}{"{"}% set evidences = evidences|add_dir_level %{"}"}{"{"}% for entry in evidences -%{"}"}
    EVIDENCE #{"{{"} nb {"}}"}: copy on a digital media of a {"{"}% if entry.is_file %{"}"}file{"{"}% else %{"}"}directory{"{"}% endif %{"}"} named “{"{{"} entry.name {"}}"}”{"{"}% if entry.is_dir %{"}"} containing {"{{"} entry.size|nb_letters {"}}"} files:{"{"}% endif %{"}"}
    {"{"}%- if entry.is_dir %{"}"}{"{"}% for sub_entry in entry.evidences %{"}"}
    “{"{{"} sub_entry.name {"}}"}” {"{"}% with evidence = sub_entry %{"}"}{ -cpn_clipboard_file_data_txt }{"{"}% endwith %{"}"}
    {"{"}%- endfor %{"}"}{"{"}% endif -%{"}"}
    {"{"}% if entry.is_file %{"}"} {"{"}% with evidence = entry %{"}"}{ -cpn_clipboard_file_data_txt }{"{"}% endwith %{"}"}{"{"}% endif %{"}"}
    {"{"}% set nb = nb + 1 %{"}"}
    {"{"}% endfor %{"}"}

cpn_clipboard_list_full_html = {"{"}% set nb = nb_start %{"}"}{"{"}% set evidences = evidences|add_dir_level %{"}"}{"{"}% for entry in evidences %{"}"}<p>
    <b>EVIDENCE #{"{{"} nb {"}}"}:</b> copy on a digital media of a {"{"}% if entry.is_file %{"}"}file{"{"}% else %{"}"}directory{"{"}% endif %{"}"} named “{"{{"} entry.name {"}}"}”{"{"}% if entry.is_dir %{"}"} containing {"{{"} entry.size {"}}"} files:{"{"}% else %{"}"}<br>{"{"}% endif %{"}"}
    {"{"}% if entry.is_dir %{"}"}<ul>{"{"}% for sub_entry in entry.evidences %{"}"}
    <li>“{"{{"} sub_entry.name {"}}"}”<br>{"{"}% with evidence = sub_entry %{"}"}{ -cpn_clipboard_file_data_html }{"{"}% endwith %{"}"}</li>
    {"{"}% endfor %{"}"}</ul>{"{"}% endif %{"}"}
    {"{"}% if entry.is_file %{"}"}{"{"}% with evidence = entry %{"}"}{ -cpn_clipboard_file_data_html }{"{"}% endwith %{"}"}{"{"}% endif %{"}"}
    </p>{"{"}% set nb = nb + 1 %{"}"}{"{"}% endfor %{"}"}

## Numbers

cpn_nb_letters = letters
cpn_nb_western_arabic_numerals = western Arabic numerals

cpn_nb_main_sep = { $space }
cpn_nb_last_sep = { $space }and{ $space }

cpn_nb_zero = zero
cpn_nb_one = one
cpn_nb_two = two
cpn_nb_three = three
cpn_nb_four = four
cpn_nb_five = five
cpn_nb_six = six
cpn_nb_seven = seven
cpn_nb_eight = eight
cpn_nb_nine = nine
cpn_nb_ten = ten
cpn_nb_eleven = eleven
cpn_nb_twelve = twelve
cpn_nb_thirteen = thirteen
cpn_nb_fourteen = fourteen
cpn_nb_fifteen = fifteen
cpn_nb_sixteen = sixteen
cpn_nb_seventeen = seventeen
cpn_nb_eighteen = eighteen
cpn_nb_nineteen = nineteen
cpn_nb_twenty = twenty
cpn_nb_twenty-one = twenty-one
cpn_nb_twenty-two = twenty-two
cpn_nb_twenty-three = twenty-three
cpn_nb_twenty-four = twenty-four
cpn_nb_twenty-five = twenty-five
cpn_nb_twenty-six = twenty-six
cpn_nb_twenty-seven = twenty-seven
cpn_nb_twenty-eight = twenty-eight
cpn_nb_twenty-nine = twenty-nine
cpn_nb_thirty = thirty
cpn_nb_thirty-one = thirty-one
cpn_nb_thirty-two = thirty-two
cpn_nb_thirty-three = thirty-three
cpn_nb_thirty-four = thirty-four
cpn_nb_thirty-five = thirty-five
cpn_nb_thirty-six = thirty-six
cpn_nb_thirty-seven = thirty-seven
cpn_nb_thirty-eight = thirty-eight
cpn_nb_thirty-nine = thirty-nine
cpn_nb_forty = forty
cpn_nb_forty-one = forty-one
cpn_nb_forty-two = forty-two
cpn_nb_forty-three = forty-three
cpn_nb_forty-four = forty-four
cpn_nb_forty-five = forty-five
cpn_nb_forty-six = forty-six
cpn_nb_forty-seven = forty-seven
cpn_nb_forty-eight = forty-eight
cpn_nb_forty-nine = forty-nine
cpn_nb_fifty = fifty
cpn_nb_fifty-one = fifty-one
cpn_nb_fifty-two = fifty-two
cpn_nb_fifty-three = fifty-three
cpn_nb_fifty-four = fifty-four
cpn_nb_fifty-five = fifty-five
cpn_nb_fifty-six = fifty-six
cpn_nb_fifty-seven = fifty-seven
cpn_nb_fifty-eight = fifty-eight
cpn_nb_fifty-nine = fifty-nine
cpn_nb_sixty = sixty
cpn_nb_sixty-one = sixty-one
cpn_nb_sixty-two = sixty-two
cpn_nb_sixty-three = sixty-three
cpn_nb_sixty-four = sixty-four
cpn_nb_sixty-five = sixty-five
cpn_nb_sixty-six = sixty-six
cpn_nb_sixty-seven = sixty-seven
cpn_nb_sixty-eight = sixty-eight
cpn_nb_sixty-nine = sixty-nine
cpn_nb_seventy = seventy
cpn_nb_seventy-one = seventy-one
cpn_nb_seventy-two = seventy-two
cpn_nb_seventy-three = seventy-three
cpn_nb_seventy-four = seventy-four
cpn_nb_seventy-five = seventy-five
cpn_nb_seventy-six = seventy-six
cpn_nb_seventy-seven = seventy-seven
cpn_nb_seventy-eight = seventy-eight
cpn_nb_seventy-nine = seventy-nine
cpn_nb_eighty = eighty
cpn_nb_eighty-one = eighty-one
cpn_nb_eighty-two = eighty-two
cpn_nb_eighty-three = eighty-three
cpn_nb_eighty-four = eighty-four
cpn_nb_eighty-five = eighty-five
cpn_nb_eighty-six = eighty-six
cpn_nb_eighty-seven = eighty-seven
cpn_nb_eighty-eight = eighty-eight
cpn_nb_eighty-nine = eighty-nine
cpn_nb_ninety = ninety
cpn_nb_ninety-one = ninety-one
cpn_nb_ninety-two = ninety-two
cpn_nb_ninety-three = ninety-three
cpn_nb_ninety-four = ninety-four
cpn_nb_ninety-five = ninety-five
cpn_nb_ninety-six = ninety-six
cpn_nb_ninety-seven = ninety-seven
cpn_nb_ninety-eight = ninety-eight
cpn_nb_ninety-nine = ninety-nine
cpn_nb_hundred = { $nb_str }{ cpn_nb_main_sep }hundred
cpn_nb_thousand = { $nb_str }{ cpn_nb_main_sep }thousand
cpn_nb_million = { $nb_str }{ cpn_nb_main_sep }million
cpn_nb_billion = { $nb_str }{ cpn_nb_main_sep }billion
