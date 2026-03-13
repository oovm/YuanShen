use super::*;
use crate::objects::ObjectID;
use serde::Serialize;

/// SnapShotDifference 结构体定义了快照之间的差异
/// 包括删除的项、新增的项以及修改的项。每个项都通过其对应的路径进行标识。
#[derive(PartialEq, Eq, Debug, Clone, Serialize)]
pub struct SnapShotDifference {
    /// 被删除的文件或目录路径集合
    pub deleted: BTreeSet<String>,
    /// 新增的文件或目录信息映射
    pub added: BTreeMap<String, DirectoryEntry>,
    /// 被修改的文件或目录信息映射
    pub modified: BTreeMap<String, DifferenceEntry>,
}

/// DifferenceEntry 枚举定义了差异条目的类型，可以是文件或目录。
/// 文件类型包含一个 ObjectID，目录类型包含一个嵌套的 SnapShotDifference 结构。
#[derive(PartialEq, Eq, Debug, Clone, Serialize)]
pub enum DifferenceEntry {
    /// 文件条目
    File(ObjectID),
    /// 目录条目
    Directory(Box<SnapShotDifference>),
}

/// DifferenceStackType 枚举定义了差异栈的操作类型，包括删除、添加和修改。
#[derive(Copy, Clone, Debug)]
pub enum DifferenceStackType {
    /// 被删除的路径
    Deleted,
    /// 新增的路径及条目信息
    Added,
    /// 被修改的路径及条目信息
    Modified,
}

/// DifferenceStackItem 枚举定义了差异栈的具体项，对应于不同类型的差异操作。
/// 包括被删除的路径、新增的路径及条目信息、被修改的路径及条目信息。
#[derive(Clone, Debug)]
pub enum DifferenceStackItem {
    /// 被删除的路径
    Deleted(PathBuf),
    /// 新增的路径及条目信息
    Added(PathBuf, DirectoryEntry),
    /// 被修改的路径及条目信息
    Modified(PathBuf, DifferenceEntry),
}

impl DirectoryEntry {
    pub fn difference(&self, other: &DirectoryEntry) -> Option<DifferenceEntry> {
        match (self, other) {
            (DirectoryEntry::Directory(old_dir), DirectoryEntry::Directory(new_dir)) => {
                let old_tree = SnapShotTree { root: old_dir.entries.clone() };
                let new_tree = SnapShotTree { root: new_dir.entries.clone() };
                let diff = old_tree.difference(&new_tree);
                if diff.deleted.is_empty() && diff.added.is_empty() && diff.modified.is_empty() {
                    None
                } else {
                    Some(DifferenceEntry::Directory(Box::new(diff)))
                }
            }
            (DirectoryEntry::TextStandalone(old_file), DirectoryEntry::TextStandalone(new_file)) => {
                if old_file.file_id != new_file.file_id {
                    Some(DifferenceEntry::File(new_file.file_id))
                } else {
                    None
                }
            }
            (DirectoryEntry::TextIncremental(old_file), DirectoryEntry::TextIncremental(new_file)) => {
                if old_file.data_id != new_file.data_id {
                    Some(DifferenceEntry::File(new_file.data_id))
                } else {
                    None
                }
            }
            (DirectoryEntry::Subtree(old_subtree), DirectoryEntry::Subtree(new_subtree)) => {
                if old_subtree.id != new_subtree.id {
                    Some(DifferenceEntry::File(new_subtree.id))
                } else {
                    None
                }
            }
            _ => {
                Some(DifferenceEntry::File(ObjectID::new()))
            }
        }
    }
}

impl SnapShotTree {
    /// Compute the diff between this directory structure and the one
    /// which is currently located at the path.
    pub fn difference(&self, other: &SnapShotTree) -> SnapShotDifference {
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
                other
                    .root
                    .get(file_name)
                    .and_then(|other_dir_entry| dir_entry.difference(other_dir_entry).map(|diff| (file_name.clone(), diff)))
            })
            .collect();
        SnapShotDifference { added, deleted, modified }
    }
}

impl Display for SnapShotDifference {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut entries: Vec<String> = Vec::new();

        for path in &self.added {
            entries.push(format!("A {}", path.0));
        }

        for (path, entry) in &self.modified {
            match entry {
                DifferenceEntry::File(_) => {
                    entries.push(format!("M {}", path));
                }
                DifferenceEntry::Directory(sub_diff) => {
                    for line in sub_diff.to_string().lines() {
                        if !line.is_empty() {
                            let parts: Vec<_> = line.splitn(2, ' ').collect();
                            if parts.len() == 2 {
                                entries.push(format!("{} {}/{}", parts[0], path, parts[1]));
                            }
                        }
                    }
                }
            }
        }

        for path in &self.deleted {
            entries.push(format!("D {}", path));
        }

        entries.sort_by(|a, b| {
            let a_path = &a[2..];
            let b_path = &b[2..];
            a_path.cmp(b_path)
        });

        for entry in entries {
            writeln!(f, "{}", entry)?;
        }

        Ok(())
    }
}

impl DifferenceStackType {
    /// 特征符号
    pub fn character_symbol(&self) -> char {
        match self {
            Self::Deleted => 'D',
            Self::Added => 'A',
            Self::Modified => 'M',
        }
    }
}
