CREATE TABLE IF NOT EXISTS document (
    id              SERIAL PRIMARY KEY,
    nom             VARCHAR(255) NOT NULL,
    nom_fichier     VARCHAR(255) NOT NULL,
    taille          BIGINT NOT NULL,
    content_type    VARCHAR(100) NOT NULL,
    s3_key          VARCHAR(500) NOT NULL UNIQUE,
    uploaded_by     INTEGER REFERENCES utilisateur(id) ON DELETE SET NULL,
    date_creation   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_document_uploaded_by ON document(uploaded_by);
