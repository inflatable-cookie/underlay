-- Acme baseline: example domain tables.
--
-- This demonstrates a simple task list as an example domain model.
-- Replace these tables with your own domain entities.

CREATE SCHEMA IF NOT EXISTS acme;

-- =========================================
-- Projects (task containers)
-- =========================================

CREATE TABLE IF NOT EXISTS acme.projects (
    id UUID PRIMARY KEY,
    owner_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'archived')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_acme_projects_owner
    ON acme.projects (owner_id, status);

-- =========================================
-- Tasks
-- =========================================

CREATE TABLE IF NOT EXISTS acme.tasks (
    id UUID PRIMARY KEY,
    project_id UUID NOT NULL REFERENCES acme.projects(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'in_progress', 'completed', 'cancelled')),
    priority TEXT NOT NULL DEFAULT 'medium' CHECK (priority IN ('low', 'medium', 'high', 'urgent')),
    due_date DATE,
    completed_at TIMESTAMPTZ,
    position INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_acme_tasks_project
    ON acme.tasks (project_id, status, position);

CREATE INDEX IF NOT EXISTS idx_acme_tasks_due_date
    ON acme.tasks (due_date)
    WHERE status NOT IN ('completed', 'cancelled');

-- =========================================
-- Task comments
-- =========================================

CREATE TABLE IF NOT EXISTS acme.task_comments (
    id UUID PRIMARY KEY,
    task_id UUID NOT NULL REFERENCES acme.tasks(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_acme_task_comments_task
    ON acme.task_comments (task_id, created_at);

-- =========================================
-- Tags (for organizing tasks)
-- =========================================

CREATE TABLE IF NOT EXISTS acme.tags (
    id UUID PRIMARY KEY,
    project_id UUID NOT NULL REFERENCES acme.projects(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    color TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT tags_unique_name_per_project UNIQUE (project_id, name)
);

CREATE TABLE IF NOT EXISTS acme.task_tags (
    task_id UUID NOT NULL REFERENCES acme.tasks(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES acme.tags(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, tag_id)
);
