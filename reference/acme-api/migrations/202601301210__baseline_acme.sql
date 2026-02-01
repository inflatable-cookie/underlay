-- Acme baseline: core app tables.
--
-- App-owned tables live under `acme`.

CREATE SCHEMA IF NOT EXISTS acme;

-- =========================================
-- User grants (fine-grained roles)
-- =========================================

CREATE TABLE IF NOT EXISTS acme.user_grants (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    scope_type TEXT NOT NULL CHECK (scope_type IN ('platform', 'business', 'person')),
    scope_id UUID NULL,
    role TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT user_grants_unique UNIQUE (user_id, scope_type, scope_id, role)
);

-- =========================================
-- Businesses
-- =========================================

CREATE TABLE IF NOT EXISTS acme.businesses (
    id UUID PRIMARY KEY,
    display_name TEXT NOT NULL,
    slug TEXT NOT NULL,
    description TEXT,
    website_url TEXT,
    claim_status TEXT NOT NULL DEFAULT 'unclaimed'
        CHECK (claim_status IN ('unclaimed', 'pending', 'claimed')),
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'inactive')),
    created_from TEXT NOT NULL DEFAULT 'compliment_flow'
        CHECK (created_from IN ('compliment_flow', 'signup', 'import')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT businesses_slug_unique UNIQUE (slug)
);

CREATE INDEX IF NOT EXISTS idx_acme_businesses_display_name
    ON acme.businesses (display_name);

-- =========================================
-- People
-- =========================================

CREATE TABLE IF NOT EXISTS acme.people (
    id UUID PRIMARY KEY,
    display_name TEXT NOT NULL,
    slug TEXT NOT NULL,
    bio TEXT,
    avatar_url TEXT,
    claim_status TEXT NOT NULL DEFAULT 'unclaimed'
        CHECK (claim_status IN ('unclaimed', 'pending', 'claimed')),
    publish_status TEXT NOT NULL DEFAULT 'private'
        CHECK (publish_status IN ('private', 'published')),
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'inactive')),
    created_from TEXT NOT NULL DEFAULT 'compliment_flow'
        CHECK (created_from IN ('compliment_flow', 'signup', 'invitation', 'import')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT people_slug_unique UNIQUE (slug)
);

CREATE INDEX IF NOT EXISTS idx_acme_people_display_name
    ON acme.people (display_name);

-- =========================================
-- Contact points
-- =========================================

CREATE TABLE IF NOT EXISTS acme.contact_points (
    id UUID PRIMARY KEY,
    entity_type TEXT NOT NULL CHECK (entity_type IN ('business', 'person')),
    entity_id UUID NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('email', 'phone', 'url_form')),
    value TEXT NOT NULL,
    value_normalized TEXT NOT NULL,
    label TEXT,
    source TEXT NOT NULL DEFAULT 'user_entered'
        CHECK (source IN ('user_entered', 'ai_suggested', 'discovered', 'claimed', 'import')),
    verification_status TEXT NOT NULL DEFAULT 'unverified'
        CHECK (verification_status IN ('unverified', 'pending', 'verified', 'bounced', 'suppressed')),
    verified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_acme_contact_points_entity
    ON acme.contact_points (entity_type, entity_id);

CREATE INDEX IF NOT EXISTS idx_acme_contact_points_value
    ON acme.contact_points (kind, value_normalized);

-- =========================================
-- Memberships (person works for business)
-- =========================================

CREATE TABLE IF NOT EXISTS acme.business_memberships (
    id UUID PRIMARY KEY,
    business_id UUID NOT NULL REFERENCES acme.businesses(id) ON DELETE CASCADE,
    person_id UUID NOT NULL REFERENCES acme.people(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'verified', 'inactive')),
    role_title TEXT,
    started_at TIMESTAMPTZ,
    ended_at TIMESTAMPTZ,
    verified_by_user_id UUID NULL REFERENCES auth.users(id),
    verified_at TIMESTAMPTZ,
    verification_method TEXT
        CHECK (verification_method IS NULL OR verification_method IN ('business_admin', 'work_email', 'manual_review')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_acme_memberships_business
    ON acme.business_memberships (business_id);

CREATE INDEX IF NOT EXISTS idx_acme_memberships_person
    ON acme.business_memberships (person_id);

-- =========================================
-- Entity claims
-- =========================================

CREATE TABLE IF NOT EXISTS acme.entity_claims (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    entity_type TEXT NOT NULL CHECK (entity_type IN ('business', 'person')),
    entity_id UUID NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('pending', 'approved', 'rejected', 'revoked')),
    method TEXT NOT NULL CHECK (method IN ('domain', 'email', 'invitation', 'manual_review')),
    evidence JSONB NOT NULL DEFAULT '{}',
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMPTZ,
    resolved_by_user_id UUID NULL REFERENCES auth.users(id)
);

CREATE INDEX IF NOT EXISTS idx_acme_entity_claims_entity
    ON acme.entity_claims (entity_type, entity_id);

CREATE INDEX IF NOT EXISTS idx_acme_entity_claims_user_status
    ON acme.entity_claims (user_id, status);

-- =========================================
-- Compliments
-- =========================================

CREATE TABLE IF NOT EXISTS acme.compliments (
    id UUID PRIMARY KEY,
    author_user_id UUID NULL REFERENCES auth.users(id),
    author_contact TEXT,
    business_id UUID NULL REFERENCES acme.businesses(id),
    person_id UUID NULL REFERENCES acme.people(id),
    membership_id UUID NULL REFERENCES acme.business_memberships(id),
    subject TEXT,
    body TEXT NOT NULL,
    body_format TEXT NOT NULL DEFAULT 'plain' CHECK (body_format IN ('plain', 'markdown')),
    allow_contact BOOLEAN NOT NULL DEFAULT FALSE,
    contact_details TEXT,
    publishable BOOLEAN NOT NULL DEFAULT FALSE,
    visibility_scope TEXT NOT NULL DEFAULT 'inbox' CHECK (visibility_scope IN ('inbox', 'public', 'both')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT compliments_has_target CHECK (business_id IS NOT NULL OR person_id IS NOT NULL)
);

CREATE INDEX IF NOT EXISTS idx_acme_compliments_business
    ON acme.compliments (business_id, created_at DESC)
    WHERE business_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_acme_compliments_person
    ON acme.compliments (person_id, created_at DESC)
    WHERE person_id IS NOT NULL;

-- =========================================
-- Delivery attempts
-- =========================================

CREATE TABLE IF NOT EXISTS acme.delivery_attempts (
    id UUID PRIMARY KEY,
    compliment_id UUID NOT NULL REFERENCES acme.compliments(id) ON DELETE CASCADE,
    to_contact_point_id UUID NULL REFERENCES acme.contact_points(id),
    channel TEXT NOT NULL CHECK (channel IN ('email', 'url_form', 'internal')),
    status TEXT NOT NULL CHECK (status IN ('queued', 'sent', 'failed', 'suppressed')),
    provider TEXT,
    provider_message_id TEXT,
    error_code TEXT,
    error_message TEXT,
    attempted_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_acme_delivery_attempts_compliment
    ON acme.delivery_attempts (compliment_id, attempted_at DESC);

CREATE INDEX IF NOT EXISTS idx_acme_delivery_attempts_status
    ON acme.delivery_attempts (status, attempted_at);

-- =========================================
-- Merge workflow
-- =========================================

CREATE TABLE IF NOT EXISTS acme.merge_candidates (
    id UUID PRIMARY KEY,
    entity_type TEXT NOT NULL CHECK (entity_type IN ('business', 'person')),
    primary_entity_id UUID NOT NULL,
    candidate_entity_id UUID NOT NULL,
    confidence NUMERIC NOT NULL,
    reasons JSONB NOT NULL DEFAULT '[]',
    status TEXT NOT NULL CHECK (status IN ('open', 'merged', 'rejected')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMPTZ,
    resolved_by_user_id UUID NULL REFERENCES auth.users(id),
    CONSTRAINT merge_candidates_unique UNIQUE (entity_type, primary_entity_id, candidate_entity_id)
);

CREATE TABLE IF NOT EXISTS acme.merge_events (
    id UUID PRIMARY KEY,
    entity_type TEXT NOT NULL CHECK (entity_type IN ('business', 'person')),
    from_entity_id UUID NOT NULL,
    to_entity_id UUID NOT NULL,
    performed_by_user_id UUID NULL REFERENCES auth.users(id),
    data JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
