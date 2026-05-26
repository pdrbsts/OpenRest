-- Phase 1 tables
CREATE TABLE IF NOT EXISTS families (
    id TEXT PRIMARY KEY,
    code INTEGER NOT NULL,
    name TEXT NOT NULL
);

-- Modify articles (this is for dev, we can just recreate or add columns if needed, but since we are early dev we'll just drop and recreate or use alter if needed. Wait, SQLite alter is limited. Let's recreate since it's dev).
DROP TABLE IF EXISTS articles;
CREATE TABLE IF NOT EXISTS articles (
    id TEXT PRIMARY KEY,
    family_id TEXT,
    code INTEGER NOT NULL,
    name TEXT NOT NULL,
    price INTEGER NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS tables (
    id TEXT PRIMARY KEY,
    code INTEGER NOT NULL,
    name TEXT,
    is_open BOOLEAN NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS employees (
    id TEXT PRIMARY KEY,
    code INTEGER NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS payment_methods (
    id TEXT PRIMARY KEY,
    code INTEGER NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS documents (
    id TEXT PRIMARY KEY,
    table_id TEXT,
    employee_id TEXT,
    total INTEGER NOT NULL DEFAULT 0,
    is_closed BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS document_details (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    article_id TEXT NOT NULL,
    qty INTEGER NOT NULL DEFAULT 1,
    unit_price INTEGER NOT NULL,
    total INTEGER NOT NULL
);
