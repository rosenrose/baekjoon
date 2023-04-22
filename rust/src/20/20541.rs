use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write;
use std::io;

#[derive(Debug)]
struct Album<'a> {
    name: &'a str,
    parent: Option<i32>,
    child_albums: BTreeMap<&'a str, i32>,
    photos: BTreeSet<&'a str>,
}

impl<'a> Album<'a> {
    fn from(name: &'a str, parent: Option<i32>) -> Self {
        Album {
            name,
            parent,
            child_albums: BTreeMap::new(),
            photos: BTreeSet::new(),
        }
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n: i32 = input().parse().unwrap();
    let mut albums = HashMap::with_capacity(50_000);
    let mut id = 0;
    let mut cur_id = id;

    albums.insert(cur_id, Album::from("album", None));
    id += 1;

    for (op, arg) in (0..n).map(|_| (input(), input())) {
        match op {
            "mkalb" => {
                if albums[&cur_id].child_albums.contains_key(arg) {
                    writeln!(output, "duplicated album name").unwrap();
                    continue;
                }

                let new_album = Album::from(arg, Some(cur_id));
                let new_id = id;
                id += 1;

                albums.insert(new_id, new_album);
                albums
                    .entry(cur_id)
                    .and_modify(|Album { child_albums, .. }| {
                        child_albums.insert(arg, new_id);
                    });
            }
            "rmalb" => {
                let (mut rm_albums, mut rm_photos) = (0, 0);

                match arg {
                    "-1" => {
                        if let Some((_, &first)) = albums[&cur_id].child_albums.first_key_value() {
                            (rm_albums, rm_photos) = remove_album(&mut albums, first)
                        }
                    }
                    "0" => {
                        let child_ids: Vec<_> =
                            albums[&cur_id].child_albums.values().copied().collect();

                        for child_id in child_ids {
                            let result = remove_album(&mut albums, child_id);
                            rm_albums += result.0;
                            rm_photos += result.1;
                        }
                    }
                    "1" => {
                        if let Some((_, &last)) = albums[&cur_id].child_albums.last_key_value() {
                            (rm_albums, rm_photos) = remove_album(&mut albums, last)
                        }
                    }
                    _ => {
                        if let Some(&id) = albums[&cur_id].child_albums.get(arg) {
                            (rm_albums, rm_photos) = remove_album(&mut albums, id);
                        }
                    }
                };

                writeln!(output, "{rm_albums} {rm_photos}").unwrap();
            }
            "insert" => {
                if albums[&cur_id].photos.contains(arg) {
                    writeln!(output, "duplicated photo name").unwrap();
                    continue;
                }

                albums.entry(cur_id).and_modify(|Album { photos, .. }| {
                    photos.insert(arg);
                });
            }
            "delete" => {
                let mut del_count = 0;

                match arg {
                    "-1" => {
                        if let Some(&first) = albums[&cur_id].photos.first() {
                            albums.entry(cur_id).and_modify(|Album { photos, .. }| {
                                del_count = if photos.remove(first) { 1 } else { 0 };
                            });
                        }
                    }
                    "0" => del_count = delete_photos(&mut albums, cur_id),
                    "1" => {
                        if let Some(&last) = albums[&cur_id].photos.last() {
                            albums.entry(cur_id).and_modify(|Album { photos, .. }| {
                                del_count = if photos.remove(last) { 1 } else { 0 };
                            });
                        }
                    }
                    _ => {
                        albums.entry(cur_id).and_modify(|Album { photos, .. }| {
                            del_count = if photos.remove(arg) { 1 } else { 0 };
                        });
                    }
                };

                writeln!(output, "{del_count}").unwrap();
            }
            "ca" => {
                cur_id = match arg {
                    ".." => albums[&cur_id].parent.unwrap_or(cur_id),
                    "/" => 0,
                    _ => *albums[&cur_id].child_albums.get(arg).unwrap_or(&cur_id),
                };

                writeln!(output, "{}", albums[&cur_id].name).unwrap();
            }
            _ => (),
        }
        // println!("{op} {arg}\n{}\n{albums:#?}\n", albums[&cur_id].name);
    }

    print!("{output}");
}

fn remove_album<'a>(albums: &mut HashMap<i32, Album<'a>>, id: i32) -> (usize, usize) {
    let name = albums[&id].name;
    let child_ids: Vec<_> = albums[&id].child_albums.values().copied().collect();

    let (mut album_count, mut photo_count) = (1, delete_photos(albums, id));

    for child_id in child_ids {
        let result = remove_album(albums, child_id);
        album_count += result.0;
        photo_count += result.1;
    }

    if let Some(parent) = albums[&id].parent {
        albums
            .entry(parent)
            .and_modify(|Album { child_albums, .. }| {
                child_albums.remove(name);
            });
    }

    albums.remove(&id);

    (album_count, photo_count)
}

fn delete_photos<'a>(albums: &mut HashMap<i32, Album>, id: i32) -> usize {
    let mut photo_count = 0;

    albums.entry(id).and_modify(|Album { photos, .. }| {
        photo_count = photos.len();
        photos.clear();
    });

    photo_count
}
