-- Migrer le contenu des articles vers S3
-- titre et resume restent en DB comme cache pour le listing
ALTER TABLE page_information ADD COLUMN IF NOT EXISTS s3_key VARCHAR(500);
ALTER TABLE page_information DROP COLUMN IF EXISTS contenu;
