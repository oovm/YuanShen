use super::*;

/// SnapShotDifference 结构体定义了快照之间的差异
/// 包括删除的项、新增的项以及修改的项。每个项都通过其对应的路径进行标识。
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
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
#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
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
        use crate::snapshot::directory::DirectoryEntry::*;
        match (self, other) {
            (File(id), File(id_)) => {
                if id != id_ {
                    Some(DifferenceEntry::File(*id_))
                }
                else {
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
                }
                else {
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
                        }
                        else {
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
            writeln!(f, "{}", diff_item.character_symbol())?;
            path.to_str().unwrap();
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
