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
#### Installation (Linux):
```
https://github.com/FailC/notes.git
cd notes
cargo build --release
mv ./target/release/notes /usr/local/bin
```
It creates a hidden file `~/.notes_storage_file` after running for the first time.
You must change the code for a user-defined path yourself (suckless do it yourself mindset..)
