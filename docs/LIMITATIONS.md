# GrafiTalk — Known limitations



Honest constraints of **GrafiTalk Core RC**. Use when setting expectations or planning the next milestone.



---



## Draft generation



| Limitation | Detail |

|------------|--------|

| Deterministic only | No AI paraphrasing; output follows fixed rules and section templates |

| Pattern coverage | Compact-note rewrite handles known patterns; generic prose is structured, not fully rewritten |

| Five fixed templates | User picks from registry; no template editor |

| Empty context | Generate blocked until context is present |

| No fact inference | Generator must not invent work, bugs, or dates not in context |



---



## Sending and integrations



| Limitation | Detail |

|------------|--------|

| No auto-send | Copy-only outbound path |

| No email/chat plugins | User pastes into external tools manually |

| Graf-ID export UI | Lives in separate Graf-ID repo; GrafiTalk provides contract + import only |

| No live sync | Import is manual file pick, not background sync |



---



## Data and collaboration



| Limitation | Detail |

|------------|--------|

| No cloud sync | Data stays on one machine |

| No multi-user | Single-user local database |

| No draft history | Only latest draft per project |

| Fast quit edge case | Debounced save (500 ms) may not flush if process is killed instantly |



---



## Product UI



| Limitation | Detail |

|------------|--------|

| No archive UI | **Resolved in Core RC** — archive, restore, and rename in sidebar |

| No settings screen | Minimal read-aloud preference in diagnostics modal |

| No template editor | Template logic is Rust code |

| Client label | API supports `client_label`; create form uses name only |

| Grafi advisor (in-workbench) | Upstream Grafi UI via portal host at bottom-left |



---



## Platform and quality



| Limitation | Detail |

|------------|--------|

| Primary target | Windows desktop (Tauri 2) |

| No frontend E2E tests | Vitest unit tests + manual QA checklist |

| No installer signing | `scripts/release.ps1` builds unsigned bundle; SmartScreen may warn |

| Corrupt DB recovery | Basic errors surface as strings; no guided repair wizard |



---



## What is intentionally stable



- Three-panel workbench layout

- Copy-only outbound behavior

- Local SQLite as source of truth

- One draft per project

- English UI text



---



## Next milestone candidates

1. Permanent project delete UI

2. Windows code signing / store pipeline

3. AI / Mentor Mode (explicit future milestone — not Pre-AI scope)



See [Core status](CORE_STATUS.md) for release gate and sequencing.


