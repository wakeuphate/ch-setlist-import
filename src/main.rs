extern crate globwalk;
use std::fs;
use configparser::ini::Ini;
use std::io::{self, Read};
use clap::Parser;
use encoding_rs::UTF_8;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::time::Duration;
use std::error::Error;
use std::collections::HashMap;

// TODO: Accept & parse an actual .setlist file
// TODO: Add some tests

/// A small utility to list your favourite songs from a (somewhat already parsed) setlist
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
  /// Path to your charts directory
  #[clap(short, long)]
  charts_path: String,

  /// Path to the \n delimited MD5 hashes from the setlist
  #[clap(short, long)]
  setlist: String,
}

struct Song {
  artist: String,
  name: String,
  charter: String,
  hash: String
}

impl Song {
  fn from_ini(ini: Ini, hash: String) -> Song {
    let artist = ini.get("song", "artist").unwrap();
    let name = ini.get("song", "name").unwrap();
    let charter = ini.get("song", "charter").unwrap_or("Unknown Charter".to_string());
    let hash = hash.to_uppercase();
    Song {
      artist,
      name,
      charter,
      hash
    }
  }
}

fn main() {
    let args  = Args::parse();
    let charts_path = args.charts_path;
    let setlist = args.setlist;
    let chart_list: Vec<Song> = process_charts(&charts_path);
    let setlist = fs::read_to_string(setlist).expect("Something went wrong reading the file");
    let mut song_map: HashMap<&str, Song> = HashMap::new();

    for song in chart_list {
      song_map.insert(&song.hash, song);
    }

    println!("================");
    println!("Your Favourites:");
    println!("================");
    let mut found_count = 0;
    for fav_md5 in setlist.lines() {
      if let Some(song) = song_map.get(fav_md5) {
        found_count += 1;
        println!("{}: {} - {} (Charted by {})", found_count, song.artist, song.name, song.charter);
    }
    println!("================");
    println!("{} song(s) found", found_count);
    println!("================");
}

fn process_charts(charts_path: &str) -> Vec<Song> {
  let mut chart_list: Vec<Song> = Vec::new();

  let walker = globwalk::GlobWalkerBuilder::from_patterns(
      charts_path,
      &["**/notes.{mid,chart}"]
    )
    .follow_links(true)
    .build()
    .unwrap();
  let charts = walker.filter_map(|result| result.ok());

  let spinner = ProgressBar::new_spinner();
  spinner.enable_steady_tick(Duration::from_millis(100));
  spinner.set_style(
    ProgressStyle::default_spinner()
      .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈⠈")
  );

  for chart in charts {
    let chart_path = chart.path();
    let song_ini_path = chart.path().parent().unwrap().display().to_string() + "/song.ini";

    let song_ini = match parse_ini(&song_ini_path) {
      Ok(ini) => ini,
      Err(e) => {
          eprintln!("Error parsing INI file: {}", e);
          continue; // Skip to the next chart
      }
    };

    let chart_hash = match calculate_md5_hash(chart_path.to_str().unwrap()) {
      Ok(hash) => hash,
      Err(e) => panic!("Error calculating MD5 hash: {}", e),
    };

    let song = Song::from_ini(song_ini, chart_hash);
    spinner.set_message(format!("Processing: {} - {} (Charter: {})", song.artist, song.name, song.charter));
    spinner.inc(1);
    chart_list.push(song);
  }
  chart_list
}

fn parse_ini(file: &str) -> Result<Ini, Box<dyn Error>> {
  let content_bytes = fs::read(file).expect("Something went wrong reading the file");
  let (contents, _, had_errors) = UTF_8.decode(&content_bytes);

  if had_errors {
    println!("Cannot Parse File - Non UTF-8 sequence detected: {}", file);
  }

  let mut ini = Ini::new();
  match ini.read(contents.to_string()).map_err(|e| e.to_string()) {
    Ok(_) => Ok(ini),
    Err(e) => Err(format!("Error parsing ini file @ {}: {}", file, e).into()),
  }
}

fn calculate_md5_hash(file_path: &str) -> io::Result<String> {
  let mut file = std::fs::File::open(file_path)?;
  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer)?;
  let hash = md5::compute(&buffer);
  Ok(format!("{:x}", hash))
}
