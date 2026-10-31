//! Model-facing view of the pre-compaction transcript.
//!
//! Compaction already persists the complete history before replacing it, in
//! `artifacts/context-transfer-<checkpoint>.json`. This module turns those
//! snapshots into addressable entries: ids derive from role and rendered text,
//! so a message that survives several compactions keeps one id and is offered
//! once.
//!
//! Known limits: thinking blocks are not archived; only the most recent
//! [`MAX_LOAD_BYTES`] of snapshots are loaded; `seq` is the message index in the
//! snapshot it first appeared in, not a global ordinal; every tool call reloads
//! and re-parses the snapshots, so paging through one large entry repeats that
//! work.

use std::io::{self, Read};
use std::path::Path;
use std::time::UNIX_EPOCH;

use codewhale_models::{ContentBlock, Message};
use sha2::{Digest, Sha256};

pub const TRANSFER_DIR_NAME: &str = "artifacts";
const TRANSFER_PREFIX: &str = "context-transfer-";
const TRANSFER_SUFFIX: &str = ".json";
const ID_HEX_LEN: usize = 12;
const MAX_LOAD_BYTES: u64 = 64 * 1024 * 1024;

/// One addressable message from a pre-compaction snapshot.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArchiveEntry {
    pub id: String,
    pub seq: usize,
    pub role: String,
    pub at: u64,
    pub text: String,
}

/// What a load covered, so the tools can say what they could and could not see.
#[derive(Debug, Clone, Default)]
pub struct ArchiveLoad {
    pub entries: Vec<ArchiveEntry>,
    pub snapshots: usize,
    pub skipped_snapshots: usize,
    pub truncation_note: Option<String>,
}

/// Render one message into the plain text an archive entry stores.
#[must_use]
pub fn render_message(message: &Message) -> String {
    let mut parts: Vec<String> = Vec::new();
    for block in &message.content {
        match block {
            ContentBlock::Text { text, .. } => {
                if !text.trim().is_empty() {
                    parts.push(text.clone());
                }
            }
            ContentBlock::ToolUse { name, input, .. } => {
                parts.push(format!("[tool_use {name}] {input}"));
            }
            ContentBlock::ServerToolUse { name, input, .. } => {
                parts.push(format!("[server_tool_use {name}] {input}"));
            }
            ContentBlock::ToolResult {
                tool_use_id,
                content,
                is_error,
                ..
            } => {
                let label = if is_error.unwrap_or(false) {
                    "[tool_result error]"
                } else {
                    "[tool_result]"
                };
                parts.push(format!("{label} {tool_use_id}\n{content}"));
            }
            ContentBlock::ToolSearchToolResult {
                tool_use_id,
                content,
            } => {
                parts.push(format!(
                    "[tool_search_tool_result] {tool_use_id}\n{content}"
                ));
            }
            ContentBlock::CodeExecutionToolResult {
                tool_use_id,
                content,
            } => {
                parts.push(format!(
                    "[code_execution_tool_result] {tool_use_id}\n{content}"
                ));
            }
            _ => {}
        }
    }
    parts.join("\n\n")
}

/// The content-derived id a message is addressed by.
#[must_use]
pub fn entry_id(role: &str, text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(role.as_bytes());
    hasher.update([0u8]);
    hasher.update(text.as_bytes());
    let digest = hasher.finalize();
    let mut id = String::with_capacity(ID_HEX_LEN);
    for byte in &digest[..ID_HEX_LEN / 2] {
        id.push_str(&format!("{byte:02x}"));
    }
    id
}

fn is_transfer_snapshot(name: &str) -> bool {
    name.starts_with(TRANSFER_PREFIX)
        && name.ends_with(TRANSFER_SUFFIX)
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
}

fn snapshot_dir(session_id: &str) -> Option<std::path::PathBuf> {
    codewhale_runtime::artifacts::artifact_sessions_root()
        .map(|root| root.join(session_id).join(TRANSFER_DIR_NAME))
}

struct Snapshot {
    name: String,
    bytes: u64,
    modified: u64,
}

fn list_snapshots(session_id: &str) -> io::Result<Vec<Snapshot>> {
    let Some(dir) = snapshot_dir(session_id) else {
        return Ok(Vec::new());
    };
    let read = match std::fs::read_dir(&dir) {
        Ok(read) => read,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error),
    };
    let mut snapshots: Vec<Snapshot> = read
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().to_string();
            if !is_transfer_snapshot(&name) || !entry.file_type().ok()?.is_file() {
                return None;
            }
            let metadata = entry.metadata().ok()?;
            let modified = metadata
                .modified()
                .ok()
                .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
                .map(|elapsed| elapsed.as_secs())
                .unwrap_or_default();
            Some(Snapshot {
                name,
                bytes: metadata.len(),
                modified,
            })
        })
        .collect();
    snapshots.sort_by_key(|snapshot| (snapshot.modified, snapshot.name.clone()));
    Ok(snapshots)
}

fn read_snapshot(session_id: &str, name: &str) -> io::Result<Vec<Message>> {
    let relative = Path::new(TRANSFER_DIR_NAME).join(name);
    let file = codewhale_runtime::artifacts::open_session_relative(session_id, &relative, false)?;
    let mut body = Vec::new();
    file.open_file()?.read_to_end(&mut body)?;
    serde_json::from_slice::<Vec<Message>>(&body).map_err(io::Error::other)
}

/// Load every reachable snapshot for this session, oldest first.
pub fn load(session_id: &str) -> io::Result<ArchiveLoad> {
    let snapshots = list_snapshots(session_id)?;
    let mut budget = MAX_LOAD_BYTES;
    let mut selected: Vec<&Snapshot> = Vec::new();
    let mut skipped = 0usize;
    let mut budget_skipped = 0usize;
    let mut unreadable = 0usize;
    for snapshot in snapshots.iter().rev() {
        if snapshot.bytes > budget {
            skipped += 1;
            budget_skipped += 1;
            continue;
        }
        budget -= snapshot.bytes;
        selected.push(snapshot);
    }
    selected.reverse();

    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut entries: Vec<ArchiveEntry> = Vec::new();
    let mut first_error: Option<String> = None;
    for snapshot in &selected {
        let messages = match read_snapshot(session_id, &snapshot.name) {
            Ok(messages) => messages,
            Err(error) => {
                skipped += 1;
                unreadable += 1;
                if first_error.is_none() {
                    first_error = Some(format!("{}: {error}", snapshot.name));
                }
                continue;
            }
        };
        for (seq, message) in messages.iter().enumerate() {
            let role = message.role.as_str().to_ascii_lowercase();
            let text = render_message(message);
            if text.trim().is_empty() {
                continue;
            }
            let id = entry_id(&role, &text);
            if !seen.insert(id.clone()) {
                continue;
            }
            entries.push(ArchiveEntry {
                id,
                seq,
                role,
                at: snapshot.modified,
                text,
            });
        }
    }
    entries.sort_by_key(|entry| (entry.at, entry.seq));
    let mut notes: Vec<String> = Vec::new();
    if budget_skipped > 0 {
        notes.push(format!(
            "{budget_skipped} older pre-compaction snapshots exceeded the {MAX_LOAD_BYTES}-byte load budget and were skipped"
        ));
    }
    if unreadable > 0 {
        notes.push(match &first_error {
            Some(error) => format!("{unreadable} snapshots could not be decoded (first: {error})"),
            None => format!("{unreadable} snapshots could not be decoded"),
        });
    }
    Ok(ArchiveLoad {
        entries,
        snapshots: selected.len(),
        skipped_snapshots: skipped,
        truncation_note: (!notes.is_empty()).then(|| notes.join("; ")),
    })
}

/// Resolve an id, or a unique id prefix, to one archived message.
pub fn find_entry<'a>(
    entries: &'a [ArchiveEntry],
    reference: &str,
) -> Result<&'a ArchiveEntry, String> {
    let reference = reference
        .trim()
        .trim_start_matches('#')
        .to_ascii_lowercase();
    if reference.is_empty() {
        return Err("an archive id is required".to_string());
    }
    if let Some(entry) = entries.iter().find(|entry| entry.id == reference) {
        return Ok(entry);
    }
    let matches: Vec<&ArchiveEntry> = entries
        .iter()
        .filter(|entry| entry.id.starts_with(&reference))
        .collect();
    match matches.as_slice() {
        [] => Err(format!(
            "no archived message matches `{reference}`; use context_archive_search to find one"
        )),
        [only] => Ok(*only),
        several => Err(format!(
            "`{reference}` is ambiguous: {} archived messages start with it; use more characters",
            several.len()
        )),
    }
}

/// Case-insensitive substring search across archived messages.
#[must_use]
pub fn search_entries<'a>(
    entries: &'a [ArchiveEntry],
    query: &str,
    limit: usize,
) -> (Vec<(&'a ArchiveEntry, String)>, usize) {
    let needle = query.trim().to_ascii_lowercase();
    if needle.is_empty() {
        return (Vec::new(), 0);
    }
    let mut matches = Vec::new();
    let mut total = 0usize;
    for entry in entries {
        let haystack = entry.text.to_ascii_lowercase();
        let Some(position) = haystack.find(&needle) else {
            continue;
        };
        total += 1;
        if matches.len() < limit {
            matches.push((entry, snippet(&entry.text, position, needle.len())));
        }
    }
    (matches, total)
}

const SNIPPET_RADIUS_BYTES: usize = 120;

fn floor_char_boundary(text: &str, mut index: usize) -> usize {
    while index > 0 && !text.is_char_boundary(index) {
        index -= 1;
    }
    index
}

fn ceil_char_boundary(text: &str, mut index: usize) -> usize {
    while index < text.len() && !text.is_char_boundary(index) {
        index += 1;
    }
    index
}

/// A short window around one hit, marked so the model can see where it landed.
fn snippet(text: &str, byte_position: usize, needle_len: usize) -> String {
    let start = floor_char_boundary(text, byte_position.saturating_sub(SNIPPET_RADIUS_BYTES));
    let hit_end = ceil_char_boundary(text, (byte_position + needle_len).min(text.len()));
    let end = ceil_char_boundary(text, (hit_end + SNIPPET_RADIUS_BYTES).min(text.len()));
    let mut out = String::new();
    if start > 0 {
        out.push('…');
    }
    out.push_str(&text[start..byte_position]);
    out.push_str("<<");
    out.push_str(&text[byte_position..hit_end]);
    out.push_str(">>");
    out.push_str(&text[hit_end..end]);
    if end < text.len() {
        out.push('…');
    }
    out
}

/// Slice `text` by character offset, returning the slice and its total char count.
#[must_use]
pub fn slice_chars(text: &str, offset: usize, max_chars: usize) -> (String, usize) {
    let total = text.chars().count();
    let slice: String = text.chars().skip(offset).take(max_chars).collect();
    (slice, total)
}
