CREATE TABLE dc.documents (
    id UUID NOT NULL DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    data JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT documents_pk PRIMARY KEY (id)
);

CREATE INDEX documents_user_id ON dc.documents (user_id);

CREATE OR REPLACE FUNCTION dc.update_updated_at()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER documents_updated_at
    BEFORE UPDATE ON dc.documents
    FOR EACH ROW
    EXECUTE PROCEDURE dc.update_updated_at();