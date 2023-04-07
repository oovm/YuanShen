use std::fmt::Formatter;
use super::*;

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct SnapShotDifference {
    pub deleted: BTreeSet<String>,
    pub added: BTreeMap<String, DirectoryEntry>,
    pub modified: BTreeMap<String, DifferenceEntry>,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum DifferenceEntry {
    File(ObjectID),
    Directory(Box<SnapShotDifference>),
}

enum DifferenceStackType {
    Deleted,
    Added,
    Modified,
}

enum DifferenceStackItem {
    Deleted(PathBuf),
    Added(PathBuf, DirectoryEntry),
    Modified(PathBuf, DifferenceEntry),
}


impl DirectoryEntry {
    pub fn difference(&self, other: &DirectoryEntry) -> Option<DifferenceEntry> {
        use crate::snapshot::directory::DirectoryEntry::*;
        match (self, other) {
            (File(id), File(id_)) => {
                if id != id_ {
                    Some(DifferenceEntry::File(*id_))
                } else {
                    None
                }
            }
            (Directory(_), File(id)) => Some(DifferenceEntry::File(*id)),
            (File(_), Directory(d)) => Some(DifferenceEntry::Directory(Box::new(SnapShotDifference {
                deleted: BTreeSet::new(),
                added: d.root.clone(),
                modified: BTreeMap::new(),
            }))),
            (Directory(d), Directory(d_)) => {
                if d == d_ {
                    None
                } else {
                    Some(DifferenceEntry::Directory(Box::new(d.difference(d_))))
                }
            }
        }
    }
}

impl SnapShotDirectory {
    /// Compute the diff between this directory structure and the one
    /// which is currently located at the path.
    pub fn difference(&self, other: &SnapShotDirectory) -> SnapShotDifference {
        let added: BTreeMap<String, DirectoryEntry> = other
            .root
            .iter()
            .filter(|(file_name, _dir_entry)| !self.root.contains_key(*file_name))
            .map(|(fname, dir_entry)| (fname.clone(), dir_entry.clone()))
            .collect();
        let deleted: BTreeSet<String> = self
            .root
            .iter()
            .filter(|(file_name, _dir_entry)| !other.root.contains_key(*file_name))
            .map(|(fname, _dir_entry)| fname.clone())
            .collect();
        let modified: BTreeMap<String, DifferenceEntry> = self
            .root
            .iter()
            .filter_map(|(file_name, dir_entry)| {
                other.root.get(file_name).and_then(|other_dir_entry| {
                    dir_entry
                        .difference(other_dir_entry)
                        .map(|diff| (file_name.clone(), diff))
                })
            })
            .collect();
        SnapShotDifference {
            added,
            deleted,
            modified,
        }
    }
}

impl Display for SnapShotDifference {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut stack: Vec<DifferenceStackItem> = vec![];

        for (path, dir_entry) in self.added.clone() {
            stack.push(DifferenceStackItem::Added(PathBuf::from(path), dir_entry));
        }
        for (path, diff_entry) in self.modified.clone() {
            stack.push(DifferenceStackItem::Modified(PathBuf::from(path), diff_entry));
        }
        for path in self.deleted.clone() {
            stack.push(DifferenceStackItem::Deleted(PathBuf::from(path)));
        }


        let mut diff_paths: BTreeMap<PathBuf, DifferenceStackType> = BTreeMap::new();

        while let Some(diff_stack_item) = stack.pop() {
            match diff_stack_item {
                DifferenceStackItem::Deleted(path) => {
                    diff_paths.insert(path, DifferenceStackType::Deleted);
                }
                DifferenceStackItem::Added(path, dir_entry) => match dir_entry {
                    DirectoryEntry::File(_) => {
                        diff_paths.insert(path, DifferenceStackType::Added);
                    }
                    DirectoryEntry::Directory(dir) => {
                        if dir.root.is_empty() {
                            diff_paths.insert(path, DifferenceStackType::Added);
                        } else {
                            for (dir_name, dir_entry) in dir.root.clone() {
                                stack.push(DifferenceStackItem::Added(path.join(dir_name), dir_entry));
                            }
                        }
                    }
                },
                DifferenceStackItem::Modified(path, diff_entry) => match diff_entry {
                    DifferenceEntry::File(_) => {
                        diff_paths.insert(path, DifferenceStackType::Modified);
                    }
                    DifferenceEntry::Directory(diff) => {
                        for (dir_name, dir_entry) in diff.added.clone() {
                            stack.push(DifferenceStackItem::Added(path.join(dir_name), dir_entry))
                        }
                        for (dir_name, diff_entry) in diff.modified.clone() {
                            stack.push(DifferenceStackItem::Modified(path.join(dir_name), diff_entry))
                        }
                        for dir_name in diff.deleted.clone() {
                            stack.push(DifferenceStackItem::Deleted(path.join(dir_name)))
                        }
                    }
                },
            }
        }

        for (path, diff_item) in diff_paths {
            match diff_item {
                DifferenceStackType::Deleted => writeln!(f, "D {}", path.to_str().unwrap())?,
                DifferenceStackType::Added => writeln!(f, "A {}", path.to_str().unwrap())?,
                DifferenceStackType::Modified => writeln!(f, "M {}", path.to_str().unwrap())?,
            }
        }
        Ok(())
    }
}
