# sqlite_tables_to_json
Use sqlite cli/shell commands to export tables to json

With the rust `std::process::Command` and rayon, export sqlite tables to individual json files using builtin
sqlite shell dot commands e.g. `sqlite3 db_file '.mode json' '.once jsonfile' sql_statement`. 

Spatialite db's aswell can be exported if 'mod_spatialite' is in user/system path.
