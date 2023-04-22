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

#[derive(Debug)]
struct AlbumManager<'a> {
    albums: HashMap<i32, Album<'a>>,
    id: i32,
    counter: i32,
}

impl<'a> AlbumManager<'a> {
    fn new() -> Self {
        let mut albums = HashMap::with_capacity(50_000);
        albums.insert(0, Album::from("album", None));

        Self {
            albums,
            id: 0,
            counter: 1,
        }
    }

    fn make_album(&mut self, name: &'a str) -> bool {
        if self.albums[&self.id].child_albums.contains_key(name) {
            return false;
        }

        let new_album = Album::from(name, Some(self.id));
        let new_id = self.counter;
        self.counter += 1;

        self.albums.insert(new_id, new_album);
        self.albums
            .entry(self.id)
            .and_modify(|Album { child_albums, .. }| {
                child_albums.insert(name, new_id);
            });

        true
    }

    fn remove_album(&mut self, name: &'a str) -> (usize, usize) {
        let Some(&remove_id) = self.albums[&self.id].child_albums.get(name) else {
            return (0, 0);
        };

        self.albums
            .entry(self.id)
            .and_modify(|Album { child_albums, .. }| {
                child_albums.remove(name);
            });

        self.remove_album_by_id(remove_id)
    }
    fn remove_album_by_id(&mut self, remove_id: i32) -> (usize, usize) {
        let mut album_count = 1;
        let mut photo_count = self.albums[&remove_id].photos.len();

        let child_ids: Vec<_> = self.albums[&remove_id]
            .child_albums
            .values()
            .copied()
            .collect();

        for child_id in child_ids {
            let result = self.remove_album_by_id(child_id);
            album_count += result.0;
            photo_count += result.1;
        }

        self.albums.remove(&remove_id);

        (album_count, photo_count)
    }
    fn remove_album_first(&mut self) -> (usize, usize) {
        if let Some((first, _)) = self.albums[&self.id].child_albums.first_key_value() {
            self.remove_album(first)
        } else {
            (0, 0)
        }
    }
    fn remove_album_last(&mut self) -> (usize, usize) {
        if let Some((last, _)) = self.albums[&self.id].child_albums.last_key_value() {
            self.remove_album(last)
        } else {
            (0, 0)
        }
    }
    fn remove_album_all(&mut self) -> (usize, usize) {
        let (mut album_count, mut photo_count) = (0, 0);
        let child_names: Vec<_> = self.albums[&self.id].child_albums.keys().copied().collect();

        for child_name in child_names {
            let result = self.remove_album(child_name);
            album_count += result.0;
            photo_count += result.1;
        }

        (album_count, photo_count)
    }

    fn insert_photo(&mut self, name: &'a str) -> bool {
        if self.albums[&self.id].photos.contains(name) {
            return false;
        }

        self.albums
            .entry(self.id)
            .and_modify(|Album { photos, .. }| {
                photos.insert(name);
            });

        true
    }

    fn delete_photo(&mut self, name: &'a str) -> usize {
        let mut del_count = 0;

        self.albums
            .entry(self.id)
            .and_modify(|Album { photos, .. }| {
                if photos.remove(name) {
                    del_count = 1;
                }
            });

        del_count
    }
    fn delete_photo_first(&mut self) -> usize {
        let Some(&first) = self.albums[&self.id].photos.first() else {
            return 0;
        };

        self.albums
            .entry(self.id)
            .and_modify(|Album { photos, .. }| {
                photos.remove(first);
            });

        1
    }
    fn delete_photo_last(&mut self) -> usize {
        let Some(&last) = self.albums[&self.id].photos.last() else {
            return 0;
        };

        self.albums
            .entry(self.id)
            .and_modify(|Album { photos, .. }| {
                photos.remove(last);
            });

        1
    }
    fn delete_photo_all(&mut self) -> usize {
        let mut del_count = 0;

        self.albums
            .entry(self.id)
            .and_modify(|Album { photos, .. }| {
                del_count = photos.len();
                photos.clear();
            });

        del_count
    }

    fn change_album(&mut self, name: &'a str) -> &str {
        self.id = match name {
            ".." => self.albums[&self.id].parent.unwrap_or(self.id),
            "/" => 0,
            _ => *self.albums[&self.id]
                .child_albums
                .get(name)
                .unwrap_or(&self.id),
        };

        self.albums[&self.id].name
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n: i32 = input().parse().unwrap();
    let mut album_manager = AlbumManager::new();

    for (op, arg) in (0..n).map(|_| (input(), input())) {
        match op {
            "mkalb" => {
                if !album_manager.make_album(arg) {
                    writeln!(output, "duplicated album name").unwrap();
                }
            }
            "rmalb" => {
                let (rm_albums, rm_photos) = match arg {
                    "-1" => album_manager.remove_album_first(),
                    "0" => album_manager.remove_album_all(),
                    "1" => album_manager.remove_album_last(),
                    _ => album_manager.remove_album(arg),
                };

                writeln!(output, "{rm_albums} {rm_photos}").unwrap();
            }
            "insert" => {
                if !album_manager.insert_photo(arg) {
                    writeln!(output, "duplicated photo name").unwrap();
                }
            }
            "delete" => {
                let del_count = match arg {
                    "-1" => album_manager.delete_photo_first(),
                    "0" => album_manager.delete_photo_all(),
                    "1" => album_manager.delete_photo_last(),
                    _ => album_manager.delete_photo(arg),
                };

                writeln!(output, "{del_count}").unwrap();
            }
            "ca" => writeln!(output, "{}", album_manager.change_album(arg)).unwrap(),
            _ => (),
        }
        // println!("{op} {arg}\n{album_manager:#?}\n");
    }

    print!("{output}");
}
