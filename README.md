1. Download the SQLite3 source code amalgamation [here](https://www.sqlite.org/download.html).
2. Compile the SQLite3 source code into a static library:
```
gcc -lpthread -shared -o libsqlite3.so -fPIC sqlite3.c
```

3. Copy the newly created `libsqlite3.so` file to `/usr/local/lib` (note that this will likely require `sudo` on macOS).