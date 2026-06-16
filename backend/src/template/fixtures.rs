//! Shared regression fixtures for draft generation (PR 0 baseline).
//!
//! Tests should reference these constants instead of duplicating strings.

pub const UNSTRUCTURED_PROSE: &str = "QA handoff complete.";

pub const NARRATIVE_WITH_FILES: &str =
    "Mobile app for home developmental routines.\n\nFiles:\n\n* app/(tabs)/index.tsx";

pub const COMPACT_DEPLOYMENT_HU: &str =
    "Fullstack webshop Deploy előtt áll Barion kulcsra vár hátralévő időp ~1nap";

pub const GRAF_ID_SAMPLE_JSON: &str =
    include_str!("../../../examples/graf_id_project_context.sample.json");

pub const FEJLESZTESI_NAPLO: &str = "\
Project:\n\
Mesencsi webshop\n\
\n\
Current status:\n\
Fejlesztési napló frissítve, Barion integráció tesztelve.\n\
\n\
What changed:\n\
- Checkout flow refaktorálva.\n\
- Barion sandbox flow tested.\n\
- src/components/Checkout.tsx módosítva.\n\
\n\
Current blocker:\n\
- Barion production key / merchant approval.\n\
\n\
Next step:\n\
- Deploy once Barion credentials are available.\n\
\n\
Files:\n\
- src/components/Checkout.tsx\n\
- backend/src/payments/barion.rs";

pub const GRAF_ID_LABELED_TEXT: &str = "\
Project:\n\
Mesencsi webshop\n\
\n\
Current status:\n\
Deployment is nearly ready.\n\
\n\
What changed:\n\
- Manual QA completed.\n\
- Barion sandbox flow tested.\n\
- Production deployment is waiting for Barion production credentials.\n\
\n\
Current blocker:\n\
- Barion production key / merchant approval.\n\
\n\
Next step:\n\
- Deploy once Barion credentials are available.\n\
\n\
Estimated time:\n\
Approximately 1 day after credentials are received.\n\
\n\
Notes:\n\
This context is for draft generation only. GrafiTalk must not send messages automatically.";
