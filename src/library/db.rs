use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::Result;
use rusqlite::{params, Connection};

use super::models::{Album, Artist, Playlist, Track};

#[allow(dead_code)]
pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open(path: &std::path::Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let db = Database { conn };
        db.migrate()?;
        Ok(db)
    }

    fn migrate(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS artists (
                id   INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE
            );

            CREATE TABLE IF NOT EXISTS albums (
                id         INTEGER PRIMARY KEY,
                title      TEXT NOT NULL,
                artist_id  INTEGER REFERENCES artists(id),
                year       INTEGER,
                cover_data BLOB
            );

            CREATE TABLE IF NOT EXISTS tracks (
                id           INTEGER PRIMARY KEY,
                path         TEXT NOT NULL UNIQUE,
                title        TEXT NOT NULL,
                artist_id    INTEGER REFERENCES artists(id),
                album_id     INTEGER REFERENCES albums(id),
                track_number INTEGER,
                duration_ms  INTEGER NOT NULL DEFAULT 0,
                mtime        INTEGER NOT NULL DEFAULT 0,
                cover_data   BLOB
            );

            CREATE TABLE IF NOT EXISTS playlists (
                id   INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE
            );

            CREATE TABLE IF NOT EXISTS playlist_tracks (
                playlist_id INTEGER REFERENCES playlists(id) ON DELETE CASCADE,
                track_id    INTEGER REFERENCES tracks(id)    ON DELETE CASCADE,
                position    INTEGER NOT NULL,
                PRIMARY KEY (playlist_id, track_id)
            );

            CREATE INDEX IF NOT EXISTS idx_tracks_album   ON tracks(album_id);
            CREATE INDEX IF NOT EXISTS idx_tracks_artist  ON tracks(artist_id);
            CREATE INDEX IF NOT EXISTS idx_albums_artist  ON albums(artist_id);
            ",
        )?;
        Ok(())
    }

    // ── Artists ──────────────────────────────────────────────────────────────

    pub fn upsert_artist(&self, name: &str) -> Result<i64> {
        self.conn.execute(
            "INSERT OR IGNORE INTO artists (name) VALUES (?1)",
            params![name],
        )?;
        Ok(self.conn.query_row(
            "SELECT id FROM artists WHERE name = ?1",
            params![name],
            |r| r.get(0),
        )?)
    }

    pub fn all_artists(&self) -> Result<Vec<Artist>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name FROM artists ORDER BY name COLLATE NOCASE",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(Artist { id: r.get(0)?, name: r.get(1)? })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    // ── Albums ───────────────────────────────────────────────────────────────

    pub fn upsert_album(&self, title: &str, artist_id: i64, year: Option<u32>, cover: Option<&[u8]>) -> Result<i64> {
        self.conn.execute(
            "INSERT OR IGNORE INTO albums (title, artist_id, year, cover_data)
             VALUES (?1, ?2, ?3, ?4)",
            params![title, artist_id, year, cover],
        )?;
        Ok(self.conn.query_row(
            "SELECT id FROM albums WHERE title = ?1 AND artist_id = ?2",
            params![title, artist_id],
            |r| r.get(0),
        )?)
    }

    pub fn albums_by_artist(&self, artist_id: i64) -> Result<Vec<Album>> {
        let mut stmt = self.conn.prepare(
            "SELECT al.id, al.title, ar.name, al.year, al.cover_data
             FROM albums al
             JOIN artists ar ON ar.id = al.artist_id
             WHERE al.artist_id = ?1
             ORDER BY al.year, al.title COLLATE NOCASE",
        )?;
        let rows = stmt.query_map(params![artist_id], |r| {
            Ok(Album {
                id: r.get(0)?,
                title: r.get(1)?,
                artist: r.get(2)?,
                year: r.get(3)?,
                cover_data: r.get(4)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    pub fn all_albums(&self) -> Result<Vec<Album>> {
        let mut stmt = self.conn.prepare(
            "SELECT al.id, al.title, ar.name, al.year, al.cover_data
             FROM albums al
             JOIN artists ar ON ar.id = al.artist_id
             ORDER BY al.title COLLATE NOCASE",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(Album {
                id: r.get(0)?,
                title: r.get(1)?,
                artist: r.get(2)?,
                year: r.get(3)?,
                cover_data: r.get(4)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    // ── Tracks ───────────────────────────────────────────────────────────────

    pub fn upsert_track(
        &self,
        path: &str,
        title: &str,
        artist_id: i64,
        album_id: i64,
        album_artist: &str,
        track_number: Option<u32>,
        duration_ms: u64,
        mtime: u64,
        cover: Option<&[u8]>,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT INTO tracks (path, title, artist_id, album_id, track_number, duration_ms, mtime, cover_data)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(path) DO UPDATE SET
               title=excluded.title, artist_id=excluded.artist_id, album_id=excluded.album_id,
               track_number=excluded.track_number, duration_ms=excluded.duration_ms,
               mtime=excluded.mtime, cover_data=excluded.cover_data",
            params![path, title, artist_id, album_id, track_number, duration_ms as i64, mtime as i64, cover],
        )?;
        Ok(())
    }

    /// Remove do banco todas as faixas sob `root` cujos paths não estão em `seen`.
    /// Depois limpa álbuns e artistas que ficaram órfãos.
    pub fn remove_missing_tracks(&self, root: &Path, seen: &HashSet<String>) -> Result<usize> {
        let prefix = format!("{}/", root.to_string_lossy().trim_end_matches('/'));

        let mut stmt = self.conn.prepare(
            "SELECT path FROM tracks WHERE path LIKE ?1 || '%'"
        )?;
        let db_paths: Vec<String> = stmt.query_map(params![prefix], |r| r.get(0))?
            .collect::<rusqlite::Result<_>>()?;

        let mut removed = 0;
        for path in &db_paths {
            if !seen.contains(path) {
                self.conn.execute("DELETE FROM tracks WHERE path = ?1", params![path])?;
                removed += 1;
            }
        }

        if removed > 0 {
            // Limpa álbuns sem faixas
            self.conn.execute_batch(
                "DELETE FROM albums  WHERE id NOT IN (SELECT DISTINCT album_id  FROM tracks);
                 DELETE FROM artists WHERE id NOT IN (SELECT DISTINCT artist_id FROM tracks)
                                       AND id NOT IN (SELECT DISTINCT artist_id FROM albums);",
            )?;
        }

        Ok(removed)
    }

    pub fn track_id_by_path(&self, path: &str) -> Option<i64> {
        self.conn.query_row(
            "SELECT id FROM tracks WHERE path = ?1",
            params![path],
            |r| r.get(0),
        ).ok()
    }

    pub fn track_mtime(&self, path: &str) -> Option<u64> {
        self.conn.query_row(
            "SELECT mtime FROM tracks WHERE path = ?1",
            params![path],
            |r| r.get::<_, i64>(0),
        ).ok().map(|v| v as u64)
    }

    pub fn tracks_in_album(&self, album_id: i64) -> Result<Vec<Track>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.id, t.path, t.title, ar.name, al.title, t.album_id,
                    t.track_number, t.duration_ms, t.cover_data
             FROM tracks t
             JOIN artists ar ON ar.id = t.artist_id
             JOIN albums  al ON al.id = t.album_id
             WHERE t.album_id = ?1
             ORDER BY t.track_number, t.title COLLATE NOCASE",
        )?;
        let rows = stmt.query_map(params![album_id], |r| {
            Ok(Track {
                id: r.get(0)?,
                path: PathBuf::from(r.get::<_, String>(1)?),
                title: r.get(2)?,
                artist: r.get(3)?,
                album: r.get(4)?,
                album_id: r.get(5)?,
                track_number: r.get(6)?,
                duration: Duration::from_millis(r.get::<_, i64>(7)? as u64),
                cover_data: r.get(8)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    pub fn tracks_in_folder(&self, folder_path: &str) -> Result<Vec<Track>> {
        let pattern = format!("{}/", folder_path.trim_end_matches('/'));
        let mut stmt = self.conn.prepare(
            "SELECT t.id, t.path, t.title, ar.name, al.title, t.album_id,
                    t.track_number, t.duration_ms, t.cover_data
             FROM tracks t
             JOIN artists ar ON ar.id = t.artist_id
             JOIN albums  al ON al.id = t.album_id
             WHERE t.path LIKE ?1 || '%'
             ORDER BY al.title COLLATE NOCASE, t.track_number, t.title COLLATE NOCASE",
        )?;
        let rows = stmt.query_map(params![pattern], |r| {
            Ok(Track {
                id: r.get(0)?,
                path: PathBuf::from(r.get::<_, String>(1)?),
                title: r.get(2)?,
                artist: r.get(3)?,
                album: r.get(4)?,
                album_id: r.get(5)?,
                track_number: r.get(6)?,
                duration: Duration::from_millis(r.get::<_, i64>(7)? as u64),
                cover_data: r.get(8)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    pub fn all_tracks(&self) -> Result<Vec<Track>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.id, t.path, t.title, ar.name, al.title, t.album_id,
                    t.track_number, t.duration_ms, t.cover_data
             FROM tracks t
             JOIN artists ar ON ar.id = t.artist_id
             JOIN albums  al ON al.id = t.album_id
             ORDER BY ar.name COLLATE NOCASE, al.title COLLATE NOCASE, t.track_number",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(Track {
                id: r.get(0)?,
                path: PathBuf::from(r.get::<_, String>(1)?),
                title: r.get(2)?,
                artist: r.get(3)?,
                album: r.get(4)?,
                album_id: r.get(5)?,
                track_number: r.get(6)?,
                duration: Duration::from_millis(r.get::<_, i64>(7)? as u64),
                cover_data: r.get(8)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    // ── Playlists ─────────────────────────────────────────────────────────────

    pub fn create_playlist(&self, name: &str) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO playlists (name) VALUES (?1)",
            params![name],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn all_playlists(&self) -> Result<Vec<Playlist>> {
        let mut stmt = self.conn.prepare(
            "SELECT p.id, p.name,
                    (SELECT COUNT(*) FROM playlist_tracks pt WHERE pt.playlist_id = p.id) AS cnt
             FROM playlists p
             ORDER BY p.name COLLATE NOCASE",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(Playlist {
                id: r.get(0)?,
                name: r.get(1)?,
                track_count: r.get::<_, i64>(2)? as usize,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    pub fn playlist_tracks(&self, playlist_id: i64) -> Result<Vec<Track>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.id, t.path, t.title, ar.name, al.title, t.album_id,
                    t.track_number, t.duration_ms, t.cover_data
             FROM playlist_tracks pt
             JOIN tracks  t  ON t.id  = pt.track_id
             JOIN artists ar ON ar.id = t.artist_id
             JOIN albums  al ON al.id = t.album_id
             WHERE pt.playlist_id = ?1
             ORDER BY pt.position",
        )?;
        let rows = stmt.query_map(params![playlist_id], |r| {
            Ok(Track {
                id: r.get(0)?,
                path: PathBuf::from(r.get::<_, String>(1)?),
                title: r.get(2)?,
                artist: r.get(3)?,
                album: r.get(4)?,
                album_id: r.get(5)?,
                track_number: r.get(6)?,
                duration: Duration::from_millis(r.get::<_, i64>(7)? as u64),
                cover_data: r.get(8)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    pub fn add_track_to_playlist(&self, playlist_id: i64, track_id: i64) -> Result<()> {
        let pos: i64 = self.conn.query_row(
            "SELECT COALESCE(MAX(position), 0) + 1 FROM playlist_tracks WHERE playlist_id = ?1",
            params![playlist_id],
            |r| r.get(0),
        )?;
        self.conn.execute(
            "INSERT OR IGNORE INTO playlist_tracks (playlist_id, track_id, position) VALUES (?1, ?2, ?3)",
            params![playlist_id, track_id, pos],
        )?;
        Ok(())
    }

    pub fn delete_playlist(&self, playlist_id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM playlists WHERE id = ?1", params![playlist_id])?;
        Ok(())
    }
}
