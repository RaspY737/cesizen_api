-- Seed de démo : variantes de comptes désactivés pour présenter les garde-fous
-- (auto-suppression, dernier admin, délai de rétention 30 jours).

-- 1. lucas.petit : désactivé depuis > 30 jours → éligible à la suppression définitive
UPDATE utilisateur
SET date_desactivation = CURRENT_TIMESTAMP - INTERVAL '45 days'
WHERE email = 'lucas.petit@email.fr';

-- 2. Nouveau user désactivé récemment → suppression bloquée, message "dans 20 jours"
INSERT INTO utilisateur (email, mot_de_passe_hash, nom, prenom, role_id, est_actif, date_creation, date_desactivation) VALUES
    ('claire.demo@email.fr',
     '$argon2id$v=19$m=19456,t=2,p=1$RW4n1KIUsPRncYg+IEyN4A$VdYGdV97S1ApT6a1a2ZZSJN0zvP2KFy5kTbZ3GYByh0',
     'Demo', 'Claire', 1, FALSE,
     CURRENT_TIMESTAMP - INTERVAL '40 days',
     CURRENT_TIMESTAMP - INTERVAL '10 days')
ON CONFLICT (email) DO NOTHING;

-- 3. Second admin actif → permet de manipuler admin@cesizen.fr sans déclencher la règle "dernier admin"
INSERT INTO utilisateur (email, mot_de_passe_hash, nom, prenom, role_id, est_actif, date_creation) VALUES
    ('admin2@cesizen.fr',
     '$argon2id$v=19$m=19456,t=2,p=1$RVb9JvTv67DBBJpQlkBofQ$2NfE9kTqWmWc1HXl2NCeusETm/ro+z+MYXDQq/mVUc0',
     'Admin', 'Secondaire', 2, TRUE,
     CURRENT_TIMESTAMP - INTERVAL '30 days')
ON CONFLICT (email) DO NOTHING;
