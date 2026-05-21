-- RGPD: suppression définitive d'un utilisateur et de ses données personnelles

-- entree_tracker: supprimer les entrées de l'utilisateur
ALTER TABLE entree_tracker DROP CONSTRAINT IF EXISTS entree_tracker_utilisateur_id_fkey;
ALTER TABLE entree_tracker ADD CONSTRAINT entree_tracker_utilisateur_id_fkey
    FOREIGN KEY (utilisateur_id) REFERENCES utilisateur(id) ON DELETE CASCADE;

-- page_information: dissocier l'auteur (garder le contenu)
ALTER TABLE page_information DROP CONSTRAINT IF EXISTS page_information_auteur_id_fkey;
ALTER TABLE page_information ALTER COLUMN auteur_id DROP NOT NULL;
ALTER TABLE page_information ADD CONSTRAINT page_information_auteur_id_fkey
    FOREIGN KEY (auteur_id) REFERENCES utilisateur(id) ON DELETE SET NULL;
