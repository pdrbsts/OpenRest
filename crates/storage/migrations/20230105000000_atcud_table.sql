-- ATCUDs atribuídos pela AT (entidade autónoma, ligada à série).
CREATE TABLE IF NOT EXISTS atcud (
    id TEXT PRIMARY KEY,
    document_type TEXT NOT NULL,
    series_prefix TEXT NOT NULL,
    year INTEGER NOT NULL,
    atcud TEXT NOT NULL,
    start_date DATE NOT NULL,
    registered_at DATETIME NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1
);
CREATE INDEX IF NOT EXISTS idx_atcud_lookup
    ON atcud(document_type, series_prefix, year, is_active);

-- Pequena coluna de manutenção pode ficar mesmo que não seja lida;
-- remoção exige SQLite >= 3.35. Tornamo-la nullable e deprecada.
ALTER TABLE document_series DROP COLUMN atcud_validation;
