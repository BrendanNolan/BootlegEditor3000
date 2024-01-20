Notes On Behaviour

- Launches by printing a welcome message and some help text detailing commands
- Treats the first line of a csv as the header line
- Does not treat quotes in any special way - if they appear in a CSV cell or a command, they are
  preserved
- Uses 1-based indexing; the first row/column has index 1, not 0
- Project contains several tests - run as usual with `cargo test`

Example of a session:

```
Welcome To BootlegEditor3000. Your CSV Data Has Been Loaded.
Below is a list of commands. To see this at any time, type 'help'.
display
display_row_range <first_row> <last_row>
modify_row <row> <new_data>
delete_row <row>
modify_column <column> <new_data>
delete_column <column>
modify_column_by_name <column_name> <new_data>
delete_column_by_name <column_name>
dimensions
write_to_file <file_name>
display_headers
>>>> display
"near","carry","pattern","fourth","whatever","easier"
"environment","managed","valley","potatoes","there","century"
"his","soft","breathing","gun","barn","completely"
"community","block","along","telephone","jar","play"
"present","attention","factor","swung","path","at"
"practical","form","port","actual","bottom","hot"
"however","great","soil","captured","tribe","beyond"
>>>> display_headers
"near","carry","pattern","fourth","whatever","easier"
>>>> dimensions
Rows: 6, Columns: 6
>>>> modify_row 1 one,two,three,four,five,six
>>>> display_row_range 1 1
one,two,three,four,five,six
>>>>
```
