---

## 🧠 1. Nama Proyek (Ideas)

Kamu bisa pilih nama yang:

- Mengandung unsur “torrent / stream / seed / peer”
- Unik, tapi tetap mudah diingat
  Contoh ide nama:

| Gaya            | Ide Nama                                   | Arti Singkat                   |
| --------------- | ------------------------------------------ | ------------------------------ |
| ⚙️ Teknis       | `seedr`, `torrentd`, `rustransmit`         | menonjolkan sisi teknis        |
| 🧩 Rusty        | `rustorrent`, `rseed`, `rustyflow`         | menonjolkan Rust               |
| 🌀 Kreatif      | `Drip`, `Flowget`, `TorrentFox`, `BitPlan` | lebih bebas, branding-friendly |
| 🕒 Fokus jadwal | `SchedTorrent`, `LazySeed`, `Planr`        | menekankan fitur scheduling    |

> 💡 Rekomendasi saya: **`Rustorrent`** (gabungan _Rust_ + _Torrent_), simpel dan mudah diingat.

---

## 🧩 2. Arsitektur Umum Aplikasi

Karena kamu ingin fitur lengkap (cari, download, jadwal, lihat file), maka desainnya bisa dibagi jadi **3 layer utama**:

```
┌────────────────────────────┐
│         UI Layer            │ ← bisa web (Yew / Leptos) atau TUI (ratatui)
└─────────────┬──────────────┘
              │
┌─────────────┴──────────────┐
│     Application Layer       │ ← logic: scheduler, search, queue
└─────────────┬──────────────┘
              │
┌─────────────┴──────────────┐
│   Transmission RPC Layer    │ ← pakai crate `transmission-rpc`
│   + Torrent Search API      │ ← pakai API index (mis. 1337x.to, RARBG proxy, dll)
└────────────────────────────┘
```

---

## ⚙️ 3. Komponen & Fungsionalitas

| Fitur                                  | Implementasi di Rust                                                                                 |
| -------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| 🔍 **Cari torrent**                    | Gunakan API publik (contoh: `1337x`, `PirateBay`, `Nyaa`) dengan crate seperti `reqwest` + `scraper` |
| 🧭 **Download / kontrol Transmission** | Gunakan crate [`transmission-rpc`](https://crates.io/crates/transmission-rpc)                        |
| 🕒 **Jadwal download**                 | Gunakan crate seperti `cron` atau `tokio::time` untuk scheduling tugas                               |
| 📂 **Lihat file dalam torrent**        | Gunakan Transmission RPC (`torrent-get` field: `files`)                                              |
| 💾 **Konfigurasi user (settings)**     | Simpan di `config.toml` atau `~/.rustorrent/config.json`                                             |
| 🧰 **UI**                              | 2 opsi:                                                                                              |
|                                        | - **CLI/TUI:** pakai [`ratatui`](https://crates.io/crates/ratatui)                                   |
|                                        | - **Web UI:** backend `axum` / `warp`, frontend `Yew` atau `Leptos`                                  |

---

## 🧱 4. Implementasi Bertahap (Roadmap)

| Tahap    | Fitur Utama             | Deskripsi                                               |
| -------- | ----------------------- | ------------------------------------------------------- |
| **v0.1** | Transmission RPC client | Integrasi dasar: list torrent, add, remove, start, stop |
| **v0.2** | Torrent file viewer     | Lihat isi file dari torrent                             |
| **v0.3** | Torrent search          | Gunakan API publik untuk mencari torrent                |
| **v0.4** | Scheduler               | Tambahkan cron-like job untuk download otomatis         |
| **v0.5** | UI                      | Buat tampilan CLI atau Web sederhana                    |
| **v1.0** | Release                 | Tambahkan config, logging, dan error handling matang    |

---

## 🔧 5. Contoh Struktur Proyek

```
rustorrent/
├─ Cargo.toml
├─ src/
│  ├─ main.rs
│  ├─ config.rs
│  ├─ scheduler.rs
│  ├─ transmission.rs
│  ├─ search.rs
│  ├─ ui/
│  │   ├─ mod.rs
│  │   ├─ cli.rs
│  │   └─ web.rs
│  └─ utils.rs
└─ config/
   └─ default.toml
```

---

## 📡 6. Contoh Kode Awal

```rust
use transmission_rpc::TransClient;
use transmission_rpc::types::{TorrentGetField, TorrentGetArgs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TransClient::new("http://localhost:9091/transmission/rpc", "user", "pass");

    // ambil daftar torrent
    let args = TorrentGetArgs {
        fields: vec![TorrentGetField::Id, TorrentGetField::Name, TorrentGetField::PercentDone],
        ..Default::default()
    };

    let torrents = client.torrent_get(Some(args)).await?;
    for torrent in torrents.arguments.torrents {
        println!("{} - {:.2}%", torrent.name, torrent.percent_done.unwrap_or(0.0) * 100.0);
    }

    Ok(())
}
```

---

## 🔒 7. Potensi Pengembangan Lanjut

- Autentikasi user (JWT) jika versi web
- Integrasi search API dengan caching
- Scheduler berbasis prioritas
- Telegram bot untuk kontrol jarak jauh 😄

---
