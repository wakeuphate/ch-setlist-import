extern crate globwalk;
use std::fs;
use configparser::ini::Ini;
use std::io::{self, Read};
use clap::Parser;

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
    let charter = ini.get("song", "charter").unwrap();
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

    println!("Your Favourites:");
    for (idx, fav_md5) in setlist.lines().enumerate() {
      for song in &chart_list {
        if song.hash == fav_md5 {
          println!("{}: {} - {} (Charted by {})", idx+1, song.artist, song.name, song.charter);
        }
      }
    }
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

  for chart in charts {
    let chart_path = chart.path();
    let song_ini_path = chart.path().parent().unwrap().display().to_string() + "/song.ini";
    let song_ini = parse_ini(&song_ini_path);
    let chart_hash = calculate_md5_hash(chart_path.to_str().unwrap());

    match chart_hash {
      Ok(hash) => chart_list.push(Song::from_ini(song_ini, hash)),
      Err(e) => panic!("Error calculating md5 hash: {}", e),
    }
  }
  chart_list
}

fn parse_ini(file: &str) -> Ini {
  let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
  let mut ini = Ini::new();
  ini.read(contents).unwrap();
  ini
}

fn calculate_md5_hash(file_path: &str) -> io::Result<String> {
  let mut file = std::fs::File::open(file_path)?;
  let mut buffer = Vec::new();
  file.read_to_end(&mut buffer)?;
  let hash = md5::compute(&buffer);
  Ok(format!("{:x}", hash))
}