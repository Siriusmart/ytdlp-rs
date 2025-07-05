
# ytdlp-rs

The thinnest yt-dlp wrapper. Contains methods for all command options.

## Getting started

1. Add to dependency
```toml
[dependencies]
ytdlp-rs = "0.1.0"
```
2. To run a command, use spawn.
```rs
Builder::("/path/to/yt-dlp")
    .command_option_1()
    .command_option_2()
    .url(url)
    .spawn()?
    .wait()?;
```
3. Parse and collect command output.
```rs
let (stdout, stderr) = Builder::new("yt-dlp")
    .extract_audio()
    .format("123")
    .run()?;

let collected = ParserBuilder::new()
    .map(|s: String| serde_json::from_str(&s).unwrap())
    .read_collect(stdout);
```
