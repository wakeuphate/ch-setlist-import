# Clone Hero Setlist Exporter

This tool is designed for one small reason. [ScoreSpy](https://clonehero.scorespy.online/) is currently in the process of shutting down and I wanted to ensure I had all of my "ScoreSpy Favourites" backed up. When booting ScoreSpy's version of CH, I still get these favourites listed, but I assume that when the API is finally killed, these will no-longer be accessible. At this moment in time, my favourites are also not accessible via the ScoreSpy website, so I wanted to ensure I had a backup.

I exported all of my ScoreSpy favourites to a Clone Hero Setlist, found the file & did a bit of searching for the format. After finding larsjuhw's [Clone Hero Setlist Tool](https://github.com/larsjuhw/clonehero-setlist-tool) I decided it would be a nice learning project to rewrite the part of their tool that I wanted to use in Rust. This is the result.

> [!IMPORTANT]
> This currently does not support `.setlist` files. In time perhaps, but for now I'd already parsed through the file myself and pulled the MD5 hashes out of it into a \n separated list. i.e.:
> ```
> DB68C753B6F25AEF84B638FC0EAF17EF
> 84BCFD65596E986D50FBB3B8C0B0A95F
> CB55F5449B80047372C370ABA2E08D89
> EE04A4BD4A614470644C806210DC9895
> 8BEFA2EDAD01547E2087D9CC899FE7B8
> ```

## Usage
Build it with `cargo build --release` and then run it with `./target/release/ch-setlist-export -c <path-to-ch-charts> -s <path-to-md5-list>`. The output will be printed in the terminal.

```bash
Usage: ch-setlist-export --charts-path <charts-path> --setlist <md5-list>

Options:
  -c, --charts-path <CHARTS_PATH>  Path to your charts directory
  -s, --setlist <SETLIST>          Path to the \n delimited MD5 hashes from the setlist
  -h, --help                       Print help
  -V, --version                    Print version
```

## Disclaimer
This is a very simple tool that I wrote for my own personal use. It is not perfect and I make no guarantees that it will work for you. I am not responsible for any damage that may occur from using this tool. Use at your own risk.