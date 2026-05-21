-- Rôles
INSERT INTO role (nom, description) VALUES
    ('utilisateur', 'Utilisateur standard de la plateforme'),
    ('administrateur', 'Administrateur de la plateforme')
ON CONFLICT DO NOTHING;

-- Émotions de base
INSERT INTO emotion_base (nom, emoji, couleur) VALUES
    ('Joie', '😊', '#FFD700'),
    ('Colère', '😠', '#FF4444'),
    ('Peur', '😨', '#6699CC'),
    ('Tristesse', '😢', '#808080'),
    ('Surprise', '😲', '#FFCC00'),
    ('Dégoût', '🤢', '#CC99CC')
ON CONFLICT DO NOTHING;

-- Sous-émotions : Joie (id=1)
INSERT INTO sous_emotion (nom, emotion_base_id) VALUES
    ('Fierté', 1), ('Contentement', 1), ('Enchantement', 1),
    ('Excitation', 1), ('Émerveillement', 1), ('Gratitude', 1)
ON CONFLICT DO NOTHING;

-- Sous-émotions : Colère (id=2)
INSERT INTO sous_emotion (nom, emotion_base_id) VALUES
    ('Frustration', 2), ('Irritation', 2), ('Rage', 2),
    ('Ressentiment', 2), ('Agacement', 2), ('Hostilité', 2)
ON CONFLICT DO NOTHING;

-- Sous-émotions : Peur (id=3)
INSERT INTO sous_emotion (nom, emotion_base_id) VALUES
    ('Inquiétude', 3), ('Anxiété', 3), ('Terreur', 3),
    ('Appréhension', 3), ('Panique', 3), ('Crainte', 3)
ON CONFLICT DO NOTHING;

-- Sous-émotions : Tristesse (id=4)
INSERT INTO sous_emotion (nom, emotion_base_id) VALUES
    ('Chagrin', 4), ('Mélancolie', 4), ('Abattement', 4),
    ('Désespoir', 4), ('Solitude', 4), ('Dépression', 4)
ON CONFLICT DO NOTHING;

-- Sous-émotions : Surprise (id=5)
INSERT INTO sous_emotion (nom, emotion_base_id) VALUES
    ('Étonnement', 5), ('Stupéfaction', 5), ('Sidération', 5),
    ('Incrédulité', 5), ('Émerveillement', 5), ('Confusion', 5)
ON CONFLICT DO NOTHING;

-- Sous-émotions : Dégoût (id=6)
INSERT INTO sous_emotion (nom, emotion_base_id) VALUES
    ('Répulsion', 6), ('Déplaisir', 6), ('Nausée', 6),
    ('Dédain', 6), ('Horreur', 6), ('Dégoût profond', 6)
ON CONFLICT DO NOTHING;

-- Catégories de contenu
INSERT INTO categorie_contenu (nom, description, ordre_affichage) VALUES
    ('Santé mentale', 'Articles sur la santé mentale et sa prévention', 1),
    ('Gestion du stress', 'Techniques et conseils pour gérer le stress', 2),
    ('Bien-être', 'Conseils bien-être au quotidien', 3)
ON CONFLICT DO NOTHING;

-- ============================================================
-- UTILISATEURS
-- Mot de passe pour tous : Admin123!
-- Hash argon2id généré via cargo test
-- ============================================================
INSERT INTO utilisateur (email, mot_de_passe_hash, nom, prenom, role_id, est_actif, date_creation) VALUES
    ('admin@cesizen.fr',
     '$argon2id$v=19$m=19456,t=2,p=1$RVb9JvTv67DBBJpQlkBofQ$2NfE9kTqWmWc1HXl2NCeusETm/ro+z+MYXDQq/mVUc0',
     'Admin', 'CESIZen', 2, TRUE,
     CURRENT_TIMESTAMP - INTERVAL '90 days'),

    ('jean.dupont@email.fr',
     '$argon2id$v=19$m=19456,t=2,p=1$RW4n1KIUsPRncYg+IEyN4A$VdYGdV97S1ApT6a1a2ZZSJN0zvP2KFy5kTbZ3GYByh0',
     'Dupont', 'Jean', 1, TRUE,
     CURRENT_TIMESTAMP - INTERVAL '60 days'),

    ('marie.leroy@email.fr',
     '$argon2id$v=19$m=19456,t=2,p=1$RW4n1KIUsPRncYg+IEyN4A$VdYGdV97S1ApT6a1a2ZZSJN0zvP2KFy5kTbZ3GYByh0',
     'Leroy', 'Marie', 1, TRUE,
     CURRENT_TIMESTAMP - INTERVAL '45 days'),

    ('pierre.martin@email.fr',
     '$argon2id$v=19$m=19456,t=2,p=1$RW4n1KIUsPRncYg+IEyN4A$VdYGdV97S1ApT6a1a2ZZSJN0zvP2KFy5kTbZ3GYByh0',
     'Martin', 'Pierre', 1, TRUE,
     CURRENT_TIMESTAMP - INTERVAL '30 days'),

    ('sophie.bernard@email.fr',
     '$argon2id$v=19$m=19456,t=2,p=1$RW4n1KIUsPRncYg+IEyN4A$VdYGdV97S1ApT6a1a2ZZSJN0zvP2KFy5kTbZ3GYByh0',
     'Bernard', 'Sophie', 1, TRUE,
     CURRENT_TIMESTAMP - INTERVAL '20 days'),

    ('lucas.petit@email.fr',
     '$argon2id$v=19$m=19456,t=2,p=1$RW4n1KIUsPRncYg+IEyN4A$VdYGdV97S1ApT6a1a2ZZSJN0zvP2KFy5kTbZ3GYByh0',
     'Petit', 'Lucas', 1, FALSE,
     CURRENT_TIMESTAMP - INTERVAL '15 days')
ON CONFLICT (email) DO NOTHING;
