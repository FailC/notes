### ```notes``` allowes you to quickly save ideas or notes directly in your comfy terminal.
No more `cd`-ing into a random directory and opening a file with nvim!

#### Commands:
- `n <text>`: Create a new note
- `l`: List notes
- `d`: Select note to delete

#### Example:
```bash
notes n push notes app
```
Now you can list your notes
```bash
notes l
push notes app
```
if your shell (zsh, maybe others) has problems with special characters just wrap your note in quotation marks: 
```bash
notes n "pacman -Syu today?!"
```
#### Installation:
Rust needs to be installed -> see rust-lang.org

Linux:
```
git clone https://github.com/FailC/notes.git
cd notes
cargo build --release
mv ./target/release/notes /usr/local/bin
```
Windows:
```bash
git clone https://github.com/FailC/notes.git
cd notes
cargo build --release
move ".\target\release\notes.exe" "C:\any_folder_that_is_added_to_path"
```

It creates a hidden file in the home directory
`~/.notes_storage_file` 
after running for the first time.
You must change the code for a user-defined path (suckless do it yourself mindset..)





