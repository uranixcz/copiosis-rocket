This program is intended to run only on payer's (community administrator's) PC as it has no security at all. It does not work on Windows XP!
After start type the URL in your browser. Linux users should install sqlite3.

To compile you need Rust nightly build with cargo. I recommend https://rustup.rs

Install **libsqlite3-dev**
Windows build requires **sqlite3.dll** in the target/{debug, release}/deps folder.

Cross compiling for Win on Linux
--------------------------------
Put **sqlite3.dll** into target/i686-pc-windows-gnu/{debug,release}/deps.
Also on Debian (and possibly others) do:

    sudo apt-get install mingw-w64-i686-dev
    sudo ln -s /usr/i686-w64-mingw32/lib/libadvapi32.a /usr/i686-w64-mingw32/lib/libAdvapi32.a

and create **.cargo/config** with rustflags only for i686

    [target.i686-pc-windows-gnu]
    linker = "i686-w64-mingw32-gcc"
    rustflags = "-C panic=abort"

