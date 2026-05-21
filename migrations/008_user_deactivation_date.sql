-- Date de désactivation : base du délai de rétention avant suppression définitive
ALTER TABLE utilisateur ADD COLUMN date_desactivation TIMESTAMP NULL;

-- Backfill : pour les comptes déjà désactivés, on prend date_modification comme approximation
UPDATE utilisateur SET date_desactivation = date_modification WHERE est_actif = FALSE;
