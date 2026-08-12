CREATE DATABASE tempors;

CREATE SCHEMA IF NOT EXISTS tempors;

CREATE TYPE tempors.event_run_status AS ENUM (
    'pending',
    'running',
    'completed',
    'failed'
);

CREATE TABLE tempors.functions (
    id TEXT PRIMARY KEY,
    payload JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE TABLE tempors.events (
    id TEXT PRIMARY KEY,
    func_id TEXT NOT NULL REFERENCES tempors.functions(id) ON DELETE CASCADE,
    payload JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE TABLE tempors.event_runs (
    id UUID PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES tempors.events(id) ON DELETE CASCADE,
    payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    status tempors.event_run_status NOT NULL DEFAULT 'pending'
);

CREATE INDEX idx_events_func_id
    ON tempors.events(func_id);

CREATE INDEX idx_event_runs_event_id
    ON tempors.event_runs(event_id);

CREATE INDEX idx_event_runs_status
    ON tempors.event_runs(status);