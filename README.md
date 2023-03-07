# Anime Repository

A lightweight scraper,is designed to run in the background to automatically scrape specified directory files.

I am not very proficient in drawing, if you are willing to design a logo for it, you can contact me, thank you very much.

This is the first program I wrote in Rust and Typescript, it is my graduation design.


## Feature

- Customizable multiple rule matching files
- Compatible with BT for seeding
- Intelligent matching using OpenAI

## UI

![UI](https://raw.githubusercontent.com/Chikage0o0/Anime-Repository/master/img/ui.png)


### About Subscribe Rules

**Subscribe rules have higher priority than OpenAI recognition**

For example,The File is 
```
Downloads/AnimeRepository/[Lilith-Raws] Isekai Nonbiri Nouka - 03 [Baha][WEB-DL][1080p][AVC AAC][CHT][MP4].mp4
```

Tvshow Regex Can be `[Lilith-Raws] Isekai Nonbiri Nouka`,Just be part of the file path.
 
The Episode Regex will remain the default in most cases, you only need to change the Position and Offset.

Position is the position of the Episode in the file name matched by Regex.

For example
```
// Episode Regex = \d+ This means that only numbers are matched

[Lilith-Raws] Isekai Nonbiri Nouka - 03 [Baha][WEB-DL][1080p][AVC AAC][CHT][MP4].mp4

Position 1=03
Position 2=1080
Position 3=4
```

For the rare cases where the Episode of the file name does not match the TMDB, Offset can be used to correct it.

Here is an example rule

![Rule](https://raw.githubusercontent.com/Chikage0o0/Anime-Repository/master/img/rule.png)

### About OpenAI

**OpenAI will not match files in subfolders.**

All you need to do is fill in the OpenAI API KEY and turn on the switch, and the rest is all automatic.

### About Config File AND LOG

Windows: `%appdata%/AnimeRepository`



## Develop

Read [https://tauri.app/v1/guides/getting-started/prerequisites/](https://tauri.app/v1/guides/getting-started/prerequisites/)

```bash
git clone https://github.com/Chikage0o0/Anime-Repository
cd Anime-Repository
npm install
code .
//TMDB_KEY is v4
export TMDB_KEY="xxxxxxxxx" or $Env:TMDB_KEY="xxxxxxxxx"
yarn tauri dev 
```


## Thanks for

[Tauri](https://tauri.app/) Without it there would be no such software.

[Mantine](https://mantine.dev/) This is the best UI framework I have ever used.

[Clash-Verge](https://github.com/zzzgydi/clash-verge) I learned a lot from the source code.

[TMDB API](https://developers.themoviedb.org/) Thanks to TMDB for providing metadata and API.