-- ============================================================
-- ARTICLES + TRACKER : insertion uniquement si vide
-- Requiert le schéma post-005 (page_information.s3_key)
-- ============================================================
DO $$
BEGIN
IF (SELECT COUNT(*) FROM page_information) > 0 THEN
  RAISE NOTICE 'Articles already seeded, skipping.';
  RETURN;
END IF;

INSERT INTO page_information (titre, resume, categorie_id, est_publiee, auteur_id, s3_key, date_creation) VALUES
    ('Comprendre le stress',
     'Guide complet sur les mécanismes du stress et comment le gérer au quotidien.',
     2, TRUE, 1, 'public/articles/article-1.json',
     CURRENT_TIMESTAMP - INTERVAL '80 days'),

    ('Exercices de respiration pour se détendre',
     'Techniques de respiration pour réduire le stress et retrouver le calme.',
     2, TRUE, 1, 'public/articles/article-2.json',
     CURRENT_TIMESTAMP - INTERVAL '70 days'),

    ('Qu''est-ce que la santé mentale ?',
     'Introduction à la santé mentale : définition, piliers et conseils.',
     1, TRUE, 1, 'public/articles/article-3.json',
     CURRENT_TIMESTAMP - INTERVAL '60 days'),

    ('Les bienfaits de la méditation',
     'Découvrez comment la méditation peut améliorer votre bien-être mental.',
     3, TRUE, 1, 'public/articles/article-4.json',
     CURRENT_TIMESTAMP - INTERVAL '40 days'),

    ('L''échelle de Holmes et Rahe',
     'Outil scientifique pour évaluer votre niveau de stress selon les événements de vie.',
     1, TRUE, 1, 'public/articles/article-5.json',
     CURRENT_TIMESTAMP - INTERVAL '25 days'),

    ('5 habitudes pour un meilleur sommeil',
     'Conseils pratiques pour améliorer la qualité de votre sommeil et votre santé mentale.',
     3, TRUE, 1, 'public/articles/article-6.json',
     CURRENT_TIMESTAMP - INTERVAL '10 days');

-- ============================================================
-- ENTRÉES TRACKER
-- Données variées sur les 30 derniers jours pour les users 2-5
-- sous_emotion_id : 1-6 = Joie, 7-12 = Colère, 13-18 = Peur,
--                   19-24 = Tristesse, 25-30 = Surprise, 31-36 = Dégoût
-- ============================================================

-- === Jean Dupont (id=2) — profil plutôt joyeux, stress professionnel ===
INSERT INTO entree_tracker (utilisateur_id, sous_emotion_id, intensite, note, date_entree) VALUES
    (2, 6,  8, 'Promotion au travail, très content !', CURRENT_TIMESTAMP - INTERVAL '28 days' + INTERVAL '9 hours'),
    (2, 1,  7, 'Fier de ma présentation', CURRENT_TIMESTAMP - INTERVAL '27 days' + INTERVAL '14 hours'),
    (2, 14, 5, 'Réunion importante demain', CURRENT_TIMESTAMP - INTERVAL '26 days' + INTERVAL '21 hours'),
    (2, 7,  6, 'Embouteillages ce matin', CURRENT_TIMESTAMP - INTERVAL '25 days' + INTERVAL '8 hours'),
    (2, 2,  7, 'Weekend en famille, très agréable', CURRENT_TIMESTAMP - INTERVAL '24 days' + INTERVAL '18 hours'),
    (2, 4,  8, 'Concert génial !', CURRENT_TIMESTAMP - INTERVAL '22 days' + INTERVAL '22 hours'),
    (2, 11, 4, 'Bug en production, un peu agacé', CURRENT_TIMESTAMP - INTERVAL '21 days' + INTERVAL '10 hours'),
    (2, 6,  9, 'Mon fils a eu son examen', CURRENT_TIMESTAMP - INTERVAL '20 days' + INTERVAL '17 hours'),
    (2, 13, 3, 'Petit stress avant le dentiste', CURRENT_TIMESTAMP - INTERVAL '19 days' + INTERVAL '9 hours'),
    (2, 2,  6, 'Bon déjeuner entre collègues', CURRENT_TIMESTAMP - INTERVAL '18 days' + INTERVAL '13 hours'),
    (2, 20, 5, 'Mélancolie en écoutant de la musique', CURRENT_TIMESTAMP - INTERVAL '17 days' + INTERVAL '20 hours'),
    (2, 3,  8, 'Surprise anniversaire !', CURRENT_TIMESTAMP - INTERVAL '15 days' + INTERVAL '19 hours'),
    (2, 1,  7, 'Projet livré avec succès', CURRENT_TIMESTAMP - INTERVAL '14 days' + INTERVAL '16 hours'),
    (2, 14, 6, 'Anxieux pour les résultats médicaux', CURRENT_TIMESTAMP - INTERVAL '12 days' + INTERVAL '10 hours'),
    (2, 6,  8, 'Résultats OK, soulagement !', CURRENT_TIMESTAMP - INTERVAL '11 days' + INTERVAL '15 hours'),
    (2, 7,  5, 'Frustré par les transports', CURRENT_TIMESTAMP - INTERVAL '10 days' + INTERVAL '8 hours'),
    (2, 2,  7, 'Bonne soirée avec des amis', CURRENT_TIMESTAMP - INTERVAL '8 days' + INTERVAL '21 hours'),
    (2, 4,  9, 'Randonnée magnifique en montagne', CURRENT_TIMESTAMP - INTERVAL '7 days' + INTERVAL '14 hours'),
    (2, 25, 6, 'Appel inattendu d''un vieil ami', CURRENT_TIMESTAMP - INTERVAL '5 days' + INTERVAL '11 hours'),
    (2, 1,  7, 'Revue positive de mon manager', CURRENT_TIMESTAMP - INTERVAL '3 days' + INTERVAL '16 hours'),
    (2, 2,  8, 'Dimanche en famille', CURRENT_TIMESTAMP - INTERVAL '2 days' + INTERVAL '15 hours'),
    (2, 6,  7, 'Grateful pour cette semaine', CURRENT_TIMESTAMP - INTERVAL '1 day' + INTERVAL '20 hours');

-- === Marie Leroy (id=3) — profil équilibré, gère anxiété ===
INSERT INTO entree_tracker (utilisateur_id, sous_emotion_id, intensite, note, date_entree) VALUES
    (3, 14, 7, 'Examen vendredi, je stresse', CURRENT_TIMESTAMP - INTERVAL '30 days' + INTERVAL '22 hours'),
    (3, 16, 4, 'Petit souci avec le propriétaire', CURRENT_TIMESTAMP - INTERVAL '29 days' + INTERVAL '10 hours'),
    (3, 2,  6, 'Yoga ce matin, ça fait du bien', CURRENT_TIMESTAMP - INTERVAL '28 days' + INTERVAL '8 hours'),
    (3, 21, 5, 'Nostalgie en regardant de vieilles photos', CURRENT_TIMESTAMP - INTERVAL '27 days' + INTERVAL '20 hours'),
    (3, 6,  8, 'Examen réussi !!', CURRENT_TIMESTAMP - INTERVAL '25 days' + INTERVAL '15 hours'),
    (3, 4,  7, 'Soirée karaoké avec les copines', CURRENT_TIMESTAMP - INTERVAL '23 days' + INTERVAL '22 hours'),
    (3, 14, 6, 'Rendez-vous médecin demain', CURRENT_TIMESTAMP - INTERVAL '22 days' + INTERVAL '21 hours'),
    (3, 2,  5, 'Lecture au parc, moment calme', CURRENT_TIMESTAMP - INTERVAL '21 days' + INTERVAL '15 hours'),
    (3, 8,  4, 'Voisin bruyant encore...', CURRENT_TIMESTAMP - INTERVAL '20 days' + INTERVAL '23 hours'),
    (3, 1,  7, 'Fière de mon mémoire avancé', CURRENT_TIMESTAMP - INTERVAL '18 days' + INTERVAL '16 hours'),
    (3, 25, 8, 'Lettre d''acceptation master !', CURRENT_TIMESTAMP - INTERVAL '16 days' + INTERVAL '12 hours'),
    (3, 6,  9, 'Tellement reconnaissante', CURRENT_TIMESTAMP - INTERVAL '15 days' + INTERVAL '14 hours'),
    (3, 14, 5, 'Inquiète pour le déménagement', CURRENT_TIMESTAMP - INTERVAL '13 days' + INTERVAL '19 hours'),
    (3, 2,  6, 'Cartons faits, ça avance', CURRENT_TIMESTAMP - INTERVAL '11 days' + INTERVAL '11 hours'),
    (3, 7,  5, 'Erreur dans le contrat de bail', CURRENT_TIMESTAMP - INTERVAL '10 days' + INTERVAL '9 hours'),
    (3, 3,  8, 'Découverte d''un super restaurant', CURRENT_TIMESTAMP - INTERVAL '8 days' + INTERVAL '20 hours'),
    (3, 21, 4, 'Un peu seule ce soir', CURRENT_TIMESTAMP - INTERVAL '6 days' + INTERVAL '22 hours'),
    (3, 2,  7, 'Brunch avec ma sœur', CURRENT_TIMESTAMP - INTERVAL '5 days' + INTERVAL '11 hours'),
    (3, 14, 4, 'Légère anxiété, respiration faite', CURRENT_TIMESTAMP - INTERVAL '3 days' + INTERVAL '21 hours'),
    (3, 1,  8, 'Soutenance validée !', CURRENT_TIMESTAMP - INTERVAL '2 days' + INTERVAL '14 hours'),
    (3, 6,  9, 'Moment de gratitude profonde', CURRENT_TIMESTAMP - INTERVAL '1 day' + INTERVAL '7 hours');

-- === Pierre Martin (id=4) — profil stressé, travaille beaucoup ===
INSERT INTO entree_tracker (utilisateur_id, sous_emotion_id, intensite, note, date_entree) VALUES
    (4, 14, 7, 'Deadline lundi, beaucoup de travail', CURRENT_TIMESTAMP - INTERVAL '28 days' + INTERVAL '23 hours'),
    (4, 7,  8, 'Client mécontent, pas ma faute', CURRENT_TIMESTAMP - INTERVAL '27 days' + INTERVAL '17 hours'),
    (4, 19, 6, 'Pas vu ma famille depuis 2 semaines', CURRENT_TIMESTAMP - INTERVAL '26 days' + INTERVAL '20 hours'),
    (4, 14, 8, 'Insomnie, trop de stress', CURRENT_TIMESTAMP - INTERVAL '25 days' + INTERVAL '2 hours'),
    (4, 2,  5, 'Pause café avec un collègue sympa', CURRENT_TIMESTAMP - INTERVAL '24 days' + INTERVAL '10 hours'),
    (4, 7,  7, 'Encore des heures supp...', CURRENT_TIMESTAMP - INTERVAL '22 days' + INTERVAL '19 hours'),
    (4, 21, 6, 'Solitude du dimanche', CURRENT_TIMESTAMP - INTERVAL '21 days' + INTERVAL '16 hours'),
    (4, 16, 5, 'Appréhension pour la réunion DG', CURRENT_TIMESTAMP - INTERVAL '20 days' + INTERVAL '7 hours'),
    (4, 1,  6, 'Réunion bien passée finalement', CURRENT_TIMESTAMP - INTERVAL '20 days' + INTERVAL '18 hours'),
    (4, 14, 7, 'Trop de mails à traiter', CURRENT_TIMESTAMP - INTERVAL '18 days' + INTERVAL '9 hours'),
    (4, 11, 5, 'Collègue qui ne fait pas sa part', CURRENT_TIMESTAMP - INTERVAL '17 days' + INTERVAL '14 hours'),
    (4, 4,  7, 'Weekend à la campagne, ça fait du bien', CURRENT_TIMESTAMP - INTERVAL '15 days' + INTERVAL '12 hours'),
    (4, 2,  6, 'Sport ce matin, je me sens mieux', CURRENT_TIMESTAMP - INTERVAL '14 days' + INTERVAL '8 hours'),
    (4, 7,  6, 'Frustration avec le nouveau logiciel', CURRENT_TIMESTAMP - INTERVAL '12 days' + INTERVAL '11 hours'),
    (4, 14, 8, 'Peur de l''entretien annuel', CURRENT_TIMESTAMP - INTERVAL '10 days' + INTERVAL '22 hours'),
    (4, 6,  7, 'Entretien positif ! Augmentation !', CURRENT_TIMESTAMP - INTERVAL '9 days' + INTERVAL '17 hours'),
    (4, 25, 7, 'Surprise de mes collègues pour mon anniversaire', CURRENT_TIMESTAMP - INTERVAL '7 days' + INTERVAL '12 hours'),
    (4, 2,  6, 'Dîner avec ma compagne', CURRENT_TIMESTAMP - INTERVAL '5 days' + INTERVAL '20 hours'),
    (4, 14, 5, 'Nouveau projet qui commence, un peu stressé', CURRENT_TIMESTAMP - INTERVAL '3 days' + INTERVAL '9 hours'),
    (4, 2,  7, 'Cinéma en amoureux', CURRENT_TIMESTAMP - INTERVAL '1 day' + INTERVAL '21 hours');

-- === Sophie Bernard (id=5) — profil positif, active socialement ===
INSERT INTO entree_tracker (utilisateur_id, sous_emotion_id, intensite, note, date_entree) VALUES
    (5, 4,  8, 'Cours de danse incroyable !', CURRENT_TIMESTAMP - INTERVAL '25 days' + INTERVAL '19 hours'),
    (5, 2,  7, 'Brunch avec les amies', CURRENT_TIMESTAMP - INTERVAL '24 days' + INTERVAL '11 hours'),
    (5, 6,  8, 'Mon chat m''a fait un câlin', CURRENT_TIMESTAMP - INTERVAL '23 days' + INTERVAL '18 hours'),
    (5, 25, 9, 'Promotion inattendue !', CURRENT_TIMESTAMP - INTERVAL '22 days' + INTERVAL '16 hours'),
    (5, 1,  8, 'Fière de mon équipe', CURRENT_TIMESTAMP - INTERVAL '20 days' + INTERVAL '17 hours'),
    (5, 8,  3, 'Petit agacement, vite oublié', CURRENT_TIMESTAMP - INTERVAL '19 days' + INTERVAL '12 hours'),
    (5, 2,  7, 'Marche en forêt', CURRENT_TIMESTAMP - INTERVAL '18 days' + INTERVAL '10 hours'),
    (5, 4,  8, 'Atelier cuisine entre amis', CURRENT_TIMESTAMP - INTERVAL '16 days' + INTERVAL '19 hours'),
    (5, 6,  7, 'Reconnaissante pour mes proches', CURRENT_TIMESTAMP - INTERVAL '15 days' + INTERVAL '21 hours'),
    (5, 14, 4, 'Légère inquiétude pour un ami malade', CURRENT_TIMESTAMP - INTERVAL '13 days' + INTERVAL '20 hours'),
    (5, 2,  8, 'Mon ami va mieux !', CURRENT_TIMESTAMP - INTERVAL '12 days' + INTERVAL '14 hours'),
    (5, 3,  7, 'Nouveau hobby : aquarelle', CURRENT_TIMESTAMP - INTERVAL '10 days' + INTERVAL '15 hours'),
    (5, 20, 3, 'Petite nostalgie de l''été', CURRENT_TIMESTAMP - INTERVAL '8 days' + INTERVAL '18 hours'),
    (5, 2,  8, 'Soirée jeux de société', CURRENT_TIMESTAMP - INTERVAL '7 days' + INTERVAL '21 hours'),
    (5, 4,  9, 'Voyage surprise organisé par mon copain', CURRENT_TIMESTAMP - INTERVAL '5 days' + INTERVAL '17 hours'),
    (5, 6,  8, 'Coucher de soleil magnifique', CURRENT_TIMESTAMP - INTERVAL '4 days' + INTERVAL '20 hours'),
    (5, 1,  7, 'Objectifs du mois atteints', CURRENT_TIMESTAMP - INTERVAL '2 days' + INTERVAL '16 hours'),
    (5, 2,  8, 'Dimanche parfait', CURRENT_TIMESTAMP - INTERVAL '1 day' + INTERVAL '15 hours');

END;
$$;
